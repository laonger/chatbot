
use std::{
    fmt::format,
    error,
    result,
    env,
};

use http::status::StatusCode;

use hyper::{body::Buf, header, Body, Client, Request, Error};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};

use crate::cache;


#[derive(Deserialize, Debug, Clone)]
struct ResponseMessageUnit {
    message:cache::ContentUnit,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<ResponseMessageUnit>,
}

#[derive(Deserialize, Debug, Clone)]
struct ResponseErrorContent {
    message:String,
}

#[derive(Deserialize, Debug)]
struct OpenAIErrorResponse {
    error: ResponseErrorContent,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIRequest {
    model: String,
    messages: Vec<cache::ContentUnit>,
}

pub type OError = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> 
    = std::result::Result<T, OError>;

pub async fn get(messages: Vec<cache::ContentUnit>) -> Result<String> {

    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/chat/completions";

    let model = String::from("gpt-3.5-turbo");
    let stop = String::from("\n");

    let mut api_key = String::new();

    match env::var("OPENAI_API_KEY") {
        Ok(x) => {
            api_key = x;
        },
        Err(e) => {
            println!("Need OPENAI_API_KEY");
            return Ok("".to_string());
        }
    };
    let auth_header_val = format!("Bearer {}", api_key);

    let openai_request = OpenAIRequest {
        model,
        messages,
    };

    let body = Body::from(serde_json::to_string(&openai_request)?);

    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)?;

    let res = client.request(req).await?;
    match res.status() {
        StatusCode::OK => {
            let body = hyper::body::aggregate(res).await?;
            let json: OpenAIResponse = serde_json::from_reader(body.reader())?;
            match json.choices[0].clone() {
                ResponseMessageUnit{message:cache::ContentUnit::assistant(x)} => {
                    Ok(x)
                },
                ResponseMessageUnit{message:cache::ContentUnit::user(x)} => {
                    Ok(format!("Human: {}", x))
                },
                ResponseMessageUnit{message:cache::ContentUnit::system(x)} => {
                    Ok(format!("System: {}", x))
                }
            }
        },
        StatusCode::BAD_REQUEST => {
            let body = hyper::body::aggregate(res).await?;
            let error: OpenAIErrorResponse = serde_json::from_reader(body.reader())?;
            Ok(error.error.message)
        },
        _ => {
            eprintln!("Error res: {:?}", res);
            let body = hyper::body::aggregate(res).await?;
            let error: OpenAIErrorResponse = serde_json::from_reader(body.reader())?;
            Ok(error.error.message)
        }
    }


}

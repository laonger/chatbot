
use std::{
    fmt::format,
    str::FromStr,
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

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIImageRequest {
    prompt: String,
    n: i32,
    size: String
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIImageResponseData {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIImageResponse {
    created: i64,
    data: Vec<OpenAIImageResponseData>
}

pub type OError = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> 
    = std::result::Result<T, OError>;

pub async fn ask(messages: Vec<cache::ContentUnit>) -> Result<String> {

    //let mut re:Vec<String> = Vec::new();
    //for i in messages.clone() {
    //    match i {
    //        cache::ContentUnit::user(s) => {
    //            re.push(s)
    //        },
    //        _ => {
    //        }
    //    }
    //};
    //return Ok(re.join(""));


    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/chat/completions";

    let model = String::from("gpt-3.5-turbo");
    //let stop = String::from("\n");

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

pub async fn draw(prompt: String, n: i32, size: String) -> Result<String> {

    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/images/generations";

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

    let s:&str = &size;

    match s {
        "1024x1024" => {},
        "512x512" => {},
        "256x256" => {},
        _ => {
            return Ok("size only support: 1024x1024, 512x512, 256x256".to_string())
        }
    }

    let openai_request = OpenAIImageRequest {
        prompt,
        n,
        size
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
            let json: OpenAIImageResponse
                = serde_json::from_reader(body.reader())?;
            let mut result: Vec<String> = Vec::new();
            for i in json.data {
                result.push(i.url)
            }
            return Ok(result.join("\n"))
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

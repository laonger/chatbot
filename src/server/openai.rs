
use std::{
    fmt::format,
    error,
    result,
    env,
};

use hyper::{body::Buf, header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};

use crate::cache;


//#[derive(Deserialize, Debug)]
//struct OpenAIChoices {
//    text: String,
//}
//
#[derive(Deserialize, Debug, Clone)]
struct ResponseMessageUnit {
    message:cache::ContentUnit,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<ResponseMessageUnit>,
}

//#[derive(Serialize, Deserialize, Debug)]
//struct OpenAIMessage {
//    role: String,
//    content: String
//}

#[derive(Serialize, Deserialize, Debug)]
struct OpenAIRequest {
    model: String,
    messages: Vec<cache::ContentUnit>,
    //prompt: String,
    max_tokens: u32,
    //stop: String,
}


pub type Result<T> 
    = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn get(messages: Vec<cache::ContentUnit>) -> Result<String> {

    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/chat/completions";

    //let model = String::from("text-davinci-003");
    let model = String::from("gpt-3.5-turbo");
    let stop = String::from("\n");

    //let prompt = format!("The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly. {prompt}\nAI:");

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
        //prompt,
        max_tokens: 4000,
        //stop,
    };

    //let body = Body::from(serde_json::to_vec(&openai_request)?);
    let body = Body::from(serde_json::to_string(&openai_request)?);

    println!("openai request body: {body:?}");

    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)?;

    println!("openai request: {req:?}");

    let res = client.request(req).await?;

    println!("openai response: {res:?}");
    let body = hyper::body::aggregate(res).await?;

    let json: OpenAIResponse = match serde_json::from_reader(body.reader()) {
        Ok(response) => response,
        Err(e) => {
            println!("Error: {:?}", e);
            std::process::exit(1);
        }
    };
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
    //Ok(
    //    json.message[0]
    //        .text
    //        .split('\n')
    //        .map(|s| s.trim())
    //        .filter(|s| !s.is_empty())
    //        .collect::<Vec<_>>()
    //        .join("\n")
    //)

}


use std::fmt::format;
use std::error;
use std::result;

use hyper::{body::Buf, header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};

const API_KEY:&str = "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";

#[derive(Deserialize, Debug)]
struct OpenAIChoices {
    text: String,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoices>,
}

#[derive(Serialize, Debug)]
struct OpenAIRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
    stop: String,
}


pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn get(prompt: String) -> Result<String> {

    println!("Prompt: {prompt}");
    
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/completions";

    let model = String::from("text-davinci-003");
    let stop = String::from("Text");

    let prompt = format!("The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly. {prompt}\nAI:");

    let auth_header_val = format!("Bearer {}", API_KEY);

    let openai_request = OpenAIRequest {
        model,
        prompt,
        max_tokens: 64,
        stop,
    };

    let body = Body::from(serde_json::to_vec(&openai_request)?);

    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)
        .unwrap();

    let res = client.request(req).await?;

    let body = hyper::body::aggregate(res).await?;

    let json: OpenAIResponse = match serde_json::from_reader(body.reader()) {
        Ok(response) => response,
        Err(_) => {
            println!("Error calling OpenAI. Check environment variable OPENAI_KEY");
            std::process::exit(1);
        }
    };
    Ok(
        json.choices[0]
            .text
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    )

}

// This file should define a function that takes a `message::Message` as
// a parameter and sends a POST Request  to a webhook URL argument.

use crate::{hidden, message::Message};
use reqwest::Client;
use serde::Deserialize;
// use serde_json::to_string_pretty;
use std::sync::Arc;

pub async fn send(url: String, msg: Arc<Message>) -> Status {
    // hidden!("send() started!");

    // You'd be surprised to hear how many times I uncommented this line
    // and pasted its output to https://discohook.com to figure out what
    // was wrong.
    // println!("{}", to_string_pretty(&*msg).unwrap());

    let client = Client::new();
    let req = client.post(url.to_owned()).json(&*msg).send().await;

    if let Ok(res) = req {
        hidden!("Sent webhook to {}!, Status: {}", url, res.status());

        let status = res.status().as_u16();

        if status == 204 | 200 {
            Status::Success
        } else if status == 201 {
            Status::Invalid
        } else if status == 429 {
            if let Ok(info) = res.json::<RateLimit>().await {
                Status::RateLimit(Some(info.retry_after))
            } else {
                Status::RateLimit(None)
            }

        // For some reason the compiler requires me to include this clause
        } else {
            Status::Unknown
        };
    } else if let Err(e) = req {
        hidden!("Error sending webhook to {}: {}", url, e);
    };

    Status::Unknown
}

#[derive(PartialEq, Debug)]
pub enum Status {
    Success,
    Invalid,
    RateLimit(Option<f64>),
    Unknown,
}

#[derive(Deserialize)]
pub struct RateLimit {
    pub message: String,
    pub retry_after: f64,
    pub global: bool,
}

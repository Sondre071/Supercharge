use reqwest;
use serde::{Deserialize, Serialize};

use crate::data;
use crate::menu;

pub fn run() {
    let data = data::get_app_data();

    let mut message_history: Vec<Message> = vec![];

    menu::write_headers("New chat", Some(&vec![&data.model, ""]));

    loop {
        let message = menu::read_line("You: ");
        println!();

        message_history.push(Message {
            role: "user",
            content: message.clone(),
        });

        let res = send_message(&message_history);
        println!("\x1b[0;96m{}\x1b[0m\n", res);

        message_history.push(Message {
            role: "assistant",
            content: res.clone(),
        });
    }
}

#[tokio::main]
async fn send_message(messages: &Vec<Message>) -> String {
    let data = data::get_app_data();

    let body = RequestBody {
        model: &data.model,
        input: messages.clone(),
    };

    let res = reqwest::Client::new()
        .post("https://openrouter.ai/api/v1/responses")
        .json(&body)
        .bearer_auth(&data.api_key)
        .send()
        .await
        .expect("Request failed.")
        .error_for_status()
        .expect("Non success status from API.");

    let content: Response = res.json().await.expect("Failed to read response body.");

    content.output[0].content[0].text.clone()
}

#[derive(Serialize, Clone)]
struct Message<'a> {
    role: &'a str,
    content: String,
}

#[derive(Serialize)]
struct RequestBody<'a> {
    model: &'a str,
    input: Vec<Message<'a>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ResponseMessage {
    r#type: String,
    text: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Output {
    id: String,
    role: String,
    r#type: String,
    status: String,
    content: Vec<ResponseMessage>,
}

#[derive(Debug, Deserialize)]
struct Response {
    output: Vec<Output>,
}

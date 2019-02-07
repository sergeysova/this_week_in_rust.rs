use reqwest;
use reqwest::header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::Url;
use serde::Serialize;
use serde_json;

type ReqwestResult = reqwest::Result<reqwest::Response>;

const URL: &'static str = "https://api.telegram.org/bot";

#[derive(Debug)]
pub struct Bot {
    token: String,
}

impl Bot {
    pub fn new(token: String) -> Self {
        Bot { token }
    }

    pub fn send(&self, method: impl Into<String>, body: impl Serialize) -> ReqwestResult {
        let method: String = method.into();
        let url = format!(
            "{url}{token}/{method}",
            url = URL,
            token = self.token,
            method = method,
        );

        let href = Url::parse(url.as_str()).unwrap();
        let req = reqwest::Client::new().post(href);
        let content = serde_json::to_string(&body).unwrap();
        let content_len = content.len();

        // Because, req.json(&body) sends noise instead of JSON
        req.body(content)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(CONTENT_LENGTH, HeaderValue::from(content_len))
            .send()
    }

    pub fn send_message(&self, chat_id: String, text: String) -> ReqwestResult {
        let body = SendMessage {
            chat_id,
            text,
            parse_mode: "HTML".to_string(),
            disable_web_page_preview: true,
        };

        self.send("sendMessage", &body)
    }

    pub fn forward_message(
        &self,
        from_chat_id: String,
        chat_id: String,
        message_id: usize,
    ) -> ReqwestResult {
        let body = ForwardMessage {
            from_chat_id,
            chat_id,
            message_id,
        };

        self.send("sendMessage", &body)
    }

    pub fn response_id(mut res: reqwest::Response) -> Option<usize> {
        res.text()
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .map(|msg: Message| msg.message_id)
    }
}

#[derive(Debug, Deserialize)]
struct Message {
    message_id: usize,
    // the rest fields are unused and might be extended based on:
    // https://core.telegram.org/bots/api#message
}

#[derive(Debug, Serialize)]
struct SendMessage {
    chat_id: String,
    text: String,
    parse_mode: String,
    disable_web_page_preview: bool,
}

#[derive(Debug, Serialize)]
struct ForwardMessage {
    chat_id: String,
    from_chat_id: String,
    message_id: usize,
}

impl Default for SendMessage {
    fn default() -> Self {
        SendMessage {
            chat_id: "".to_string(),
            text: "".to_string(),
            parse_mode: "HTML".to_string(),
            disable_web_page_preview: false,
        }
    }
}

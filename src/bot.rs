use reqwest;


const URL: &'static str = "https://api.telegram.org/bot";

pub fn request<T: AsRef<str>, M: AsRef<str>>(token: T, method: M) {
    let url = format!(
        "{url}{token}/{method}",
        url = URL,
        token = token.as_ref(),
        method = method.as_ref()
    );

    // reqwest::Client::new().post(url)
}

#[derive(Debug, Serialize)]
struct SendMessage {
    chat_id: String,
    text: String,
    parse_mode: String,
    disable_web_page_preview: bool,
}

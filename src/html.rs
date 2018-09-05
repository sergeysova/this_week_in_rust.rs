pub fn escape(text: String) -> String {
    text.replace("\"", "&quot;")
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

pub fn escape(text: String) -> String {
    text.replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

#[cfg(test)]
mod tests {
    use super::escape;

    #[test]
    fn escape_all() {
        assert_eq!(escape("foo-bar".to_string()), "foo-bar".to_string());
        assert_eq!(escape("foo\"bar".to_string()), "foo&quot;bar".to_string());
        assert_eq!(
            escape("<foo\"bar".to_string()),
            "&lt;foo&quot;bar".to_string()
        );
        assert_eq!(
            escape("<foo\"bar>".to_string()),
            "&lt;foo&quot;bar&gt;".to_string()
        );
        assert_eq!(
            escape("<&foo\"bar>".to_string()),
            "&lt;&amp;foo&quot;bar&gt;".to_string()
        );
    }
}

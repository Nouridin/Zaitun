pub mod sys {
    #[cfg(target_os = "windows")]
    pub fn line_ending() -> String {
        "\r\n".into()
    }

    #[cfg(not(target_os = "windows"))]
    pub fn line_ending() -> String {
        "\n".into()
    }
}
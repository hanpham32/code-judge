pub enum ModelProviderName {
    OPENAI,
    OPENROUTER,
    ANTHROPIC,
}

impl ModelProviderName {
    fn as_str(&self) -> &'static str {
        match self {
            ModelProviderName::OPENAI => "openai",
            ModelProviderName::OPENROUTER => "openrouter",
            ModelProviderName::ANTHROPIC => "anthropic",
        }
    }
}

use super::model::ModelProviderName;

struct Character {
    name: &'static str,
    model_provider: ModelProviderName,
    system_prompt: &'static str,
    bio: Bio,
    lore: Vec<&'static str>,
    topics: Vec<&'static str>,
    adjectives: Vec<&'static str>,
}

enum Bio {
    Single(&'static str),
    Multiple(Vec<&'static str>),
}

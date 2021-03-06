use crate::types::*;
use crate::{rules::RulesOptions, tokenizer::TokenizerOptions};
use lazy_static::lazy_static;

lazy_static! {
    static ref TOKENIZER_CONFIGS: DefaultHashMap<String, TokenizerOptions> = {
        serde_json::from_slice(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/",
            "tokenizer_configs.json"
        )))
        .expect("tokenizer configs must be valid JSON")
    };
}

lazy_static! {
    static ref RULES_CONFIGS: DefaultHashMap<String, RulesOptions> = {
        serde_json::from_slice(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/",
            "rules_configs.json"
        )))
        .expect("rules configs must be valid JSON")
    };
}

/// Gets the tokenizer language options for the language code
pub(crate) fn tokenizer_options(lang_code: &str) -> Option<&'static TokenizerOptions> {
    TOKENIZER_CONFIGS.get(lang_code)
}

/// Gets the rules language options for the language code
pub(crate) fn rules_options(lang_code: &str) -> Option<&'static RulesOptions> {
    RULES_CONFIGS.get(lang_code)
}

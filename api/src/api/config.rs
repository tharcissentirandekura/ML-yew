use std::env;

pub fn get_model_path() -> String {
    
    env::var("MODEL_DIR").unwrap_or_else(|_| "model".to_string());
    model_path
}
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

const MODEL_PATH: &str = "./llama-2-7b.Q4_K_M.gguf";

pub fn request(query: String) -> String {
    let model_options = ModelOptions::default();

    let llama = LLama::new(MODEL_PATH.into(), &model_options).unwrap();

    let predict_options = PredictOptions {
        token_callback: Some(Box::new(|token| {
            println!("token1: {}", token);

            true
        })),
        ..Default::default()
    };

    return llama.predict(query.into(), predict_options).unwrap();
}

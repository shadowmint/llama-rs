use llama_cpp_rs::{
    LContext, LContextConfig, LGenerator, LGeneratorParams, LSampleParams, LToken, LTokenSequence,
};
use std::io::Write;

pub fn main() {
    // Setup params
    let mut config = LContextConfig::new("../models/13B/model.bin");
    config.n_ctx = 512;
    config.seed = 0;

    // Load model
    let mut context = LContext::new(config).unwrap();

    // Run the generator
    let prompt =
        "bob is a space pilot. Alice is a potato. This is a conversation between bob and alice:";
    print!("{}", prompt);

    let mut generator = LGenerator::new(context);
    let output = generator
        .generate_incremental(
            prompt,
            LGeneratorParams {
                worker_thread_count: 8,
                prediction_window_length: 64,
                generate_tokens: 256,
                sample_params: LSampleParams {
                    top_k: 40,
                    top_p: 0.95f32,
                    temp: 0.8f32,
                    repeat_penalty: 1.1f32,
                },
            },
            |v| {
                print!("{}", v);
                std::io::stdout().flush().unwrap();
            },
        )
        .unwrap();

    println!("\n\nfull output:\n{}", output);
}

use callm::pipelines::PipelineText;
use callm::templates::MessageRole;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    model_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Build pipeline
    let mut pipeline = PipelineText::builder()
        .with_location(&cli.model_path)
        .with_temperature(0.1)
        .build()?;

    // Prepare conversation messages
    let messages = vec![
        (
            MessageRole::System,
            "You are impersonating Linus Torvalds.".to_string(),
        ),
        (
            MessageRole::User,
            "What is your opinion on Rust in Linux kernel development?".to_string(),
        ),
    ];

    // Run chat-style inference
    let assistant_response = pipeline.run_chat(&messages)?;
    println!("{assistant_response}");

    Ok(())
}

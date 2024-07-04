use callm::pipelines::PipelineText;
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

    // Build pipeline with custom sampling parameters
    let mut pipeline = PipelineText::builder()
        .with_location(&cli.model_path)
        .with_temperature(0.65)
        .with_top_k(25)
        .build()?;

    // Adjust sampling parameters later on
    pipeline.set_seed(42);
    pipeline.set_top_p(0.3);

    // Run inference
    let text_completion = pipeline.run("Write an article about Pentium F00F bug")?;
    println!("{text_completion}");

    Ok(())
}

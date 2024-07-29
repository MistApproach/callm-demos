#[allow(unused_imports)]
use callm::device::{Device, DeviceConfig};
use callm::pipelines::PipelineText;
use callm::templates::MessageRole;
use clap::Parser;
use std::io;
use std::io::prelude::*;

#[derive(Parser)]
struct Cli {
    model_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let cli = Cli::parse();

    // load pipeline
    let mut pipeline = PipelineText::builder()
        .with_location(&cli.model_path)
        // .with_device(DeviceConfig::new(Device::Metal(0)))
        .build()?;

    log::info!("Inference device: {:?}", pipeline.device().device());

    let mut messages = vec![];
    let mut running = true;
    while running {
        // read user input
        print!("You: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = String::from(input.trim());

        // update messages
        messages.push((MessageRole::User, input.clone()));

        // process
        if !input.is_empty() {
            let completion = pipeline.run_chat(&messages)?;
            println!("Assistant: {}", completion);
            messages.push((MessageRole::Assistant, completion));
        } else {
            running = false;
        }
    }

    Ok(())
}

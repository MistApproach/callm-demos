use anyhow::{Error, Result};
use callm::device::{Device, DeviceConfig};
use callm::pipelines::PipelineText;
use callm::templates::MessageRole;
use clap::Parser;
use std::io;
use std::io::prelude::*;
use std::thread;

#[derive(Parser)]
struct Cli {
    model_path: String,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let cli = Cli::parse();

    // load pipeline in a separate thread
    let loader_handle = thread::spawn(move || {
        let pipeline = PipelineText::builder()
            .with_location(&cli.model_path)
            .with_device(DeviceConfig::new(Device::Cuda(0)))
            .build()?;

        Ok::<PipelineText, Error>(pipeline)
    });

    let mut pipeline = loader_handle.join().unwrap().unwrap();

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

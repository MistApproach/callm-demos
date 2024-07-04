use callm::loaders::gguf::LoaderGguf;
use callm::loaders::LoaderImpl;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    gguf_filename: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let cli = Cli::parse();

    let mut loader = LoaderGguf::new(&cli.gguf_filename);
    let _model = loader.load()?;

    println!("Loaded");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}

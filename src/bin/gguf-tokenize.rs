use callm::loaders::gguf::LoaderGguf;
use callm::loaders::LoaderImpl;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    gguf_filename: String,
    #[arg(short, long)]
    add_special_tokens: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();

    let mut loader = LoaderGguf::new(&cli.gguf_filename);
    let _model = loader.load()?;
    let tokenizer = loader.tokenizer()?;
    let _template = loader.template()?;

    const TEXT_TO_TOKENIZE: &str = "This is the text to tokenize.";

    let tokens = tokenizer
        .encode(TEXT_TO_TOKENIZE, cli.add_special_tokens)
        .expect("Tokenizer encode failed");

    println!("{TEXT_TO_TOKENIZE}");
    println!("tokens: {:?}", tokens.get_tokens());
    println!("ids: {:?}", tokens.get_ids());
    println!("type_ids: {:?}", tokens.get_type_ids());
    println!("offsets: {:?}", tokens.get_offsets());
    println!("attention_mask: {:?}", tokens.get_attention_mask());
    println!(
        "special_tokens_mask: {:?}",
        tokens.get_special_tokens_mask()
    );
    println!("overflowing: {:?}", tokens.get_overflowing());

    Ok(())
}

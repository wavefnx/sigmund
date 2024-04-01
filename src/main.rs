use clap::Parser;
use sigmund::{config::Config, Sigmund};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build and parse the command-line interface arguments.
    let cli = Config::parse();
    // Sigmund 🗿
    let sigmund = Sigmund::from_config(cli);
    // Sigmund 🗿
    sigmund.execute().await?;

    Ok(())
}

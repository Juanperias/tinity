mod lexer;
mod binary;

use tracing::Level;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Cannot set default subscriber");

    info!("Welcome to tinity");
}

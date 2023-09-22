pub use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Add { name: Option<String> },
}

#[derive(Parser)]
#[command(name = "spotify-cli")]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
#[command(propagate_version = true)]
pub struct CoffeeCli {
    #[command(subcommand)]
    command: Commands,
}

pub fn start() {
    let cli = CoffeeCli::parse();

    match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {name:?}")
        }
    }
}

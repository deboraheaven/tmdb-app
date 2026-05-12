mod playing;
mod popular;
mod top;
mod upcoming;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "tmdb-app", version = "0.1.0", about = "TMDB CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Playing,
    Popular,
    Top,
    Upcoming,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Playing) => {
            println!("Now Playing");
            playing::fetch_movies().await?;
        }

        Some(Command::Popular) => {
            println!("Popular Movies");
            popular::fetch_movies().await?;
        }

        Some(Command::Top) => {
            println!("Top Rated Movies");
            top::fetch_movies().await?;
        }

        Some(Command::Upcoming) => {
            println!("Upcoming Movies");
            upcoming::fetch_movies().await?;
        }

        None => {
            println!("No command provided");
        }
    }

    Ok(())
}

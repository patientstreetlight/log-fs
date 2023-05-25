use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sets a value
    Set {
        key: String,
        value: String,
    },

    /// Gets a value
    Get {
        key: String,
    },

    /// Removes a value
    Rm {
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Set { .. } => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        Commands::Get { .. } => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        Commands::Rm { .. } => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
    }
}
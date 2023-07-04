use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sets a value
    Set { key: String, value: String },

    /// Gets a value
    Get { key: String },

    /// Removes a value
    Rm { key: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut store = KvStore::open(std::env::current_dir()?)?;
    match cli.command {
        Commands::Set { key, value } => {
            store.set(key, value)?;
        }
        Commands::Get { key } => {
            let v = store.get(key)?;
            let val = v.as_deref().unwrap_or("Key not found");
            println!("{val}");
        }
        Commands::Rm { key } => {
            store.remove(key)?;
        }
    }
    Ok(())
}

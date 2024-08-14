use clap::{Args, Parser, Subcommand};
use kvs::{KvsClient, Result};
#[derive(Parser)]
#[command(version, about = "A key-value store client")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Set a key-value pair")]
    Set(Set),
    #[clap(about = "Get a value by key")]
    Get(Get),
    #[clap(about = "Remove a key-value pair")]
    Rm(Rm),
}

#[derive(Args)]
struct Set {
    key: String,
    value: String,
    #[arg(long, value_name = "IP:PORT", default_value = "127.0.0.1:4000")]
    addr: String,
}

#[derive(Args)]
struct Get {
    key: String,
    #[arg(long, value_name = "IP:PORT", default_value = "127.0.0.1:4000")]
    addr: String,
}

#[derive(Args)]
struct Rm {
    key: String,
    #[arg(long, value_name = "IP:PORT", default_value = "127.0.0.1:4000")]
    addr: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::Set(set) => {
            let mut client = KvsClient::connect(set.addr)?;
            client.set(set.key, set.value)?;
        }
        Commands::Get(get) => {
            let mut client = KvsClient::connect(get.addr)?;
            if let Some(value) = client.get(get.key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Commands::Rm(rm) => {
            let mut client = KvsClient::connect(rm.addr)?;
            client.remove(rm.key)?;
        }
    }
    Ok(())
}

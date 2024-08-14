use std::{
    env::current_dir,
    fs,
    net::{SocketAddr, TcpListener},
    process::exit,
};

use clap::{Args, Parser, Subcommand};
use kvs::{KvStore, KvsEngine, KvsError, KvsServer, Result, SledKvsEngine};
use log::{error, info, warn};

#[derive(Parser)]
#[command(version, about = "A key-value store server")]
struct Cli {
    #[arg(long, value_name = "IP:PORT", default_value = "127.0.0.1:4000")]
    addr: SocketAddr,
    #[arg(long, value_name = "ENGINE-NAME")]
    engine: Option<String>,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let mut cli = Cli::parse();
    let res = current_engine().and_then(move |curr_engine| {
        if cli.engine.is_none() {
            cli.engine = curr_engine;
        } else if curr_engine.is_some() && cli.engine != curr_engine {
            error!("Wrong engine");
            exit(1);
        }
        run(cli)
    });
    if let Err(e) = res {
        error!("{}", e);
        exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let engine = cli.engine.unwrap_or("kvs".to_owned());
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {}", engine);
    info!("Listening on {}", cli.addr);
    fs::write(current_dir()?.join("engine"), &engine)?;
    match engine.as_str() {
        "kvs" => run_with_engine(KvStore::open(current_dir()?)?, cli.addr),
        "sled" => run_with_engine(SledKvsEngine::new(sled::open(current_dir()?)?), cli.addr),
        _ => {
            eprintln!("Engine must be either \"kvs\" or \"sled\"");
            std::process::exit(1);
        }
    }
}

fn run_with_engine<E: KvsEngine>(engine: E, addr: SocketAddr) -> Result<()> {
    let server = KvsServer::new(engine);
    server.run(addr)
}
fn current_engine() -> Result<Option<String>> {
    let engine = current_dir()?.join("engine");
    if !engine.exists() {
        return Ok(None);
    }
    match fs::read_to_string(engine)?.parse() {
        Ok(engine) => Ok(Some(engine)),
        Err(e) => {
            warn!("The content of engine file is invalid: {}", e);
            Ok(None)
        }
    }
}

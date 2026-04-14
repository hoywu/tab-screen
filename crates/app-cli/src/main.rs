use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use config::{default_config_toml, RawAppConfig};
use server_core::SessionManager;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};
use transport::{TransportServer, TransportServerConfig};

#[derive(Debug, Parser)]
#[command(name = "tab-screen")]
#[command(about = "Tab Screen server and tooling")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Serve(ServeCommand),
    Doctor,
    Probe,
    PrintDefaultConfig,
    Usb(UsbCommand),
    Version,
}

#[derive(Debug, Args)]
struct ServeCommand {
    #[arg(long)]
    config: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct UsbCommand {
    #[command(subcommand)]
    command: UsbSubcommand,
}

#[derive(Debug, Subcommand)]
enum UsbSubcommand {
    AdbReverse(AdbReverseCommand),
}

#[derive(Debug, Args)]
struct AdbReverseCommand {
    #[arg(long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing()?;

    let cli = Cli::parse();

    match cli.command {
        Command::Serve(command) => serve(command).await,
        Command::Doctor => doctor(),
        Command::Probe => probe(),
        Command::PrintDefaultConfig => print_default_config(),
        Command::Usb(command) => usb(command),
        Command::Version => version(),
    }
}

fn init_tracing() -> Result<()> {
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .with_target(false)
        .compact()
        .try_init()
        .map_err(|error| anyhow::anyhow!(error.to_string()))?;

    Ok(())
}

async fn serve(command: ServeCommand) -> Result<()> {
    let raw = RawAppConfig::default();
    let config = config::load_effective_config(raw)?;
    let server = TransportServer::new(TransportServerConfig {
        listen_host: config.server.listen_host.clone(),
        listen_port: config.server.listen_port,
        ..TransportServerConfig::default()
    });
    let session_manager = SessionManager::new();

    info!(
        event = "phase0_serve_placeholder",
        listen = %server.describe(),
        config_path = ?command.config,
        state = ?session_manager.status().state,
        "serve command placeholder ready"
    );

    println!("serve placeholder: {}", server.describe());
    Ok(())
}

fn doctor() -> Result<()> {
    println!("doctor placeholder: Phase 1 environment checks will be implemented here");
    Ok(())
}

fn probe() -> Result<()> {
    println!("probe placeholder: display and encoder capability probing will be implemented here");
    Ok(())
}

fn print_default_config() -> Result<()> {
    print!("{}", default_config_toml()?);
    Ok(())
}

fn usb(command: UsbCommand) -> Result<()> {
    match command.command {
        UsbSubcommand::AdbReverse(command) => {
            let port = command.port.unwrap_or(38491);
            println!(
                "usb adb-reverse placeholder: will run adb reverse tcp:{port} tcp:{port} in Phase 5"
            );
            Ok(())
        }
    }
}

fn version() -> Result<()> {
    println!("tab-screen {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

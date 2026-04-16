use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};
use config::{RawAppConfig, default_config_toml};
use display_backend::{DisplayBackend, EvdiDisplayBackend, VirtualDisplaySpec};
use server_core::SessionManager;
use std::path::{Path, PathBuf};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};
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
    Probe(ProbeCommand),
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
struct ProbeCommand {
    #[command(subcommand)]
    command: Option<ProbeSubcommand>,
}

#[derive(Debug, Subcommand)]
enum ProbeSubcommand {
    Display,
    ValidateDisplay(ValidateDisplayCommand),
}

#[derive(Debug, Args)]
struct ValidateDisplayCommand {
    #[arg(long, default_value = "Tab Screen Validation")]
    display_name: String,

    #[arg(long, default_value_t = 1920)]
    width: u32,

    #[arg(long, default_value_t = 1200)]
    height: u32,

    #[arg(long, default_value_t = 60)]
    refresh_rate: u16,

    #[arg(long, default_value_t = 8)]
    color_depth: u8,

    #[arg(long, default_value_t = 160)]
    dpi: u16,

    #[arg(long, default_value_t = 3000)]
    wait_ms: u64,

    #[arg(long)]
    capture: bool,
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
        Command::Probe(command) => probe(command),
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
        event = "serve_placeholder",
        listen = %server.describe(),
        config_path = ?command.config,
        state = ?session_manager.status().state,
        "serve command placeholder ready"
    );

    println!("serve placeholder: {}", server.describe());
    Ok(())
}

fn doctor() -> Result<()> {
    let backend = EvdiDisplayBackend;
    let probe = backend
        .probe()
        .context("failed to probe the evdi display backend")?;

    let mut failures = 0_u32;

    println!("Tab Screen Doctor");
    println!("=================");
    println!("Phase 1 focus: validate the selected evdi + libevdi display backend path.");
    println!();

    failures += print_check(
        "libevdi probe",
        true,
        Some(format!(
            "linked backend reported as '{}' with {} note(s)",
            probe.backend_name,
            probe.notes.len()
        )),
    );

    let module_loaded = path_exists("/sys/module/evdi");
    failures += print_check(
        "evdi kernel module",
        module_loaded,
        Some(if module_loaded {
            "/sys/module/evdi is present".to_owned()
        } else {
            "/sys/module/evdi is missing; load the module before Phase 1 validation".to_owned()
        }),
    );

    let dri_present = path_exists("/dev/dri");
    failures += print_check(
        "DRM device directory",
        dri_present,
        Some(if dri_present {
            "/dev/dri is present".to_owned()
        } else {
            "/dev/dri is missing".to_owned()
        }),
    );

    let virtual_drm_present = path_exists("/sys/devices/virtual/drm");
    print_check(
        "virtual DRM sysfs path (optional)",
        true,
        Some(if virtual_drm_present {
            "/sys/devices/virtual/drm is present".to_owned()
        } else {
            "/sys/devices/virtual/drm is missing; this path is informational and not required for Phase 1 validation".to_owned()
        }),
    );

    let modules_load_conf = path_exists("/etc/modules-load.d/evdi.conf");
    failures += print_check(
        "boot-time evdi auto-load config",
        modules_load_conf,
        Some(if modules_load_conf {
            "/etc/modules-load.d/evdi.conf is present".to_owned()
        } else {
            "create /etc/modules-load.d/evdi.conf containing 'evdi'".to_owned()
        }),
    );

    let euid_is_root = effective_uid_is_root();
    failures += print_check(
        "privileged execution",
        euid_is_root,
        Some(if euid_is_root {
            "running with effective root privileges".to_owned()
        } else {
            "not running as root; dynamic evdi node creation/opening commonly requires elevated privileges".to_owned()
        }),
    );

    println!();
    println!("Backend notes");
    println!("-------------");
    for note in &probe.notes {
        println!("- {note}");
    }

    println!();
    println!("Arch Linux prerequisites");
    println!("------------------------");
    println!("- Install `evdi-dkms`.");
    println!("- Install `linux-headers`.");
    println!(
        "- Create `/etc/modules-load.d/evdi.conf` with `evdi` to auto-load the module on boot."
    );

    println!();
    println!("Service model recommendation");
    println!("----------------------------");
    println!(
        "- Prefer a system-level privileged service model for the evdi backend path instead of `systemd --user`."
    );
    println!(
        "- Reason: dynamic evdi DRM node creation and management is tied to kernel-module-backed resources and commonly requires administrative privileges."
    );

    println!();
    if failures == 0 {
        println!("Doctor result: PASS");
        Ok(())
    } else {
        println!("Doctor result: FAIL ({failures} check(s) need attention)");
        bail!("doctor found environment issues that block or weaken Phase 1 validation")
    }
}

fn probe(command: ProbeCommand) -> Result<()> {
    match command.command {
        None | Some(ProbeSubcommand::Display) => probe_display(),
        Some(ProbeSubcommand::ValidateDisplay(command)) => validate_display(command),
    }
}

fn probe_display() -> Result<()> {
    let backend = EvdiDisplayBackend;
    let probe = backend
        .probe()
        .context("failed to probe the evdi display backend")?;

    println!("Tab Screen Display Backend Probe");
    println!("================================");
    println!("backend: {}", probe.backend_name);
    println!(
        "supports stable naming: {}",
        yes_no(probe.supports_stable_naming)
    );
    println!(
        "supports create/destroy: {}",
        yes_no(probe.supports_create_destroy)
    );
    println!("capture path: expected via display handle capture source");
    println!();

    println!("notes:");
    for note in probe.notes {
        println!("- {note}");
    }

    Ok(())
}

fn validate_display(command: ValidateDisplayCommand) -> Result<()> {
    if command.width == 0 || command.height == 0 {
        bail!("display dimensions must be non-zero");
    }
    if command.refresh_rate == 0 {
        bail!("refresh rate must be non-zero");
    }
    if command.color_depth < 8 {
        bail!("color depth must be at least 8-bit");
    }

    let backend = EvdiDisplayBackend;
    let spec = VirtualDisplaySpec {
        logical_name: command.display_name.clone(),
        width: command.width,
        height: command.height,
        refresh_rate: command.refresh_rate,
        color_depth: command.color_depth,
        dpi: command.dpi,
    };

    println!("Tab Screen Phase 1 Display Validation");
    println!("=====================================");
    println!("logical display name: {}", spec.logical_name);
    println!(
        "requested mode: {}x{} @ {} Hz, {}-bit, {} DPI",
        spec.width, spec.height, spec.refresh_rate, spec.color_depth, spec.dpi
    );
    println!("capture requested: {}", yes_no(command.capture));
    println!("wait window: {} ms", command.wait_ms);
    println!();

    println!("Step 1/4: probing backend");
    let probe = backend
        .probe()
        .context("failed to probe the evdi backend before validation")?;
    println!(
        "  result: backend='{}', create/destroy={}, stable_naming={}",
        probe.backend_name,
        yes_no(probe.supports_create_destroy),
        yes_no(probe.supports_stable_naming),
    );

    println!("Step 2/4: creating virtual display");
    let handle = backend
        .create_output(spec)
        .context("failed to create evdi virtual display")?;
    println!("  logical name: {}", handle.logical_name());
    println!("  backend id: {}", handle.backend_id());

    println!("Step 3/4: waiting for observation window");
    std::thread::sleep(std::time::Duration::from_millis(command.wait_ms));

    if command.capture {
        println!("Step 4/4: requesting one validation frame");
        let capture_result = {
            let mut source = handle
                .capture_source()
                .context("failed to obtain capture source from evdi display handle")?;
            let frame = source
                .next_frame()
                .context("failed to capture a validation frame from evdi")?;
            let format = frame.format;

            println!(
                "  captured frame: {} bytes, {}x{}, stride={}, depth={}bpp",
                frame.bytes.len(),
                format.width,
                format.height,
                format.stride,
                format.color_depth
            );

            if frame.bytes.is_empty() {
                bail!("validation capture returned an empty frame");
            }

            Ok::<(), anyhow::Error>(())
        };

        capture_result?;
    } else {
        println!("Step 4/4: capture skipped by request");
    }

    println!("Cleanup: destroying virtual display");
    handle
        .destroy()
        .context("failed to destroy evdi virtual display cleanly")?;

    println!();
    println!("Validation result: PASS");
    println!(
        "Run this command repeatedly with the same `--display-name` to validate stable logical identity reuse."
    );
    Ok(())
}

fn print_check(name: &str, passed: bool, detail: Option<String>) -> u32 {
    let status = if passed { "PASS" } else { "FAIL" };
    println!("[{status}] {name}");
    if let Some(detail) = detail {
        println!("       {detail}");
    }

    if passed { 0 } else { 1 }
}

fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn effective_uid_is_root() -> bool {
    std::env::var("EUID").ok().as_deref() == Some("0")
        || std::env::var("USER").ok().as_deref() == Some("root")
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
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

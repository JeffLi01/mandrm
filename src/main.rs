use clap::{Parser, Subcommand};
use log::{error, info, warn, LevelFilter};
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start Inspur DRM
    Start,
    /// Stop Inspur DRM
    Stop,
}

fn start_inspur_drm() {
    let exe = r"C:\Program Files (x86)\InspurDRM\DrmAgent\Bin64\CoSvcPrt.exe";
    info!("Starting: {}", exe);
    Command::new(exe)
        .spawn()
        .expect("Failed to execute command");
    thread::sleep(Duration::from_secs(10));
    let exe = r"C:\Program Files (x86)\InspurDRM\DrmAgent\Bin64\AgtMnUi.exe";
    info!("Starting: {}", exe);
    Command::new(exe)
        .spawn()
        .expect("Failed to execute command");
}

fn kill_inspur_drm() {
    for p in processes::all().unwrap() {
        let path = p.path().unwrap();
        if path.to_str().unwrap().contains("InspurDRM") {
            warn!("Stopping {:10} - {:?}", p.pid().unwrap(), p.path().unwrap());
            if let Err(err) = p.kill() {
                error!("{}", err);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2.. => LevelFilter::Trace,
    };

    env_logger::Builder::from_default_env()
        .filter(None, level)
        .format_module_path(false)
        .format_target(false)
        .init();

    match cli.command {
        Commands::Start => start_inspur_drm(),
        Commands::Stop => kill_inspur_drm(),
    };
}

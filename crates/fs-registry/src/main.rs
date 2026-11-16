//! `fs-registry` daemon. Usage: `fs-registry [--socket PATH] [--data DIR]`, `fs-registry --version`.

use fs_registry::{Config, run_blocking};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let mut config = Config::default_locations();
    let mut socket = None;
    let mut data = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--socket" => socket = args.next(),
            "--data" => data = args.next(),
            "--version" | "-V" => {
                println!("fs-registry {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "-h" | "--help" => {
                println!(
                    "fs-registry [--socket PATH] [--data DIR] [--version]\nDefaults: $XDG_RUNTIME_DIR/freesynergy/registry.sock, $XDG_DATA_HOME/freesynergy/registry"
                );
                return;
            }
            other => {
                eprintln!("unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    if let (Some(s), Some(d)) = (&socket, &data) {
        config = Some(Config::new(s, d));
    } else if let Some(c) = config.as_mut() {
        if let Some(s) = socket {
            c.socket = s.into();
        }
        if let Some(d) = data {
            c.data_dir = d.into();
        }
    }
    let Some(config) = config else {
        eprintln!("no $XDG_RUNTIME_DIR / $HOME: pass --socket and --data");
        std::process::exit(2);
    };
    if let Err(e) = run_blocking(config) {
        eprintln!("fs-registry: {e}");
        std::process::exit(1);
    }
}

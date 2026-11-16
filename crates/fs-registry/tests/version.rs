//! The real binary answers `--version` / `-V` / `--help` without starting the daemon
//! (release smoke test, SMOKE_ARGS=--version). Pattern: example-based test of the binary.

use std::process::{Command, Output};

fn run(arg: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_fs-registry"))
        .arg(arg)
        .output()
        .expect("run fs-registry")
}

#[test]
fn version_flags_print_name_and_version() {
    let want = format!("fs-registry {}\n", env!("CARGO_PKG_VERSION"));
    for flag in ["--version", "-V"] {
        let out = run(flag);
        assert!(out.status.success(), "{flag} must exit 0");
        assert_eq!(String::from_utf8_lossy(&out.stdout), want, "{flag}");
    }
}

#[test]
fn help_exits_zero() {
    for flag in ["--help", "-h"] {
        let out = run(flag);
        assert!(out.status.success(), "{flag} must exit 0");
        assert!(String::from_utf8_lossy(&out.stdout).contains("fs-registry"));
    }
}

#[test]
fn unknown_option_fails() {
    let out = run("--gibtsnicht");
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("unknown argument"));
}

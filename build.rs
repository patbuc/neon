use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[allow(clippy::print_stdout)]
fn main() {
    let version = match git(&["rev-list", "--count", "--merges", "--first-parent", "HEAD"]) {
        Some(merges) => format!(
            "{}.{}.{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            merges
        ),
        None => {
            println!("cargo:warning=git unavailable, NEON_VERSION falls back to Cargo.toml");
            env!("CARGO_PKG_VERSION").to_string()
        }
    };
    println!("cargo:rustc-env=NEON_VERSION={version}");

    for path in ["HEAD", "logs/HEAD"] {
        if let Some(path) = git(&["rev-parse", "--path-format=absolute", "--git-path", path]) {
            println!("cargo:rerun-if-changed={path}");
        }
    }
}

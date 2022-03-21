use std::path::Path;

use xshell::{cmd, Shell};

#[derive(Debug, PartialEq)]
pub enum OperatingSystem {
    Linux,
    MacOS,
}

fn main() -> Result<(), anyhow::Error> {
    let sh = Shell::new()?;

    // Check if Nix is already installed
    if cmd!(sh, "type -p nix").quiet().run().is_ok() {
        println!("Nix is already installed; skipping installation");
        return Ok(());
    }

    // Create a tempdir
    // Write config file
    // Assemble installer flags
    // Download the installer
    // Run the installer

    let workdir = sh.create_temp_dir()?;
    let mut config = vec![
        "max-jobs = auto",
        "trust-users = root $USER",
        "$EXTRA_NIX_CONFIG",
    ];

    if cmd!(
        sh,
        "[ ! $INPUT_EXTRA_NIX_CONFIG =~ \"experimental-features\" ]"
    )
    .run()
    .is_ok()
    {
        config.push("experimental-features = nix-command flakes");
    }

    let file = workdir.path().join("nix.conf");
    write_config(&sh, &file, &config.join("\n"))?;

    let _ = detect_os(&sh)?;

    Ok(())
}

fn detect_os(sh: &Shell) -> Result<OperatingSystem, anyhow::Error> {
    let output = cmd!(sh, "uname").read()?;

    if output.contains("Linux") {
        Ok(OperatingSystem::Linux)
    } else if output.contains("Darwin") {
        Ok(OperatingSystem::MacOS)
    } else {
        panic!("Unsupported OS: {}", output);
    }
}

fn write_config(sh: &Shell, file: &Path, contents: &str) -> Result<(), anyhow::Error> {
    sh.write_file(file, contents)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Shell {
        Shell::new().unwrap()
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn identifies_os_as_macos() {
        let sh = setup();
        let os = detect_os(&sh);
        assert!(os.is_ok());
        assert_eq!(os.unwrap(), OperatingSystem::MacOS);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn identifies_os_as_macos() {
        let sh = setup();
        let os = detect_os(&sh);
        assert!(os.is_ok());
        assert_eq!(os.unwrap(), OperatingSystem::MacOS);
    }

    #[test]
    fn writes_config_file_correctly() {
        let sh = setup();
        let tempdir = sh.create_temp_dir().unwrap();
        let contents = "max-jobs = 4";

        let file = tempdir.path().join("nix.conf");
        write_config(&sh, &file, contents).unwrap();

        let written_contents = sh.read_file(file).unwrap();

        assert_eq!(contents, written_contents);
    }
}

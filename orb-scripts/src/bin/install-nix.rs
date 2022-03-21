use std::path::Path;

use xshell::{cmd, Shell, TempDir};

#[derive(Debug, PartialEq)]
pub enum OperatingSystem {
    Linux,
    MacOS,
}

// TODO: Consider rewriting this to use typestates
// Rationale: The workdir has to be created to do certain actions, so encapsulating that
// in the type system would protected against misuse
// Counter-point: This script won't be shared beyond this orb, so misuse would be a concern for myself and myself only
struct NixInstaller {
    shell: Shell,
    workdir: Option<TempDir>,
}

impl NixInstaller {
    /// Create a new Nix installer
    fn new() -> Result<Self, anyhow::Error> {
        let shell = Shell::new()?;
        Ok(NixInstaller {
            shell,
            workdir: None,
        })
    }

    /// Check if nix is already installed; returns Err is it is
    fn nix_not_installed(&self) -> Result<(), anyhow::Error> {
        if cmd!(self.shell, "type -p nix").quiet().run().is_ok() {
            println!("Nix is already installed; skipping installation");
            return Err(anyhow::anyhow!(
                "Nix is present on the system; skipping installation"
            ));
        }
        Ok(())
    }

    /// Create tempdir
    fn create_workdir(&mut self) -> Result<(), anyhow::Error> {
        let tempdir = self.shell.create_temp_dir()?;
        self.workdir = Some(tempdir);
        Ok(())
    }

    /// Write config file
    fn write_config(&self) -> Result<(), anyhow::Error> {
        let extra_nix_config = self
            .shell
            .var("EXTRA_NIX_CONFIG")
            .unwrap_or_else(|_| "".to_string());
        let mut config = vec![
            "max-jobs = auto",
            "trust-users = root $USER",
            &extra_nix_config,
        ];

        if !extra_nix_config.contains("experimental-features") {
            config.push("experimental-features = nix-command flakes");
        }

        let file = self
            .workdir
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("workdir has not been created"))?
            .path()
            .join("nix.conf");

        self.shell.write_file(file, config.join("\n"))?;

        Ok(())
    }

    /// Assemble installer flags
    fn assemble_installer_flags(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    /// Download the installer
    fn download_installer(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    /// Run the installer
    fn run_installer(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    /// Returns a reference to the held `Shell`
    fn shell(&self) -> &Shell {
        &self.shell
    }

    /// Returns a reference to the held `TempDir`
    fn workdir(&self) -> &Option<TempDir> {
        &self.workdir
    }
}

fn main() -> Result<(), anyhow::Error> {
    let mut installer = NixInstaller::new()?;
    let sh = Shell::new()?;

    installer.nix_not_installed()?;

    installer.create_workdir()?;

    // Create a tempdir
    // Write config file
    // Assemble installer flags
    // Download the installer
    // Run the installer

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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> NixInstaller {
        NixInstaller::new().unwrap()
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn identifies_os_as_macos() {
        let installer = setup();
        let os = detect_os(installer.shell());
        assert!(os.is_ok());
        assert_eq!(os.unwrap(), OperatingSystem::MacOS);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn identifies_os_as_linux() {
        let installer = setup();
        let os = detect_os(installer.shell());
        assert!(os.is_ok());
        assert_eq!(os.unwrap(), OperatingSystem::Linux);
    }

    #[test]
    fn writes_config_file_correctly() {
        let mut installer = setup();
        installer.create_workdir().unwrap();
        let file = installer
            .workdir()
            .as_ref()
            .unwrap()
            .path()
            .join("nix.conf");

        installer.write_config().unwrap();

        let written_contents = installer.shell().read_file(file).unwrap();
        assert!(!written_contents.is_empty());
        assert!(written_contents.contains("max-jobs = auto"));
    }
}

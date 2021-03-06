use std::path::{Path, PathBuf};

use xshell::{cmd, Shell, TempDir};

#[derive(Debug, PartialEq)]
pub enum OperatingSystem {
    Linux,
    LinuxSystemD,
    MacOS,
}

// TODO: Consider rewriting this to use typestates
// Rationale: The workdir has to be created to do certain actions, so encapsulating that
// in the type system would protected against misuse
// Counter-point: This script won't be shared beyond this orb, so misuse would be a concern for myself and myself only
struct NixInstaller {
    shell: Shell,
    workdir: Option<TempDir>,
    os: OperatingSystem,
    installer_options: Vec<String>,
    user: String,
}

impl NixInstaller {
    /// Create a new Nix installer
    pub fn new() -> Result<Self, anyhow::Error> {
        let shell = Shell::new()?;
        let os = detect_os(&shell)?;
        let user = shell.var("USER").unwrap_or_else(|_| "circleci".to_string());
        Ok(NixInstaller {
            shell,
            workdir: None,
            os,
            installer_options: vec![],
            user,
        })
    }

    /// Check if nix is already installed; returns Err is it is
    pub fn nix_not_installed(&self) -> Result<(), anyhow::Error> {
        if cmd!(self.shell, "type -p nix").quiet().run().is_ok() {
            println!("Nix is already installed; skipping installation");
            return Err(anyhow::anyhow!(
                "Nix is present on the system; skipping installation"
            ));
        }
        Ok(())
    }

    /// Create tempdir
    pub fn create_workdir(&mut self) -> Result<(), anyhow::Error> {
        let tempdir = self.shell.create_temp_dir()?;
        self.workdir = Some(tempdir);
        Ok(())
    }

    /// Write config file
    pub fn write_config(&self) -> Result<(), anyhow::Error> {
        let extra_nix_config = self
            .shell
            .var("INPUT_EXTRA_NIX_CONFIG")
            .unwrap_or_else(|_| "".to_string());
        let trusted_user = format!("trusted-users = root {}", self.user);
        let mut config = vec!["max-jobs = auto", &trusted_user, &extra_nix_config];

        if !extra_nix_config.contains("experimental-features") {
            config.push("experimental-features = nix-command flakes");
        }

        if self.os == OperatingSystem::Linux {
            config.push("build-users-group = ");
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
    pub fn assemble_installer_flags(&mut self) -> Result<(), anyhow::Error> {
        let os = detect_os(&self.shell)?;
        let workdir = &self
            .workdir
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("workdir has not been created"))?
            .path()
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("workdir path is invalid"))?;
        self.installer_options.push("--no-channel-add".to_string());
        self.installer_options
            .push("--darwin-use-unencrypted-nix-store-volume".to_string());
        self.installer_options
            .push("--nix-extra-conf-file".to_string());
        self.installer_options.push(format!("{workdir}/nix.conf"));

        match os {
            OperatingSystem::MacOS | OperatingSystem::LinuxSystemD => {
                self.installer_options.push("--daemon".to_string());
                self.installer_options
                    .push("--daemon-user-count".to_string());
                self.installer_options
                    .push(format!("{}", num_cpus::get() * 2));
            }
            OperatingSystem::Linux => {
                self.shell.create_dir("/etc/nix")?;
                cmd!(self.shell, "cp {workdir}/nix.conf /etc/nix/nix.conf")
                    .quiet()
                    .run()?;
            }
        }

        let mut input_install_options = self
            .shell
            .var("INPUT_INSTALL_OPTIONS")
            .unwrap_or_else(|_| "".to_string())
            .split(' ')
            .map(|item| item.to_string())
            .collect::<Vec<_>>();

        input_install_options.append(&mut self.installer_options);
        self.installer_options = input_install_options;

        println!("installer options: {}", self.installer_options.join(" "));

        Ok(())
    }

    /// Download the installer
    pub fn download_installer(&self) -> Result<(), anyhow::Error> {
        let installer_url = self
            .shell
            .var("INPUT_INSTALL_URL")
            .ok()
            .map(|url| {
                if url.is_empty() {
                    "https://nixos.org/nix/install".to_string()
                } else {
                    url
                }
            })
            .unwrap_or_else(|| "https://nixos.org/nix/install".to_string());

        let response = reqwest::blocking::get(installer_url)?.text()?;

        let installer_path = self
            .workdir
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("workdir has not been created"))?
            .path()
            .join("install");

        self.shell.write_file(installer_path, response.as_bytes())?;
        Ok(())
    }

    /// Run the installer
    pub fn run_installer(&self) -> Result<(), anyhow::Error> {
        let installer = self
            .workdir
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("workdir has not been created"))?
            .path()
            .join("install");
        let options = self
            .installer_options
            .iter()
            .filter(|opt| !opt.is_empty())
            .cloned()
            .collect::<Vec<_>>();
        cmd!(self.shell, "sh {installer} {options...}")
            .quiet()
            .run()?;
        Ok(())
    }

    pub fn post_install_steps(&self) -> Result<(), anyhow::Error> {
        let mut bash_env_additions = vec![];

        if self.os == OperatingSystem::MacOS {
            let cert_file = "/nix/var/nix/profiles/default/etc/ssl/certs/ca-bundle.crt";
            bash_env_additions.push(format!("export NIX_SSL_CERT_FILE={cert_file}"));
            cmd!(
                self.shell,
                "sudo launchctl setenv NIX_SSL_CERT_FILE {cert_file}"
            )
            .quiet()
            .run()?;
        }

        let path = self.shell.var("PATH")?;
        bash_env_additions.push(format!("export PATH=/nix/var/nix/profiles/per-user/{}/profile/bin:/nix/var/nix/profiles/default/bin:{path}", self.user));

        let custom_path = self
            .shell
            .var("INPUT_NIX_PATH")
            .unwrap_or_else(|_| "".to_string());

        if !custom_path.is_empty() {
            bash_env_additions.push(format!("export NIX_PATH={custom_path}"));
        }

        let path = self.shell.var("BASH_ENV")?;
        append_file(
            &self.shell,
            &PathBuf::from(path),
            bash_env_additions.join("\n"),
        )?;

        Ok(())
    }

    /// Returns a reference to the held `Shell`
    /// Useful for testing
    #[allow(dead_code)]
    #[doc(hidden)]
    fn shell(&self) -> &Shell {
        &self.shell
    }

    /// Returns a reference to the held `TempDir`
    /// Useful for testing
    #[allow(dead_code)]
    #[doc(hidden)]
    fn workdir(&self) -> &Option<TempDir> {
        &self.workdir
    }
}

fn main() -> Result<(), anyhow::Error> {
    let mut installer = NixInstaller::new()?;

    installer.nix_not_installed()?;

    // Create a tempdir
    installer.create_workdir()?;

    // Write config file
    installer.write_config()?;

    // Assemble installer flags
    installer.assemble_installer_flags()?;

    // Download the installer
    installer.download_installer()?;

    // Run the installer
    installer.run_installer()?;

    // Set various environment variables
    installer.post_install_steps()?;

    Ok(())
}

fn detect_os(sh: &Shell) -> Result<OperatingSystem, anyhow::Error> {
    let output = cmd!(sh, "uname").quiet().read()?;

    if output.contains("Linux") {
        if cmd!(sh, "[ -e /run/systemd/system ]").quiet().run().is_ok() {
            return Ok(OperatingSystem::LinuxSystemD);
        }
        Ok(OperatingSystem::Linux)
    } else if output.contains("Darwin") {
        Ok(OperatingSystem::MacOS)
    } else {
        panic!("Unsupported OS: {}", output);
    }
}

fn append_file(sh: &Shell, path: &Path, contents: String) -> Result<(), anyhow::Error> {
    let mut to_write = vec![];
    let previous_contents = sh.read_file(path).unwrap_or_else(|_| "".to_string());
    if !previous_contents.is_empty() {
        to_write.push(previous_contents);
    }
    to_write.push(contents);

    println!("writing to {}", path.display());

    sh.write_file(path, to_write.join("\n").as_bytes())?;
    Ok(())
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
        assert_eq!(os.unwrap(), OperatingSystem::LinuxSystemD);
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

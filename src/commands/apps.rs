use std::process::{self, Command};

pub fn list_installed_apps() -> Result<(), Box<dyn std::error::Error>>
{
        let status = Command::new("powershell").arg("-c")
                                               .arg("Get-ItemProperty HKLM:\\Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\* | Select-Object DisplayName, DisplayVersion")
                                               .status()?;

        process::exit(status.code().unwrap_or(1));
}

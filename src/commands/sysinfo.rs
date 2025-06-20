use std::process::{self, Command};

pub fn sysinfo() -> Result<(), Box<dyn std::error::Error>>
{
        let status = Command::new("powershell").arg("-c")
                                               .arg("Get-ComputerInfo")
                                               .status()?;

        process::exit(status.code().unwrap_or(1));
}

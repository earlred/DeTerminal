use std::process::{Command, Stdio};

pub fn run_shell_command(command: &str) -> Result<(), std::io::Error> {
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd")
        .args(["/C", command])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait();

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait();

    match output {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Exited with status: {}", status),
        )),
        Err(e) => Err(e),
    }
}
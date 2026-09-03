/// Spawn the OS-default opener on the given URL. All three stdio streams
/// are redirected to /dev/null so the TUI in raw mode is not scrambled.
pub(super) fn open_in_browser(target: &str) -> std::io::Result<()> {
    use std::process::{Command, Stdio};
    let mut cmd: Command;
    #[cfg(target_os = "linux")]
    {
        cmd = Command::new("xdg-open");
        cmd.arg(target);
    }
    #[cfg(target_os = "macos")]
    {
        cmd = Command::new("open");
        cmd.arg(target);
    }
    #[cfg(target_os = "windows")]
    {
        cmd = Command::new("cmd");
        cmd.args(["/C", "start", ""]).arg(target);
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        let _ = target;
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "no known browser opener for this target",
        ));
    }
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

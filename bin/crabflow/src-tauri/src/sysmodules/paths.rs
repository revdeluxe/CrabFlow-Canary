use std::path::PathBuf;
use std::fs;
use directories::ProjectDirs;

#[cfg(target_os = "windows")]
pub fn get_data_dir() -> PathBuf {
    // Attempt to use ProgramData for global data
    if let Ok(program_data) = std::env::var("ProgramData") {
        let path = PathBuf::from(program_data).join("CrabFlow");
        return path;
    }
    // Fallback to local app data if ProgramData is missing (unlikely)
    if let Some(proj_dirs) = ProjectDirs::from("com", "crabflow", "CrabFlow") {
        return proj_dirs.data_dir().to_path_buf();
    }
    PathBuf::from(r"C:\CrabFlowData")
}

#[cfg(not(target_os = "windows"))]
pub fn get_data_dir() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "crabflow", "CrabFlow") {
        return proj_dirs.data_dir().to_path_buf();
    }
    // Fallback
    PathBuf::from("/var/lib/crabflow")
}

pub fn get_config_path(filename: &str) -> PathBuf {
    get_data_dir().join(filename)
}

pub fn init_data_dir() -> std::io::Result<()> {
    let path = get_data_dir();
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(())
}

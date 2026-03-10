use color_eyre::{
    Result,
    eyre::{Context, OptionExt, eyre},
};
use directories::ProjectDirs;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

pub fn parse_cli_file() -> Result<Option<PathBuf>> {
    let mut args = std::env::args().skip(1);

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "-f" | "--file" => {
                let value = args
                    .next()
                    .ok_or_else(|| eyre!("Expected file path after {}", arg))?;
                Ok(Some(PathBuf::from(value)))
            }
            _ if !arg.starts_with('-') => Ok(Some(PathBuf::from(arg))),
            _ => Err(eyre!("Unknown argument: {}", arg)),
        }
    } else {
        Ok(None)
    }
}

pub fn resolve_data_path(cli_path: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = cli_path {
        return Ok(path);
    }

    let proj_dirs = ProjectDirs::from("com", "example", "ghosted")
        .ok_or_eyre("Failed to find XDG directories")?;

    let config_dir = proj_dirs.config_dir();
    let file_path = config_dir.join("applications.json");

    if file_path.exists() {
        return Ok(file_path);
    }

    fs::create_dir_all(config_dir)?;
    fs::write(&file_path, "[]\n").wrap_err("Failed to create empty applications file")?;

    Ok(file_path)
}

pub fn load_db<A>(path: &Path) -> Result<Vec<A>>
where
    A: DeserializeOwned,
{
    let contents = fs::read_to_string(path)?;

    let apps: Vec<A> = serde_json::from_str(&contents)?;

    Ok(apps)
}

pub fn save_db<A>(path: &Path, data: &[A]) -> Result<()>
where
    A: Serialize,
{
    let file = File::create(path)?;
    Ok(serde_json::to_writer(file, data)?)
}

use crate::types::*; 
use std::path::{Path, PathBuf};
use faccess::{AccessMode, PathExt};
use snafu::ResultExt;

pub fn check_working_dir(config: &Config) -> Result<()> {
    let path = Path::new(&config.working_dir);
    check_writeable(&path)?;
    Ok(())
}

pub fn check_pid_file(config: &Config) -> Result<()> {
    let path = Path::new(&config.pid_file);
    check_writeable_file(&path)?;
    Ok(())
}

pub fn delete_file(file_path : &str) {
    let path = Path::new(&file_path);
    let _ = std::fs::remove_file(&path);
}

fn check_writeable_file(path: &Path) -> Result<()> {
    if path.exists() {
        check_is_file(path)?;
        check_writeable(path)?;
    } else {
        check_parent_dir_writeable(path)?;
    }
    Ok(())
}

fn check_parent_dir_writeable(path :&Path) -> Result<()> {
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    path_buf.pop();
    check_writeable(path_buf.as_path())?;
    Ok(())
}

fn check_is_file(path :&Path) -> Result<()> {
    let metadata = std::fs::metadata(path).context(PathMetadataError { 
        path : path.to_string_lossy().to_string()
    })?;
    if ! metadata.is_file() {
        return Err(Error::PathNoFile { 
            path : path.to_string_lossy().to_string()
        });
    }
    Ok(())
}

fn check_writeable(path : &Path) -> Result<()> {
    if ! path.access(AccessMode::READ | 
                     AccessMode::WRITE).is_ok() {
      return Err(Error::CouldNotWriteToFileOrDirectory { 
          path : path.to_string_lossy().to_string() }
      ); 
    }
    Ok(())
}
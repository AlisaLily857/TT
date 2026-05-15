#[cfg(target_os = "windows")]
pub const MAX_PATH_LEN: usize = 259;
#[cfg(not(target_os = "windows"))]
pub const MAX_PATH_LEN: usize = 4095;

pub const MIN_FILENAME_RESERVE: usize = 80;

pub const SEPARATOR_RESERVE: usize = 1;

#[derive(Debug, Clone, Copy)]
pub struct PathLimitError {
    pub limit: usize,
    pub current: usize,
    pub reserve: usize,
}

impl std::fmt::Display for PathLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "output path too long for OS limit (path uses {} of {} chars, need {} reserved for filename)",
            self.current, self.limit, self.reserve
        )
    }
}

impl std::error::Error for PathLimitError {}

pub fn validate_output_dir(output_dir: &str) -> Result<(), PathLimitError> {
    let current = output_dir.chars().count() + SEPARATOR_RESERVE;
    let reserve = MIN_FILENAME_RESERVE;
    if current + reserve > MAX_PATH_LEN {
        return Err(PathLimitError {
            limit: MAX_PATH_LEN,
            current,
            reserve,
        });
    }
    Ok(())
}

/// Validates that output_dir does not contain path-traversal sequences
/// and resolves within an allowed base directory.
pub fn sanitize_output_dir(output_dir: &str, base_dir: &std::path::Path) -> Result<std::path::PathBuf, String> {
    if output_dir.is_empty() {
        return Err("Output directory cannot be empty".to_string());
    }

    // Reject path traversal components
    for component in output_dir.split(|c| c == '/' || c == '\\') {
        if component == ".." {
            return Err("Output directory cannot contain '..'".to_string());
        }
    }

    // Reject absolute paths
    if output_dir.starts_with('/') {
        return Err("Output directory must be relative".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        if output_dir.len() >= 2 && output_dir.as_bytes()[1] == b':' {
            return Err("Output directory cannot be an absolute Windows path".to_string());
        }
        if output_dir.starts_with("\\\\") {
            return Err("Output directory cannot be a UNC path".to_string());
        }
    }

    let resolved = base_dir.join(output_dir);
    let canonical_base = base_dir.canonicalize()
        .map_err(|e| format!("Failed to canonicalize base dir: {}", e))?;
    let canonical_resolved = match resolved.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            // Path may not exist yet; canonicalize its parent and append the final component
            let parent = resolved.parent().unwrap_or(base_dir);
            let file_name = resolved.file_name()
                .ok_or_else(|| "Invalid output directory".to_string())?;
            let canonical_parent = parent.canonicalize()
                .map_err(|e| format!("Failed to canonicalize parent: {}", e))?;
            canonical_parent.join(file_name)
        }
    };

    if !canonical_resolved.starts_with(&canonical_base) {
        return Err("Output directory escapes the allowed base directory".to_string());
    }

    Ok(resolved)
}

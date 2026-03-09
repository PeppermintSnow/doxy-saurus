use std::{
    ffi::OsStr, 
    fs, 
    io, 
    path::{Path, PathBuf}
};

pub fn get_headers(
    path: &Path, 
    res: &mut Vec<PathBuf>
) -> io::Result<()> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().and_then(OsStr::to_str).unwrap();
        let ext = path.extension();

        if path.is_dir() {
            get_headers(&path, res)?;
        } else if !name.ends_with("_internal.h") && ext == Some(OsStr::new("h")) {
            res.push(path.clone());
        }
    }

    Ok(())
}

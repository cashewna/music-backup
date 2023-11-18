use std::fs::{self};
use std::path::Path;
use std::io::{self, Write};

pub fn process_dirs(dir: &Path, output_base: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(&dir).unwrap();
            let output_path = output_base.join(relative_path);

            if path.is_dir() {
                fs::create_dir_all(&output_path)?;
                process_dirs(&path, output_base)?;
            } else {
                if path.extension() == Some(std::ffi::OsStr::new("mp3")) {
                    let txt_path = output_path.with_extension("txt");
                    let mut file = fs::File::create(txt_path)?;
                    file.write_all(path.to_str().unwrap().as_bytes())?;
                }
            }
        }
    }
    Ok(())
}

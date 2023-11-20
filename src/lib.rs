use std::fs;
use std::io;
use std::path::Path;

pub fn process_dirs(dir: &Path, output_base: &Path) -> io::Result<(u32, u32)> {
    let mut total_files = 0;
    let mut music_copied = 0;

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(&dir).unwrap();
            let output_path = output_base.join(relative_path);

            total_files += 1;

            if path.is_dir() {
                fs::create_dir_all(&output_path)?;
                process_dirs(&path, output_base)?;
            } else {
                if path.extension() == Some(std::ffi::OsStr::new("mp3")) {
                    let txt_path = output_path.with_extension("txt");
                    let mut _file = fs::File::create(txt_path)?;
                    music_copied += 1;
                }
            }
        }
    }
    Ok((total_files, music_copied))
}

use std::{
    fs::{self, DirEntry},
    io,
};

fn main() {
    let removed_dirs = clean_dir("./testdata/testdir").expect("Unable to process directory");
    println!("Removed {removed_dirs} directories");
}

fn clean_dir(filepath: &str) -> Result<i32, io::Error> {
    let dirs_to_remove = ["node_modules", "__pycache__", "venv"];
    let paths = fs::read_dir(filepath)?;
    let mut dirs_removed = 0;

    for path in paths {
        let file = path?;

        let filename = file.file_name();
        let filename = match filename.to_str() {
            Some(filename) => filename,
            None => continue,
        };

        if dirs_to_remove.contains(&filename) {
            if delete_directory(file) {
                dirs_removed += 1;
            }
            continue;
        }

        if file.metadata()?.is_dir() {
            match file.path().to_str() {
                Some(filepath) => dirs_removed += clean_dir(filepath)?,
                None => continue,
            };
        }
    }

    Ok(dirs_removed)
}

fn delete_directory(file: DirEntry) -> bool {
    match fs::remove_dir_all(file.path()) {
        Ok(_) => {
            println!("Removed directory: {:?}", file.path());
            true
        }
        Err(_) => {
            println!("Failed to remove directory: '{:?}'", file.path());
            false
        }
    }
}

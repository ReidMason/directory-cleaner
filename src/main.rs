use clap::Parser;
use std::{
    fs::{self, DirEntry},
    io,
};

pub struct DirectoryCleaner {
    dirs_to_remove: Vec<String>,
    pub dirs_removed: i32,
}

/// A tool for cleaning commonly bloated directories
/// Useful for removing files before copying directories
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filepath to the target dir
    filepath: String,
}

fn main() {
    let args = Args::parse();

    let dirs_to_remove = vec![
        "node_modules".to_string(),
        "__pycache__".to_string(),
        "venv".to_string(),
    ];

    let mut directory_cleaner = DirectoryCleaner::new(dirs_to_remove);

    match directory_cleaner.clean_dir(&args.filepath) {
        Ok(_) => println!("Removed {} directories", directory_cleaner.dirs_removed),
        Err(_) => println!("Unable to read directory: '{}'", args.filepath),
    }
}

impl DirectoryCleaner {
    pub fn new(dirs_to_remove: Vec<String>) -> Self {
        Self {
            dirs_to_remove,
            dirs_removed: 0,
        }
    }

    pub fn clean_dir(&mut self, filepath: &str) -> Result<(), io::Error> {
        for path in fs::read_dir(filepath)? {
            let file = path?;

            if self.delete_directory(&file) {
                continue;
            }

            self.try_recurse_clean_dir(file)?;
        }
        Ok(())
    }

    fn try_recurse_clean_dir(&mut self, file: DirEntry) -> Result<(), io::Error> {
        if !file.metadata()?.is_dir() {
            return Ok(());
        }

        match file.path().into_os_string().into_string() {
            Ok(filepath) => self.clean_dir(&filepath)?,
            Err(_) => (),
        };

        Ok(())
    }

    fn get_filename(file: &DirEntry) -> String {
        let filename = file.file_name();
        match filename.into_string() {
            Ok(x) => return x,
            Err(_) => return String::new(),
        };
    }

    fn delete_directory(&mut self, file: &DirEntry) -> bool {
        let filename = DirectoryCleaner::get_filename(file);
        if filename.is_empty() || !self.dirs_to_remove.contains(&filename) {
            return false;
        }

        match fs::remove_dir_all(file.path()) {
            Ok(_) => {
                println!("Removed directory: {:?}", file.path());
                self.dirs_removed += 1;
                true
            }
            Err(_) => {
                println!("Failed to remove directory: '{:?}'", file.path());
                false
            }
        }
    }
}

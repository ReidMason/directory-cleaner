use std::fs;

fn main() {
    clean_dir("./testdata/testdir");
}

fn clean_dir(filepath: &str) {
    let dirs_to_remove = ["node_modules", "__pycache__", "venv"];
    let paths= fs::read_dir(filepath).expect("Unable to read directory");

    for path in paths {
        let file = path.unwrap();

        if dirs_to_remove.contains(&file.file_name().to_str().unwrap()) {
            println!("Removing: {}", file.path().display());
            fs::remove_dir_all(file.path()).unwrap();
            continue;
        }

        if file.metadata().unwrap().is_dir() {
            clean_dir(file.path().to_str().unwrap());
        }
    }
}

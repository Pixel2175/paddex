use std::{fs::read_to_string, path::Path, process};

// #[allow(dead_code)]
pub fn read_file(file: &str) -> String {
    read_to_string(file)
        .unwrap_or_else(|e| die(file,&format!("{e:#}") ))
}

// #[allow(dead_code)]
pub fn die(title:&str, message:&str) -> ! {
    eprintln!("[\x1b[33mE\x1b[0m] \x1b[31m{title}:\x1b[0m {message}");
    process::exit(-1);
}

#[allow(dead_code)]
pub fn basename(dir: &str) -> String {
    Path::new(dir)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_else( ||
            die("die", &format!("basename not found for {}", dir))
        )
        .to_string()
}

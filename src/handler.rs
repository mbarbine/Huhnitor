use std::fs::File;
use std::io::prelude::*;

use crate::error;

pub fn handle(command: String) -> String {
    let words = command.split(' ').collect::<Vec<&str>>();
    let len = words.len();
    if let Some(key) = words.get(1) {
        match key.to_uppercase().trim().as_ref() {
            "READ" => {
                if len > 2 {
                    let mut out = String::new();
                    if let Ok(mut file) = File::open(words[2].trim()) {
                        if file.read_to_string(&mut out).is_err() {
                            error!(format!("Coudldn't read file: '{}'", words[1]));
                        }
                    } else {
                        error!(format!("Couldn't open file: '{}'", words[2].trim()));
                    }

                    if !out.ends_with('\n') { out += "\n" }
                    return out;
                } else {
                    println!("Insufficient arguments");
                    println!("Command format: read [filename]");
                }
            }

            _ => error!("Command not found"),
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_handle_unknown_command() {
        let result = handle("huhn UNKNOWN_CMD".to_string());
        assert_eq!(result, "");
    }

    #[test]
    fn test_handle_read_insufficient_args() {
        let result = handle("huhn READ".to_string());
        assert_eq!(result, "");
    }

    #[test]
    fn test_handle_read_file() {
        let mut file = std::fs::File::create("test_read.txt").unwrap();
        file.write_all(b"test file content").unwrap();

        let result = handle("huhn READ test_read.txt".to_string());
        assert_eq!(result, "test file content\n");

        std::fs::remove_file("test_read.txt").unwrap();
    }

    #[test]
    fn test_handle_read_file_not_found() {
        let result = handle("huhn READ non_existent_file.txt".to_string());
        assert_eq!(result, "\n");
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_read_lines() {
        // Setup: create a temporary file
        let mut path = env::temp_dir();
        path.push("test_messages.txt");

        let test_content = "line1\nline2\nline3";
        let mut file = File::create(&path).expect("Failed to create temp file");
        file.write_all(test_content.as_bytes())
            .expect("Failed to write to temp file");

        // Test: read lines
        let lines = read_lines(&path).expect("Failed to read lines");
        assert_eq!(lines, vec!["line1", "line2", "line3"]);

        // Cleanup
        fs::remove_file(&path).expect("Failed to remove temp file");
    }

    #[test]
    fn test_read_lines_file_not_found() {
        let result = read_lines("non_existent_file.txt");
        assert!(result.is_err());
    }
}

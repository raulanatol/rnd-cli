use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_file_lines(filename: String) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| l.trim().to_string())
        .filter(|l| l.len() > 0)
        .collect();
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use filename::file_name;
    use tempfile::NamedTempFile;

    use crate::utils::files::read_file_lines;

    fn get_filename(file: &NamedTempFile) -> String {
        file_name(file).unwrap().to_str().unwrap().to_owned()
    }

    fn create_simple_file(mut file: &NamedTempFile) {
        let body = "John Doe1\nJohn Doe2\nJohn Doe3\n";
        file.write_all(body.as_bytes()).unwrap();
    }

    fn create_file_with_empty_lines(mut file: &NamedTempFile) {
        let body = "John Doe1\n\nJohn Doe2\n\n\n\n";
        file.write_all(body.as_bytes()).unwrap();
    }

    #[test]
    fn read_file_lines_with_simple() {
        let test_file = NamedTempFile::new().unwrap();
        create_simple_file(&test_file);
        let filename = get_filename(&test_file);

        let result = read_file_lines(filename).unwrap();

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn read_file_lines_and_remove_empty_lines() {
        let test_file = NamedTempFile::new().unwrap();
        create_file_with_empty_lines(&test_file);
        let filename = get_filename(&test_file);

        let names = read_file_lines(filename).unwrap();

        assert_eq!(names.len(), 2);
    }
}

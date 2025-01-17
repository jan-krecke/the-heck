use rev_buf_reader::RevBufReader;
use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;

pub fn get_history_file_path() -> PathBuf {
    let user_home_dir = home::home_dir().expect("No home directory found.");
    let root_dir = user_home_dir.as_path();
    let mut hist_file_path = PathBuf::from(root_dir);
    hist_file_path.push(".zsh_history");

    hist_file_path
}

pub fn read_last_line_history_file(hist_file_path: PathBuf) -> String {
    let file = File::open(hist_file_path).expect("Could not open file.");
    let buf = RevBufReader::new(file);
    // Takes the last 128 bytes of the file
    let last_lines_in_file: Vec<String> = buf
        .lines()
        .take(128)
        .map(|l| l.expect("Could not parse line."))
        .collect();
    // Splits the last line at the semicolon which separates the command from the timestamp
    let last_line: Vec<String> = last_lines_in_file[1]
        .split(';')
        .map(|borrow| borrow.to_owned())
        .collect();
    let last_command = last_line[1].to_string();
    // println!("Last command from shell: {}", last_command);

    last_command
}

use fst::automaton::Levenshtein;
use fst::{IntoStreamer, Set};
use std::process::Command;

pub fn correcter(split_last_command: Vec<&str>) -> Vec<String> {
    let program_name: &str = split_last_command[0];
    let wrong_command: &str = split_last_command[1];
    // println!("Wrong command: {}", wrong_command);

    let program_commands = check_known_programs(split_last_command);

    let mut fixed_command = vec!["If you see this, that's bad".to_string()];

    // TODO: Use if let instead of this mess
    if program_commands.iter().any(|&i| i == "Not implemented!") {
        // If the program name is unknown, fuzzy search the program name
        let fixed_program_name = fix_program_name(program_name).unwrap();
        let fixed_program_name = fixed_program_name.iter().map(|s| s.as_str()).collect();
        let new_program_commands = check_known_programs(fixed_program_name);
        // Fix the command using the new program name
        fixed_command = fix_command(wrong_command, new_program_commands).unwrap();
    } else {
        fixed_command = fix_command(wrong_command, program_commands).unwrap();
    };

    fixed_command
}

pub fn check_known_programs(split_last_command: Vec<&str>) -> &[&str] {
    // Checks whether the command contains calls a program known to the-heck
    let program_name: &str = split_last_command[0];
    // println!("Program name: {}", program_name);
    let program_commands = get_possible_commands(program_name);

    program_commands
}

pub fn fix_command(
    wrong_command: &str,
    possible_commands: &[&str],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Implements a fuzzy search of possible commands against the query
    let set = Set::from_iter(possible_commands)?;
    // The maximum Levenshtein distance = 2
    let lev = Levenshtein::new(wrong_command, 2)?;
    let stream = set.search(lev).into_stream();
    // Returns the list of possible commands
    let keys = stream.into_strs()?;

    Ok(keys)
}

pub fn fix_program_name(
    wrong_program_name: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let known_programs = vec!["cargo", "git", "sl"];
    // Implements a fuzzy search of possible commands against the query
    let set = Set::from_iter(known_programs)?;
    // The maximum Levenshtein distance = 2
    let lev = Levenshtein::new(wrong_program_name, 2)?;
    let stream = set.search(lev).into_stream();
    // Returns the list of possible commands
    let keys = stream.into_strs()?;

    Ok(keys)
}

pub fn push_command_to_cli(split_last_command: Vec<&str>, fixed_command: Vec<&str>) {
    let mut full_command = vec!["First arg", "Second arg"];

    if split_last_command[0] != fixed_command[0] {
        full_command = vec![fixed_command[0], split_last_command[1]];
    } else {
        full_command = vec![split_last_command[0], fixed_command[0]];
    }

    Command::new(full_command[0])
        .arg(full_command[1])
        .output()
        .expect("Command failed to start");
}

fn get_possible_commands(prog_name: &str) -> &'static [&'static str] {
    match prog_name {
        "git" => &["add", "restore", "restore --staged", "rm", "status"],
        "sl" => &["ls"],
        "cargo" => &[
            "build",
            "clippy",
            "fmt",
            "install",
            "run",
            "test",
            "uninstall",
        ],
        _ => &["Not implemented!"],
    }
}

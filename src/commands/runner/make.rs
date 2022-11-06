use std::env;
use std::process::Command;
use log::{info, error};

fn make_contest_directory(contest_name: &String) {
    let mkdir_status = Command::new("mkdir")
        .arg(contest_name)
        .status();

    match mkdir_status {
        Ok(_) => {
        info!("Command \"mkdir\" done.");
    }
        Err(err) => {
            error!("Error occurs: {}", err);
        }
    }
}

fn make_solution_files(contest_name: &String, problem_number: u8) {
    for (i, c) in ('a'..='z').enumerate() {
        if i == problem_number.into() {
            break;
        }

        let extension = match env::var("RAJ_EXTENSION") {
            Ok(extension) => extension,
            Err(_) => String::from("rs"),
        };
        let new_file_path = format!(
            "./{}/{}_{}.{}",
            contest_name,
            contest_name,
            c,
            extension
        );
        let template_file_path = match env::var("RAJ_TEMPLATE_FILE") {
            Ok(file_name) => file_name,
            Err(_) => String::from("template.rs"),
        };

        let cp_status = Command::new("cp")
            .args([
                String::from("--no-clobber"),
                template_file_path,
                new_file_path
            ])
            .status();

        match cp_status {
            Ok(_) => {
                info!("Command \"cp\" done.");
            }
            Err(err) => {
                error!("Error occurs: {}", err);
            }
        }
    }
}

pub fn run_make_command(contest_name: Option<&String>, problem_number: Option<&u8>) {
    let contest_name = match contest_name {
        Some(contest_name) => contest_name,
        None => unreachable!("Unexpected error occurs."),
    };
    let problem_number = match problem_number {
        Some(problem_number) => *problem_number,
        None => 8,
    };

    make_contest_directory(contest_name);
    make_solution_files(contest_name, problem_number);
}
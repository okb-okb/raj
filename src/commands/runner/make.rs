use super::utils::exec;
use std::env;

fn make_contest_directory(contest_name: Option<&String>) {
    let contest_name = match contest_name {
        Some(contest_name) => contest_name,
        None => unreachable!("Unexpected error occurs."),
    };
    exec((&String::from("mkdir"), vec![contest_name]));
}

fn make_cp_command(contest_name: &String, problem: char) -> (String, Vec<String>) {
    let extension = match env::var("RAJ_EXTENSION") {
        Ok(extension) => extension,
        Err(_) => String::from("rs"),
    };
    let new_file_path = format!(
        "./{}/{}_{}.{}",
        contest_name, contest_name, problem, extension
    );
    let template_file_path = match env::var("RAJ_TEMPLATE_FILE") {
        Ok(file_name) => file_name,
        Err(_) => String::from("template.rs"),
    };

    (
        String::from("cp"),
        vec![
            String::from("--no-clobber"),
            template_file_path,
            new_file_path,
        ],
    )
}

fn make_solution_files(contest_name: Option<&String>, problem_number: Option<&u8>) {
    let contest_name = match contest_name {
        Some(contest_name) => contest_name,
        None => unreachable!("Unexpected error occurs."),
    };
    let problem_number = match problem_number {
        Some(problem_number) => *problem_number,
        None => 8,
    };

    for (i, c) in ('a'..='z').enumerate() {
        if i == problem_number.into() {
            break;
        }
        exec(make_cp_command(contest_name, c));
    }
}

pub fn run_make_command(contest_name: Option<&String>, problem_number: Option<&u8>) {
    make_contest_directory(contest_name);
    make_solution_files(contest_name, problem_number);
}

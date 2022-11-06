use std::collections::VecDeque;
use std::env;
use std::process::Command;
use log::{info, error};

fn run_oj_download(contest_name: &String, problem_name: &String) {
    let url = format!(
        "https://{}.contest.atcoder.jp/tasks/{}_{}",
        contest_name,
        contest_name,
        problem_name
    );

    let status = Command::new("oj")
        .args([
            "d",
            &url
        ])
        .status();

    match status {
        Ok(_) => {
            info!("Command \"oj d\" done.");
        }
        Err(err) => {
            error!("Error occurs: {}", err);
        }
    };
}

fn compile(file_name: &String) {
    let (program, args) = match env::var("RAJ_COMPILE") {
        Ok(command) => {
            let splited_command: Vec<String> = command
                .split_whitespace()
                .map(String::from)
                .collect();
            let mut splited_command: VecDeque<String> = VecDeque::from(splited_command);
            
            let program = match splited_command.get(0) {
                Some(program) => program.clone(),
                None => {
                    panic!("Invalid command to complie is provided. Check \"RAJ_COMPILE\".");
                }
            };

            splited_command.pop_front();
            splited_command.push_back(file_name.clone());

            (program, Vec::from(splited_command))
        },
        Err(_) => (
            String::from("rustc"),
            ["-o", "a.out", file_name].map(String::from).to_vec()
        )
    };

    let status = Command::new(program)
        .args(args)
        .status();

    match status {
        Ok(_) => {
            info!("Compile done.");
        }
        Err(err) => {
            error!("Error occurs: {}", err);
        }
    };
}

fn run_oj_test(tolerance: Option<&u8>) {
    let mut args = vec!["t"];
    let mut parameter = String::from("1e-");

    match tolerance {
        Some(tolerance) => {
            args.push("-e");

            parameter.push(*tolerance as char);
            args.push(&parameter);
        },
        None => (),
    }

    let status = Command::new("oj")
        .args(args)
        .status();

        match status {
            Ok(_) => {
                info!("Command \"oj t\" done.");
            }
            Err(err) => {
                error!("Error occurs: {}", err);
            }
        };
}

pub fn run_test_command(
    contest_name: Option<&String>,
    problem_name: Option<&String>,
    file_name: Option<&String>,
    tolerance: Option<&u8>
) {
    let (contest_name, problem_name) = match (contest_name, problem_name) {
        (Some(contest_name), Some(problem_name)) => (contest_name, problem_name),
        _ => unreachable!("Unexpected error occurs."),
    };

    let extension = match env::var("RAJ_EXTENSION") {
        Ok(extension) => extension,
        Err(_) => String::from("rs"),
    };

    let file_name = match file_name {
        Some(file_name) => file_name.clone(),
        None => format!(
            "{}_{}.{}",
            contest_name,
            problem_name,
            extension
        ),
    };

    run_oj_download(contest_name, problem_name);
    compile(&file_name);
    run_oj_test(tolerance);
}
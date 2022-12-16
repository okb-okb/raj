use super::utils::exec;
use std::env;

fn make_submit_command(
    contest_name: Option<&String>,
    problem_name: Option<&String>,
    file_name: Option<&String>,
) -> (String, Vec<String>) {
    let (contest_name, problem_name) = match (contest_name, problem_name) {
        (Some(contest_name), Some(problem_name)) => (contest_name, problem_name),
        _ => unreachable!("Unexpected error occurs."),
    };

    let url = format!(
        "https://{}.contest.atcoder.jp/tasks/{}_{}",
        contest_name, contest_name, problem_name
    );

    let extension = match env::var("RAJ_EXTENSION") {
        Ok(extension) => extension,
        Err(_) => String::from("rs"),
    };

    let file_name = match file_name {
        Some(file_name) => file_name.clone(),
        None => format!("{}_{}.{}", contest_name, problem_name, extension),
    };

    (String::from("oj"), vec![String::from("s"), url, file_name])
}

pub fn run_submit_command(
    contest_name: Option<&String>,
    problem_name: Option<&String>,
    file_name: Option<&String>,
) {
    exec(make_submit_command(contest_name, problem_name, file_name));
}

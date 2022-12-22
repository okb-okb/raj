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

#[cfg(test)]
mod tests {
    use super::make_submit_command;
    use serial_test::serial;
    use std::env;

    #[test]
    fn success_make_submit_command() {
        let contest_name = String::from("abc150");
        let problem_name = String::from("a");
        let file_name = String::from("template.rs");
        let (command, args) =
            make_submit_command(Some(&contest_name), Some(&problem_name), Some(&file_name));
        assert_eq!(command, String::from("oj"));
        assert_eq!(
            args,
            vec![
                String::from("s"),
                String::from("https://abc150.contest.atcoder.jp/tasks/abc150_a"),
                String::from("template.rs")
            ]
        );
    }

    #[test]
    #[serial]
    fn success_make_submit_command_no_file_name() {
        let contest_name = String::from("abc150");
        let problem_name = String::from("a");
        let (command, args) = make_submit_command(Some(&contest_name), Some(&problem_name), None);
        assert_eq!(command, String::from("oj"));
        assert_eq!(
            args,
            vec![
                String::from("s"),
                String::from("https://abc150.contest.atcoder.jp/tasks/abc150_a"),
                String::from("abc150_a.rs")
            ]
        );
    }

    #[test]
    #[serial]
    fn success_make_submit_command_with_extension() {
        let contest_name = String::from("abc150");
        let problem_name = String::from("a");
        env::set_var("RAJ_EXTENSION", "cpp");
        let (command, args) = make_submit_command(Some(&contest_name), Some(&problem_name), None);
        assert_eq!(command, String::from("oj"));
        assert_eq!(
            args,
            vec![
                String::from("s"),
                String::from("https://abc150.contest.atcoder.jp/tasks/abc150_a"),
                String::from("abc150_a.cpp")
            ]
        );
        env::remove_var("RAJ_EXTENSION");
    }

    #[test]
    #[should_panic]
    fn fail_make_submit_command_no_required_parameters() {
        let (_command, _args) = make_submit_command(None, None, None);
    }
}

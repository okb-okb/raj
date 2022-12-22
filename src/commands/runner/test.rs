use super::utils::exec;
use std::collections::VecDeque;
use std::env;

fn get_test_file_info<'a>(
    contest_name: Option<&'a String>,
    problem_name: Option<&'a String>,
    file_name: Option<&String>,
) -> (&'a String, &'a String, String) {
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
        None => format!("{}_{}.{}", contest_name, problem_name, extension),
    };

    (contest_name, problem_name, file_name)
}

fn make_download_command(contest_name: &String, problem_name: &String) -> (String, Vec<String>) {
    let url = format!(
        "https://{}.contest.atcoder.jp/tasks/{}_{}",
        contest_name, contest_name, problem_name
    );

    (String::from("oj"), vec![String::from("d"), url])
}

fn run_oj_download(contest_name: &String, problem_name: &String) {
    exec(make_download_command(contest_name, problem_name));
}

fn make_compile_command(file_name: &String) -> (String, Vec<String>) {
    let (program, args) = match env::var("RAJ_COMPILE") {
        Ok(command) => {
            let splited_command: Vec<String> =
                command.split_whitespace().map(String::from).collect();
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
        }
        Err(_) => (
            String::from("rustc"),
            ["-o", "a.out", file_name.as_str()]
                .map(String::from)
                .to_vec(),
        ),
    };

    (program, args)
}

fn compile(file_name: &String) {
    exec(make_compile_command(file_name));
}

fn make_test_command(tolerance: Option<&u8>) -> (String, Vec<String>) {
    let mut args = vec![String::from("t")];
    let mut parameter = String::from("1e-");

    match tolerance {
        Some(tolerance) => {
            args.push(String::from("-e"));

            parameter.push(*tolerance as char);
            args.push(parameter);
        }
        None => (),
    }

    (String::from("oj"), args)
}

fn run_oj_test(tolerance: Option<&u8>) {
    exec(make_test_command(tolerance));
}

pub fn run_test_command(
    contest_name: Option<&String>,
    problem_name: Option<&String>,
    file_name: Option<&String>,
    tolerance: Option<&u8>,
) {
    let (contest_name, problem_name, file_name) =
        get_test_file_info(contest_name, problem_name, file_name);
    run_oj_download(contest_name, problem_name);
    compile(&file_name);
    run_oj_test(tolerance);
}

mod tests {
    use super::*;

    #[test]
    fn success_get_test_file_info() {}

    #[test]
    fn success_get_test_file_info_with_extension() {}

    #[test]
    fn success_get_test_file_info_no_file_name() {}

    #[test]
    #[should_panic]
    fn fail_get_test_file_info_no_required_parameters() {
        panic!("panic");
    }

    #[test]
    fn success_make_download_command() {
        let contest_name = String::from("abc150");
        let problem_name = String::from("a");
        let (command, args) = make_download_command(&contest_name, &problem_name);
        assert_eq!(command, String::from("oj"));
        assert_eq!(
            args,
            vec![
                String::from("d"),
                String::from("https://abc150.contest.atcoder.jp/tasks/abc150_a")
            ]
        );
    }

    #[test]
    fn success_make_compile_command() {}

    #[test]
    fn success_make_compile_command_with_args() {}

    #[test]
    #[should_panic]
    fn fail_make_compile_command_invalid_args() {
        panic!("panic");
    }

    #[test]
    fn success_make_test_command() {}

    #[test]
    fn success_make_test_command_with_tolerance() {}
}

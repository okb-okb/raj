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

fn make_compile_command(file_name: &str) -> (String, Vec<String>) {
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
            splited_command.push_back(file_name.to_owned());

            (program, Vec::from(splited_command))
        }
        Err(_) => (
            String::from("rustc"),
            ["-o", "a.out", file_name].map(String::from).to_vec(),
        ),
    };

    (program, args)
}

fn compile(file_name: &str) {
    exec(make_compile_command(file_name));
}

fn make_test_command(tolerance: Option<&u8>) -> (String, Vec<String>) {
    let mut args = vec![String::from("t")];
    let parameter = String::from("1e-");

    if let Some(tolerance) = tolerance {
        args.push(String::from("-e"));
        args.push(parameter + &tolerance.to_string());
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

#[cfg(test)]
mod tests {
    use super::{
        get_test_file_info, make_compile_command, make_download_command, make_test_command,
    };
    use serial_test::serial;
    use std::{env, panic};

    #[test]
    #[serial]
    fn success_get_test_file_info() {
        let contest_name = String::from("abc150");
        let problem_name = String::from("a");
        let file_name = String::from("template.rs");
        let (contest_name, problem_name, file_name) =
            get_test_file_info(Some(&contest_name), Some(&problem_name), Some(&file_name));
        assert_eq!(contest_name, &String::from("abc150"));
        assert_eq!(problem_name, &String::from("a"));
        assert_eq!(file_name, String::from("template.rs"));
    }

    #[test]
    #[serial]
    fn success_get_test_file_info_with_extension() {
        let contest_name = String::from("abc150");
        let problem_name = String::from("a");
        env::set_var("RAJ_EXTENSION", "cpp");
        let (contest_name, problem_name, file_name) =
            get_test_file_info(Some(&contest_name), Some(&problem_name), None);
        assert_eq!(contest_name, &String::from("abc150"));
        assert_eq!(problem_name, &String::from("a"));
        assert_eq!(file_name, String::from("abc150_a.cpp"));
        env::remove_var("RAJ_EXTENSION");
    }

    #[test]
    #[serial]
    fn success_get_test_file_info_default_extension() {
        let contest_name = String::from("abc150");
        let problem_name = String::from("a");
        let (contest_name, problem_name, file_name) =
            get_test_file_info(Some(&contest_name), Some(&problem_name), None);
        assert_eq!(contest_name, &String::from("abc150"));
        assert_eq!(problem_name, &String::from("a"));
        assert_eq!(file_name, String::from("abc150_a.rs"));
    }

    #[test]
    #[should_panic]
    fn fail_get_test_file_info_no_required_parameters() {
        let (_contest_name, _problem_name, _file_name) = get_test_file_info(None, None, None);
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
    #[serial]
    fn success_make_compile_command() {
        let file_name = String::from("template.rs");
        let (command, args) = make_compile_command(&file_name);
        assert_eq!(command, String::from("rustc"));
        assert_eq!(
            args,
            vec![
                String::from("-o"),
                String::from("a.out"),
                String::from("template.rs")
            ]
        );
    }

    #[test]
    #[serial]
    fn success_make_compile_command_with_args() {
        let file_name = String::from("template.rs");
        env::set_var("RAJ_COMPILE", "test compile command");
        let (command, args) = make_compile_command(&file_name);
        assert_eq!(command, String::from("test"));
        assert_eq!(
            args,
            vec![
                String::from("compile"),
                String::from("command"),
                String::from("template.rs")
            ]
        );
    }

    #[test]
    #[serial]
    fn fail_make_compile_command_invalid_args() {
        let file_name = String::from("template.rs");
        env::set_var("RAJ_COMPILE", "");
        let result = panic::catch_unwind(|| {
            let (_command, _args) = make_compile_command(&file_name);
        });
        env::remove_var("RAJ_COMPILE");
        assert!(result.is_err());
    }

    #[test]
    fn success_make_test_command() {
        let (command, args) = make_test_command(Some(&8));
        assert_eq!(command, String::from("oj"));
        assert_eq!(
            args,
            vec![String::from("t"), String::from("-e"), String::from("1e-8")]
        );
    }

    #[test]
    fn success_make_test_command_no_tolerance() {
        let (command, args) = make_test_command(None);
        assert_eq!(command, String::from("oj"));
        assert_eq!(args, vec![String::from("t")]);
    }
}

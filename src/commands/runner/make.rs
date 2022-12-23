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

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::{make_contest_directory, make_cp_command, run_make_command};
    use std::path::Path;
    use std::{env, fs};

    #[test]
    #[serial]
    fn success_make_contest_directory() {
        make_contest_directory(Some(&String::from("testdir")));
        assert!(Path::new("testdir").exists());
        fs::remove_dir("testdir").unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_make_contest_directory_no_contest_name() {
        make_contest_directory(None);
    }

    #[test]
    #[serial]
    fn success_make_cp_command() {
        let contest_name = String::from("abc150");
        let problem = 'a';
        let (command, args) = make_cp_command(&contest_name, problem);
        assert_eq!(command, String::from("cp"));
        assert_eq!(
            args,
            vec![
                String::from("--no-clobber"),
                String::from("template.rs"),
                String::from("./abc150/abc150_a.rs")
            ]
        );
    }

    #[test]
    #[serial]
    fn success_make_cp_command_with_extension() {
        let contest_name = String::from("abc150");
        let problem = 'a';
        env::set_var("RAJ_EXTENSION", "cpp");
        let (command, args) = make_cp_command(&contest_name, problem);
        assert_eq!(command, String::from("cp"));
        assert_eq!(
            args,
            vec![
                String::from("--no-clobber"),
                String::from("template.rs"),
                String::from("./abc150/abc150_a.cpp")
            ]
        );
        env::remove_var("RAJ_EXTENSION");
    }

    #[test]
    #[serial]
    fn success_make_cp_command_with_template_file() {
        let contest_name = String::from("abc150");
        let problem = 'a';
        env::set_var("RAJ_TEMPLATE_FILE", "my_template.rs");
        let (command, args) = make_cp_command(&contest_name, problem);
        assert_eq!(command, String::from("cp"));
        assert_eq!(
            args,
            vec![
                String::from("--no-clobber"),
                String::from("my_template.rs"),
                String::from("./abc150/abc150_a.rs")
            ]
        );
        env::remove_var("RAJ_TEMPLATE_FILE");
    }

    #[test]
    #[serial]
    fn success_run_make_command() {
        let contest_name = String::from("abc150");
        let problem_number = 6;
        run_make_command(Some(&contest_name), Some(&problem_number));
        assert!(Path::new("./abc150/abc150_a.rs").exists());
        assert!(Path::new("./abc150/abc150_f.rs").exists());
        assert!(!Path::new("./abc150/abc150_g.rs").is_file());
        fs::remove_dir_all("./abc150").unwrap();
    }

    #[test]
    #[serial]
    fn success_run_make_command_no_problem_number() {
        let contest_name = String::from("abc150");
        run_make_command(Some(&contest_name), None);
        assert!(Path::new("./abc150/abc150_a.rs").exists());
        assert!(Path::new("./abc150/abc150_h.rs").exists());
        assert!(!Path::new("./abc150/abc150_i.rs").is_file());
        fs::remove_dir_all("./abc150").unwrap();
    }

    #[test]
    #[should_panic]
    fn fail_run_make_command_no_contest_name() {
        run_make_command(None, None);
    }
}

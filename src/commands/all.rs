use clap::{arg, value_parser, Command};

pub fn commands() -> Vec<Command> {
    let test_command = Command::new("test")
        .about("Download data and execute local test")
        .args([
            arg!(<CONTEST>),
            arg!(<PROBLEM>),
            arg!(
                -f
                --file
                <FILE>
                "File name to test"
            ),
            arg!(
                -e
                --error
                <TOLERANCE>
                "Tolerance of error for problems handle floating point number (1e-{TOLERANCE})"
            )
            .value_parser(value_parser!(u8).range(1..10)),
        ]);

    let submit_command = Command::new("submit").about("Submit a program").args([
        arg!(<CONTEST>),
        arg!(<PROBLEM>),
        arg!(
            -f
            --file
            <FILE>
            "File name to submit"
        ),
    ]);

    let make_command = Command::new("make")
        .about("Make files for a contest")
        .args([
            arg!(<CONTEST>),
            arg!(
                -n
                --number
                <NUMBER>
                "The number of problems the contest has"
            )
            .value_parser(value_parser!(u8).range(1..=26)),
        ]);

    vec![test_command, submit_command, make_command]
}

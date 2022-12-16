mod commands;

use clap::command;
use commands::{all, runner};

fn main() {
    env_logger::init();

    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(all::commands())
        .get_matches();

    match matches.subcommand() {
        Some(("test", sub_matches)) => runner::test::run_test_command(
            sub_matches.get_one::<String>("CONTEST"),
            sub_matches.get_one::<String>("PROBLEM"),
            sub_matches.get_one::<String>("file"),
            sub_matches.get_one::<u8>("error"),
        ),
        Some(("submit", sub_matches)) => runner::submit::run_submit_command(
            sub_matches.get_one::<String>("CONTEST"),
            sub_matches.get_one::<String>("PROBLEM"),
            sub_matches.get_one::<String>("file"),
        ),
        Some(("make", sub_matches)) => runner::make::run_make_command(
            sub_matches.get_one::<String>("CONTEST"),
            sub_matches.get_one::<u8>("number"),
        ),
        _ => unreachable!("Unexpected error occurs."),
    }
}

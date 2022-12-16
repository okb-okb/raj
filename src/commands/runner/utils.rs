use log::error;
use std::{ffi::OsStr, process::Command};

pub fn exec<I, S>(p: (S, I))
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let (command, args) = p;
    let command_status = Command::new(command).args(args).status();

    match command_status {
        Ok(_) => (),
        Err(err) => error!("Error occurs: {}", err),
    };
}

// SPDX-License-Identifier: AGPL-3.0-only

use std::env;
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};

const PROGRAM: &str = env!("EXEC_TARGET_PATH");

fn main() {
    let err = Command::new(PROGRAM)
        .args(env::args_os().skip(1))
        .exec();

    eprintln!("{err}");

    exit(if err.raw_os_error() == Some(libc::ENOENT) {
        127
    } else {
        126
    });
}

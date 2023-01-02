// agnish (the Ain't Gonna Need It SHell) is:
// * my login shell on Arch Linux
// * simple, so I'm fine with it running all the time
// * for starting fancier shells
//
// Ignoring unsuccessful writes to standard output is simple and okay.

use std::{
    ffi::OsStr,
    io::{stdin, stdout, ErrorKind::Interrupted, Read, Write},
    os::unix::ffi::OsStrExt,
    process::Command,
};

const PS: &[u8] = b"agnish> ";
const INIT_COMMAND: &[u8] = b"sway";

fn main() {
    let mut the_stdin = stdin().lock();
    let mut the_stdout = stdout().lock();

    let _ = the_stdout.write(PS);
    let _ = the_stdout.write(INIT_COMMAND);
    let _ = the_stdout.write(b"\n");
    let _ = the_stdout.flush();
    let _ = Command::new(OsStr::from_bytes(INIT_COMMAND)).status();

    let mut buf: [u8; 1] = Default::default();
    let mut command_state = Vec::new();
    let _ = the_stdout.write(PS);
    let _ = the_stdout.flush();
    loop {
        match the_stdin.read(&mut buf) {
            Ok(n) => match (n == 0, buf[0] == b'\n') {
                (true, _) => break,
                (false, true) => {
                    let _ = Command::new(OsStr::from_bytes(&command_state)).status();
                    command_state = Vec::new();
                    let _ = the_stdout.write(PS);
                    let _ = the_stdout.flush();
                }
                (false, false) => command_state.push(buf[0]),
            },
            Err(err) => match err.kind() == Interrupted {
                true => {}
                false => panic!(),
            },
        }
    }
}

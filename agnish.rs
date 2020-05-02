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
    let the_unlocked_stdin = stdin();
    let mut the_stdin = the_unlocked_stdin.lock();
    let the_unlocked_stdout = stdout();
    let mut the_stdout = the_unlocked_stdout.lock();

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
            Ok(0) => break,
            Ok(_) => {
                if buf[0] == b'\n' {
                    let _ = Command::new(OsStr::from_bytes(&command_state)).status();
                    command_state = Vec::new();
                    let _ = the_stdout.write(PS);
                    let _ = the_stdout.flush();
                } else {
                    command_state.push(buf[0])
                }
            }
            Err(err) => {
                if err.kind() != Interrupted {
                    panic!()
                }
            }
        }
    }
}

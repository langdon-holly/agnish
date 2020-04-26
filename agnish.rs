use std::{
    ffi::OsStr,
    io::{stdin, stdout, ErrorKind::Interrupted, Read, Write},
    os::unix::ffi::OsStrExt,
    process::Command,
};

const PS: &[u8] = b"agnish> ";

fn main() {
    let the_unlocked_stdin = stdin();
    let mut the_stdin = the_unlocked_stdin.lock();
    let the_unlocked_stdout = stdout();
    let mut the_stdout = the_unlocked_stdout.lock();

    for command in &[b"sway"] {
        let _ = the_stdout.write(PS);
        let _ = the_stdout.write(*command);
        let _ = the_stdout.flush();
        let _ = Command::new(OsStr::from_bytes(*command)).status();
    }

    let mut command_state = Vec::new();
    let _ = the_stdout.write(PS);
    let _ = the_stdout.flush();

    let mut buf: [u8; 1] = Default::default();
    loop {
        match the_stdin.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

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

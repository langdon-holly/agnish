use std::{
    ffi::OsStr,
    io::{stdin, stdout, ErrorKind::Interrupted, Read, StdoutLock, Write},
    os::unix::ffi::OsStrExt,
    process::Command,
};

const PS: &[u8] = b"agnish> ";

fn handle_byte(command_state: &mut Vec<u8>, byte: u8, the_stdout: &mut StdoutLock) {
    if byte == b'\n' {
        let _ = Command::new(OsStr::from_bytes(command_state)).status();
        *command_state = Vec::new();
        let _ = the_stdout.write(PS);
        let _ = the_stdout.flush();
    } else {
        command_state.push(byte)
    }
}

fn main() {
    let the_unlocked_stdin = stdin();
    let mut the_stdin = the_unlocked_stdin.lock();
    let the_unlocked_stdout = stdout();
    let mut the_stdout = the_unlocked_stdout.lock();

    let mut command_state = Vec::new();
    let _ = the_stdout.write(PS);
    let _ = the_stdout.flush();

    for byte in b"sway\n" {
        let _ = the_stdout.write(&[*byte]);
        let _ = the_stdout.flush();
        handle_byte(&mut command_state, *byte, &mut the_stdout)
    }

    let mut buf: [u8; 1] = Default::default();
    loop {
        match the_stdin.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
            }
            Err(err) => {
                if err.kind() == Interrupted {
                    continue;
                } else {
                    panic!()
                }
            }
        }
        handle_byte(&mut command_state, buf[0], &mut the_stdout)
    }
}

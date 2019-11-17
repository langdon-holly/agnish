use std::{
    env::args_os,
    ffi::OsStr,
    io::{stdin, stdout, Read, StdoutLock, Write},
    mem::replace,
    os::unix::ffi::OsStrExt,
    process::Command,
};

struct ParsingState<'a> {
    words: Vec<Box<[u8]>>,
    current_word: Vec<u8>,
    nesting_level: usize,
    escaping: bool,
    bad: bool,
    stdout: StdoutLock<'a>,
}

fn handle_byte(state: &mut ParsingState, byte: u8) {
    if state.bad {
        if byte == b'\n' {
            state.bad = false;
            state.stdout.write(b"bad syntax\n[do]").unwrap();
            state.stdout.flush().unwrap();
        }
    } else {
        if state.nesting_level == 0 {
            match byte {
                b'[' => state.nesting_level = 1,
                b'\n' => {
                    if state.words.len() == 0 {
                        if byte == b'\n' {
                            state.stdout.write(b"bad syntax\n[do]").unwrap();
                            state.stdout.flush().unwrap();
                        }
                    } else {
                        let mut command = Command::new(OsStr::from_bytes(&*state.words[0]));
                        for i in 1..state.words.len() {
                            command.arg(OsStr::from_bytes(&*state.words[i]));
                        }
                        if let Err(_) = command.status() {
                            state.stdout.write(b"unsuccessful\n").unwrap();
                        }
                        state.words = Vec::new();
                        state.stdout.write(b"[do]").unwrap();
                        state.stdout.flush().unwrap();
                    }
                }
                _ => state.bad = true,
            }
        } else {
            if state.escaping {
                state.escaping = false;
                state.current_word.push(byte);
            } else {
                match byte {
                    b'[' => {
                        state.nesting_level += 1;
                        state.current_word.push(byte);
                    }
                    b']' => {
                        state.nesting_level -= 1;
                        if state.nesting_level == 0 {
                            state.words.push(
                                replace(&mut state.current_word, Vec::new()).into_boxed_slice(),
                            );
                        } else {
                            state.current_word.push(byte);
                        }
                    }
                    b'\\' => state.escaping = true,
                    _ => state.current_word.push(byte),
                }
            }
        }
    }
}

fn main() {
    let the_unlocked_stdin = stdin();
    let mut the_stdin = the_unlocked_stdin.lock();
    let the_unlocked_stdout = stdout();

    let mut state = ParsingState {
        words: Vec::new(),
        current_word: Vec::new(),
        nesting_level: 0,
        escaping: false,
        bad: false,
        stdout: the_unlocked_stdout.lock(),
    };

    state.stdout.write(b"[do]").unwrap();
    state.stdout.flush().unwrap();

    for arg in args_os().skip(1) {
        for byte in arg.as_bytes() {
            state.stdout.write(&[*byte]).unwrap();
            state.stdout.flush().unwrap();
            handle_byte(&mut state, *byte)
        }
    }

    let mut buf: [u8; 1] = Default::default();
    while the_stdin.read(&mut buf).unwrap() > 0 {
        handle_byte(&mut state, buf[0])
    }
}

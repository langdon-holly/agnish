use std::{
    ffi::OsStr,
    io::{stdin, stdout, Read, Write},
    os::unix::ffi::OsStrExt,
    process::Command,
};

fn main() {
    let the_unlocked_stdin = stdin();
    let mut the_stdin = the_unlocked_stdin.lock();
    let the_unlocked_stdout = stdout();
    let mut the_stdout = the_unlocked_stdout.lock();
    'OUTER: loop {
        the_stdout.write(b"[run]").unwrap();
        the_stdout.flush().unwrap();

        let mut words: Vec<Box<[u8]>> = Vec::new();
        let mut current_word: Vec<u8> = Vec::new();
        let mut nesting_level = 0;
        let mut escaping = false;
        let mut bad = false;
        loop {
            let mut buf: [u8; 1] = Default::default();
            if the_stdin.read(&mut buf).unwrap() == 0 {
                break 'OUTER;
            }
            let byte = buf[0];
            if nesting_level == 0 {
                match byte {
                    b'[' => {
                        nesting_level = 1;
                    }
                    b'\n' => {
                        if words.len() == 0 || bad {
                            bad = false;
                            the_stdout.write(b"bad syntax\n[run]").unwrap();
                            the_stdout.flush().unwrap();
                            words = Vec::new();
                        } else {
                            break;
                        }
                    }
                    _ => {
                        bad = true;
                    }
                }
            } else {
                if escaping {
                    escaping = false;
                    current_word.push(byte);
                } else {
                    match byte {
                        b'[' => {
                            nesting_level += 1;
                            current_word.push(byte);
                        }
                        b']' => {
                            nesting_level -= 1;
                            if nesting_level == 0 {
                                words.push(current_word.into_boxed_slice());
                                current_word = Vec::new();
                            } else {
                                current_word.push(byte);
                            }
                        }
                        b'\\' => {
                            escaping = true;
                        }
                        _ => {
                            current_word.push(byte);
                        }
                    }
                }
            }
        }

        let mut command = Command::new(OsStr::from_bytes(&*words[0]));
        for i in 1..words.len() {
            command.arg(OsStr::from_bytes(&*words[i]));
        }
        if let Err(_) = command.status() {
            the_stdout.write(b"wouldn't run\n").unwrap();
        }
    }
}

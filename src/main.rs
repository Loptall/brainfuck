mod buffer;

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_source(path: &Path) -> String {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).ok();
    buf
}

fn main() {
    let arg = args().nth(1).unwrap();
    let path = Path::new(&arg);

    // dbg!(path);

    let src: Vec<char> = read_source(path).chars().collect();

    // dbg!(&src, src.len());

    let mut buffer = buffer::Buffer::new();
    let mut i = 0;

    loop {
        if src.len() <= i {
            return;
        }

        // dbg!(i, src[i]);

        match src[i] {
            '+' => {
                buffer.inc();
            }
            '-' => {
                buffer.dec();
            }
            '<' => {
                buffer.shl();
            }
            '>' => {
                buffer.shr();
            }
            '[' => {
                if *buffer.get().unwrap() == 0 {
                    let mut s = 0usize;
                    loop {
                        i += 1;

                        if src[i] == ']' && s == 0 {
                            break;
                        } else {
                            if src[i] == ']' {
                                s -= 1;
                            }
                            if src[i] == '[' {
                                s += 1;
                            }
                        }
                    }
                }
            }
            ']' => {
                if *buffer.get().unwrap() != 0 {
                    let mut s = 0usize;
                    loop {
                        i -= 1;

                        if src[i] == '[' && s == 0 {
                            break;
                        } else {
                            if src[i] == ']' {
                                s += 1;
                            }
                            if src[i] == '[' {
                                s -= 1;
                            }
                        }
                    }
                }
            }
            '.' => {
                buffer.put();
            }
            ',' => {
                buffer.read();
            }
            _ => { /* ignore */ }
        }

        i += 1;
    }
}

use std::fs::File;
use std::io::Read;
use std::process::exit;

const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = ",.;:!?@#$%^&*-_+=";
const HEX: &str = "0123456789abcdef";

const HELP: &str = "passgen - throwaway test password generator

USAGE: passgen [FLAGS]

FLAGS:
  -l, --length N   password length (default 32)
  -c, --count N    how many passwords to print (default 1)
  -a, --alnum      letters + digits
  -L, --letters    letters only
  -d, --digits     digits only
  -x, --hex        hex characters only
  -s, --symbols    lowercase + digits + symbols 
  -f, --full       upper + lower + digits + symbols
  -h, --help       show this help
";

fn die(msg: &str) -> ! {
    eprintln!("passgen: {msg}\n\n{HELP}");
    exit(1);
}

fn rand_below(urandom: &mut File, n: usize) -> usize {
    // rejection sampling to avoid modulo bias
    let limit = u32::MAX - (u32::MAX % n as u32);
    loop {
        let mut b = [0u8; 4];
        urandom.read_exact(&mut b).expect("read /dev/urandom");
        let v = u32::from_le_bytes(b);
        if v < limit {
            return (v % n as u32) as usize;
        }
    }
}

fn main() {
    let mut length = 32usize;
    let mut count = 1usize;
    let mut classes: Vec<&str> = vec![LOWER, UPPER, DIGITS, SYMBOLS];

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-l" | "--length" | "-c" | "--count" => {
                let n = args
                    .next()
                    .and_then(|v| v.parse::<usize>().ok())
                    .filter(|&n| n > 0)
                    .unwrap_or_else(|| die(&format!("{arg} needs a positive number")));
                if arg == "-l" || arg == "--length" {
                    length = n;
                } else {
                    count = n;
                }
            }
            "-a" | "--alnum" => classes = vec![LOWER, UPPER, DIGITS],
            "-L" | "--letters" => classes = vec![LOWER, UPPER],
            "-d" | "--digits" => classes = vec![DIGITS],
            "-x" | "--hex" => classes = vec![HEX],
            "-s" | "--symbols" => classes = vec![LOWER, DIGITS, SYMBOLS],
            "-f" | "--full" => classes = vec![LOWER, UPPER, DIGITS, SYMBOLS],
            "-h" | "--help" => {
                print!("{HELP}");
                return;
            }
            other => die(&format!("unknown flag: {other}")),
        }
    }

    if length < classes.len() {
        die(&format!("length must be at least {} for this mode", classes.len()));
    }

    let all: Vec<char> = classes.iter().flat_map(|c| c.chars()).collect();
    let mut urandom = File::open("/dev/urandom").expect("open /dev/urandom");
    for _ in 0..count {
        // one char from every class guarantees each requirement is met,
        // the rest come from the full pool, then shuffle the lot
        let mut pw: Vec<char> = Vec::with_capacity(length);
        if classes.len() > 1 {
            for class in &classes {
                let cs: Vec<char> = class.chars().collect();
                pw.push(cs[rand_below(&mut urandom, cs.len())]);
            }
        }
        while pw.len() < length {
            pw.push(all[rand_below(&mut urandom, all.len())]);
        }
        for i in (1..pw.len()).rev() {
            pw.swap(i, rand_below(&mut urandom, i + 1));
        }
        println!("{}", pw.into_iter().collect::<String>());
    }
}

pub mod scanner;

use std::{env::Args, io::BufRead, num::NonZeroUsize};

use anyhow::bail;

#[derive(Default)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn main(mut args: Args) -> anyhow::Result<()> {
        // skip the program name
        let _ = args.next().unwrap();

        if args.len() > 1 {
            println!("Usage: jlox [script]");
            bail!("64");
        } else if args.len() == 1 {
            let mut me = Self::default();

            me.run_file(args.next().unwrap())
        } else {
            let mut me = Self::default();
            me.run_prompt()
        }
    }

    fn run_file(&mut self, path: String) -> anyhow::Result<()> {
        println!("run_file");
        let bytes: Vec<u8> = std::fs::read(path)?;

        self.run(String::from_utf8(bytes)?);

        if self.had_error {
            bail!("65");
        }

        Ok(())
    }

    fn run_prompt(&mut self) -> anyhow::Result<()> {
        println!("run_prompt");
        let stdin = std::io::stdin();
        let mut buf = String::new();

        loop {
            print!("> ");

            let count = stdin.lock().read_line(&mut buf)?;

            if count == 0 {
                break;
            }

            self.run(buf.to_string());
            buf.clear();
            self.had_error = false;
        }

        Ok(())
    }

    fn run(&self, source: String) {
        let mut scanner = scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{token:?}");
        }
    }

    fn error(line: NonZeroUsize, message: String) {
        Self::report(line, String::new(), message);
    }

    fn report(line: NonZeroUsize, loc: String, message: String) {
        eprintln!("[line {line}] Error{loc}: {message}")
    }
}

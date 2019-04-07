use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct ResultFormatter {
    stdout: bool,
    stderr: bool,
}

impl ResultFormatter {
    pub fn new() -> Self {
        use atty::Stream;
        Self {
            stdout: atty::is(Stream::Stdout),
            stderr: atty::is(Stream::Stderr),
        }
    }

    pub fn print_success(&self, line: usize, url: &str) {
        if self.stdout {
            {
                let mut stream = StandardStream::stdout(ColorChoice::Always);
                let _ = stream.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
                let _ = stream.write(b"[Success]");
                let _ = stream.set_color(ColorSpec::new().set_fg(None));
            }
            println!(" #{} {}", line, url);
        } else {
            println!("#{} {}", line, url);
        }
    }

    pub fn print_error(&self, line: usize, url: &str) {
        if self.stderr {
            {
                let mut stream = StandardStream::stderr(ColorChoice::Always);
                let _ = stream.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                let _ = stream.write(b"[Failure]");
                let _ = stream.set_color(ColorSpec::new().set_fg(None));
            }
            eprintln!(" #{} {}", line, url);
        } else {
            eprintln!("{}", url);
        }
    }
}

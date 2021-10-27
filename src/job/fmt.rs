use owo_colors::OwoColorize;

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
            println!("{} #{} {}", "[Success]".green(), line, url);
        } else {
            println!("#{} {}", line, url);
        }
    }

    pub fn print_error(&self, line: usize, url: &str) {
        if self.stderr {
            eprintln!("{} #{} {}", "[Failure]".red(), line, url);
        } else {
            eprintln!("{}", url);
        }
    }
}

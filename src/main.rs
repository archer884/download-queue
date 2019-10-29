mod application;
mod download;
mod error;
mod fmt;
mod job;
mod opt;

fn main() -> error::Result<()> {
    use application::Application;
    use opt::Opt;

    Application::new(Opt::from_args()).run()
}

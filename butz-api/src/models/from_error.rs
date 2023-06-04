#[derive(Debug)]
pub enum FromError {
    TooLarge,
    Io(std::io::Error),
}

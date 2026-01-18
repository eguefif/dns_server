use std::fmt;

#[derive(Debug)]
pub enum DNSError {
    RequestHeaderSizeError(usize),
}

impl std::error::Error for DNSError {}

impl fmt::Display for DNSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DNSError::RequestHeaderSizeError(size) => {
                write!(f, "Request header size is wrong {}", size)
            }
        }
    }
}

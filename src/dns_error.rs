use std::fmt;

#[derive(Debug)]
pub enum DNSError {
    NoFollowServer,
    RequestHeaderSizeError(usize),
    FollowServerRequestError,
}

impl std::error::Error for DNSError {}

impl fmt::Display for DNSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DNSError::RequestHeaderSizeError(size) => {
                write!(f, "Request header size is wrong {}", size)
            }
            DNSError::FollowServerRequestError => {
                write!(f, "Follow Server response is malformatted")
            },
            DNSError::NoFollowServer => {
                write!(f, "Missing follow server address")
            }
        }
    }
}

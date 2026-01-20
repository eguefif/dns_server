use std::fmt;

#[derive(Debug)]
pub enum DNSError {
    NoFollowServer,
    HeaderSizeError(usize),
    QuestionSizeError,
    AnswerSizeError,
    FollowServerRequestError,
    FollowServerParseError,
    LabelParsingError(String),
}

impl std::error::Error for DNSError {}

impl fmt::Display for DNSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DNSError::HeaderSizeError(size) => {
                write!(f, "DNSMessage header size is wrong {}", size)
            }
            DNSError::QuestionSizeError => {
                write!(f, "DNSMessage question is too short")
            }
            DNSError::AnswerSizeError => {
                write!(f, "DNSMessage answer is too short")
            }
            DNSError::LabelParsingError(message) => {
                write!(f, "Label parsing went wront: {}", message)
            }
            DNSError::FollowServerRequestError => {
                write!(f, "Follow Server response is malformatted")
            }
            DNSError::NoFollowServer => {
                write!(f, "Missing follow server address")
            }
            DNSError::FollowServerParseError => {
                write!(f, "Follow Server response parsing error")
            }
        }
    }
}

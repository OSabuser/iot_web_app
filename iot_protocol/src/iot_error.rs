use std::error::Error;
use std::{fmt, io};

/// Ошибка отправки сообщения
#[derive(Debug)]
pub enum TransmissionError {
    Io(io::Error),
}

impl fmt::Display for TransmissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransmissionError::Io(e) => write!(f, "Internal IO error occured: {}", e),
        }
    }
}

impl From<io::Error> for TransmissionError {
    fn from(e: io::Error) -> Self {
        TransmissionError::Io(e)
    }
}

impl Error for TransmissionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let Self::Io(e) = self;
        Some(e)
    }
}

/// Ошибка приема сообщения
#[derive(Debug)]
pub enum ReceptionError {
    Io(io::Error),
    BadFormat,
    BadCRC,
}

impl fmt::Display for ReceptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReceptionError::Io(e) => write!(f, "Internal IO error occured: {}", e),
            ReceptionError::BadFormat => write!(f, "Incorrect message format!"),
            ReceptionError::BadCRC => write!(f, "Bad CRC!"),
        }
    }
}

impl From<io::Error> for ReceptionError {
    fn from(e: io::Error) -> Self {
        ReceptionError::Io(e)
    }
}

impl Error for ReceptionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReceptionError::Io(e) => Some(e),
            _ => None,
        }
    }
}

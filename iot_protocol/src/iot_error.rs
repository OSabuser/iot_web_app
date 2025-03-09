use std::error::Error;
use std::{fmt, io};

/// Ошибка при обмене данными с сервером.
#[derive(Debug)]
pub enum RequestError {
    /// Ошибка отправки.
    Send(TransmissionError),

    /// Ошибка приема.
    Recv(ReceptionError),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::Send(e) => write!(f, "send error: {e}"),
            RequestError::Recv(e) => write!(f, "recv error: {e}"),
        }
    }
}

impl From<TransmissionError> for RequestError {
    fn from(value: TransmissionError) -> Self {
        Self::Send(value)
    }
}

impl From<ReceptionError> for RequestError {
    fn from(value: ReceptionError) -> Self {
        Self::Recv(value)
    }
}

impl Error for RequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RequestError::Send(e) => Some(e),
            RequestError::Recv(e) => Some(e),
        }
    }
}

/// Ошибка соединения.
#[derive(Debug)]
pub enum ConnectError {
    /// Неудачный handshake.
    BadHandshake,

    /// Внутренняя ошибка IO.
    Io(io::Error),
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadHandshake => write!(f, "bad handshake"),
            Self::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<io::Error> for ConnectError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Error for ConnectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::BadHandshake => None,
        }
    }
}

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

use crate::iot_error::{ConnectError, RequestError};
use crate::iot_message::IotMessage;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// Клиент IoT.
pub struct IotClient {
    stream: TcpStream,
}

impl IotClient {
    /// Пытаемся подключится к серверу и проверяем, что он поддерживает STP.
    pub fn connect<Addrs>(addrs: Addrs) -> Result<Self, ConnectError>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Self::try_handshake(stream)
    }

    /// Проводим handshake, чтобы убедиться, что сервер поддерживает IoT protocol:
    /// 1) отправляем байты "iot_clnt",
    /// 1) ожидаем байты "iot_serv" в ответ.
    fn try_handshake(mut stream: TcpStream) -> Result<Self, ConnectError> {
        stream.write_all(b"iot_clnt")?;
        let mut buf = [0; 8];
        stream.read_exact(&mut buf)?;
        if &buf != b"iot_serv" {
            return Err(ConnectError::BadHandshake);
        }
        Ok(Self { stream })
    }

    /// Отправка запроса на сервер и получение ответа.
    pub fn send_request(&mut self, req: IotMessage) -> Result<IotMessage, RequestError> {
        crate::send_message(req, &mut self.stream)?;
        let response = crate::receive_message(&mut self.stream)?;
        Ok(response)
    }
}

use crate::iot_error::{ConnectError, RequestError};
use crate::iot_message::IotMessage;
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

/// IoT сервер.
pub struct IotServer {
    tcp: TcpListener,
}

impl IotServer {
    /// Закрепляем сервер на сокете.
    pub fn bind<Addrs>(addrs: Addrs) -> io::Result<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(Self { tcp })
    }

    /// Принимаем входящее соединение и производим handshake.
    pub fn accept(&self) -> Result<IotConnection, ConnectError> {
        let (stream, _) = self.tcp.accept()?;
        Self::try_handshake(stream)
    }

    /// Проводим handshake, чтобы убедиться, что клиент поддерживает IoT protocol:
    /// 1) ожидаем байты "iot_clnt",
    /// 1) отправляем байты "iot_serv" в ответ.
    fn try_handshake(mut stream: TcpStream) -> Result<IotConnection, ConnectError> {
        let mut buf = [0; 8];
        stream.read_exact(&mut buf)?;
        if &buf != b"iot_clnt" {
            return Err(ConnectError::BadHandshake);
        }
        stream.write_all(b"iot_serv")?;
        Ok(IotConnection { stream })
    }
}

/// Соединение с клиентом.
/// Позволяет обрабатывать запросы.
pub struct IotConnection {
    stream: TcpStream,
}

impl IotConnection {
    /// Обрабатываем запрос и возвращаем ответ используя логику
    /// предоставленную вызывающей стороной.
    pub fn process_request<F>(&mut self, message_handler: F) -> Result<(), RequestError>
    where
        F: FnOnce(IotMessage) -> IotMessage,
    {
        let request = super::receive_message(&mut self.stream)?;
        let response = message_handler(request);
        super::send_message(response, &mut self.stream)?;
        Ok(())
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}

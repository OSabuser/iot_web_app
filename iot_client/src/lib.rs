use iot_protocol::iot_client::IotClient;
use iot_protocol::iot_error::{ConnectError, RequestError};
use iot_protocol::iot_message::{CommandType, IotMessage};
use std::net::ToSocketAddrs;
/// Клиент чата.
pub struct SmartClient {
    clnt: IotClient,
}

impl SmartClient {
    /// Подключаемся к серверу.
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> Result<Self, ConnectError> {
        let clnt = IotClient::connect(addr)?;
        Ok(Self { clnt })
    }

    /// Включение розетки.
    pub fn turn_on(&mut self) -> Result<String, RequestError> {
        let request = IotMessage::new(47, CommandType::SetPowerOn, "Dummy".to_string());
        let response = self.clnt.send_request(request)?;
        Ok(response.get_message_data())
    }

    /// Выключение розетки.
    pub fn turn_off(&mut self) -> Result<String, RequestError> {
        let request = IotMessage::new(47, CommandType::SetPowerOff, "Dummy".to_string());
        let response = self.clnt.send_request(request)?;
        Ok(response.get_message_data())
    }

    /// Получение состояния розетки
    pub fn get_state(&mut self) -> Result<String, RequestError> {
        let request = IotMessage::new(47, CommandType::GetStatus, "Dummy".to_string());
        let response = self.clnt.send_request(request)?;
        Ok(response.get_message_data())
    }
}

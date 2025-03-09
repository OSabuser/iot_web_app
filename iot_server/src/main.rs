use iot_protocol::iot_message::{CommandType, IotMessage};
use iot_protocol::iot_server::IotServer;
use smart_socket::{SmartDevicePowerState, SmartSocket}; // SmartSocket
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Читаем IP-адрес сервера из файла или используем значение по умолчанию.
    let addr = String::from("127.0.0.1:55331");
    let server = IotServer::bind(addr)?;

    // Создание инстансов умных устройств для имитации управления
    let mut my_smart_socket = SmartSocket::new("SmartSocket_1", 47);

    // Обрабатываем подключения клиентов.
    loop {
        let Ok(mut connection) = server.accept() else {
            continue;
        };

        // Обрабатываем запрос.
        connection.process_request(|req| {
            let device_id = req.get_id();
            // Парсинг сообщения, полученного от клиента
            let command = req.get_command_type();
            if device_id == 47 {
                let mut response_string = "Response: ".to_string();
                match command {
                    CommandType::SetPowerOn => {
                        if let Err(x) =
                            my_smart_socket.set_power_state(SmartDevicePowerState::Enabled)
                        {
                            response_string.push_str(x.to_string().as_str());
                        }
                        response_string.push_str("smart socket has been enabled");
                    }
                    CommandType::SetPowerOff => {
                        if let Err(x) =
                            my_smart_socket.set_power_state(SmartDevicePowerState::Disabled)
                        {
                            response_string.push_str(x.to_string().as_str());
                        }
                        response_string.push_str("smart socket has been disabled");
                    }
                    CommandType::GetStatus => {
                        response_string.push_str(my_smart_socket.get_text_report().as_str());
                    }
                }
                IotMessage::new(device_id, command, response_string)
            } else {
                IotMessage::new(
                    device_id,
                    command,
                    "Обращение к несуществующему устройству".to_string(),
                )
            }
        })?;
    }
}

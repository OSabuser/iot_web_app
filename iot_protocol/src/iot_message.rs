use crc16::{State, ARC};

pub const CRC_LENGTH: usize = 2;

/// Поддерживаемые команды
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CommandType {
    SetPowerOn = 0x01,
    SetPowerOff = 0x02,
    GetStatus = 0x03,
}

/// Структура посылки
#[derive(Debug, Clone)]
pub struct IotMessage {
    id: u8,
    command: CommandType,
    message_data: String,
    data_length: u16,
    crc: u16,
}

impl IotMessage {
    pub fn new(device_id: u8, command: CommandType, data: String) -> Self {
        let mut temp = IotMessage {
            id: device_id,
            command,
            data_length: data.trim().len() as u16,
            message_data: data,
            crc: 0,
        };
        temp.crc = temp.calculate_crc();
        temp
    }

    /// Получение вида команды
    pub fn get_command_type(&self) -> CommandType {
        self.command
    }

    /// Получение данных
    pub fn get_message_data(&self) -> String {
        self.message_data.clone()
    }

    /// Получение идентификатора устройства
    pub fn get_id(&self) -> u8 {
        self.id
    }

    /// Расчёт CRC16 по алгоритму ARC
    pub fn calculate_crc(&self) -> u16 {
        let mut state = State::<ARC>::new();
        state.update(&[self.id]);
        state.update(&[self.command as u8]);

        state.update(&[self.data_length.to_be_bytes()[0]]);
        state.update(&[self.data_length.to_be_bytes()[1]]);
        state.update(self.message_data.as_bytes());

        state.get()
    }

    /// Сериализация сообщения в "сырые" байты
    pub fn serialize_to_raw_byte_data(self) -> Vec<u8> {
        let mut raw_bytes = vec![
            self.id,
            self.command as u8,
            self.data_length.to_be_bytes()[0],
            self.data_length.to_be_bytes()[1],
        ];

        if self.data_length > 0 {
            raw_bytes.append(&mut self.message_data.as_bytes().to_vec());
        } else {
            raw_bytes.push(0);
        }

        raw_bytes.append(&mut self.crc.to_be_bytes().to_vec());
        raw_bytes
    }

    /// Десериализация сообщения из "сырых" байт
    pub fn deserialize_from_raw_byte_data(raw_bytes: Vec<u8>) -> Option<IotMessage> {
        let id = raw_bytes[0];
        let command = match raw_bytes[1] {
            1 => CommandType::SetPowerOn,
            2 => CommandType::SetPowerOff,
            3 => CommandType::GetStatus,
            _ => return None,
        };
        let data_length = u16::from_be_bytes([raw_bytes[2], raw_bytes[3]]);

        let message = IotMessage {
            id,
            command,
            data_length,
            message_data: String::from_utf8_lossy(&raw_bytes[4..(4 + data_length as usize)])
                .to_string(),
            crc: u16::from_be_bytes([
                raw_bytes[4 + data_length as usize],
                raw_bytes[4 + data_length as usize + 1],
            ]),
        };

        Some(message)
    }
}

/// Реализация оператора равенства для IotMessage
impl PartialEq for IotMessage {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.command == other.command
            && self.data_length == other.data_length
            && self.message_data == other.message_data
            && self.crc == other.crc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Проверка расчёта CRC
    /// [CRC Calculator](https://crccalc.com/?crc=123456789&method=&datatype=ascii&outtype=hex)
    #[test]
    fn test_crc() {
        let command = IotMessage::new(1, CommandType::SetPowerOn, "test".to_string());
        assert_eq!(command.crc, 0x340E);
    }

    /// Проверка сериализации
    #[test]
    fn test_serialize() {
        let command = IotMessage::new(1, CommandType::SetPowerOn, "test".to_string());
        assert_eq!(
            command.serialize_to_raw_byte_data(),
            vec![1, 1, 0, 4, 116, 101, 115, 116, 52, 14]
        );
    }

    /// Проверка десериализации
    #[test]
    fn test_deserialize() {
        let command = IotMessage::new(1, CommandType::SetPowerOn, "test".to_string());
        let raw_bytes = vec![1, 1, 0, 4, 116, 101, 115, 116, 52, 14];

        let the_same_command = IotMessage::deserialize_from_raw_byte_data(raw_bytes);

        assert_eq!(the_same_command.is_some(), true);

        if let Some(message) = the_same_command {
            assert_eq!(message, command);
        }
    }
}

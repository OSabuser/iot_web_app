use iot_error::{ReceptionError, TransmissionError};
use std::io::{Read, Write};

use iot_message::IotMessage;

pub mod iot_client;
pub mod iot_device;
pub mod iot_error;
pub mod iot_message;
pub mod iot_server;

/// Отправка сообщения
/// # Формат
/// Запрос: ID + команда + CRC
/// Отклик: ID + команда + длина данных + данные + CRC
fn send_message<Writer: Write>(
    message: &mut IotMessage,
    writer: &mut Writer,
) -> Result<(), iot_error::TransmissionError> {
    let raw_bytes = message.serialize_to_raw_byte_data();

    writer.write_all(raw_bytes.as_slice())?;

    Ok(())
}

/// Прием сообщения
///
fn receive_message<Reader: Read>(
    reader: &mut Reader,
) -> Result<IotMessage, iot_error::ReceptionError> {
    let mut raw_bytes = [0; 4];
    let mut raw_message: Vec<u8> = Vec::new();

    reader.read_exact(&mut raw_bytes)?;
    raw_message.append(&mut raw_bytes.as_slice().to_vec());

    let data_length = u16::from_be_bytes([raw_bytes[2], raw_bytes[3]]);

    if data_length > 0 {
        let mut message_data = vec![0; data_length as usize]; // data field
        let mut crc = [0; 2]; // CRC
        reader.read_exact(&mut message_data)?;
        raw_message.append(&mut message_data);

        reader.read_exact(&mut crc)?;
        raw_message.append(&mut crc.to_vec());
    } else {
        let mut raw_bytes = [0; 3]; // CRC + zero byte
        reader.read_exact(&mut raw_bytes)?;
        raw_message.append(&mut raw_bytes.to_vec());
    }

    Ok(IotMessage::deserialize_from_raw_byte_data(raw_message))
}

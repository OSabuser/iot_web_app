use iot_client::SmartClient;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let addr = get_server_addr();

    // Читаем аргументы командной строки.
    let mut cli_args = std::env::args().skip(1);
    let Some(action) = cli_args.next() else {
        return Err(
            String::from("No action provided, use 'enable', 'disable', or 'status'").into(),
        );
    };

    println!("Performing action: {action}...");

    // Соединяемся с сервером чата.
    let mut client = SmartClient::new(addr)?;

    if action == "enable" {
        // Включение розетки
        let status = client.turn_on()?;
        println!("{}", status);
        return Ok(());
    }

    if action == "disable" {
        // Выключение розетки
        let status = client.turn_off()?;
        println!("{}", status);
        return Ok(());
    }

    if action == "status" {
        // Получение статуса работы розетки
        let status = client.get_state()?;
        println!("{}", status);
        return Ok(());
    }

    Err(String::from("Unknown action, use 'enable', 'disable', or 'status'").into())
}

fn get_server_addr() -> String {
    String::from("127.0.0.1:55331")
}

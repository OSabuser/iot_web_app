///
/// Тип описывающий характеристики и поведение девайса "Умная розетка"
///
pub struct SmartSocket {
    /// Пользовательский псевдоним для розетки
    pub name: (String, u8),

    /// Текущая мощность (Вт), потребляемая подключёнными к розетке устройствами
    power_consumption: f32,

    // Cтатус работы (ВКЛ,ВЫКЛ/ОШИБКА)
    status: SmartDeviceStatus,
}

#[derive(Clone)]
pub enum SmartDeviceStatus {
    /// Состояние питания умного устройства
    PowerState(SmartDevicePowerState),
    /// Возможные ошибки в работе умного устройства
    Malfunction(SmartDeviceErrorCode),
}
#[derive(Clone)]
pub enum SmartDeviceErrorCode {
    /// Ошибка: перегрузка по току
    Overcurrent,

    /// Ошибка: перегрузка по напряжению
    Overvoltage,

    /// Ошибка: перегрев
    Overheat,

    /// Ошибка: слишком низкая температура
    Underheat,
}

/// Перечисление возможных состояний питания умного устройства
#[derive(Clone)]
pub enum SmartDevicePowerState {
    /// Устройство включено
    Enabled,
    /// Устройство выключено
    Disabled,
}

impl SmartSocket {
    /// Создание экземпляра умной розетки с псевдонимом `name`
    ///
    /// По умолчанию розетка выключена, потребление - `0.0 Вт`
    ///
    /// ## Пример
    /// ```ignore
    /// let my_plug = SmartSocket::new("MyPreciousPlug_1");
    /// ```
    ///
    pub fn new(name: &str, id: u8) -> Self {
        Self {
            name: (name.to_string(), id),
            power_consumption: 0.0,
            status: SmartDeviceStatus::PowerState(SmartDevicePowerState::Disabled),
        }
    }

    pub fn set_power_state(
        &mut self,
        state: SmartDevicePowerState,
    ) -> Result<(), SmartDeviceErrorCode> {
        match &self.status {
            SmartDeviceStatus::PowerState(_) => {
                self.status = SmartDeviceStatus::PowerState(state);
                Ok(())
            }
            SmartDeviceStatus::Malfunction(y) => Err((*y).clone()),
        }
    }

    /// Получение имени устройства
    pub fn get_name(&self) -> &str {
        &self.name.0
    }

    /// Получение идентификатора устройства
    pub fn get_id(&self) -> u8 {
        self.name.1
    }

    /// Получение текстовой информации о состоянии устройства
    pub fn get_text_report(&self) -> String {
        format!(
            "#{}: current power consumption is {}, status: {} \n",
            self.name.0, self.power_consumption, self.status
        )
    }
}

use std::fmt::{self, Display};

impl Display for SmartDevicePowerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Disabled => write!(f, "Disabled."),
            Self::Enabled => write!(f, "Enabled."),
        }
    }
}
impl Display for SmartDeviceErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Underheat => write!(f, "Underheat error."),
            Self::Overcurrent => write!(f, "Overcurrent error."),
            Self::Overheat => write!(f, "Overheat error."),
            Self::Overvoltage => write!(f, "Overvoltage error."),
        }
    }
}
impl Display for SmartDeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Malfunction(x) => write!(f, "{}", x),
            Self::PowerState(y) => write!(f, "{}", y),
        }
    }
}

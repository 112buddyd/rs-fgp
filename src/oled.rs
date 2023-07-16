use esp_idf_hal::gpio::{Gpio21, Gpio22};
use esp_idf_hal::i2c::*;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::prelude::*;
use ssd1306::mode::TerminalMode;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

pub fn setup<'a>(
    _i2c: I2C0,
    sda: Gpio21,
    scl: Gpio22,
) -> Ssd1306<I2CInterface<I2cDriver<'a>>, DisplaySize128x64, TerminalMode> {
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(_i2c, sda, scl, &config).unwrap();

    let interface = I2CDisplayInterface::new(i2c);

    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();
    display.init().unwrap();
    display.clear().unwrap();
    display
}

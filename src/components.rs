use crate::keypad::Keypad;
use crate::oled;
use esp_idf_hal::{
    gpio::{Gpio14, Gpio16, Gpio17, Gpio18, Gpio19, Gpio23, Gpio4, Input, Output, PinDriver},
    i2c::I2cDriver,
    peripherals::Peripherals,
};
use esp_idf_sys as _;
use smart_leds_trait::RGB8;
use ssd1306::{mode::TerminalMode, prelude::I2CInterface, size::DisplaySize128x64, Ssd1306};
use std::time::Instant;
use ws2812_esp32_rmt_driver::{driver::color::LedPixelColorGrb24, LedPixelEsp32Rmt};

pub const LED_COUNT: usize = 9;

pub struct Components<'a> {
    pub time: Instant,
    pub keypad: Keypad<
        PinDriver<'a, Gpio4, Input>,
        PinDriver<'a, Gpio16, Input>,
        PinDriver<'a, Gpio17, Input>,
        PinDriver<'a, Gpio18, Output>,
        PinDriver<'a, Gpio19, Output>,
        PinDriver<'a, Gpio23, Output>,
    >,
    pub leds: LedPixelEsp32Rmt<RGB8, LedPixelColorGrb24>,
    pub buzzer: PinDriver<'a, Gpio14, Output>,
    pub display: Ssd1306<I2CInterface<I2cDriver<'a>>, DisplaySize128x64, TerminalMode>,
}

impl Components<'_> {
    pub fn new() -> Self {
        println!("Setting up components.");
        let peripherals = Peripherals::take().unwrap();
        let time = Instant::now();

        // KEYPAD
        let rows = (
            PinDriver::input(peripherals.pins.gpio4).unwrap(),
            PinDriver::input(peripherals.pins.gpio16).unwrap(),
            PinDriver::input(peripherals.pins.gpio17).unwrap(),
        );
        let columns = (
            PinDriver::output_od(peripherals.pins.gpio18).unwrap(),
            PinDriver::output_od(peripherals.pins.gpio19).unwrap(),
            PinDriver::output_od(peripherals.pins.gpio23).unwrap(),
        );
        let keypad = Keypad::new(rows, columns);

        // LED
        let leds = LedPixelEsp32Rmt::<RGB8, LedPixelColorGrb24>::new(0, 13).unwrap();

        // BUZZER
        let buzzer = PinDriver::output(peripherals.pins.gpio14).unwrap();

        // OLED
        let i2c = peripherals.i2c0;
        let sda = peripherals.pins.gpio21;
        let scl = peripherals.pins.gpio22;
        let display = oled::setup(i2c, sda, scl);

        Components {
            time,
            keypad,
            leds,
            buzzer,
            display,
        }
    }
}

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use esp_idf_hal::delay::Ets;

const KEY_0: u16 = 1 << 7;
const KEY_1: u16 = 1;
const KEY_2: u16 = 1 << 4;
const KEY_3: u16 = 1 << 8;
const KEY_4: u16 = 1 << 1;
const KEY_5: u16 = 1 << 5;
const KEY_6: u16 = 1 << 9;
const KEY_7: u16 = 1 << 2;
const KEY_8: u16 = 1 << 6;

// Defines a type that makes it easier to supply the four pins required for rows in the keypad.
// These pins need to support the `embedded_hal::digital::v2::InputPin` trait
pub type Rows<R0, R1, R2> = (R0, R1, R2);

// Defines a type that makes it easier to supply the four pins required for rows in the keypad
// These pins need to support the `embedded_hal::digital::v2::OutputPin` trait
pub type Columns<C0, C1, C2> = (C0, C1, C2);

// Manages the pins and the logic for scanning a keypad
pub struct Keypad<
    R0: InputPin,
    R1: InputPin,
    R2: InputPin,
    C0: OutputPin,
    C1: OutputPin,
    C2: OutputPin,
> {
    rows: Rows<R0, R1, R2>,
    columns: Columns<C0, C1, C2>,
}

impl<R0: InputPin, R1: InputPin, R2: InputPin, C0: OutputPin, C1: OutputPin, C2: OutputPin>
    Keypad<R0, R1, R2, C0, C1, C2>
{
    // Create a new instance of this structure
    pub fn new(rows: Rows<R0, R1, R2>, columns: Columns<C0, C1, C2>) -> Self {
        Self { rows, columns }
    }

    // Reads a character from the keypad. This method returns even if no keys are pressed.
    // It will return:
    //
    //  `'0'` through `'9'`
    //  `'*'`
    //  `'#'`
    //  `' '` if no keys are pressed.

    pub fn read_char(&mut self) -> Option<char> {
        let raw = self.read();
        if raw == 0 {
            return None;
        }
        Some(self.get_char(raw))
    }

    // Performs a "raw" read of the keypad and returns a bit set for each key down. Note,
    // this doesn't mean this code supports multiple key presses.
    fn read(&mut self) -> u16 {
        let mut res = 0;

        self.columns.0.set_low().unwrap_or_default();
        res |= self.read_column() << 0;
        self.columns.0.set_high().unwrap_or_default();

        self.columns.1.set_low().unwrap_or_default();
        res |= self.read_column() << 4;
        self.columns.1.set_high().unwrap_or_default();

        self.columns.2.set_low().unwrap_or_default();
        res |= self.read_column() << 8;
        self.columns.2.set_high().unwrap_or_default();

        res
    }

    // Converts the raw value from the read() method into a character that corresponds to the
    // label on each key
    fn get_char(&self, raw_value: u16) -> char {
        let value = self.convert(raw_value);
        char::from_digit(value as u32, 10).unwrap()
    }

    fn read_column(&self) -> u16 {
        let mut res = 0;

        Ets.delay_ms(1u16);
        if self.rows.0.is_low().unwrap_or_default() {
            res |= 1 << 0;
        }
        if self.rows.1.is_low().unwrap_or_default() {
            res |= 1 << 1;
        }
        if self.rows.2.is_low().unwrap_or_default() {
            res |= 1 << 2;
        }
        res
    }

    // Converts the raw value (2^N) from the read() method into a keypad digit. This will be
    //      0..9    digits
    pub fn convert(&self, value: u16) -> i16 {
        match value {
            KEY_0 => 0,
            KEY_1 => 1,
            KEY_2 => 2,
            KEY_3 => 3,
            KEY_4 => 4,
            KEY_5 => 5,
            KEY_6 => 6,
            KEY_7 => 7,
            KEY_8 => 8,
            _ => -10,
        }
    }
}

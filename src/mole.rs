use crate::components::{Components, LED_COUNT};
use esp_idf_hal::delay::FreeRtos;
use rand::seq::SliceRandom;
use smart_leds::RGB8;
use smart_leds_trait::SmartLedsWrite;
use std::{fmt::Write, thread::sleep, time::Duration};

const DEFAULT_MOLE_ESCAPE_MS: u128 = 4000;
const DEFAULT_SPAWN_SPEED_MS: u128 = 3000;
const DEFAULT_ESCAPE_SPEED_CHANGE: f32 = 1.0;
const DEFAULT_SPAWN_SPEED_CHANGE: f32 = 0.9;
const GREEN: RGB8 = RGB8::new(0, 255, 0);
const YELLOW: RGB8 = RGB8::new(255, 255, 0);
const RED: RGB8 = RGB8::new(255, 0, 0);
const BLANK: RGB8 = RGB8::new(0, 0, 0);

pub struct Mole<'a> {
    components: Components<'a>,
    score: u8,
    mole_timers: [u128; LED_COUNT],
    spawn_timer: u128,
    spawn_speed_ms: u128,
    spawn_speed_change: f32,
    mole_escape_ms: u128,
    escape_speed_change: f32,
    round_num: u8,
    hits: u8,
    lives: u8,
}

impl Mole<'_> {
    pub fn new<'a>(components: Components<'a>) -> Mole<'a> {
        Mole {
            components,
            score: 0,
            mole_timers: [0; LED_COUNT],
            spawn_timer: 0,
            spawn_speed_ms: DEFAULT_SPAWN_SPEED_MS,
            spawn_speed_change: DEFAULT_SPAWN_SPEED_CHANGE,
            mole_escape_ms: DEFAULT_MOLE_ESCAPE_MS,
            escape_speed_change: DEFAULT_ESCAPE_SPEED_CHANGE,
            round_num: 1,
            hits: 0,
            lives: 5,
        }
    }

    fn time_diff(&self, ms: u128) -> u128 {
        self.components.time.elapsed().as_millis() - ms
    }

    fn mole_is_running(&self, idx: usize) -> bool {
        !self.mole_is_stopped(idx) && self.mole_timers[idx] != 0
    }

    fn mole_is_stopped(&self, idx: usize) -> bool {
        self.mole_timers[idx] == 0
    }

    fn mole_has_escaped(&self, idx: &usize) -> bool {
        !self.mole_is_stopped(*idx) && self.time_diff(self.mole_timers[*idx]) > self.mole_escape_ms
    }

    fn render_moles(&self) {
        unimplemented!()
        //   // using interpolation color moles based on how close they are to expiring
        //   if ((millis() - lastFrame) < 1000 / FPS) {
        //     return;
        //   }
        //   for (int i=0; i<LED_COUNT; i++) {
        //     if (mole_is_running(i)) {
        //       unsigned long delta = millis() - moleTimers[i];
        //       leds[i].red = colorOne.red * (moleEscapeMs - delta) / moleEscapeMs + colorTwo.red *delta/ moleEscapeMs;
        //       // leds[i].blue = colorOne[i].blue * (moleEscapeMs - delta) / moleEscapeMs + colorTwo[i].blue *delta/ moleEscapeMs;
        //       leds[i].green = colorOne.green * (moleEscapeMs - delta) / moleEscapeMs + colorTwo.green *delta/ moleEscapeMs;
        //     } else {
        //       leds[i] = CRGB(0, 0, 0);
        //     }
        //   }
        //   lastFrame = millis();
        //   FastLED.show();
    }

    fn spawn_random_mole(&mut self) {
        // randomly choose to spawn new mole in empty hole and start timer
        let stopped: Vec<usize> = self
            .mole_timers
            .iter()
            .filter(|t| t.to_string() == "0") // this seems silly
            .enumerate()
            .map(|(i, _)| i)
            .collect();
        if stopped.len() == 0 {
            return;
        }
        let new_idx = stopped.choose(&mut rand::thread_rng()).unwrap();
        self.mole_timers[*new_idx] = self.components.time.elapsed().as_millis();
    }

    fn check_for_mole_hits(&mut self) {
        let ckey = &mut self.components.keypad.read_char();
        if ckey.is_none() {
            return;
        }
        let key = ckey.unwrap() as usize;
        if self.mole_is_running(key) {
            self.hits += 1;
            self.score += 1;
            self.reset_mole(&key);
        }
    }

    fn reset_mole(&mut self, idx: &usize) {
        self.mole_timers[*idx] = 0;
    }

    fn check_for_mole_escapes(&mut self) {
        //   // check to see if mole's timer timed out
        //   // if so decrement lives
        //   // then reset mole
        let escapees = self
            .mole_timers
            .iter()
            .enumerate()
            .filter(|(i, _)| self.mole_has_escaped(&i))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        self.lives -= escapees.len() as u8;
        escapees.into_iter().for_each(|i| {
            self.reset_mole(&i);
        });
    }

    fn mole_game_reset(&mut self) {
        self.score = 0;
        self.mole_timers = [0; LED_COUNT];
        self.spawn_timer = 0;
        self.spawn_speed_ms = DEFAULT_SPAWN_SPEED_MS;
        self.spawn_speed_change = DEFAULT_SPAWN_SPEED_CHANGE;
        self.mole_escape_ms = DEFAULT_MOLE_ESCAPE_MS;
        self.escape_speed_change = DEFAULT_ESCAPE_SPEED_CHANGE;
        self.round_num = 1;
        self.hits = 0;
        self.lives = 5;
    }

    fn display_startup(&mut self) {
        let _ = self.components.display.clear();
        let _ = self.components.display.set_position(0, 0);
        let _ = self.components.display.write_str("FINN'S\n");
        let _ = self.components.display.write_str("GAME\n");
        let _ = self.components.display.write_str("PAD\n");
        FreeRtos::delay_ms(2000);
    }

    fn start_animation(&mut self) {
        let _ = self.components.display.clear();
        let _ = self.components.display.set_position(0, 0);
        let _ = self.components.display.write_str("WHACKAMOLE\n");
        let _ = self.components.display.write_str("GO GO GO\n");
        FreeRtos::delay_ms(2000);

        let mut pixels = std::iter::repeat(RED).take(LED_COUNT);
        self.components.leds.write(pixels).unwrap();
        sleep(Duration::from_millis(750));
        pixels = std::iter::repeat(YELLOW).take(LED_COUNT);
        self.components.leds.write(pixels).unwrap();
        sleep(Duration::from_millis(750));
        pixels = std::iter::repeat(GREEN).take(LED_COUNT);
        self.components.leds.write(pixels).unwrap();
        sleep(Duration::from_millis(1250));
    }

    fn end_animation(&mut self) {
        let _ = self.components.display.clear();
        let _ = self.components.display.set_position(0, 0);
        let _ = self.components.display.write_str("GAME OVER\n");
        let _ = self.components.display.write_str("Score: ");
        let _ = self
            .components
            .display
            .write_str(format!("{}", &self.score).as_str());

        for _ in 0..3 {
            let reds = std::iter::repeat(RED).take(LED_COUNT);
            self.components.leds.write(reds).unwrap();
            sleep(Duration::from_millis(500));
            let blanks = std::iter::repeat(BLANK).take(LED_COUNT);
            self.components.leds.write(blanks).unwrap();
            sleep(Duration::from_millis(500));
        }
    }

    pub fn run(&mut self) -> u8 {
        self.start_animation();

        self.spawn_timer = self.components.time.elapsed().as_millis();

        while self.lives > 0 {
            while self.hits < 10 {
                self.draw_game_state();
                self.check_for_mole_escapes();
                if self.lives <= 0 {
                    self.end_animation();
                    return self.score;
                }
                if self.time_diff(self.spawn_timer) > self.spawn_speed_ms {
                    self.spawn_random_mole();
                    self.spawn_timer = self.components.time.elapsed().as_millis();
                }
                self.check_for_mole_hits();
                self.render_moles();
            }
            self.hits = 0;
            self.spawn_speed_ms = (self.spawn_speed_ms as f32 * self.spawn_speed_change) as u128;
            self.mole_escape_ms = (self.mole_escape_ms as f32 * self.escape_speed_change) as u128;
            self.lives += 1;
            self.round_num += 1;
        }
        self.end_animation();
        self.score
    }

    fn draw_game_state(&mut self) {
        let _ = self.components.display.clear();
        let _ = self.components.display.set_position(0, 0);
        let _ = self.components.display.write_str("GAME OVER!\n");
        let _ = self.components.display.write_str("Score: ");
        let _ = self
            .components
            .display
            .write_str(format!("{}", &self.score).as_str());
    }

    fn draw_new_game_screen(&mut self) {
        let _ = self.components.display.clear();
        let _ = self.components.display.set_position(0, 0);
        let _ = self.components.display.write_str("WHACKAMOLE\n");
        let _ = self.components.display.write_str("Hit a key!");
    }
}

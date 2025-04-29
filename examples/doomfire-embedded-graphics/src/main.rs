use doomfire::DoomFire;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{thread, time::Duration};

const SCREEN_WIDTH: usize = 128;
const SCREEN_HEIGHT: usize = 128;

fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));

    let output_settings = OutputSettingsBuilder::new().scale(5).build();
    let mut window = Window::new("DoomFire", &output_settings);

    let mut doom_fire = DoomFire::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    'running: loop {
        display.clear(Rgb565::BLACK)?;
        doom_fire.update();
        doom_fire.draw(&mut display)?;

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
        thread::sleep(Duration::from_millis(10));
    }
}

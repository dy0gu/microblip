use embassy_time::Duration;
use microbit_bsp::{
    LedMatrix,
    display::{Brightness, Frame},
};

use super::SOUND_SIGNAL;

#[embassy_executor::task]
pub async fn led_updater(mut display: LedMatrix) {
    const DISPLAY_DURATION: Duration = Duration::from_millis(300);
    display.set_brightness(Brightness::MAX);

    loop {
        let history = SOUND_SIGNAL.wait().await;
        let led_frame = sound_to_led_frame(history);
        display.display(led_frame, DISPLAY_DURATION).await;
    }
}

fn sound_to_led_frame(history: [u8; 5]) -> Frame<5, 5> {
    let mut led_frame = Frame::<5, 5>::empty();
    const MAX_ROWS: usize = 5;

    // Each column maps to a historical reading: col 0 = newest, col 4 = oldest
    // Scrolls the sound history from right to left
    for (col, _) in history.iter().enumerate() {
        let level = history[MAX_ROWS - 1 - col];
        let height: usize = match level {
            0..=9 => 0,
            10..=49 => 1,
            50..=99 => 2,
            100..=149 => 3,
            150..=199 => 4,
            200..=255 => MAX_ROWS,
        };
        for row in (MAX_ROWS - height)..MAX_ROWS {
            led_frame.set(col, row);
        }
    }

    led_frame
}

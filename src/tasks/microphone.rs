use microbit_bsp::mic::Microphone;

use super::SOUND_SIGNAL;

#[embassy_executor::task]
pub async fn sound_listener(mut microphone: Microphone<'static>) {
    let mut history: [u8; 5] = [0; 5];
    loop {
        let sound_level = microphone.sound_level().await;
        if history[4] != sound_level {
            // Shift history left, newest reading goes into the rightmost slot
            history[0] = history[1];
            history[1] = history[2];
            history[2] = history[3];
            history[3] = history[4];
            history[4] = sound_level;
            defmt::info!("SOUND LEVEL: {}", sound_level);
            SOUND_SIGNAL.signal(history);
        }
    }
}

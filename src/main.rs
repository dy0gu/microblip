#![no_std]
#![no_main]

mod tasks;

use embassy_executor::Spawner;
use microbit_bsp::{
    Microbit,
    embassy_nrf::{bind_interrupts, saadc::InterruptHandler},
    mic::Microphone,
};
use {defmt_rtt as _, panic_probe as _};

use tasks::display::led_updater;
use tasks::microphone::sound_listener;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = Microbit::default();
    defmt::info!("STARTED");

    bind_interrupts!(
        struct InterruptRequests {
            SAADC => InterruptHandler;
        }
    );

    let irqs = InterruptRequests {};
    let microphone = Microphone::new(board.saadc, irqs, board.microphone, board.micen);

    spawner.spawn(sound_listener(microphone)).unwrap();
    spawner.spawn(led_updater(board.display)).unwrap();
}

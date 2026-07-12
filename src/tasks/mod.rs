pub mod display;
pub mod microphone;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};

pub static SOUND_SIGNAL: Signal<CriticalSectionRawMutex, [u8; 5]> = Signal::new();

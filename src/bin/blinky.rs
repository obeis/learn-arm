#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut green_led = Output::new(p.PD10, Level::High, Speed::Low);
    let mut yellow_led = Output::new(p.PD13, Level::High, Speed::Low);
    let mut red_led = Output::new(p.PB7, Level::High, Speed::Low);

    loop {
        info!("green on");
        green_led.set_high();
        Timer::after_millis(1000).await;

        info!("green off");
        green_led.set_low();
        info!("yellow on");
        yellow_led.set_high();
        Timer::after_millis(1000).await;

        info!("yellow off");
        yellow_led.set_low();
        info!("red on");
        red_led.set_high();
        Timer::after_millis(1000).await;
        red_led.set_low();
    }
}

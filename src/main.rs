#![no_std]
#![no_main]

use panic_halt as _; // Panic handler

use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

use hal::{
    pac::USART1,
    prelude::*,
    serial::{Config, Rx, Serial},
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("init");

    // Get peripherals
    let dp = hal::pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = dp.GPIOA.split();
    let rx_pin = gpioa.pa10; // RX

    // Configure USART1 with a baud rate of 115200
    let mut rx: Rx<USART1, u8> =
        Serial::rx(dp.USART1, rx_pin, Config::default().baudrate(115200.bps()), &clocks).unwrap();

    let mut buffer = [0u8; 32];
    let mut index = 0;

    // Read bytes from the UART
    loop {
        if let Ok(byte) = rx.read() {
            if index == 0 && byte != 32 {
                // Ignore bytes until the first byte is 32 (space character)
                continue;
            }

            buffer[index] = byte;
            index += 1;

            // If the 32nd byte is encountered, process the buffer
            if index == 32 {
                rprintln!("Data: {:?}", buffer);
                index = 0; // Reset index for the next packet
            }
        }
    }
}

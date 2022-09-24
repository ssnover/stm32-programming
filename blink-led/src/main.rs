#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::gpio::GpioExt;
use stm32f3xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC.constrain();
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);

    let mut led = gpiob
        .pb13
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    loop {
        led.toggle().unwrap();
        asm::delay(8_000_000);
    }
}

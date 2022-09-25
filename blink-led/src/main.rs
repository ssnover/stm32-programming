#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::gpio::GpioExt;
use stm32f3xx_hal::{delay::Delay, pac, prelude::*};

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut rcc = peripherals.RCC.constrain();
    let systick = core.SYST;
    let mut flash = peripherals.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);

    let mut delay = Delay::new(systick, clocks);
    let mut led = gpiob
        .pb13
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    loop {
        led.toggle().unwrap();
        delay.delay_ms(500u32);
    }
}

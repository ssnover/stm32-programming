#![no_std]
#![no_main]

use core::convert::Infallible;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::gpio::GpioExt;
use stm32f3xx_hal::hal::digital::v2::OutputPin;
use stm32f3xx_hal::{delay::Delay, pac, prelude::*};

type Output = dyn OutputPin<Error = Infallible>;

struct PovToy<'a> {
    pub leds: [&'a mut Output; 8],
}

impl<'a> PovToy<'a> {
    pub fn set_state(&mut self, state: u8) {
        for idx in 0..self.leds.len() {
            if (state & (1u8 << idx as u8)) != 0 {
                self.leds[idx].set_high().unwrap();
            } else {
                self.leds[idx].set_low().unwrap();
            }
        }
    }

    pub fn display(&mut self, state: u8, delay: &mut Delay) {
        self.set_state(state);
        delay.delay_ms(2u32);
    }
}

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut rcc = peripherals.RCC.constrain();
    let systick = core.SYST;
    let mut flash = peripherals.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);

    let mut led_0 = gpiob
        .pb0
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut led_1 = gpiob
        .pb1
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut led_2 = gpiob
        .pb2
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut led_3 = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut led_4 = gpiob
        .pb4
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut led_5 = gpiob
        .pb5
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut led_6 = gpiob
        .pb6
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut led_7 = gpiob
        .pb7
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let pov_toy = PovToy {
        leds: [
            &mut led_0, &mut led_1, &mut led_2, &mut led_3, &mut led_4, &mut led_5, &mut led_6,
            &mut led_7,
        ],
    };
    let delay = Delay::new(systick, clocks);

    app_main(pov_toy, delay);
}

fn app_main(mut pov_toy: PovToy, mut delay: Delay) -> ! {
    loop {
        pov_toy.display(0b00001110, &mut delay);
        pov_toy.display(0b00011000, &mut delay);
        pov_toy.display(0b10111101, &mut delay);
        pov_toy.display(0b01110110, &mut delay);
        pov_toy.display(0b00111100, &mut delay);
        pov_toy.display(0b00111100, &mut delay);
        pov_toy.display(0b00111100, &mut delay);
        pov_toy.display(0b01110110, &mut delay);
        pov_toy.display(0b10111101, &mut delay);
        pov_toy.display(0b00011000, &mut delay);
        pov_toy.display(0b00001110, &mut delay);

        pov_toy.set_state(0);
        delay.delay_ms(10u32);
    }
}

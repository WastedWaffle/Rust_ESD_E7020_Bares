#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

//extern crate panic_halt;
extern crate cortex_m;

use panic_halt as _;	
use cortex_m::asm;
use cortex_m_rt::entry;
use stm32l0::stm32l0x2;
//use stm32l0xx_hal::{pac, prelude::*, pwm, rcc::Config};
use stm32l0xx_hal as _;


fn wait(i: u32) {
    for _ in 0..i {
        cortex_m::asm::nop(); // no operation (cannot be optimized out)
    }
}
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Initialize TIM2 for PWM
    let pwm = pwm::Timer::new(dp.TIM2, 4.khz(), &mut rcc);

    //#[cfg(feature = "stm32l0x1")]
    //let mut pwm = pwm.channel2.assign(gpioa.pb2);

    //One of theese should work 
    //#[cfg(feature = "stm32l0x2")]
    //#[cfg(features = ["stm32l0x2", "rt"])]
    #[cfg(any(feature = "stm32l0x2", feature = "stm32l0x3"))]
    let mut pwm = pwm.channel1.assign(gpiob.pb2);

    let max = pwm.get_max_duty();

        pwm.enable();

   
        //sound power = max
        pwm.set_duty(max);
        asm::bkpt();

        //sound power = 0.5*max
        pwm.set_duty(max / 2);
        asm::bkpt();
        
        //sound power = 0.25*max
        pwm.set_duty(max / 4);
        asm::bkpt();

        //sound power = 0.125*max
        pwm.set_duty(max / 8);
        asm::bkpt();

        //sound power = 0
        pwm.set_duty(0);
        asm::bkpt();

   
}

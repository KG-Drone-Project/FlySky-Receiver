#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use rtic::app;

#[app(device = stm32f4xx_hal::pac)]
mod app {
    use cortex_m::{asm, peripheral};
    use cortex_m_rt::entry;
    use core::cell::{Cell, RefCell};
    use stm32f4xx_hal::{
        pac::{self, TIM4},
        prelude::*,
        timer::{Timer, PwmInput}
    };

    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {

    }

    #[local]
    struct Local {
        monitor: PwmInput<TIM4>
    }


    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        rtt_init_print!();
        rprintln!("Init");

        let dp = ctx.device;

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();
    
        let gpioa = dp.GPIOA.split();
        let gpiod = dp.GPIOD.split();
        let gpiob = dp.GPIOB.split();
    
        // Configure a pin into TIM4_CH1 mode, which will be used to observe an input PWM signal.
        let pwm_reader_ch1 = gpiob.pb6;
        let pwm_reader_ch2 = gpiob.pb7;
    
        let monitor: PwmInput<TIM4> = Timer::new(dp.TIM4, &clocks).pwm_input(50.Hz(), (pwm_reader_ch1));
   

        (Shared {}, Local {monitor})
    }

    #[task(binds = TIM4, local=[monitor])]
    fn flysky_receiver(ctx: flysky_receiver::Context) {
        let duty_cycle = ctx.local.monitor.get_duty_cycle();

        rprintln!("FlySky duty cycle: {:?}", duty_cycle);
        ctx.local.monitor.clear_all_flags();
    }

}




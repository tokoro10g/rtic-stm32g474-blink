#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;

// Set free interrupts as dispatchers for software tasks
#[app(device=stm32g4xx_hal::stm32, peripherals=true, dispatchers=[TIM1_BRK_TIM15, TIM1_TRG_COM])]
mod app {
    use hal::gpio::{gpioa::PA5, Output, PushPull};
    use hal::prelude::*;
    use stm32g4xx_hal as hal;
    use systick_monotonic::*;

    #[shared]
    struct Shared {
        led: PA5<Output<PushPull>>,
    }

    #[local]
    struct Local {}

    #[monotonic(binds=SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut rcc = cx.device.RCC.constrain();
        let gpioa = cx.device.GPIOA.split(&mut rcc);
        let ccdr = rcc.freeze(hal::rcc::Config::default());
        let mono = MonoTimer::new(cx.core.SYST, ccdr.clocks.sys_clk.0);

        let led = gpioa.pa5.into_push_pull_output();

        blink::spawn().unwrap();
        blink2::spawn().unwrap();

        (Shared { led }, Local {}, init::Monotonics(mono))
    }

    /// Toggles LED and spawn the blink2 task after 1 s.
    #[task(priority=1, shared = [led])]
    fn blink(mut cx: blink::Context) {
        cx.shared.led.lock(|led| {
            led.toggle().unwrap();
        });
        blink::spawn_after(1.secs()).unwrap();
    }

    /// Toggles LED and spawn the blink task after 300 ms.
    #[task(priority=2, shared = [led])]
    fn blink2(mut cx: blink2::Context) {
        cx.shared.led.lock(|led| {
            led.toggle().unwrap();
        });
        blink2::spawn_after(300.millis()).unwrap();
    }
}

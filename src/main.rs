#![no_std]
#![no_main]
use embedded_hal::blocking::delay::DelayMs;
use panic_halt as _;

// Borrowed from the C SDK
const GPIO_FUN_GPIO :u8= 11;

#[riscv_rt::entry]
fn main() -> ! {
    let pac = bl702_pac::Peripherals::take().unwrap();
    // With arrayified gpio, each gpio_pinmode corresponds to a specific gpio_cfgctl register
    // eg pac.GLB.gpio_pinmode[1] == pac.GLB.gpio_cfgctl1
    // each gpio_cfgctl contains config for 2 GPIO pins
    // so:
    //   pac.GLB.gpio_pinmode[0].reg_gpio_0_ie == pac.GLB.gpio_cfgctl0.reg_gpio_0_ie
    //   pac.GLB.gpio_pinmode[0].reg_gpio_1_ie == pac.GLB.gpio_cfgctl0.reg_gpio_1_ie
    //   pac.GLB.gpio_pinmode[1].reg_gpio_0_ie == pac.GLB.gpio_cfgctl1.reg_gpio_2_ie
    //   pac.GLB.gpio_pinmode[1].reg_gpio_1_ie == pac.GLB.gpio_cfgctl1.reg_gpio_3_ie
    //   pac.GLB.gpio_pinmode[2].reg_gpio_0_ie == pac.GLB.gpio_cfgctl2.reg_gpio_4_ie
    // etc
    // We're setting pin2 as a push-pull output, exactly as the C SDK does
    pac.GLB.gpio_pinmode[1].modify(|_r,w| { 
        unsafe {w.reg_gpio_0_drv().bits(0);}
        unsafe {w.reg_gpio_0_func_sel().bits(GPIO_FUN_GPIO);} // TODO: check this first. should default to gpio though
        w.reg_gpio_0_ie().clear_bit();
        w.reg_gpio_0_pd().set_bit();
        w.reg_gpio_0_pu().clear_bit();
        w.reg_gpio_0_smt().clear_bit();
        w
    });

    // enable Output Enable for GPIO2
    pac.GLB.gpio_cfgctl34.modify(|_r,w| {
        w.reg_gpio_2_oe().set_bit()
    });

    // According to the flasher, we should be running at 144Mhz at startup
    // certainly seems plausible
    let mut delay = riscv::delay::McycleDelay::new(144_000_000);
    
    loop{
        // pin2 high
        pac.GLB.gpio_cfgctl32.modify(|_r,w| {
            w.reg_gpio_2_o().set_bit()
        });
        delay.delay_ms(1000);
        // pin2 low
        pac.GLB.gpio_cfgctl32.modify(|_r,w| {
            w.reg_gpio_2_o().clear_bit()
        });
        delay.delay_ms(1000);
    }
}

#![no_main]
#![no_std]

/**
 * Sample code for using OLED SSD1306 based on Adafruit-mbed driver converted to Rust
 * Serial UART is defined here 
 * A Timer for delay is  included. 
 * BLUE Button Interrupt is also included
 * No multitasking/Async is supported with this example. 
 * Run connect to connect to your Nucleo-STM32F767ZI
 * NOTE: The driver uses a mix of embedded-hal v0.2.7 and v1.0.0 since stm32f7xx hal is not implemented for embedded-hal1.0.0 at this time (18/07/2025)
 *       look at the Cargo.toml in the library folder to see the dependencies
 * 
 * Author : Mariwan Jalal 18/07/2025
 */

// dont forget to run -----  cargo build --target thumbv7em-none-eabihf or have the .cargo folder with config.toml included
use core::fmt::Write;
use adafruit::{adafruit_gfx_h::{AdafruitGFX, Drawable}, adafruit_ssd1306::DelayWrapper, adafruit_ssd1306_h::{AdafruitSSD1306, Display}};
use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use embedded_hal::delay::DelayNs;
use fugit::{Duration, ExtU32, HertzU32, Instant};
use stm32f7xx_hal::{gpio::{Edge, ExtiPin, GpioExt, Output, PinState, PushPull, PB11, PG6}, i2c::{BlockingI2c, Mode}, pac, rcc::{HSEClock, HSEClockMode, RccExt, PLLP}, rtc::{Rtc, RtcClock}, serial::{Config, Serial}};
mod time;
use crate::time::{MyTimer, Ticker};
use panic_halt as _;


static mut BUTTON_CALLBACK: Option<fn()> = None;


#[entry]
fn main() -> ! {

    //////FROM HAL /////
    // Acquire the GPIOC peripheral
    // NOTE: `dp` is the device peripherals from the `PAC` crate
    // let lse_var = LSEClock::new(LSEClockMode::Oscillator); LSE in HAL is not implemented for stm32f7xx -- we cannot use it /Mariwan
        let mut dp = pac::Peripherals::take().unwrap();
        let cp = cortex_m::Peripherals::take().unwrap(); // Access Cortex-M peripherals
        
        // Initialize the system clock
        let mut rcc = dp.RCC.constrain();
        // Configure the clock based on STM32IDECubeMX
        let clocks = rcc
            .cfgr
           // .lse(lse_var)   lse in HAL is not implemented
            .hse(HSEClock::new(HertzU32::MHz(8), HSEClockMode::Oscillator)) //8 MHz HSE crystal
            .sysclk(HertzU32::MHz(216)) // Set system clock to 216 MHz
            .pllq(9) // Set PLLQ
            .plln(216)
            .pllp(PLLP::Div2)
            .pllm(4)
            .freeze();
        hprintln!("Initializing clocks...");
        let gpiob = dp.GPIOB.split();  //LED digital out
        let gpioc =dp.GPIOC.split();    // Button Digital in
        let gpiod =dp.GPIOD.split();    // UART2 

        // Configure the serial instance
        let mut serial_config = Config::default();
        serial_config.baud_rate=HertzU32::Hz(115200);
        // Configure UART2 on PD5 (TX) and PD6 (RX)
        
        let tx = gpiod.pd5.into_alternate(); // Set PA9 to alternate function for TX
        let rx = gpiod.pd6.into_alternate(); // Set PA10 to alternate function for RX
        
        //new(usart: U, pins: PINS, clocks: &Clocks, config: Config)
        let serial = Serial::new(dp.USART2,(tx,rx), &clocks, serial_config );
        let (mut txU,mut rxU)=serial.split();
        let mut led1 = gpiob.pb0.into_push_pull_output_in_state(PinState::High);

        let mut but_blue=gpioc.pc13;
        let mut syscfg = dp.SYSCFG;
        but_blue.make_interrupt_source( &mut syscfg, &mut rcc.apb2);
        but_blue.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
        but_blue.enable_interrupt(&mut dp.EXTI);


       //Initialize tiker : 

        /*
        regs: RTC,
        prediv_s: u16,
        prediv_a: u8,
        clock_source: RtcClock,
        clocks: Clocks,
        apb1: &mut APB1,
        pwr: &mut PWR,
*/
       let mut rtc = Rtc::new(dp.RTC, 7999, 127, RtcClock::Hse { divider: (8) }, clocks, &mut rcc.apb1, &mut dp.PWR).expect("RTC initialization failed");
       let ticker =&mut Ticker::new(rtc);
       let mut my_timer=MyTimer::new(500000u32.micros().into(),ticker);
       //let mut my_timer=MyTimer::new_default(ticker);

       // Set the callback function
        unsafe {
            BUTTON_CALLBACK = Some(button_press_handler);
        }

        // Enable NVIC interrupt for EXTI15_10
        unsafe {
            cortex_m::peripheral::NVIC::unmask(stm32f7xx_hal::pac::Interrupt::EXTI15_10);
        }
        // Enable NVIC interrupt for EXTI15_10
        unsafe {
            cortex_m::peripheral::NVIC::unmask(stm32f7xx_hal::pac::Interrupt::EXTI15_10);
        }
        /**Configure LCD I2C  */
       let  sda = gpiob.pb9.into_alternate_open_drain(); // SDA pin
       let  scl = gpiob.pb8.into_alternate_open_drain(); // SCL pin
       //let  rst = gpiog.pg6.into_push_pull_output(); // SCL pin

        let mut myi2c = BlockingI2c::i2c1(dp.I2C1, (scl,sda), Mode::Standard { frequency: HertzU32::kHz(100) }, &clocks,  &mut rcc.apb1, 10000);
        let mut var_name: [u8; 2] = [0, 2];
        var_name[0]=0xA4;
        var_name[1]=0xA5;

        //                                fn new(ni2c: I2C, n_rst: GPIO, w: u8, h: u8, delay: DELAY, ngfx: AdafruitGFX) -> Self
        let mut delay =Delay::new(cp.SYST, HertzU32::MHz(216).raw());
        let mydelay= DelayWrapper::new(delay);
        let mut rst: PB11<Output<PushPull>> = gpiob.pb11.into_push_pull_output();
        rst.set_low();
        let gg: AdafruitGFX= AdafruitGFX::new(128, 32);
        let mut display=AdafruitSSD1306::new(myi2c, rst, mydelay,gg);
        display.clear_display();
        display.begin(1);
        display.splash();
        display.show();
        my_timer.set_duration(3000000.micros().into());
        my_timer.blocking_is_ready();
        display.gfx.write_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        //display.gfx.draw_circle(65, 12, 12, 1);
        display.show();
        hprintln!("\nTest this as we are using it");

        hprintln!("\nTest Scroll- r");
        display.scroll_diagnol_r();
        my_timer.set_duration(1000000.micros().into());
        my_timer.blocking_is_ready();

        hprintln!("\nTest Scroll- l");
        display.scroll_diagnol_l();
        my_timer.set_duration(1000000.micros().into());
        my_timer.blocking_is_ready();
        

        
        hprintln!("\nTest Scroll- u");
        display.scroll_vertical_u();
        my_timer.set_duration(1000000.micros().into());
        my_timer.blocking_is_ready();
        

        
        hprintln!("\nTest Scroll- LEFT");
        display.scroll_horizontal_l();
        my_timer.set_duration(1000000.micros().into());
        my_timer.blocking_is_ready();

        hprintln!("\nTest Scroll- RIGHT");
        display.scroll_horizontal_r();
        my_timer.set_duration(1000000.micros().into());        
        my_timer.blocking_is_ready();

    loop {
        led1.set_high();
        display.invert_display(true);
        my_timer.blocking_is_ready();
        led1.set_low();
        display.invert_display(false);
        my_timer.blocking_is_ready();
        txU.write_str("Test me Serial\r\n");
    }
}



use critical_section::{self, acquire};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _critical_section_1_0_acquire() {
    acquire();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _critical_section_1_0_release() {
    // No action needed; just return.
}

use crate::pac::interrupt;
#[interrupt]
fn QUADSPI() {
    // dummy QUADSPI interrupt handler or you get link error, must be a BUG!!!
}



// Interrupt handler for EXTI0
#[interrupt]
fn EXTI15_10() {
    // Clear the interrupt flag
    unsafe {
        let exti = &*stm32f7xx_hal::pac::EXTI::ptr();
        exti.pr.modify(|_, w| w.pr0().set_bit());
        // Call the callback function if it's set
        if let Some(callback) = BUTTON_CALLBACK {
            callback();
        }
    }
}


// Callback function
fn button_press_handler() {
    // Handle the button press event
    // For example, toggle an LED or log a message
     hprintln!("I am the blue button");
}



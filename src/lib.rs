#![no_std]

// Library includes ===============================================================================
pub use cortex_m_rt::{entry, exception};
pub use stm32f4::stm32f446::{NVIC, Interrupt, interrupt};

pub use heapless::{Vec, String};
pub use rtt_target::{rtt_init_print, rprint, rprintln};

pub use include::pins::*;
pub use gpio::*;
pub use analog::{adc_resolution, analog_read, analog_write, analog_write_noise, analog_write_triangle, analog_wave_freq};
pub use time::{pwm_write, delay, start_time, millis};


// Submodule includes =============================================================================
pub mod include;
pub mod gpio;
pub mod analog;
pub mod time;
pub mod uart;
pub mod i2c;
pub mod spi;


// Panic handler ==================================================================================
use core::panic::PanicInfo;
use core::sync::atomic::{compiler_fence, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  cortex_m::interrupt::disable();
  rtt_target::rprintln!("{}", info);
  loop {compiler_fence(Ordering::SeqCst);}
}

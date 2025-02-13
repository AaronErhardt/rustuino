//! This module contains everything that is related to the digital IO functionality.

use crate::analog::enable_channel;
use crate::time::setup_pwm;
use crate::include::{stm_peripherals, GpioError, ProgError};
use rtt_target::rprintln;
use heapless::Vec;

// Represents available GPIO modes.
pub enum GpioMode {
  Input,
  Output,
  AlternateFunction(u32),
  Analog,
  PWM
}

// pub struct Pin<T> {
//   block: char,
//   pin: u8,
//   internal: T
// }

/// Represents the options to configure the GPIO speed of a pin.
///
/// Keep in mind, that the internal clock of the MC needs to be faster than the IO speed to be used fully.
///
/// | Speed  | Max. IO Frequency |
/// | ------ | ----------------- |
/// | Low    | 4MHz              |
/// | Medium | 25MHz             |
/// | Fast   | 50MHz             |
/// | High   | 100MHz            |
pub enum GpioSpeed {
  Low, Medium, Fast, High
}

// Represents the options for pullup-/pulldown-resistors.
pub enum GpioBias {
  None, Pullup, Pulldown
}




// Public Functions ===============================================================================
pub fn pin_mode(pin: (char, u8), mode: GpioMode) -> Result<(), ProgError> {
  let peripheral_ptr = stm_peripherals();
  let rcc = &peripheral_ptr.RCC;

  static mut PIN_CONF: Vec<(char, u8), 50> = Vec::new();

  if let Err(error) = check_pin(pin) {return Err(error);}
  if let GpioMode::AlternateFunction(af) = mode {
    if af > 15 {
      rprintln!("Only alternate funtion values between 0 and 15 are valid! | pin_mode()");
      return Err(ProgError::InvalidConfiguration);
    }
  }

  unsafe {
    if PIN_CONF.contains(&pin) == false {PIN_CONF.push(pin).unwrap();}
    else {
      rprintln!("P{}{} is already configured! | pin_mode()", pin.0.to_uppercase(), pin.1);
      return Err(ProgError::AlreadyConfigured);
    }
  }

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      match mode {
        GpioMode::Input => gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioMode::Output => gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioMode::AlternateFunction(af) => {
          gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
          if pin.1 > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
          else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
        },
        GpioMode::Analog => {
          gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
          if let Err(error) = enable_channel(pin) {return Err(error);}
        }
        GpioMode::PWM => setup_pwm(pin).unwrap()
      };
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      match mode {
        GpioMode::Input => gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioMode::Output => gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioMode::AlternateFunction(af) => {
          gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
          if pin.1 > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
          else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
        },
        GpioMode::Analog => {
          gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
          if let Err(error) = enable_channel(pin) {return Err(error);}
        }
        GpioMode::PWM => setup_pwm(pin).unwrap()
      };
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      match mode {
        GpioMode::Input => gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioMode::Output => gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioMode::AlternateFunction(af) => {
          gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
          if pin.1 > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
          else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
        },
        GpioMode::Analog => {
          gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
          if let Err(error) = enable_channel(pin) {return Err(error);}
        }
        GpioMode::PWM => setup_pwm(pin).unwrap()
      };
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      match mode {
        GpioMode::Input => gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioMode::Output => gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioMode::AlternateFunction(af) => {
          gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
          if pin.1 > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
          else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
        },
        GpioMode::Analog => {
          gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
          if let Err(error) = enable_channel(pin) {return Err(error);}
        }
        GpioMode::PWM => setup_pwm(pin).unwrap()
      };
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      rcc.ahb1enr.modify(|_, w| w.gpiohen().enabled());
      match mode {
        GpioMode::Input => gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioMode::Output => gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioMode::AlternateFunction(af) => {
          gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))});
          if pin.1 > 7 {gpioh.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * (pin.1 - 8))))});}
          else {gpioh.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (af << (4 * pin.1)))});}
        },
        GpioMode::Analog => {
          gpioh.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (3 << (2 * pin.1)))});
          if let Err(error) = enable_channel(pin) {return Err(error);}
        }
        GpioMode::PWM => setup_pwm(pin).unwrap()
      };
    },
    _   => unreachable!()
  };

  return Ok(());
}

pub fn digital_write(pin: (char, u8), value: bool) -> Result<(), GpioError> {
  let peripheral_ptr = stm_peripherals();

  if let Err(error) = check_pin(pin) {return Err(GpioError::Prog(error));}

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      if gpioa.moder.read().bits() & (3 << (2 * pin.1)) != (1 << (2 * pin.1)) {
        rprintln!("P{}{} is not in output mode! | digital_write()", pin.0.to_uppercase(), pin.1);
        return Err(GpioError::WrongMode);
      }
      if value == true {gpioa.bsrr.write(|w| unsafe {w.bits(1 << pin.1)});}
      else {gpioa.bsrr.write(|w| unsafe {w.bits(1 << (pin.1 + 16))});}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      if gpiob.moder.read().bits() & (3 << (2 * pin.1)) != (1 << (2 * pin.1)) {
        rprintln!("P{}{} is not in output mode! | digital_write()", pin.0.to_uppercase(), pin.1);
        return Err(GpioError::WrongMode);
      }
      if value == true {gpiob.bsrr.write(|w| unsafe {w.bits(1 << pin.1)});}
      else {gpiob.bsrr.write(|w| unsafe {w.bits(1 << (pin.1 + 16))});}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      if gpioc.moder.read().bits() & (3 << (2 * pin.1)) != (1 << (2 * pin.1)) {
        rprintln!("P{}{} is not in output mode! | digital_write()", pin.0.to_uppercase(), pin.1);
        return Err(GpioError::WrongMode);
      }
      if value == true {gpioc.bsrr.write(|w| unsafe {w.bits(1 << pin.1)});}
      else {gpioc.bsrr.write(|w| unsafe {w.bits(1 << (pin.1 + 16))});}
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      if gpiod.moder.read().bits() & (3 << (2 * pin.1)) != (1 << (2 * pin.1)) {
        rprintln!("P{}{} is not in output mode! | digital_write()", pin.0.to_uppercase(), pin.1);
        return Err(GpioError::WrongMode);
      }
      if value == true {gpiod.bsrr.write(|w| unsafe {w.bits(1 << pin.1)});}
      else {gpiod.bsrr.write(|w| unsafe {w.bits(1 << (pin.1 + 16))});}
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      if gpioh.moder.read().bits() & (3 << (2 * pin.1)) != (1 << (2 * pin.1)) {
        rprintln!("P{}{} is not in output mode! | digital_write()", pin.0.to_uppercase(), pin.1);
        return Err(GpioError::WrongMode);
      }
      if value == true {gpioh.bsrr.write(|w| unsafe {w.bits(1 << pin.1)});}
      else {gpioh.bsrr.write(|w| unsafe {w.bits(1 << (pin.1 + 16))});}
    },
    _   => unreachable!()
  };

  return Ok(());
}

pub fn digital_read(pin: (char, u8)) -> Result<bool, GpioError> {
  let peripheral_ptr = stm_peripherals();

  if let Err(error) = check_pin(pin) {return Err(GpioError::Prog(error));}

  let bits = match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      if gpioa.moder.read().bits() & (3 << (2 * pin.1)) == 0 {gpioa.idr.read().bits()}
      else if gpioa.moder.read().bits() & (3 << (2 * pin.1)) == (1 << (2 * pin.1)) {gpioa.odr.read().bits()}
      else {
        rprintln!("P{}{} is not in input or output mode! | digital_read()", pin.0.to_uppercase(), pin.1);
        return Err(GpioError::WrongMode);
      }
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      if gpiob.moder.read().bits() & (3 << (2 * pin.1)) == 0 {gpiob.idr.read().bits()}
      else if gpiob.moder.read().bits() & (3 << (2 * pin.1)) == (1 << (2 * pin.1)) {gpiob.odr.read().bits()}
      else {
        rprintln!("P{}{} is not in input or output mode! | digital_read()", pin.0.to_uppercase(), pin.1);
        return Err(GpioError::WrongMode);
      }
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      if gpioc.moder.read().bits() & (3 << (2 * pin.1)) == 0 {gpioc.idr.read().bits()}
      else if gpioc.moder.read().bits() & (3 << (2 * pin.1)) == (1 << (2 * pin.1)) {gpioc.odr.read().bits()}
      else {
        rprintln!("P{}{} is not in input or output mode! | digital_read()", pin.0.to_uppercase(), pin.1);
        return Err(GpioError::WrongMode);
      }
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      if gpiod.moder.read().bits() & (3 << (2 * pin.1)) == 0 {gpiod.idr.read().bits()}
      else if gpiod.moder.read().bits() & (3 << (2 * pin.1)) == (1 << (2 * pin.1)) {gpiod.odr.read().bits()}
      else {
        rprintln!("P{}{} is not in input or output mode! | digital_read()", pin.0.to_uppercase(), pin.1);
        return Err(GpioError::WrongMode);
      }
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      if gpioh.moder.read().bits() & (3 << (2 * pin.1)) == 0 {gpioh.idr.read().bits()}
      else if gpioh.moder.read().bits() & (3 << (2 * pin.1)) == (1 << (2 * pin.1)) {gpioh.odr.read().bits()}
      else {
        rprintln!("P{}{} is not in input or output mode! | digital_read()", pin.0.to_uppercase(), pin.1);
        return Err(GpioError::WrongMode);
      }
    },
    _   => unreachable!()
  };

  if bits & (1 << pin.1) == (1 << pin.1) {return Ok(true);}
  else {return Ok(false);}
}

pub fn set_bias(pin: (char, u8), bias: GpioBias) -> Result<(), GpioError> {
  let peripheral_ptr = stm_peripherals();

  if let Err(error) = check_pin(pin) {return Err(GpioError::Prog(error));}

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      match bias {
        GpioBias::None => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioBias::Pullup => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioBias::Pulldown => gpioa.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))})
      };
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      match bias {
        GpioBias::None => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioBias::Pullup => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioBias::Pulldown => gpiob.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))})
      };
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      match bias {
        GpioBias::None => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioBias::Pullup => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioBias::Pulldown => gpioc.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))})
      };
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      match bias {
        GpioBias::None => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioBias::Pullup => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioBias::Pulldown => gpiod.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))})
      };
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      match bias {
        GpioBias::None => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioBias::Pullup => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioBias::Pulldown => gpioh.pupdr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))})
      };
    },
    _   => unreachable!()
  };

  return Ok(());
}

pub fn set_speed(pin: (char, u8), speed: GpioSpeed) -> Result<(), GpioError> {
  let peripheral_ptr = stm_peripherals();

  if let Err(error) = check_pin(pin) {return Err(GpioError::Prog(error));}

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      match speed {
        GpioSpeed::Low => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioSpeed::Medium => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioSpeed::Fast => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))}),
        GpioSpeed::High => gpioa.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.1)))})
      };
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      match speed {
        GpioSpeed::Low => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioSpeed::Medium => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioSpeed::Fast => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))}),
        GpioSpeed::High => gpiob.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.1)))})
      };
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      match speed {
        GpioSpeed::Low => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioSpeed::Medium => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioSpeed::Fast => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))}),
        GpioSpeed::High => gpioc.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.1)))})
      };
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      match speed {
        GpioSpeed::Low => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioSpeed::Medium => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioSpeed::Fast => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))}),
        GpioSpeed::High => gpiod.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.1)))})
      };
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      match speed {
        GpioSpeed::Low => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)))}),
        GpioSpeed::Medium => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (1 << (2 * pin.1)))}),
        GpioSpeed::Fast => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin.1)) | (2 << (2 * pin.1)))}),
        GpioSpeed::High => gpioh.ospeedr.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (2 * pin.1)))})
      };
    },
    _   => unreachable!()
  };

  return Ok(());
}

pub fn open_drain(pin: (char, u8), op: bool) -> Result<(), GpioError> {
  let peripheral_ptr = stm_peripherals();

  if let Err(error) = check_pin(pin) {return Err(GpioError::Prog(error));}

  match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      if op == true {gpioa.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.1))});}
      else {gpioa.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (0 << pin.1))});}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      if op == true {gpiob.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.1))});}
      else {gpiob.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (0 << pin.1))});}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      if op == true {gpioc.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.1))});}
      else {gpioc.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (0 << pin.1))});}
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      if op == true {gpiod.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.1))});}
      else {gpiod.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (0 << pin.1))});}
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      if op == true {gpioh.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (1 << pin.1))});}
      else {gpioh.otyper.modify(|r, w| unsafe {w.bits(r.bits() | (0 << pin.1))});}
    },
    _   => unreachable!()
  };

  return Ok(());
}

pub fn return_pinmode(pin: (char, u8)) -> Result<GpioMode, GpioError> {
  let peripheral_ptr = stm_peripherals();

  if let Err(error) = check_pin(pin) {return Err(GpioError::Prog(error));}

  let mut bits = match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      gpioa.moder.read().bits()
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      gpiob.moder.read().bits()
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      gpioc.moder.read().bits()
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      gpiod.moder.read().bits()
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      gpioh.moder.read().bits()
    },
    _   => unreachable!()
  };

  let mut af = match pin.0 {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      if pin.1 > 7 {gpioa.afrh.read().bits()}
      else {gpioa.afrl.read().bits()}
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      if pin.1 > 7 {gpiob.afrh.read().bits()}
      else {gpiob.afrl.read().bits()}
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      if pin.1 > 7 {gpioc.afrh.read().bits()}
      else {gpioc.afrl.read().bits()}
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      if pin.1 > 7 {gpiod.afrh.read().bits()}
      else {gpiod.afrl.read().bits()}
    },
    'h' => {
      let gpioh = &peripheral_ptr.GPIOH;
      if pin.1 > 7 {gpioh.afrh.read().bits()}
      else {gpioh.afrl.read().bits()}
    },
    _   => unreachable!()
  };

  bits = (bits & (3 << (2 * pin.1))) >> pin.1;
  
  if pin.1 > 7 {af = (af & (15 << (4 * (pin.1 - 8)))) >> (4 * (pin.1 - 8))}
  else {af = (af & (15 << (4 * pin.1))) >> (4 * pin.1)}

  match bits {
    0 => return Ok(GpioMode::Input),
    1 => return Ok(GpioMode::Output),
    2 => return Ok(GpioMode::AlternateFunction(af)),
    3 => return Ok(GpioMode::Analog),
    _   => unreachable!()
  };
}


// Private Functions ==============================================================================
fn check_pin(pin: (char, u8)) -> Result<(), ProgError> {
  if pin.1 > 15 || (pin.1 != 2 && pin.0 == 'd') || ((pin.1 != 0 && pin.0 == 'h') && (pin.1 != 1 && pin.0 == 'h')) {
    rprintln!("P{}{} is not an available GPIO Pin!", pin.0.to_uppercase(), pin.1);
    return Err(ProgError::InvalidConfiguration);
  }
  else {return Ok(());}
}

use core::usize;

use libm::*;
use heapless::String;
use cortex_m_semihosting::hprintln;
use super::common::*;
use super::include::PERIPHERAL_PTR;
use super::include::data_maps::{UART_MAP, UART_CONF};


// Converter implementations ======================================================================
impl<const B: char, const P: u8> ToUart for GpioPin<B, P, 8> {
  fn uart(self, baud: u32, rxint: bool, txint: bool) -> UartPin<Self> {
    self.block = B;
    self.pin = P;

    let channel: usize;
    let direction: bool;

    if UART_MAP.rx_pin.contains(&(self.block, self.pin)) == true {
      channel = UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&i| i == (self.block, self.pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {UART_CONF[channel - 1] = true}
        else {
          hprintln!("UART channel {} already in use!", channel);
          return UartPin {
            inner: self
          };
        }
      }
      direction = false;
    }
    else if UART_MAP.tx_pin.contains(&(self.block, self.pin)) == true {
      channel = UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&i| i == (self.block, self.pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {UART_CONF[channel - 1] = true}
        else {
          hprintln!("UART channel {} already in use!", channel);
          return UartPin {
            inner: self
          };
        }
      }
      direction = true;
    }
    else {panic!("{}{} can not be used for UART communication!", self.block.to_uppercase(), self.pin);}

    uart_init(channel, direction, baud, rxint, txint);

    return UartPin {
      inner: self
    };
  }
}


// Function implementations =======================================================================
impl<const B: char, const P: u8> UART for UartPin<GpioPin<B, P, 8>> {
  fn rxint_enable(&self) {
    let block = B;
    let pin = P;
    let channel: usize;

    if UART_MAP.rx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
    }
    else if UART_MAP.tx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
    }
    
    rx_interrupt(channel, true);
  }

  fn rxint_disable(&self) {
    let block = B;
    let pin = P;
    let channel: usize;

    if UART_MAP.rx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
    }
    else if UART_MAP.tx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
    }
    
    rx_interrupt(channel, false);
  }

  fn txint_enable(&self) {
    let block = B;
    let pin = P;
    let channel: usize;

    if UART_MAP.rx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
    }
    else if UART_MAP.tx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
    }
    
    tx_interrupt(channel, true);
  }

  fn txint_disable(&self) {
    let block = B;
    let pin = P;
    let channel: usize;

    if UART_MAP.rx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
    }
    else if UART_MAP.tx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
    }
    
    tx_interrupt(channel, false);
  }

  fn change_baud(&self, baud: u32) {
    let block = B;
    let pin = P;
    let channel: usize;

    if UART_MAP.rx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {
          hprintln!("UART Channel {} is not active!", channel);
          return;
        }
      }
    }
    else if UART_MAP.tx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {
          hprintln!("UART Channel {} is not active!", channel);
          return;
        }
      }
    }
    else {panic!("{}{} can not be used for UART communication!", block.to_uppercase(), pin);}

    set_baud(channel, baud);
  }

  fn send_char(&self, c: char) {
    let block = B;
    let pin = P;
    let channel: usize;

    if UART_MAP.rx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {
          hprintln!("UART Channel {} is not active!", channel);
          return;
        }
      }
    }
    else if UART_MAP.tx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {
          hprintln!("UART Channel {} is not active!", channel);
          return;
        }
      }
    }
    else {panic!("{}{} can not be used for UART communication!", block.to_uppercase(), pin);}

    transmit_char(channel, c);
  }

  fn send_string(&self, s: &str) {
    let block = B;
    let pin = P;
    let channel: usize;

    if UART_MAP.rx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {
          hprintln!("UART Channel {} is not active!", channel);
          return;
        }
      }
    }
    else if UART_MAP.tx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {
          hprintln!("UART Channel {} is not active!", channel);
          return;
        }
      }
    }
    else {panic!("{}{} can not be used for UART communication!", block.to_uppercase(), pin);}

    for c in s.chars() {transmit_char(channel, c);}
  }

  fn get_char(&self) -> char {
    let block = B;
    let pin = P;
    let channel: usize;

    if UART_MAP.rx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {
          hprintln!("UART Channel {} is not active!", channel);
          return '?';
        }
      }
    }
    else if UART_MAP.tx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {
          hprintln!("UART Channel {} is not active!", channel);
          return '?';
        }
      }
    }
    else {panic!("{}{} can not be used for UART communication!", block.to_uppercase(), pin);}

    return recieve_char(channel);
  }

  fn get_string(&self, stopper: char) -> heapless::String<30> {
    let block = B;
    let pin = P;
    let channel: usize;
    let buffer: char;
    let string_buffer: String<30> = String::new();

    if stopper.is_ascii() == false {
      hprintln!("Stop character is not an ASCII character!");
      return string_buffer;
    }

    if UART_MAP.rx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.rx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {
          hprintln!("UART Channel {} is not active!", channel);
          return string_buffer;
        }
      }
    }
    else if UART_MAP.tx_pin.contains(&(block, pin)) == true {
      channel = UART_MAP.channel[UART_MAP.tx_pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      unsafe {
        if UART_CONF[channel - 1] == false {
          hprintln!("UART Channel {} is not active!", channel);
          return string_buffer;
        }
      }
    }
    else {panic!("{}{} can not be used for UART communication!", block.to_uppercase(), pin);}

    loop {
      buffer = recieve_char(channel);
      if buffer == stopper {return string_buffer;}
      string_buffer.push(buffer).expect("String buffer overflow!");  
    }
  }
}


// Helper functions ===============================================================================
fn uart_init(channel: usize, direction: bool, baud: u32, rxint: bool, txint: bool) {
  let rcc = &PERIPHERAL_PTR.RCC;
  
  match channel {
    1 => {
      let usart1 = &PERIPHERAL_PTR.USART1;
      rcc.apb2enr.modify(|_, w| w.usart1en().enabled());
          
      set_baud(channel, baud);
          
      if direction == true {usart1.cr1.modify(|_, w| w.te().enabled());}
      else {usart1.cr1.modify(|_, w| w.re().enabled());}
      
      usart1.cr1.modify(|_, w| w.ue().enabled());
    },
    3 => {
      let usart3 = &PERIPHERAL_PTR.USART3;
      rcc.apb1enr.modify(|_, w| w.usart3en().enabled());
          
      set_baud(channel, baud);
          
      if direction == true {usart3.cr1.modify(|_, w| w.te().enabled());}
      else {usart3.cr1.modify(|_, w| w.re().enabled());}
      
      usart3.cr1.modify(|_, w| w.ue().enabled());
    },
    4 => {
      let uart4 = &PERIPHERAL_PTR.UART4;
      rcc.apb1enr.modify(|_, w| w.uart4en().enabled());
          
      set_baud(channel, baud);
          
      if direction == true {uart4.cr1.modify(|_, w| w.te().enabled());}
      else {uart4.cr1.modify(|_, w| w.re().enabled());}
      
      uart4.cr1.modify(|_, w| w.ue().enabled());
    },
    5 => {
      let uart5 = &PERIPHERAL_PTR.UART5;
      rcc.apb1enr.modify(|_, w| w.uart5en().enabled());
          
      set_baud(channel, baud);
          
      if direction == true {uart5.cr1.modify(|_, w| w.te().enabled());}
      else {uart5.cr1.modify(|_, w| w.re().enabled());}
      
      uart5.cr1.modify(|_, w| w.ue().enabled());
    },
    6 => {
      let usart6 = &PERIPHERAL_PTR.USART6;
      rcc.apb2enr.modify(|_, w| w.usart6en().enabled());
          
      set_baud(channel, baud);
          
      if direction == true {usart6.cr1.modify(|_, w| w.te().enabled());}
      else {usart6.cr1.modify(|_, w| w.re().enabled());}
      
      usart6.cr1.modify(|_, w| w.ue().enabled());
    },
    _ => panic!("{} is not a valid UART peripheral!", channel)
  };

  if rxint == true {rx_interrupt(channel, true);}
  if txint == true {tx_interrupt(channel, true);}
}

fn rx_interrupt(channel: usize, enable: bool) {
  match channel {
    1 => {
      let usart1 = &PERIPHERAL_PTR.USART1;
      if enable == true {usart1.cr1.modify(|_, w| w.rxneie().enabled());}
      else {usart1.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    2 => {
      let usart2 = &PERIPHERAL_PTR.USART2;
      if enable == true {usart2.cr1.modify(|_, w| w.rxneie().enabled());}
      else {usart2.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    3 => {
      let usart3 = &PERIPHERAL_PTR.USART3;
      if enable == true {usart3.cr1.modify(|_, w| w.rxneie().enabled());}
      else {usart3.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    4 => {
      let uart4 = &PERIPHERAL_PTR.UART4;
      if enable == true {uart4.cr1.modify(|_, w| w.rxneie().enabled());}
      else {uart4.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    5 => {
      let uart5 = &PERIPHERAL_PTR.UART5;
      if enable == true {uart5.cr1.modify(|_, w| w.rxneie().enabled());}
      else {uart5.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    6 => {
      let usart6 = &PERIPHERAL_PTR.USART6;
      if enable == true {usart6.cr1.modify(|_, w| w.rxneie().enabled());}
      else {usart6.cr1.modify(|_, w| w.rxneie().disabled());}
    },
    _ => panic!("{} is not a valid UART peripheral!", channel)
  };
}

fn tx_interrupt(channel: usize, enable: bool) {
  match channel {
    1 => {
      let usart1 = &PERIPHERAL_PTR.USART1;
      if enable == true {usart1.cr1.modify(|_, w| w.tcie().enabled());}
      else {usart1.cr1.modify(|_, w| w.tcie().disabled());}
    },
    2 => {
      let usart2 = &PERIPHERAL_PTR.USART2;
      if enable == true {usart2.cr1.modify(|_, w| w.tcie().enabled());}
      else {usart2.cr1.modify(|_, w| w.tcie().disabled());}
    },
    3 => {
      let usart3 = &PERIPHERAL_PTR.USART3;
      if enable == true {usart3.cr1.modify(|_, w| w.tcie().enabled());}
      else {usart3.cr1.modify(|_, w| w.tcie().disabled());}
    },
    4 => {
      let uart4 = &PERIPHERAL_PTR.UART4;
      if enable == true {uart4.cr1.modify(|_, w| w.tcie().enabled());}
      else {uart4.cr1.modify(|_, w| w.tcie().disabled());}
    },
    5 => {
      let uart5 = &PERIPHERAL_PTR.UART5;
      if enable == true {uart5.cr1.modify(|_, w| w.tcie().enabled());}
      else {uart5.cr1.modify(|_, w| w.tcie().disabled());}
    },
    6 => {
      let usart6 = &PERIPHERAL_PTR.USART6;
      if enable == true {usart6.cr1.modify(|_, w| w.tcie().enabled());}
      else {usart6.cr1.modify(|_, w| w.tcie().disabled());}
    },
    _ => panic!("{} is not a valid UART peripheral!", channel)
  };
}

fn set_baud(channel: usize, baud: u32) {
  // (Mantisse, Fractal)
  let usartdiv: (f64, f64) = modf(16000000.0 / (16.0 * baud as f64));

  match channel {
    1 => {
      let usart1 = &PERIPHERAL_PTR.USART1;
      usart1.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    2 => {
      let usart2 = &PERIPHERAL_PTR.USART2;
      usart2.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    3 => {
      let usart3 = &PERIPHERAL_PTR.USART3;
      usart3.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    4 => {
      let uart4 = &PERIPHERAL_PTR.UART4;
      uart4.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    5 => {
      let uart5 = &PERIPHERAL_PTR.UART5;
      uart5.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    6 => {
      let usart6 = &PERIPHERAL_PTR.USART6;
      usart6.brr.modify(|_, w| {
        w.div_mantissa().bits(usartdiv.0 as u16);
        w.div_fraction().bits(usartdiv.1 as u8)
      });
    },
    _ => panic!("{} is not a valid UART peripheral!", channel)
  }
}

fn transmit_char(channel: usize, c: char) {
  match channel {
    1 => {
      let usart1 = &PERIPHERAL_PTR.USART1;
      if c.is_ascii() == true {
        while usart1.sr.read().txe().bit_is_set() == true {}
        usart1.dr.write(|w| w.dr().bits(c as u16));
        while usart1.sr.read().txe().bit_is_set() == true {}
      }
      else {
        c = '?';
        hprintln!("{} is not an ASCII character!", c);
    
        while usart1.sr.read().txe().bit_is_set() == true {}
        usart1.dr.write(|w| w.dr().bits(c as u16));
        while usart1.sr.read().txe().bit_is_set() == true {}
      }
    },
    2 => {
      let usart2 = &PERIPHERAL_PTR.USART2;
      if c.is_ascii() == true {
        while usart2.sr.read().txe().bit_is_set() == true {}
        usart2.dr.write(|w| w.dr().bits(c as u16));
        while usart2.sr.read().txe().bit_is_set() == true {}
      }
      else {
        c = '?';
        hprintln!("{} is not an ASCII character!", c);
    
        while usart2.sr.read().txe().bit_is_set() == true {}
        usart2.dr.write(|w| w.dr().bits(c as u16));
        while usart2.sr.read().txe().bit_is_set() == true {}
      }
    },
    3 => {
      let usart3 = &PERIPHERAL_PTR.USART3;
      if c.is_ascii() == true {
        while usart3.sr.read().txe().bit_is_set() == true {}
        usart3.dr.write(|w| w.dr().bits(c as u16));
        while usart3.sr.read().txe().bit_is_set() == true {}
      }
      else {
        c = '?';
        hprintln!("{} is not an ASCII character!", c);
    
        while usart3.sr.read().txe().bit_is_set() == true {}
        usart3.dr.write(|w| w.dr().bits(c as u16));
        while usart3.sr.read().txe().bit_is_set() == true {}
      }
    },
    4 => {let uart4 = &PERIPHERAL_PTR.UART4;
      if c.is_ascii() == true {
        while uart4.sr.read().txe().bit_is_set() == true {}
        uart4.dr.write(|w| w.dr().bits(c as u16));
        while uart4.sr.read().txe().bit_is_set() == true {}
      }
      else {
        c = '?';
        hprintln!("{} is not an ASCII character!", c);
    
        while uart4.sr.read().txe().bit_is_set() == true {}
        uart4.dr.write(|w| w.dr().bits(c as u16));
        while uart4.sr.read().txe().bit_is_set() == true {}
      }},
    5 => {
      let uart5 = &PERIPHERAL_PTR.UART5;
      if c.is_ascii() == true {
        while uart5.sr.read().txe().bit_is_set() == true {}
        uart5.dr.write(|w| w.dr().bits(c as u16));
        while uart5.sr.read().txe().bit_is_set() == true {}
      }
      else {
        c = '?';
        hprintln!("{} is not an ASCII character!", c);
    
        while uart5.sr.read().txe().bit_is_set() == true {}
        uart5.dr.write(|w| w.dr().bits(c as u16));
        while uart5.sr.read().txe().bit_is_set() == true {}
      }
    },
    6 => {
      let usart6 = &PERIPHERAL_PTR.USART6;
      if c.is_ascii() == true {
        while usart6.sr.read().txe().bit_is_set() == true {}
        usart6.dr.write(|w| w.dr().bits(c as u16));
        while usart6.sr.read().txe().bit_is_set() == true {}
      }
      else {
        c = '?';
        hprintln!("{} is not an ASCII character!", c);
    
        while usart6.sr.read().txe().bit_is_set() == true {}
        usart6.dr.write(|w| w.dr().bits(c as u16));
        while usart6.sr.read().txe().bit_is_set() == true {}
      }
    },
    _ => panic!("{} is not a valid UART peripheral!", channel)
  };
}

fn recieve_char(channel: usize) -> char {
  let buffer = match channel {
    1 => {
      let usart1 = &PERIPHERAL_PTR.USART1;
      while usart1.sr.read().rxne().bit_is_clear() == true {}
      usart1.dr.read().dr().bits() as u8
    },
    2 => {
      let usart2 = &PERIPHERAL_PTR.USART2;
      while usart2.sr.read().rxne().bit_is_clear() == true {}
      usart2.dr.read().dr().bits() as u8
    },
    3 => {
      let usart3 = &PERIPHERAL_PTR.USART3;
      while usart3.sr.read().rxne().bit_is_clear() == true {}
      usart3.dr.read().dr().bits() as u8
    },
    4 => {
      let uart4 = &PERIPHERAL_PTR.UART4;
      while uart4.sr.read().rxne().bit_is_clear() == true {}
      uart4.dr.read().dr().bits() as u8
    },
    5 => {
      let uart5 = &PERIPHERAL_PTR.UART5;
      while uart5.sr.read().rxne().bit_is_clear() == true {}
      uart5.dr.read().dr().bits() as u8
    },
    6 => {
      let usart6 = &PERIPHERAL_PTR.USART6;
      while usart6.sr.read().rxne().bit_is_clear() == true {}
      usart6.dr.read().dr().bits() as u8
    },
    _ => panic!("{} is not a valid UART peripheral!", channel)
  };

  return buffer as char;
}


// UART Serial connection =========================================================================
pub mod Serial {
  use libm::*;
  use cortex_m_semihosting::hprintln;
  use super::super::include::PERIPHERAL_PTR;
  use super::super::include::data_maps::UART_CONF;

  pub fn init(baud: u32, rxint: bool, txint: bool) {
    let rcc = &PERIPHERAL_PTR.RCC;
    let usart2 = &PERIPHERAL_PTR.USART2;
    let gpioa = &PERIPHERAL_PTR.GPIOA;
    
    // (Mantisse, Fractal)
    let usartdiv: (f64, f64) = modf(16000000.0 / (16.0 * baud as f64));

    unsafe {
      if UART_CONF[1] == true {
        hprintln!("Serial connection already configured!");
        usart2.cr1.modify(|_, w| w.ue().disabled());
      }
    }
    
    rcc.apb1enr.modify(|_, w| w.usart2en().enabled());
    
    usart2.brr.write(|w| {
      w.div_mantissa().bits(usartdiv.0 as u16);
      w.div_fraction().bits(usartdiv.1 as u8)
    });
    
    if rxint == true {usart2.cr1.modify(|_, w| w.rxneie().enabled());}
    if txint == true {usart2.cr1.modify(|_, w| w.tcie().enabled());}
    
    usart2.cr1.modify(|_, w| {
      w.re().enabled();
      w.te().enabled();
      w.ue().enabled()
    });
    
    unsafe {UART_CONF[1] = true;}
    
    rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
    gpioa.moder.modify(|_, w| w.moder2().alternate());
    gpioa.moder.modify(|_, w| w.moder3().alternate());
    gpioa.afrl.modify(|_, w| w.afrl2().af7());
    gpioa.afrl.modify(|_, w| w.afrl3().af7());
  }

  pub fn send_char_usb(c: char) {
    let usart2 = &PERIPHERAL_PTR.USART2;
    
    unsafe {if UART_CONF[1] == false {panic!("UART USB channel ist not PINCONFIGured!");}}

    if c.is_ascii() == true {
      while usart2.sr.read().txe().bit_is_set() == true {}
      usart2.dr.write(|w| w.dr().bits(c as u16));
      while usart2.sr.read().txe().bit_is_set() == true {}
    }
    else {
      c = '?';
      hprintln!("{} is not an ASCII character!", c);

      while usart2.sr.read().txe().bit_is_set() == true {}
      usart2.dr.write(|w| w.dr().bits(c as u16));
      while usart2.sr.read().txe().bit_is_set() == true {}
    }
  }
  
  pub fn recieve_char_usb() -> char {
    let usart2 = &PERIPHERAL_PTR.USART2;
    let buffer: u8;
    
    unsafe {if UART_CONF[1] == false {panic!("UART USB channel ist not PINCONFIGured!");}}
    
    while usart2.sr.read().rxne().bit_is_clear() == true {}
    buffer = usart2.dr.read().dr().bits() as u8;
    
    return buffer as char;
  }


  // Macro declerations ===========================================================================
  #[macro_export]
  macro_rules! sprint {
    ($param:expr) => {
      use core::fmt;

      let mut txt_buff: String<50> = String::new();
      if fmt::write(&mut txt_buff, format_args!($param)).is_err() {txt_buff = String::from("~\r\n")};
    
      for c in txt_buff.chars() {
        if c.is_ascii() == true {send_char_usb(c);}
        else {send_char_usb('?');}
      }
    };
  }

  #[macro_export]
  macro_rules! sprintln {
    ($param:expr) => {
      use core::fmt;

      let mut txt_buff: String<50> = String::new();
      if fmt::write(&mut txt_buff, format_args!(" ")).is_err() {txt_buff = String::from("~\r\n")};
    
      for c in txt_buff.chars() {
        if c.is_ascii() == true {send_char_usb(c);}
        else {send_char_usb('?');}
      }

      send_char_usb('\r');
      send_char_usb('\n');
    };
  }

  #[macro_export]
  macro_rules! sread {
    () => {{
      let c_buff: char = recieve_char_usb();  
      c_buff
    }};

    ($c:expr) => {{
      let found: bool;

      if recieve_char_usb() == $c {found = true;}
      else {found = false;}

      found
    }};
  }

  #[macro_export]
  macro_rules! sreads {
    ($stop:expr) => {{
      let mut str: String<50> = String::new();
      let mut buff: char;
      loop {
        buff = recieve_char_usb();
        if buff == $stop as char {break;}
        str.push(buff).expect("String buffer full!");
      }
      str
    }};
  }
}

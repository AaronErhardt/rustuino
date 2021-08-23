use crate::include::pins::*;
use crate::include:: {TIMER_MAP, TIMER_CONF, TIME_COUNTER, DELAY_COUNTER};
use cortex_m::peripheral::NVIC;
use stm32f4::stm32f446::{Interrupt, interrupt};

pub struct PwmPin {
  pub block: char,
  pub pin: u8
}

pub trait ToPwm: Sized {
  fn pwm() -> PwmPin;
}


// Converter implementations ======================================================================
macro_rules! generate_ToPwm {
  ($([$letter:literal, $number:literal]),+) => {
    use paste::paste;
    
    paste!{
      $(
        impl ToPwm for [<P $letter:upper $number>] {
          fn pwm() -> PwmPin {
            let block = $letter;
            let pin = $number;
            let timer: usize;
            let channel: usize;
            
            if TIMER_MAP.pin.contains(&(block, pin)) {
              timer = TIMER_MAP.timer[TIMER_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
              channel = TIMER_MAP.ccch[TIMER_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
              
              unsafe {
                if TIMER_CONF[(timer * 4) - channel] == false {TIMER_CONF[(timer * 4) - channel] = true;}
                else {panic!("Timer {} channel {} already in use! | .pwm()", timer, channel);}
              }
            }
            else {panic!("P{}{} is not available for pwm output! | .pwm()", block.to_uppercase(), pin);}
            
            pwm_init(timer, channel, block, pin);
            
            return PwmPin {
              block,
              pin
            };
          }
        }
      )+
    }
  };
}

generate_ToPwm![
['a', 0],
['a', 1],
['a', 2],
['a', 3],
['a', 5],
['a', 6],
['a', 7],
['a', 8],
['a', 9],
['a', 10],
['a', 11],
['a', 15],

['b', 0],
['b', 1],
['b', 2],
['b', 3],
['b', 4],
['b', 5],
['b', 6],
['b', 7],
['b', 8],
['b', 9],
['b', 10],
['b', 11],
['b', 14],
['b', 15],

['c', 6],
['c', 7],
['c', 8],
['c', 9],

['d', 12],
['d', 13],
['d', 14],
['d', 15],

['e', 5],
['e', 6],
['e', 9],
['e', 11],
['e', 13],
['e', 14],

['f', 6],
['f', 7],
['f', 8],
['f', 9]
];


// Function implementations =======================================================================
impl PwmPin {
  pub fn pwm_write(&self, value: u8) {
    let block = self.block;
    let pin = self.pin;
    let timer: usize;
    let channel: usize;
    
    if TIMER_MAP.pin.contains(&(block, pin)) {
      timer = TIMER_MAP.timer[TIMER_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      channel = TIMER_MAP.ccch[TIMER_MAP.pin.iter().position(|&i| i == (block, pin)).unwrap()] as usize;
      
      unsafe {
        if TIMER_CONF[(timer * 4) - channel] == false {
          panic!("Timer {} channel {} not configured! | .pwm_write(...)", timer, channel);
        }
      }
    }
    else {panic!("P{}{} is not available for pwm output! | .pwm_write(...)", block.to_uppercase(), pin);}
    
    pwm_set_duty(timer, channel, value);
  }
}


// Helper functions ===============================================================================
fn pwm_init(timer: usize, channel: usize, block: char, pin: u8) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  
  match block {
    'a' => {
      let gpioa = &peripheral_ptr.GPIOA;
      rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
      gpioa.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioa.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpioa.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    'b' => {
      let gpiob = &peripheral_ptr.GPIOB;
      rcc.ahb1enr.modify(|_, w| w.gpioben().enabled());
      gpiob.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiob.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpiob.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    'c' => {
      let gpioc = &peripheral_ptr.GPIOC;
      rcc.ahb1enr.modify(|_, w| w.gpiocen().enabled());
      gpioc.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioc.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpioc.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    'd' => {
      let gpiod = &peripheral_ptr.GPIOD;
      rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());
      gpiod.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiod.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpiod.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    'e' => {
      let gpioe = &peripheral_ptr.GPIOE;
      rcc.ahb1enr.modify(|_, w| w.gpioeen().enabled());
      gpioe.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpioe.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpioe.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    'f' => {
      let gpiof = &peripheral_ptr.GPIOF;
      rcc.ahb1enr.modify(|_, w| w.gpiofen().enabled());
      gpiof.moder.modify(|r, w| unsafe {w.bits(r.bits() & !(3 << (2 * pin)) | (2 << (2 * pin)))});
      if timer == 1 || timer == 2 {
        if pin > 7 {gpiof.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * (pin - 8))))});}
        else {gpiof.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (1 << (4 * pin)))});}
      }
      else if timer == 3 || timer == 4 || timer == 5 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiof.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * (pin - 8))))});}
          else {gpiof.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (2 << (4 * pin)))});}
        }
      }
      else if timer == 8 || timer == 9 || timer == 10 || timer == 11 {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiof.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * (pin - 8))))});}
          else {gpiof.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (3 << (4 * pin)))});}
        }
      }
      else {
        if timer == 1 || timer == 2 {
          if pin > 7 {gpiof.afrh.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * (pin - 8))))});}
          else {gpiof.afrl.modify(|r, w| unsafe {w.bits(r.bits() | (9 << (4 * pin)))});}
        }
      }
    },
    _   => panic!("P{}{} is not available for PWM output! | pwm_init(...)", block.to_uppercase(), pin)
  };
  
  match timer {
    1 => {
      let tim1 = &peripheral_ptr.TIM1;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim1.cr1.modify(|_, w| w.arpe().enabled());
      tim1.psc.write(|w| w.psc().bits(1000));
      tim1.arr.write_with_zero(|w| w.arr().bits(255));
      tim1.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim1.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim1.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim1.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim1.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    2 => {
      let tim2 = &peripheral_ptr.TIM2;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim2.cr1.modify(|_, w| w.arpe().enabled());
      tim2.psc.write(|w| w.psc().bits(1000));
      tim2.arr.write_with_zero(|w| w.arr().bits(255));
      tim2.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim2.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim2.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim2.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim2.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    3 => {
      let tim3 = &peripheral_ptr.TIM3;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim3.cr1.modify(|_, w| w.arpe().enabled());
      tim3.psc.write(|w| w.psc().bits(1000));
      tim3.arr.write_with_zero(|w| w.arr().bits(255));
      tim3.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim3.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim3.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim3.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim3.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    4 => {
      let tim4 = &peripheral_ptr.TIM4;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim4.cr1.modify(|_, w| w.arpe().enabled());
      tim4.psc.write(|w| w.psc().bits(1000));
      tim4.arr.write_with_zero(|w| w.arr().bits(255));
      tim4.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim4.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim4.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim4.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim4.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    5 => {
      let tim5 = &peripheral_ptr.TIM5;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim5.cr1.modify(|_, w| w.arpe().enabled());
      tim5.psc.write(|w| w.psc().bits(1000));
      tim5.arr.write_with_zero(|w| w.arr().bits(255));
      tim5.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim5.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim5.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim5.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim5.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    8 => {
      let tim8 = &peripheral_ptr.TIM8;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim8.cr1.modify(|_, w| w.arpe().enabled());
      tim8.psc.write(|w| w.psc().bits(1000));
      tim8.arr.write_with_zero(|w| w.arr().bits(255));
      tim8.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim8.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim8.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        3 => tim8.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        4 => tim8.ccmr2_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    9 => {
      let tim9 = &peripheral_ptr.TIM9;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim9.cr1.modify(|_, w| w.arpe().enabled());
      tim9.psc.write(|w| w.psc().bits(1000));
      tim9.arr.write_with_zero(|w| unsafe {w.arr().bits(255)});
      tim9.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim9.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim9.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    10 => {
      let tim10 = &peripheral_ptr.TIM10;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim10.cr1.modify(|_, w| w.arpe().enabled());
      tim10.psc.write(|w| w.psc().bits(1000));
      tim10.arr.write_with_zero(|w| unsafe {w.arr().bits(255)});
      tim10.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim10.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim10.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    11 => {
      let tim11 = &peripheral_ptr.TIM11;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim11.cr1.modify(|_, w| w.arpe().enabled());
      tim11.psc.write(|w| w.psc().bits(1000));
      tim11.arr.write_with_zero(|w| unsafe {w.arr().bits(255)});
      tim11.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim11.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim11.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    12 => {
      let tim12 = &peripheral_ptr.TIM12;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim12.cr1.modify(|_, w| w.arpe().enabled());
      tim12.psc.write(|w| w.psc().bits(1000));
      tim12.arr.write_with_zero(|w| unsafe {w.arr().bits(255)});
      tim12.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim12.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim12.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    13 => {
      let tim13 = &peripheral_ptr.TIM13;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim13.cr1.modify(|_, w| w.arpe().enabled());
      tim13.psc.write(|w| w.psc().bits(1000));
      tim13.arr.write_with_zero(|w| unsafe {w.arr().bits(255)});
      tim13.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim13.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim13.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    14 => {
      let tim14 = &peripheral_ptr.TIM14;
      
      rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
      tim14.cr1.modify(|_, w| w.arpe().enabled());
      tim14.psc.write(|w| w.psc().bits(1000));
      tim14.arr.write(|w| unsafe {w.arr().bits(255)});
      tim14.egr.write(|w| w.ug().set_bit());
      
      match channel {
        1 => tim14.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 3))}),
        2 => tim14.ccmr1_output_mut().modify(|r, w| unsafe {w.bits(r.bits() | (0xD << 11))}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    _  => panic!("Timer {} is not a valid timer! | pwm_init(...)", timer)
  };
}

fn pwm_set_duty(timer: usize, channel: usize, value: u8) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  
  match timer {
    1 => {
      let tim1 = &peripheral_ptr.TIM1;
      match channel {
        1 => tim1.ccr1.write_with_zero(|w| w.ccr().bits(value as u16)),
        2 => tim1.ccr2.write_with_zero(|w| w.ccr().bits(value as u16)),
        3 => tim1.ccr3.write_with_zero(|w| w.ccr().bits(value as u16)),
        4 => tim1.ccr4.write_with_zero(|w| w.ccr().bits(value as u16)),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    2 => {
      let tim2 = &peripheral_ptr.TIM2;
      match channel {
        1 => tim2.ccr1.write_with_zero(|w| w.ccr().bits(value as u32)),
        2 => tim2.ccr2.write_with_zero(|w| w.ccr().bits(value as u32)),
        3 => tim2.ccr3.write_with_zero(|w| w.ccr().bits(value as u32)),
        4 => tim2.ccr4.write_with_zero(|w| w.ccr().bits(value as u32)),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    3 => {
      let tim3 = &peripheral_ptr.TIM3;
      match channel {
        1 => tim3.ccr1.write_with_zero(|w| w.ccr().bits(value as u16)),
        2 => tim3.ccr2.write_with_zero(|w| w.ccr().bits(value as u16)),
        3 => tim3.ccr3.write_with_zero(|w| w.ccr().bits(value as u16)),
        4 => tim3.ccr4.write_with_zero(|w| w.ccr().bits(value as u16)),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    4 => {
      let tim4 = &peripheral_ptr.TIM4;
      match channel {
        1 => tim4.ccr1.write_with_zero(|w| w.ccr().bits(value as u16)),
        2 => tim4.ccr2.write_with_zero(|w| w.ccr().bits(value as u16)),
        3 => tim4.ccr3.write_with_zero(|w| w.ccr().bits(value as u16)),
        4 => tim4.ccr4.write_with_zero(|w| w.ccr().bits(value as u16)),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    5 => {
      let tim5 = &peripheral_ptr.TIM5;
      match channel {
        1 => tim5.ccr1.write_with_zero(|w| w.ccr().bits(value as u32)),
        2 => tim5.ccr2.write_with_zero(|w| w.ccr().bits(value as u32)),
        3 => tim5.ccr3.write_with_zero(|w| w.ccr().bits(value as u32)),
        4 => tim5.ccr4.write_with_zero(|w| w.ccr().bits(value as u32)),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    8 => {
      let tim8 = &peripheral_ptr.TIM8;
      match channel {
        1 => tim8.ccr1.write_with_zero(|w| w.ccr().bits(value as u16)),
        2 => tim8.ccr2.write_with_zero(|w| w.ccr().bits(value as u16)),
        3 => tim8.ccr3.write_with_zero(|w| w.ccr().bits(value as u16)),
        4 => tim8.ccr4.write_with_zero(|w| w.ccr().bits(value as u16)),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    9 => {
      let tim9 = &peripheral_ptr.TIM9;
      match channel {
        1 => tim9.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        2 => tim9.ccr2.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    10 => {
      let tim10 = &peripheral_ptr.TIM10;
      match channel {
        1 => tim10.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    11 => {
      let tim11 = &peripheral_ptr.TIM11;
      match channel {
        1 => tim11.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    12 => {
      let tim12 = &peripheral_ptr.TIM12;
      match channel {
        1 => tim12.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        2 => tim12.ccr2.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    13 => {
      let tim13 = &peripheral_ptr.TIM13;
      match channel {
        1 => tim13.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    14 => {
      let tim14 = &peripheral_ptr.TIM14;
      match channel {
        1 => tim14.ccr1.write_with_zero(|w| unsafe {w.ccr().bits(value as u16)}),
        _ => panic!("Channel {} is not a valid CC channel! | pwm_init(...)", channel)
      };
    },
    _ => panic!("Timer {} is not a valid timer! | adc_init(...)", timer)
  };
}


// Standalone time functions ======================================================================  
pub fn delay(ms: u32) {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let tim6 = &peripheral_ptr.TIM6;

  if rcc.apb1enr.read().tim6en().is_disabled() == true {
    rcc.apb1enr.modify(|_, w| w.tim6en().enabled());
    tim6.cr1.modify(|_, w| w.arpe().enabled());

    tim6.dier.modify(|_, w| w.uie().enabled());
    unsafe {NVIC::unmask(Interrupt::TIM6_DAC);}
    
    // 16MHz -> 1MHz : 1000 = 1kHz -> 1ms
    tim6.psc.write(|w| w.psc().bits(16));
    tim6.arr.write(|w| w.arr().bits(1000));
    tim6.egr.write(|w| w.ug().update());
    tim6.cr1.modify(|_, w| w.cen().enabled());
  }
  else {tim6.cr1.modify(|_, w| w.cen().enabled());}

  unsafe {
    DELAY_COUNTER.1 = 0;
    DELAY_COUNTER.0 = true;
    while DELAY_COUNTER.1 < ms {}
    DELAY_COUNTER.0 = false;
  }

  tim6.cr1.modify(|_, w| w.cen().disabled());
}

pub fn start_time() {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let rcc = &peripheral_ptr.RCC;
  let tim7 = &peripheral_ptr.TIM7;
  
  unsafe {
    if TIMER_CONF[20] == false {TIMER_CONF[20] = true;}
    else {
      rtt_target::rprintln!("Millis Timer already configured! | start_time()");
      return;
    }
  }
  
  rcc.apb1enr.modify(|_, w| w.tim7en().enabled());
  tim7.cr1.modify(|_, w| w.arpe().enabled());
  
  tim7.dier.modify(|_, w| w.uie().enabled());
  unsafe {NVIC::unmask(Interrupt::TIM7);}
  
  // 16MHz -> 1MHz : 1000 = 1kHz -> 1ms
  tim7.psc.write(|w| w.psc().bits(16));
  tim7.arr.write(|w| w.arr().bits(1000));
  tim7.egr.write(|w| w.ug().update());
  tim7.cr1.modify(|_, w| w.cen().enabled());
}

pub fn millis() -> usize {
  let peripheral_ptr;
  unsafe {peripheral_ptr = stm32f4::stm32f446::Peripherals::steal();}
  let tim7 = &peripheral_ptr.TIM6;

  let buffer: usize;
  
  tim7.cr1.modify(|_, w| w.cen().disabled());
  unsafe {buffer = TIME_COUNTER;}
  tim7.cr1.modify(|_, w| w.cen().enabled());
  
  return buffer;
}


// Interrupts and Exceptions ====================================================================
#[allow(non_snake_case)]
#[interrupt]
fn TIM6_DAC() {
  unsafe {
    if DELAY_COUNTER.0 == true {DELAY_COUNTER.1 += 1;}
    rtt_target::rprintln!("Delay-counter: {}", DELAY_COUNTER.1);
  }
}

#[allow(non_snake_case)]
#[interrupt]
fn TIM7() {
  unsafe {TIME_COUNTER += 1;}
}

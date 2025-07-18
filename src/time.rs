#![no_std]

/**
 * 
 * Author : Mariwan Jalal 18/07/2025
 */

use core::convert;

use cortex_m_semihosting::hprintln;
use fugit::{Duration, Instant};

use stm32f7xx_hal::{pac, rcc::Clocks, rtc::{Rtc}};


type TickInstant = Instant<u64, 1, 1000000>;
type TickDuration = Duration<u64, 1, 1000000>;

pub struct MyTimer<'a> {
    end_time: TickInstant,
    ticker: &'a mut Ticker,
    dur: TickDuration,
}

impl<'a> MyTimer<'a> {
    pub fn new(duration: TickDuration, ticker: &'a mut Ticker) -> Self {
        Self {
            end_time: ticker.now() + duration,
            ticker,
            dur: duration,
        }
    }
    pub fn set_duration(&mut self,new_dur : TickDuration){
        self.dur=new_dur;
    }
    pub fn new_default(ticker: &'a mut Ticker) -> Self {
        Self {
            end_time: ticker.now() + TickDuration::micros(500_000),
            ticker,
            dur:  TickDuration::micros(500_000),
        }
    }

    pub fn is_ready(& mut self) -> bool {
        if self.ticker.now() > self.end_time {
            self.end_time=self.ticker.now()+self.dur;
            return true
        }           
        false
    }
    pub fn blocking_is_ready(& mut self)->bool{
        while self.ticker.now() <= self.end_time {
            //hprintln!("1 {} {}", self.ticker.now(), self.end_time);
        }
        //hprintln!("2 {} {}", self.ticker.now(), self.end_time);
        self.end_time=self.ticker.now()+self.dur;
        true
    }
}


pub struct Ticker {
    rtc: Rtc,
}

impl Ticker {
    /// Create on startup to get RTC0 going.
    pub fn new(mut nrtc:Rtc) -> Self {
      Ticker { rtc: nrtc } // Return a Ticker instance
    }

    pub fn now(& mut self) -> TickInstant {
        let val= self.rtc.get_datetime();
        let (h,m,s,mic)=val.time().as_hms_micro();
        let converted = (h as u64 * 3_600_000_000u64)
              + (m as u64 * 60_000_000u64)
              + (s as u64 * 1_000_000u64)
              + (mic as u64);
        TickInstant::from_ticks(converted)
    }
}

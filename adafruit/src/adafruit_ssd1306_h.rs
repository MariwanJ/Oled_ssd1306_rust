/*********************************************************************
This is a library for our Monochrome OLEDs based on SSD1306 drivers

  Pick one up today in the adafruit shop!
  ------> http://www.adafruit.com/category/63_98

These displays use SPI to communicate, 4 or 5 pins are required to  
interface

Adafruit invests time and resources providing this open source code, 
please support Adafruit and open-source hardware by purchasing 
products from Adafruit!

Written by Limor Fried/Ladyada  for Adafruit Industries.  
BSD license, check license.txt for more information
All text above, and the splash screen must be included in any redistribution
*********************************************************************/

/*
 *  Modified by Neal Horman 7/14/2012 for use in mbed
 *  Converted to RUST by Mariwan Jalal 18/07/2025 for Nucleo-STM32F767ZI
 */


#![no_std]




use crate::adafruit_gfx_h::{ AdafruitGFX };

pub const SSD1306_EXTERNALVCC:u8 = 0x0;
pub const SSD1306_SWITCHCAPVCC:u8 = 0x1;
use embedded_hal::delay::DelayNs;


/** The pure base class for the SSD1306 display driver.
 *
 * You should derive from this for a new transport interface type,
 * such as the SPI and I2C drivers.
 */

pub struct AdafruitSSD1306<I2C, GPIO, DELAY> 
   where DELAY: DelayNs {
    pub rst: GPIO,
    pub i2c: I2C,
    pub address: u8,
    pub delay: DELAY,
    pub gfx : AdafruitGFX,
}

pub trait Display<I2C, GPIO, DELAY> {
    fn new(ni2c: I2C, n_rst: GPIO, delay: DELAY, ngfx:AdafruitGFX) -> Self;
    fn begin(&mut self, vccstate: u8);
    fn clear_display(&mut self);
    fn invert_display(&mut self, i: bool);
    fn show(&mut self);
    fn splash(&mut self);
    fn send_display_buffer(&mut self);
    // Transport methods
    fn command(&mut self, c: u8);
    fn data(&mut self, c: u8);
    fn copy_adafruit_logo(&mut self, ada_fruit_logo: &[u8], raw_height: i16);
    fn activate_scroll(&mut self);
    fn deactivate_scroll(&mut self);

    fn scroll_horizontal_r(&mut self);
    fn scroll_horizontal_l(&mut self);
    
    fn scroll_diagnol_r(&mut self);
    fn scroll_diagnol_l(&mut self);

    fn scroll_vertical_u(&mut self);
}
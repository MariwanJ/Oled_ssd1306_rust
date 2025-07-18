
/***********************************
This is a our graphics core library, for all our displays. 
We'll be adapting all the
existing libaries to use this core to make updating, support 
and upgrading easier!

Adafruit invests time and resources providing this open source code, 
please support Adafruit and open-source hardware by purchasing 
products from Adafruit!

Written by Limor Fried/Ladyada  for Adafruit Industries.  
BSD license, check license.txt for more information
All text above must be included in any redistribution
****************************************/

/*
 *  Modified by Neal Horman 7/14/2012 for use in mbed
 *  Converted to RUST by Mariwan Jalal 18/07/2025 for Nucleo-STM32F767ZI
 */


#![no_std]

pub fn bv(bit: u8) -> u8{
    1 << bit
}

pub const  BLACK:u8 = 0;
pub const WHITE:u8 = 1;


/**
 * This is a Text and Graphics element drawing class.
 * These functions draw to the display buffer.
 *
 * Display drivers should be derived from here.
 * The Display drivers push the display buffer to the
 * hardware based on application control.
 *
 */
pub struct AdafruitGFX {
    pub raw_width: i16,     // 'raw' display width/height - never changes
    pub raw_height: i16,
    pub width: i16,         // dependent on rotation
    pub height: i16,
    pub cursor_x: i16,
    pub cursor_y: i16,
    pub textcolor: u8,
    pub textbgcolor: u8,
    pub textsize: i16,
    pub rotation: u8,
    pub wrap: bool,         // If set, 'wrap' text at right edge of display
    pub buffer: [u8; 1024], //Uncomment this line and comment the below if you have 128x64 oLED
    //pub buffer: [u8; 512], // <-- fixed-size array syntax (128*32)/8
}

/// Trait for drawable displays
pub trait Drawable {
    /// Paint one BLACK or WHITE pixel in the display buffer
    fn draw_pixel(&mut self, x: i16, y: i16, color: u8); // Required to implement

    /// Stream implementation - provides printf() interface
    fn putc(&mut self, value: char) -> u8 {
        self.write_char(value)
    }

    fn getc(&self) -> i32 {
        -1 // Default implementation (optional)
    }

    /// Helper method to write a character
    fn write_char(&mut self, value: char) -> u8 ;

    /// Helper method to string of character
    fn write_string(&mut self, value: &str)   ;

    /// Draw a horizontal line
    fn draw_fast_h_line(&mut self, x: i16, y: i16, w: i16, color: u8);

    /// Draw a rectangle
    fn draw_rect(&mut self, x: i16, y: i16, w: i16, h: i16, color: u8);

    /// Fill the entire display
    fn fill_screen(&mut self, color: u8);

    /// Draw a circle
    fn draw_circle(&mut self, x0: i16, y0: i16, r: i16, color: u8);

    /// Draw circle helper
    fn draw_circle_helper(&mut self, x0: i16, y0: i16, r: i16, cornername: u8, color: u8);

    /// Fill a circle
    fn fill_circle(&mut self, x0: i16, y0: i16, r: i16, color: u8);

    /// Fill circle helper
    fn fill_circle_helper(&mut self, x0: i16,y0: i16,r: i16, cornername: u8, delta: i16,color: u8,);

    /// Draw a triangle
    fn draw_triangle(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, x2: i16, y2: i16, color: u8);

    /// Fill a triangle
    fn fill_triangle(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, x2: i16, y2: i16, color: u8);

    /// Draw a rounded rectangle
    fn draw_round_rect(&mut self, x0: i16, y0: i16, w: i16, h: i16, radius: i16, color: u8);

    /// Fill a rounded rectangle
    fn fill_round_rect(&mut self, x0: i16, y0: i16, w: i16, h: i16, radius: i16, color: u8);

    /// Draw a bitmap
    fn draw_bitmap(&mut self, x: i16, y: i16, bitmap: &[u8], w: i16, h: i16, color: u8);

    /// Draw a line
    fn draw_line(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, color: u8);

    /// Draw a vertical line
    fn draw_fast_v_line(&mut self, x: i16, y: i16, h: i16, color: u8);

    /// Fill a rectangle
    fn fill_rect(&mut self, x: i16, y: i16, w: i16, h: i16, color: u8);

    /// Draw a text character at a specified pixel location
    fn draw_char(&mut self, x: i16, y: i16, c: u8, color: u8, bg: u8, size: i16);

    /// Get the width of the display in pixels
    fn get_width(&self) -> i16;

    /// Get the height of the display in pixels
    fn get_height(&self) -> i16;

    /// Set the text cursor location
    fn set_text_cursor(&mut self, x: i16, y: i16);

    /// Set the size of the text to be drawn
    fn set_text_size(&mut self, s: i16);

    /// Set the text foreground and background colors to be the same
    fn set_text_color(&mut self, c: u8);

    /// Set the text foreground and background colors independently
    fn set_text_color_independent(&mut self, c: u8, b: u8);

    /// Set text wrapping mode
    fn set_text_wrap(&mut self, w: bool);

    /// Set the display rotation
    fn set_rotation(&mut self, r: u8);

    /// Get the current rotation
    fn get_rotation(&mut self) -> u8;

    fn swap(&mut self, a: &mut i16, b: &mut i16);
}

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

use crate::adafruit_gfx_h::{ bv, AdafruitGFX, Drawable, BLACK, WHITE };
use crate::glcdfont::{ self, FONT };

impl AdafruitGFX {
    pub fn new(w: i16, h: i16) -> Self {
    let buffer: [u8; 1024] = [0; 1024];        //  (128x64)/8   Uncomment this line if you have 64 bit, and comment the below line. 
    //let buffer: [u8; 512] = [0; 512];          // (128x32)/8 
        Self {
            raw_width: w, // this is the 'raw' display w/h - never changes
            raw_height: h,
            width: w,           // dependent on rotation w/h
            height: h,
            cursor_x: 0,
            cursor_y: 0,
            textcolor: WHITE, 
            textbgcolor: BLACK, 
            textsize: 1,
            rotation: 0,
            wrap: true,
            buffer,
        }
    }
}
impl Drawable for AdafruitGFX {
    fn draw_circle(&mut self, x0: i16, y0: i16, r: i16, color: u8) {
        let mut f: i32 = 1 - (r as i32);
        let mut ddF_x = 1;
        let mut ddF_y: i32 = -2 * (r as i32);
        let mut x = 0;
        let mut y = r;

        self.draw_pixel(x0, y0 + r, color);
        self.draw_pixel(x0, y0 - r, color);
        self.draw_pixel(x0 + r, y0, color);
        self.draw_pixel(x0 - r, y0, color);

        while x < y {
            if f >= 0 {
                y -= 1;
                ddF_y += 2;
                f += ddF_y;
            }
            x += 1;
            ddF_x += 2;
            f += ddF_x;

            self.draw_pixel(x0 + x, y0 + y, color);
            self.draw_pixel(x0 - x, y0 + y, color);
            self.draw_pixel(x0 + x, y0 - y, color);
            self.draw_pixel(x0 - x, y0 - y, color);
            self.draw_pixel(x0 + y, y0 + x, color);
            self.draw_pixel(x0 - y, y0 + x, color);
            self.draw_pixel(x0 + y, y0 - x, color);
            self.draw_pixel(x0 - y, y0 - x, color);
        }
    }

    fn draw_pixel(&mut self, mut x: i16, mut  y: i16, color: u8) {
        if x >= self.get_width() || y >= self.get_height() {
            return;
        }

        // Check rotation, move pixel around if necessary
        match self.get_rotation() {
            1 => {
                // Swap x and y
                let temp = x;
                x = y;
                y = temp;
                x = self.raw_width - x - 1;
            }
            2 => {
                x = self.raw_width - x - 1;
                y = self.raw_height - y - 1;
            }
            3 => {
                // Swap x and y
                let temp = x;
                x = y;
                y = temp;
                y = self.raw_height - y - 1;
            }
            _ => {}
        }

        // x is which column
        if color == WHITE {
            self.buffer[(x + (y / 8) * self.raw_width) as usize] |= bv((y % 8) as u8);
        } else {
            self.buffer[(x + (y / 8) * self.raw_width) as usize] &= !bv((y % 8) as u8);
        }
    }

    // Draw a circle helper
    fn draw_circle_helper(&mut self, x0: i16, y0: i16, r: i16, cornername: u8, color: u8) {
        let mut f: i32 = 1 - (r as i32);
        let mut ddF_x = 1;
        let mut ddF_y: i32 = -2 * (r as i32);
        let mut x = 0;
        let mut y = r;

        while x < y {
            if f >= 0 {
                y -= 1;
                ddF_y += 2;
                f += ddF_y;
            }
            x += 1;
            ddF_x += 2;
            f += ddF_x;

            if (cornername & 0x4) != 0 {
                self.draw_pixel(x0 + x, y0 + y, color);
                self.draw_pixel(x0 + y, y0 + x, color);
            }

            if (cornername & 0x2) != 0 {
                self.draw_pixel(x0 + x, y0 - y, color);
                self.draw_pixel(x0 + y, y0 - x, color);
            }

            if (cornername & 0x8) != 0 {
                self.draw_pixel(x0 - y, y0 + x, color);
                self.draw_pixel(x0 - x, y0 + y, color);
            }

            if (cornername & 0x1) != 0 {
                self.draw_pixel(x0 - y, y0 - x, color);
                self.draw_pixel(x0 - x, y0 - y, color);
            }
        }
    }

    // Fill a circle
    fn fill_circle(&mut self, x0: i16, y0: i16, r: i16, color: u8) {
        self.draw_fast_v_line(x0, y0 - r, 2 * r + 1, color);
        self.fill_circle_helper(x0, y0, r, 3, 0, color);
    }

    // Fill circle helper
    fn fill_circle_helper(
        &mut self,
        x0: i16,
        y0: i16,
        r: i16,
        cornername: u8,
        delta: i16,
        color: u8
    ) {
        let mut f: i32 = 1 - (r as i32);
        let mut ddF_x = 1;
        let mut ddF_y: i32 = -2 * (r as i32);
        let mut x = 0;
        let mut y = r;

        while x < y {
            if f >= 0 {
                y -= 1;
                ddF_y += 2;
                f += ddF_y;
            }
            x += 1;
            ddF_x += 2;
            f += ddF_x;

            if (cornername & 0x1) != 0 {
                self.draw_fast_v_line(x0 + x, y0 - y, 2 * y + 1 + delta, color);
                self.draw_fast_v_line(x0 + y, y0 - x, 2 * x + 1 + delta, color);
            }

            if (cornername & 0x2) != 0 {
                self.draw_fast_v_line(x0 - x, y0 - y, 2 * y + 1 + delta, color);
                self.draw_fast_v_line(x0 - y, y0 - x, 2 * x + 1 + delta, color);
            }
        }
    }

    // Bresenham's algorithm for drawing a line
    fn draw_line(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, color: u8) {
        let dx = ((x1 as i32) - (x0 as i32)).abs();
        let dy = ((y1 as i32) - (y0 as i32)).abs();
        let steep = dy > dx;

        let (x0, y0, x1, y1) = if steep {
            (y0, x0, y1, x1) // Swap x and y
        } else {
            (x0, y0, x1, y1)
        };

        let (x0, x1) = if x0 > x1 { (x1, x0) } else { (x0, x1) };

        let dx: i32 = (x1 as i32) - (x0 as i32);
        let dy = ((y1 as i32) - (y0 as i32)).abs();
        let mut err: i32 = dx / 2;
        let ystep: i32 = if y0 < y1 { 1 } else { -1 };

        let mut y: i32 = y0 as i32;

        for x in x0..=x1 {
            if steep {
                self.draw_pixel(y as i16, x, color);
            } else {
                self.draw_pixel(x, y as i16, color);
            }

            err -= dy;
            if err < 0 {
                y += ystep;
                err += dx;
            }
        }
    }

    // Draw a vertical line
    fn draw_fast_v_line(&mut self, x: i16, y: i16, h: i16, color: u8) {
        self.draw_line(x, y, x, y + h - 1, color);
    }

    // Fill a rectangle
    fn fill_rect(&mut self, x: i16, y: i16, w: i16, h: i16, color: u8) {
        for i in x..x + w {
            self.draw_fast_v_line(i, y, h, color);
        }
    }

    // Draw a rectangle
    fn draw_rect(&mut self, x: i16, y: i16, w: i16, h: i16, color: u8) {
        self.draw_fast_h_line(x, y, w, color);
        self.draw_fast_h_line(x, y + h - 1, w, color);
        self.draw_fast_v_line(x, y, h, color);
        self.draw_fast_v_line(x + w - 1, y, h, color);
    }

    // Draw a horizontal line
    fn draw_fast_h_line(&mut self, x: i16, y: i16, w: i16, color: u8) {
        self.draw_line(x, y, x + w - 1, y, color);
    }

    // Fill the entire display
    fn fill_screen(&mut self, color: u8) {
        self.fill_rect(0, 0, self.width as i16, self.height as i16, color);
    }

    // Draw a rounded rectangle
    fn draw_round_rect(&mut self, x: i16, y: i16, w: i16, h: i16, r: i16, color: u8) {
        self.draw_fast_h_line(x + r, y, w - 2 * r, color); // Top
        self.draw_fast_h_line(x + r, y + h - 1, w - 2 * r, color); // Bottom
        self.draw_fast_v_line(x, y + r, h - 2 * r, color); // Left
        self.draw_fast_v_line(x + w - 1, y + r, h - 2 * r, color); // Right
        self.draw_circle_helper(x + r, y + r, r, 1, color); // Top-left
        self.draw_circle_helper(x + w - r - 1, y + r, r, 2, color); // Top-right
        self.draw_circle_helper(x + w - r - 1, y + h - r - 1, r, 4, color); // Bottom-right
        self.draw_circle_helper(x + r, y + h - r - 1, r, 8, color); // Bottom-left
    }

    // Fill a rounded rectangle
    fn fill_round_rect(&mut self, x: i16, y: i16, w: i16, h: i16, r: i16, color: u8) {
        self.fill_rect(x + r, y, w - 2 * r, h, color);
        self.fill_circle_helper(x + w - r - 1, y + r, r, 1, h - 2 * r - 1, color);
        self.fill_circle_helper(x + r, y + r, r, 2, h - 2 * r - 1, color);
    }

    // Draw a triangle
    fn draw_triangle(
        &mut self,
        x0: i16,
        y0: i16,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        color: u8
    ) {
        self.draw_line(x0, y0, x1, y1, color);
        self.draw_line(x1, y1, x2, y2, color);
        self.draw_line(x2, y2, x0, y0, color);
    }

    // Fill a triangle
    fn fill_triangle(
        &mut self,
        mut x0: i16,
        mut y0: i16,
        mut x1: i16,
        mut y1: i16,
        mut x2: i16,
        mut y2: i16,
        mut color: u8
    ) {
        let mut a: i16 = 0;
        let mut b: i16 = 0;
        let mut y: i32 = 0;
        let mut last: i16 = 0;

        // Sort coordinates by Y order (y2 >= y1 >= y0)
        if y0 > y1 {
            (&mut y0, &mut y1);
            self.swap(&mut x0, &mut x1);
        }
        if y1 > y2 {
            self.swap(&mut y2, &mut y1);
            self.swap(&mut x2, &mut x1);
        }
        if y0 > y1 {
            self.swap(&mut y0, &mut y1);
            self.swap(&mut x0, &mut x1);
        }

        if y0 == y2 {
            a = x0;
            b = x0;
            if x1 < a {
                a = x1;
            } else if x1 > b {
                b = x1;
            }
            if x2 < a {
                a = x2;
            } else if x2 > b {
                b = x2;
            }
            self.draw_fast_h_line(a, y0, b - a + 1, color);
            return;
        }

        let dx01 = x1 - x0;
        let dy01 = y1 - y0;
        let dx02 = x2 - x0;
        let dy02 = y2 - y0;
        let dx12 = x2 - x1;
        let dy12 = y2 - y1;
        let mut sa = 0;
        let mut sb = 0;

        if y1 == y2 {
            last = y1;
        } else {
            last = y1 - 1;
        }

        for y in y0..=last {
            a = x0 + sa / dy01;
            b = x0 + sb / dy02;
            sa += dx01;
            sb += dx02;
            if a > b {
                self.swap(&mut a, &mut b);
            }
            self.draw_fast_h_line(a, y, b - a + 1, color);
        }

        sa = dx12 * (((y as i32) - (y1 as i32)) as i16);
        sb = dx02 * (((y as i32) - (y0 as i32)) as i16);
        for y in last + 1..=y2 {
            a = x1 + sa / dy12;
            b = x0 + sb / dy02;
            sa += dx12;
            sb += dx02;
            if a > b {
                self.swap(&mut a, &mut b);
            }
            self.draw_fast_h_line(a, y, b - a + 1, color);
        }
    }

    // Draw a bitmap
    fn draw_bitmap(&mut self, x: i16, y: i16, bitmap: &[u8], w: i16, h: i16, color: u8) {
        for j in 0..h {
            for i in 0..w {
                if (bitmap[(i as usize) + ((j / 8) as usize) * (w as usize)] & bv((1 << j % 8) as u8)) != 0 {
                    self.draw_pixel(x + i, y + j, color);
                }
            }
        }
    }

    // Write a character
    fn write_char(&mut self, c: char) -> u8 {
        if c == '\n' {
            self.cursor_y += (self.textsize * 8) as i16;
            self.cursor_x = 0;
        } else if c == '\r' {
            self.cursor_x = 0;
        } else {
            self.draw_char(
                self.cursor_x as i16,
                self.cursor_y as i16,
                c as u8,            //TODO:Is this correct?
                self.textcolor,
                self.textbgcolor,
                self.textsize
            );
            self.cursor_x += self.textsize * 6;
            if self.wrap && self.cursor_x > (self.width as i16) - self.textsize * 6 {
                self.cursor_y += self.textsize * 8;
                self.cursor_x = 0;
            }
        }
        1 // Return the number of characters written
    }
    fn write_string(&mut self, value: &str) {
        for chr in value.chars() {
            self.write_char(chr);
        }
    }
    
   fn draw_char(&mut self, x: i16, y: i16, c: u8, color: u8, bg: u8, size: i16) {
    if x >= self.width || y >= self.height || (x + (5 * size) - 1) < 0 || (y + (8 * size) - 1) < 0 {
        return;
    }

    for i in 0..6 {
        let mut line = if i == 5 { 0x0 } else { FONT[(c as usize) * 5 + i] };

        for j in 0..8 {
            if (line & 0x1) != 0 {
                if size == 1 {
                    self.draw_pixel(x + (i as i16), y + j, color);
                } else {
                    self.fill_rect(
                        x + (i as i16) * size,
                        y + j * size,
                        size,
                        size,
                        color
                    );
                }
            } else if bg != color {
                if size == 1 {
                    self.draw_pixel(x + (i as i16), y + j, bg);
                } else {
                    self.fill_rect(
                        x + (i as i16) * size,
                        y + j * size,
                        size,
                        size,
                        bg
                    );
                }
            }
            line >>= 1; // Shift line to the right
        }
    }
}

    // Set the display rotation
    fn set_rotation(&mut self, x: u8) {
        self.rotation = x % 4; // Can't be higher than 3
        match self.rotation {
            0 | 2 => {
                self.width = self.raw_width;
                self.height = self.raw_height;
            }
            1 | 3 => {
                self.width = self.raw_height;
                self.height = self.raw_width;
            }
            _ => {}
        }
    }
    fn swap(&mut self, a: &mut i16, b: &mut i16) {
        core::mem::swap(a, b);
    }

    fn get_width(&self) -> i16 {
        self.width
    }

    fn get_height(&self) -> i16 {
        self.height
    }

    fn set_text_cursor(&mut self, x: i16, y: i16) {
        self.cursor_x = x;
        self.cursor_y = y;
    }

    fn set_text_size(&mut self, s: i16) {
        self.textsize = if s > 0 { s } else { 1 };
    }

    fn set_text_color(&mut self, c: u8) {
        self.textcolor = c;
        self.textbgcolor = c;
    }

    fn set_text_color_independent(&mut self, c: u8, b: u8) {
        self.textcolor = c;
        self.textbgcolor = b;
    }

    fn set_text_wrap(&mut self, w: bool) {
        self.wrap = w;
    }

    fn get_rotation(&mut self) -> u8 {
        self.rotation %= 4;
        self.rotation
    }

}

# Oled_ssd1306_rust

![Oled SSD1306](https://cdn-shop.adafruit.com/970x728/931-10.jpg)

Rust implementation of the Adafruit OLED SSD1306 driver for the STM32F7XX Nucleo Board. The original driver was created for Mbed, as per the link below:  
[Adafruit GFX on Mbed](https://os.mbed.com/users/nkhorman/code/Adafruit_GFX/)

This project serves as a proof-of-concept demonstrating the conversion of C++ to Rust. But it involved a lot of effort and challenges.. Please note that I had to use `embedded-hal` version 0.2.7, which is an older version, as the newer releases are incompatible with the `STM32F7XX-hal`. While this is unfortunate, it reflects the current situation.

## My Thoughts After Converting This Driver:

1. The STM32F7XX HAL and `embedded-hal` are problematic.
2. There is a lack of compatibility among different STM32 microcontroller HALs. Several Rust drivers for this OLED exist, but none worked for my Nucleo board.
3. Writing libraries for embedded Rust is a challenging task.
4. Using the driver in a multi-tasking environment has not been tested. It employs blocking I2C. Is it safe? I don't know, and it hasn't been tested.
5. I did not implement any tests for the driver, which is advisable. My goal was simply to assemble a "working" driver.
6. In the example file, you will find how to define serial, timer, and delay for the mentioned STM32.
7. Use the connect.bat to connect start openocd server in a terminal, and then "cargo run" in another terminal to run the code.

Lastly, Rust is not easy, especially for embedded systems. Developing anything in Rust will require significantly more time and effort compared to C or C++. 
Additionally, it may not necessarily be safer either.

## Implementations of the Graphical Driver:

I added additional functionality to the original driver, such as scrolling, which is not present in the C++ version.

## License:

My work, which involves the conversion, is licensed under the MIT License. The original code is under the BSD License, so you can choose between them.

**Warning!!**  
Use this code at your own risk. I provide no warranty that the code will work or is safe.

Mariwan Jalal 18/07/2025

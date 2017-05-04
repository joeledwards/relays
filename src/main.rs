extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;

/* List of pins

+3.3v             1 --  2  +5v
GPIO 2 / SDA1     3 --  4  +5v
GPIO 3 / SCL1     5 --  6  GND
GPIO 4            7 --  8  GPIO 14 / TXD0
GND               9 -- 10  GPIO 15 / RXD0
GPIO 17          11 -- 12  GPIO 18
GPIO 27          13 -- 14  GND
GPIO 22          15 -- 16  GPIO 23
+3.3v            17 -- 18  GPIO 24
GPIO 10 / MOSI   19 -- 20  GND
GPIO 9  / MISO   21 -- 22  GPIO 25
GPIO 11 / SCLK   23 -- 24  GPIO 8  / CE0#
GND              25 -- 26  GPIO 7  / CE1#
GPIO 0  / ID_SD  27 -- 28  GPIO 1  / ID_SC
GPIO 5           29 -- 30  GND
GPIO 6           31 -- 32  GPIO 12
GPIO 13          33 -- 34  GND
GPIO 19 / MISO   35 -- 36  GPIO 16 / CE2#
GPIO 26          37 -- 38  GPIO 20 / MOSI
GND              39 -- 40  GPIO 21 / SCLK

*/

fn main() {
  let mut relays = HashMap::new();
  relays.insert(1, Pin::new(6));  //  6 - 29
  relays.insert(2, Pin::new(13)); // 13 - 31
  relays.insert(3, Pin::new(19)); // 19 - 33
  relays.insert(4, Pin::new(26)); // 26 - 35
  relays.insert(5, Pin::new(12)); // 12 - 32
  relays.insert(6, Pin::new(16)); // 16 - 36
  relays.insert(7, Pin::new(20)); // 20 - 38
  relays.insert(8, Pin::new(21)); // 21 - 40

  // TODO: add control logic.
  
  println!("=== Relays ===");
}


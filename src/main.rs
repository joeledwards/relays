extern crate sysfs_gpio;

use std::collections::HashMap;
use std::collections::LinkedList;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::thread;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

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
    let mut pins: LinkedList<u64> = LinkedList::new();

    pins.push_back(26); //  6 - 29
    pins.push_back(19); // 13 - 31
    pins.push_back(13); // 19 - 33
    pins.push_back(6);  // 26 - 35
    pins.push_back(21); // 12 - 32
    pins.push_back(20); // 16 - 36
    pins.push_back(16); // 20 - 38
    pins.push_back(12); // 21 - 40

    let mut pipes = LinkedList::new();

    for pinNumber in pins {
        let (tx, rx) = channel();
        let pin = Pin::new(pinNumber);
        pipes.push_back(tx);
        thread::spawn(move || {
            pinWorker(&pin, rx)
        });
    }

    let mut state = true;

    println!("=== Relays ===");

    loop {
        for pipe in &pipes {
            sleep(Duration::from_millis(500));
            pipe.send(state);
        }

        state = !state;
    }
  
    println!("=== End ===");
}

fn togglePin(pinNumber: u64) {
  let pin = Pin::new(pinNumber);

  pin.with_exported(|| {
      sleep(Duration::from_millis(80));
      try!(pin.set_direction(Direction::Low));

      loop {
          println!("Setting {} to 0", pinNumber);
          pin.set_value(0).unwrap();
          sleep(Duration::from_millis(1000));
          println!("Setting {} to 1", pinNumber);
          pin.set_value(1).unwrap();
          sleep(Duration::from_millis(1000));
      }
  }).unwrap();
}

fn pinWorker(pin: &Pin, rx: Receiver<bool>) {
    //println!("Pin {} worker started.", pin);

    pin.with_exported(|| {
        sleep(Duration::from_millis(80));
        try!(pin.set_direction(Direction::Low));

        loop {
            let state = match rx.recv().unwrap() {
                true => 1,
                false => 0
            };

            pin.set_value(state).unwrap();
        }
    }).unwrap();
}


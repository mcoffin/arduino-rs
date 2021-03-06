#![no_std]

#![feature(asm)]
#![feature(core)]
#![feature(no_std)]

extern crate core;

pub static INPUT:u32        = 0x00;
pub static OUTPUT:u32       = 0x01;
pub static INPUT_PULLUP:u32 = 0x02;

pub static LOW:u8           = 0x00;
pub static HIGH:u8          = 0x01;
pub static CHANGE:u8        = 0x02;
pub static FALLING:u8       = 0x03;
pub static RISING:u8        = 0x04;

pub static EXTERNAL:u32     = 0x00;
pub static DEFAULT:u32      = 0x01;

#[allow(non_snake_case)]
pub mod ffi {
    #[repr(C)]
    pub enum AnalogReference {
        AR_DEFAULT, // rustc E0083
        Dummy,
    }
    pub use self::AnalogReference::AR_DEFAULT;

	extern "C" {
		pub fn init();

		pub fn pinMode(pin:u32, mode:u32);

		pub fn digitalWrite(pin:u32, value:u8);
		pub fn digitalRead(pin:u32) -> i32;

		pub fn analogReference(mode: AnalogReference);
		pub fn analogRead(pin:u32) -> i32;
		pub fn analogWrite(pin:u32, value:u32);

		pub fn analogReadResolution(res:i32);
		pub fn analogWriteResolution(res:i32);

		pub fn tone(pin:u32, frequency:u32, duration:u32);
		pub fn noTone(pin:u32);

		pub fn shiftOut(data_pin:u32, clock_pin:u32, bit_order:u32, value:u32);
		pub fn shiftIn(data_pin:u32, clock_pin:u32, bit_order:u32) -> u32;
		pub fn pulseIn(pin:u32, state:u32, timeout:u32) -> u32;

		pub fn millis() -> u32;
		pub fn micros() -> u32;
		pub fn delay(ms:u32);
		pub fn delayMicroseconds(us:u32);

		pub fn attachInterrupt(pin:u32, callback:extern "C" fn(), mode:u32);
		pub fn detachInterrupt(pin:u32);
	}
}

pub use self::ffi::*;

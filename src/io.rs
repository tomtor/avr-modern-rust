#[cfg(feature = "attiny1614")]
use avr_device::attiny1614::porta;
#[cfg(feature = "attiny402")]
use avr_device::attiny402::porta;
#[cfg(feature = "avr128db28")]
use avr_device::avr128db28::porta;

pub fn set_high(r: &porta::RegisterBlock, b: u8) {
    unsafe {
        r.out().modify(|r, w| w.bits(r.bits() | b));
    }
}
pub fn set_low(r: &porta::RegisterBlock, b: u8) {
    unsafe {
        r.out().modify(|r, w| w.bits(r.bits() & !b));
    }
}
pub fn set(r: &porta::RegisterBlock, b: u8, v: bool) {
    unsafe {
        r.out()
            .modify(|r, w| w.bits(if v { r.bits() | b } else { r.bits() & !b }));
    }
}
// pub fn get(r: &porta::RegisterBlock, b: u8) -> bool {
//     unsafe {
//         r.input().read
//     }
// }

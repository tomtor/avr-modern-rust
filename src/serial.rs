use crate::{pac::Peripherals, FREQ};

use embedded_io;

use core::convert::Infallible;

pub struct Serial<'a> {
    p: &'a Peripherals,
}

macro_rules! uart {
($usartnr:ident) => {
impl<'a> Serial<'a> {
    pub fn new(dp: &'a Peripherals) -> Serial<'a> {
        #[cfg(any(feature = "attiny402", feature = "attiny1614"))]
        {
            dp.PORTA.out().write(|w| w.pa6().set_bit());
            dp.PORTA.dirset().write(|w| w.pa6().set_bit());
        }
        #[cfg(any(feature = "attiny1614"))]
        dp.PORTMUX.usartroutea().write(|w| w.usart0().alt1());
        #[cfg(any(feature = "avr128db28"))]
        {
            dp.PORTC.out().write(|w| w.pc0().set_bit());
            dp.PORTC.dirset().write(|w| w.pc0().set_bit());
        }
        unsafe {
            #[cfg(feature = "avr128db28")]
            {
                dp.USART1
                    .baud()
                    .write(|w| w.bits((4 * FREQ / 115200) as u16));
            }
            #[cfg(not(feature = "avr128db28"))]
            {
                dp.USART0
                    .baud()
                    .write(|w| w.bits((4 * FREQ / 115200) as u16));
            }
        }
        #[cfg(feature = "avr128db28")]
        dp.USART1.ctrlb().write(|w| w.txen().set_bit());
        #[cfg(not(feature = "avr128db28"))]
        dp.USART0.ctrlb().write(|w| w.txen().set_bit());

        Serial { p: dp }
    }

    pub fn write_c(&self, b: u8) {
        {
            while self.p.$usartnr.status().read().dreif() == false {} // Wait for empty transmit buffer
            unsafe {
                self.p.$usartnr.txdatal().write(|w| w.bits(b));
            }
        }

    }

    pub fn write_ba(&self, s: &[u8]) {
        for b in s {
            self.write_c(*b);
        }
    }

    pub fn write_int(&self, i: u16) {
        if i > 9 {
            self.write_int(i / 10);
            self.write_int(i % 10);
        } else {
            self.write_c(b'0' + i as u8);
        }
    }
}
}
}

#[cfg(feature = "avr128db28")]
uart!(USART1);
#[cfg(not(feature = "avr128db28"))]
uart!(USART0);

impl<'a> ufmt::uWrite for Serial<'a> {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_ba(s.as_bytes());
        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        let mut buf: [u8; 4] = [0; 4];
        self.write_str(c.encode_utf8(&mut buf)).unwrap();
        //self.write_c(c as u8);
        Ok(())
    }
}

impl<'a> embedded_io::ErrorType for Serial<'a> {
    type Error = Infallible;
}

impl<'a> embedded_io::Write for Serial<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.write_ba(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

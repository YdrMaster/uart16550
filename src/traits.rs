use crate::{blocking_read, blocking_write, is_transfer_complete, Register, Uart16550};

impl<R: Register> embedded_io::ErrorType for Uart16550<R> {
    type Error = core::convert::Infallible;
}

impl<R: Register> embedded_io::Read for Uart16550<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Ok(blocking_read(self, buf))
    }
}

impl<R: Register> embedded_io::Write for Uart16550<R> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        Ok(blocking_write(self, buf))
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        while !is_transfer_complete(self) {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

impl<R: Register> embedded_hal_nb::serial::ErrorType for Uart16550<R> {
    type Error = core::convert::Infallible;
}

impl<R: Register> embedded_hal_nb::serial::Read for Uart16550<R> {
    #[inline]
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let mut buf = [0];
        let len = blocking_read(self, &mut buf);
        match len {
            0 => Err(nb::Error::WouldBlock),
            _ => Ok(buf[0]),
        }
    }
}

impl<R: Register> embedded_hal_nb::serial::Write for Uart16550<R> {
    #[inline]
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let len = blocking_write(self, &[word]);
        match len {
            0 => Err(nb::Error::WouldBlock),
            _ => Ok(()),
        }
    }

    #[inline]
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        match is_transfer_complete(self) {
            true => Ok(()),
            false => Err(nb::Error::WouldBlock),
        }
    }
}

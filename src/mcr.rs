use crate::{Register, MCR};

impl<R: Register> MCR<R> {
    #[inline]
    pub fn read(&self) -> ModemControl {
        ModemControl(unsafe { self.0.get().read_volatile() }.val())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ModemControl(u8);

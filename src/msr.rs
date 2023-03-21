use crate::{Register, MSR};

impl<R: Register> MSR<R> {
    #[inline]
    pub fn read(&self) -> ModemStatus {
        ModemStatus(unsafe { self.0.get().read_volatile() }.val())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ModemStatus(u8);

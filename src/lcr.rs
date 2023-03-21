use crate::{Register, LCR};

impl<R: Register> LCR<R> {
    #[inline]
    pub fn read(&self) -> LineControl {
        LineControl(unsafe { self.0.get().read_volatile() }.val())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct LineControl(u8);

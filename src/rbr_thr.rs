use crate::{Register, RBR_THR};

impl<R: Register> RBR_THR<R> {
    #[inline]
    pub fn rx_data(&self) -> u8 {
        unsafe { self.0.get().read_volatile() }.val()
    }

    #[inline]
    pub fn tx_data(&self, val: u8) {
        unsafe { self.0.get().write_volatile(val.into()) }
    }
}

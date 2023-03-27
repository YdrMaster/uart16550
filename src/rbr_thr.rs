use crate::{Register, RBR_THR};

impl<R: Register> RBR_THR<R> {
    /// 从接收缓冲寄存器读取字符。
    #[inline]
    pub fn rx_data(&self) -> u8 {
        unsafe { self.0.get().read_volatile() }.val()
    }

    /// 向发送缓冲寄存器写入字符。
    #[inline]
    pub fn tx_data(&self, val: u8) {
        unsafe { self.0.get().write_volatile(val.into()) }
    }
}

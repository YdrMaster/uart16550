use crate::{Register, IIR_FCR};

impl<R: Register> IIR_FCR<R> {
    /// 写入队列控制设置。
    #[inline]
    pub fn write(&self, val: FifoControl) {
        unsafe { self.0.get().write_volatile(R::from(val.0)) }
    }
}

/// 队列控制设置。
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct FifoControl(u8);

/// 接收队列触发阈值。
///
/// 若接收队列中的字节数不小于阈值设置数据就绪中断。
#[repr(u8)]
pub enum TriggerLevel {
    /// 1 字节。
    _1 = 0b00 << 6,
    /// 4 字节。
    _4 = 0b01 << 6,
    /// 8 字节。
    _8 = 0b10 << 6,
    /// 14 字节。
    _14 = 0b11 << 6,
}

impl TriggerLevel {
    /// 产生一个指定阈值并重置接收队列的队列控制设置。
    #[inline]
    pub const fn and_reset(self) -> FifoControl {
        FifoControl(self as u8 | 0b110)
    }

    /// 产生一个指定阈值但不重置接收队列的队列控制设置。
    #[inline]
    pub const fn without_reset(self) -> FifoControl {
        FifoControl(self as u8)
    }
}

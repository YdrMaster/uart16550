use crate::{Register, IIR_FCR};

impl<R: Register> IIR_FCR<R> {
    #[inline]
    pub fn write(&self, val: FifoStatus) {
        unsafe { self.0.get().write_volatile(R::from(val.0)) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct FifoStatus(u8);

pub enum TriggerLevel {
    _1,
    _4,
    _8,
    _14,
}

impl TriggerLevel {
    #[inline]
    pub const fn and_reset(self) -> FifoStatus {
        match self {
            TriggerLevel::_1 => FifoStatus(0b00_000_11_0),
            TriggerLevel::_4 => FifoStatus(0b01_000_11_0),
            TriggerLevel::_8 => FifoStatus(0b10_000_11_0),
            TriggerLevel::_14 => FifoStatus(0b11_000_11_0),
        }
    }

    #[inline]
    pub const fn without_reset(self) -> FifoStatus {
        match self {
            TriggerLevel::_1 => FifoStatus(0b00),
            TriggerLevel::_4 => FifoStatus(0b01 << 6),
            TriggerLevel::_8 => FifoStatus(0b10 << 6),
            TriggerLevel::_14 => FifoStatus(0b11 << 6),
        }
    }
}

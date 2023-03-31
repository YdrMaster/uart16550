use crate::{Register, MCR};

impl<R: Register> MCR<R> {
    /// 写入调制解调器控制设置。
    #[inline]
    pub fn write(&self, val: ModemControl) {
        unsafe { self.0.get().write_volatile(R::from(val.0)) }
    }

    /// 读取调制解调器控制设置。
    #[inline]
    pub fn read(&self) -> ModemControl {
        ModemControl(unsafe { self.0.get().read_volatile() }.val())
    }
}

/// 调制解调器控制设置。
///
/// TODO: 一般用不到，未实现方法，可以取出值操作。
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct ModemControl(pub u8);

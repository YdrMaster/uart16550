use crate::{Register, MSR};

impl<R: Register> MSR<R> {
    /// 读取调制解调器状态。
    #[inline]
    pub fn read(&self) -> ModemStatus {
        ModemStatus(unsafe { self.0.get().read_volatile() }.val())
    }
}

/// 调制解调器状态。
///
/// TODO: 一般用不到，未实现方法，可以取出值操作。
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct ModemStatus(pub u8);

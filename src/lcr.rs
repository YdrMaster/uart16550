use crate::{Register, LCR};

impl<R: Register> LCR<R> {
    /// 写入线控制设置。
    #[inline]
    pub fn write(&self, interrupts: LineControl) {
        unsafe { self.0.get().write_volatile(R::from(interrupts.0)) }
    }

    /// 读取线控制设置。
    #[inline]
    pub fn read(&self) -> LineControl {
        LineControl(unsafe { self.0.get().read_volatile() }.val())
    }
}

/// 线控制设置。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LineControl(u8);

/// 奇偶校验方式。
#[repr(u8)]
pub enum PARITY {
    /// 无校验位。
    NONE = 0b00 << 3,
    /// 奇校验。
    EVEN = 0b10 << 3,
    /// 偶校验。
    ODD = 0b11 << 3,
}

/// 帧负载长度。
#[repr(u8)]
pub enum CharLen {
    /// 5 位数据位。
    FIVE = 0b00,
    /// 6 位数据位。
    SIX = 0b01,
    /// 7 位数据位。
    SEVEN = 0b10,
    /// 8 位数据位。
    EIGHT = 0b11,
}

impl Default for LineControl {
    #[inline]
    fn default() -> Self {
        Self::CONFIG_8N1
    }
}

impl LineControl {
    /// 8 位数据位，无校验位，1 位停止位。
    pub const CONFIG_8N1: Self = Self(0b11);

    const DLAB: u8 = 1 << 7;
    const BREAK_CTRL_EN: u8 = 1 << 6;
    const STICK_PARITY_EN: u8 = 1 << 5;
    const STOP_BIT_SEL: u8 = 1 << 2;

    /// 使能访问分频控制寄存器。
    #[inline]
    #[allow(unused)]
    pub(crate) const fn enable_dlr_access(self) -> Self {
        Self(self.0 | Self::DLAB)
    }

    /// 禁止访问分频控制寄存器。
    #[inline]
    #[allow(unused)]
    pub(crate) const fn disable_dlr_access(self) -> Self {
        Self(self.0 & !Self::DLAB)
    }

    /// 是否允许访问分频控制寄存器。
    #[inline]
    pub const fn dlr_access_enabled(self) -> bool {
        self.0 & Self::DLAB == Self::DLAB
    }

    /// 使能 break 控制。
    #[inline]
    pub const fn enable_break_control(self) -> Self {
        Self(self.0 | Self::BREAK_CTRL_EN)
    }

    /// 禁止 break 控制。
    #[inline]
    pub const fn disable_break_control(self) -> Self {
        Self(self.0 & !Self::BREAK_CTRL_EN)
    }

    /// 是否允许 break 控制。
    #[inline]
    pub const fn break_control_enabled(self) -> bool {
        self.0 & Self::BREAK_CTRL_EN == Self::BREAK_CTRL_EN
    }

    /// 使能 stick 校验。
    #[inline]
    pub const fn enable_stick_parity(self) -> Self {
        Self(self.0 | Self::STICK_PARITY_EN)
    }

    /// 禁止 stick 校验。
    #[inline]
    pub const fn disable_stick_parity(self) -> Self {
        Self(self.0 & !Self::STICK_PARITY_EN)
    }

    /// 是否允许 stick 校验。
    #[inline]
    pub const fn stick_parity_enabled(self) -> bool {
        self.0 & Self::STICK_PARITY_EN == Self::STICK_PARITY_EN
    }

    /// 设置奇偶校验方式。
    #[inline]
    pub const fn set_parity(self, sel: PARITY) -> Self {
        Self((self.0 & !(0b11 << 3)) | sel as u8)
    }

    /// 奇偶校验方式。
    #[inline]
    pub const fn parity(self) -> PARITY {
        const EVEN: u8 = PARITY::EVEN as _;
        const ODD: u8 = PARITY::ODD as _;
        match self.0 & (0b11 << 3) {
            EVEN => PARITY::EVEN,
            ODD => PARITY::ODD,
            _ => PARITY::NONE,
        }
    }

    /// 设置一位停止位。
    ///
    /// - `val`=`true`，设置一位停止位。
    /// - `val`=`false`，设置超过一位停止位。
    #[inline]
    pub const fn set_one_stop_bit(self, val: bool) -> Self {
        if val {
            Self(self.0 & !Self::STOP_BIT_SEL)
        } else {
            Self(self.0 | Self::STOP_BIT_SEL)
        }
    }

    /// 是否设置一位停止位。
    #[inline]
    pub const fn is_one_stop_bit(self) -> bool {
        self.0 & Self::STOP_BIT_SEL != Self::STOP_BIT_SEL
    }

    /// 设置帧负载长度。
    #[inline]
    pub const fn set_char_len(self, len: CharLen) -> Self {
        Self((self.0 & !0b11) | len as u8)
    }

    /// 帧负载长度。
    #[inline]
    pub const fn char_len(self) -> CharLen {
        unsafe { core::mem::transmute(self.0 & 0b11) }
    }
}

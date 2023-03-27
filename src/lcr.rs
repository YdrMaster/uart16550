use crate::{Register, LCR};

impl<R: Register> LCR<R> {
    #[inline]
    pub fn write(&self, interrupts: LineControl) {
        unsafe { self.0.get().write_volatile(R::from(interrupts.0)) }
    }

    #[inline]
    pub fn read(&self) -> LineControl {
        LineControl(unsafe { self.0.get().read_volatile() }.val())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LineControl(u8);

pub enum PARITY {
    NONE,
    EVEN,
    ODD,
}

#[repr(u8)]
pub enum CharLen {
    FIVE = 0b00,
    SIX = 0b01,
    SEVEN = 0b10,
    EIGHT = 0b11,
}

impl Default for LineControl {
    #[inline]
    fn default() -> Self {
        Self::CONFIG_8N1
    }
}

impl LineControl {
    pub const CONFIG_8N1: Self = Self(0b11);

    const DLAB: u8 = 1 << 7;
    const BREAK_CTRL_EN: u8 = 1 << 6;
    const STICK_PARITY_EN: u8 = 1 << 5;
    const EVEN_PARITY_SEL: u8 = 1 << 4;
    const PARITY_EN: u8 = 1 << 3;
    const STOP_BIT_SEL: u8 = 1 << 2;

    #[inline]
    pub const fn enable_dlr_access(self) -> Self {
        Self(self.0 | Self::DLAB)
    }

    #[inline]
    pub const fn disable_dlr_access(self) -> Self {
        Self(self.0 & !Self::DLAB)
    }

    #[inline]
    pub const fn dlr_access_enabled(self) -> bool {
        self.0 & Self::DLAB == Self::DLAB
    }

    #[inline]
    pub const fn enable_break_control(self) -> Self {
        Self(self.0 | Self::BREAK_CTRL_EN)
    }

    #[inline]
    pub const fn disable_break_control(self) -> Self {
        Self(self.0 & !Self::BREAK_CTRL_EN)
    }

    #[inline]
    pub const fn break_control_enabled(self) -> bool {
        self.0 & Self::BREAK_CTRL_EN == Self::BREAK_CTRL_EN
    }

    #[inline]
    pub const fn enable_stick_parity(self) -> Self {
        Self(self.0 | Self::STICK_PARITY_EN)
    }

    #[inline]
    pub const fn disable_stick_parity(self) -> Self {
        Self(self.0 & !Self::STICK_PARITY_EN)
    }

    #[inline]
    pub const fn stick_parity_enabled(self) -> bool {
        self.0 & Self::STICK_PARITY_EN == Self::STICK_PARITY_EN
    }

    #[inline]
    pub const fn set_parity(self, sel: PARITY) -> Self {
        match sel {
            PARITY::NONE => return Self(self.0 & !Self::PARITY_EN & !Self::EVEN_PARITY_SEL),
            PARITY::ODD => return Self((self.0 | Self::PARITY_EN) & !Self::EVEN_PARITY_SEL),
            PARITY::EVEN => return Self(self.0 | Self::PARITY_EN | Self::EVEN_PARITY_SEL),
        }
    }

    #[inline]
    pub const fn get_parity(self) -> PARITY {
        const ALL: u8 = LineControl::PARITY_EN | LineControl::EVEN_PARITY_SEL;
        match self.0 & ALL {
            Self::PARITY_EN => PARITY::ODD,
            ALL => PARITY::EVEN,
            _ => PARITY::NONE,
        }
    }

    #[inline]
    pub const fn set_one_stop_bit(self, val: bool) -> Self {
        if val {
            Self(self.0 & !Self::STOP_BIT_SEL)
        } else {
            Self(self.0 | Self::STOP_BIT_SEL)
        }
    }

    #[inline]
    pub const fn get_one_stop_bit(self) -> bool {
        self.0 & Self::STOP_BIT_SEL != Self::STOP_BIT_SEL
    }

    #[inline]
    pub const fn set_char_len(self, len: CharLen) -> Self {
        Self((self.0 & !0b11) | len as u8)
    }

    #[inline]
    pub const fn char_len(self) -> CharLen {
        unsafe { core::mem::transmute(self.0 & 0b11) }
    }
}

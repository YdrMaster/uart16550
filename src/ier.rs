use crate::{Register, IER};

impl<R: Register> IER<R> {
    #[inline]
    pub fn write(&self, interrupts: InterruptTypes) {
        unsafe { self.0.get().write_volatile(R::from(interrupts.0)) }
    }

    #[inline]
    pub fn read(&self) -> InterruptTypes {
        InterruptTypes(unsafe { self.0.get().read_volatile() }.val())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct InterruptTypes(u8);

impl InterruptTypes {
    pub const ZERO: Self = Self(0);

    #[inline]
    pub const fn enable_rda(self) -> Self {
        Self(self.0 | (1 << 0))
    }

    #[inline]
    pub const fn disable_rda(self) -> Self {
        Self(self.0 & !(1 << 0))
    }

    #[inline]
    pub const fn enable_thre(self) -> Self {
        Self(self.0 | (1 << 1))
    }

    #[inline]
    pub const fn disable_thre(self) -> Self {
        Self(self.0 & !(1 << 1))
    }

    #[inline]
    pub const fn enable_rls(self) -> Self {
        Self(self.0 | (1 << 2))
    }

    #[inline]
    pub const fn disable_rls(self) -> Self {
        Self(self.0 & !(1 << 2))
    }

    #[inline]
    pub const fn enable_ms(self) -> Self {
        Self(self.0 | (1 << 3))
    }

    #[inline]
    pub const fn disable_ms(self) -> Self {
        Self(self.0 & !(1 << 3))
    }
}

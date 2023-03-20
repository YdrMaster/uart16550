#![no_std]
#![deny(warnings)]

mod fcr;
mod ier;
mod iir;
mod lsr;

use core::cell::UnsafeCell;

pub use fcr::{FifoStatus, TriggerLevel};
pub use ier::InterruptTypes;
pub use iir::{InterruptIdentification, PendingInterrupt};

pub trait Register: From<u8> {
    fn val(self) -> u8;
}

impl Register for u8 {
    #[inline]
    fn val(self) -> u8 {
        self
    }
}

impl Register for u32 {
    #[inline]
    fn val(self) -> u8 {
        self as _
    }
}

#[allow(non_camel_case_types)]
pub struct RBR_THR<R: Register>(UnsafeCell<R>);
pub struct IER<R: Register>(UnsafeCell<R>);
#[allow(non_camel_case_types)]
pub struct IIR_FCR<R: Register>(UnsafeCell<R>);
pub struct LCR<R: Register>(UnsafeCell<R>);
pub struct MCR<R: Register>(UnsafeCell<R>);
pub struct LSR<R: Register>(UnsafeCell<R>);
pub struct MSR<R: Register>(UnsafeCell<R>);

#[allow(unused)]
pub struct Uart16550<R: Register> {
    rbr_thr: RBR_THR<R>, // offset = 0(0x00)
    ier: IER<R>,         // offset = 1(0x04)
    iir_fcr: IIR_FCR<R>, // offset = 2(0x08)
    lcr: LCR<R>,         // offset = 3(0x0c)
    mcr: MCR<R>,         // offset = 4(0x10)
    lsr: LSR<R>,         // offset = 5(0x14)
    msr: MSR<R>,         // offset = 6(0x18)
}

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

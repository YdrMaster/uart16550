#![no_std]
#![deny(warnings)]

mod fcr;
mod ier;
mod iir;
mod lcr;
mod lsr;
mod mcr;
mod msr;
mod rbr_thr;

use core::cell::UnsafeCell;

pub use fcr::{FifoStatus, TriggerLevel};
pub use ier::InterruptTypes;
pub use iir::{InterruptIdentification, PendingInterrupt};
pub use lcr::LineControl;
pub use lsr::LineStatus;
pub use mcr::ModemControl;
pub use msr::ModemStatus;

/// 寄存器特质。
///
/// 由于 16550 设计历史悠久，它的实现有 8 位寄存器和 32 位寄存器两种模式，
/// 在驱动中用这个特质来描述。但无论哪种模式，只要是兼容 16550 定义，就只有 8 位是有效的。
pub trait Register: From<u8> {
    /// 取出寄存器中的有效数字。
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

/// 接收缓冲寄存器和发送保持寄存器。
#[allow(non_camel_case_types)]
pub struct RBR_THR<R: Register>(UnsafeCell<R>);

/// 中断使能寄存器。
pub struct IER<R: Register>(UnsafeCell<R>);

/// 中断识别寄存器和队列控制寄存器。
#[allow(non_camel_case_types)]
pub struct IIR_FCR<R: Register>(UnsafeCell<R>);

/// 线路控制寄存器。
pub struct LCR<R: Register>(UnsafeCell<R>);

/// 调制解调器控制寄存器。
pub struct MCR<R: Register>(UnsafeCell<R>);

/// 线路状态寄存器。
pub struct LSR<R: Register>(UnsafeCell<R>);

/// 调制解调器状态寄存器。
pub struct MSR<R: Register>(UnsafeCell<R>);

/// 工作状态的 uart16550 数据结构。
pub struct Uart16550<R: Register> {
    rbr_thr: RBR_THR<R>, // offset = 0(0x00)
    ier: IER<R>,         // offset = 1(0x04)
    iir_fcr: IIR_FCR<R>, // offset = 2(0x08)
    lcr: LCR<R>,         // offset = 3(0x0c)
    mcr: MCR<R>,         // offset = 4(0x10)
    lsr: LSR<R>,         // offset = 5(0x14)
    msr: MSR<R>,         // offset = 6(0x18)
}

impl<R: Register> Uart16550<R> {
    #[inline]
    pub fn rbr_thr(&self) -> &RBR_THR<R> {
        &self.rbr_thr
    }

    #[inline]
    pub fn ier(&self) -> &IER<R> {
        &self.ier
    }

    #[inline]
    pub fn iir_fcr(&self) -> &IIR_FCR<R> {
        &self.iir_fcr
    }

    #[inline]
    pub fn lcr(&self) -> &LCR<R> {
        &self.lcr
    }

    #[inline]
    pub fn mcr(&self) -> &MCR<R> {
        &self.mcr
    }

    #[inline]
    pub fn lsr(&self) -> &LSR<R> {
        &self.lsr
    }

    pub fn msr(&self) -> &MSR<R> {
        &self.msr
    }

    pub fn read(&self, buf: &mut [u8]) -> usize {
        let mut count = 0usize;
        for c in buf {
            if self.lsr.read().is_data_ready() {
                *c = self.rbr_thr.rx_data();
                count += 1;
            } else {
                break;
            }
        }
        return count;
    }

    pub fn write(&self, buf: &[u8]) -> usize {
        let mut count = 0usize;
        for c in buf {
            if self.lsr.read().is_transmitter_fifo_empty() {
                self.rbr_thr.tx_data(*c);
                count += 1;
            } else {
                break;
            }
        }
        return count;
    }
}

//! Provide definition of 16550 uart registers.

#![no_std]
#![deny(warnings, missing_docs)]

mod fcr;
mod ier;
mod iir;
mod lcr;
mod lsr;
mod mcr;
mod msr;
mod rbr_thr;

use core::cell::UnsafeCell;

pub use fcr::{FifoControl, TriggerLevel};
pub use ier::InterruptTypes;
pub use iir::{InterruptIdentification, PendingInterrupt};
pub use lcr::{CharLen, LineControl, PARITY};
pub use lsr::LineStatus;
pub use mcr::ModemControl;
pub use msr::ModemStatus;

/// 寄存器特质。
///
/// 由于 16550 设计历史悠久，它的实现有 8 位寄存器和 32 位寄存器两种模式，
/// 在驱动中用这个特质来描述。但无论哪种模式，只要是兼容 16550 定义，就只有 8 位是有效的。
pub trait Register: From<u8> {
    /// 取出寄存器中的有效位。
    fn val(self) -> u8;
}

/// 寄存器的 8 位模式。
impl Register for u8 {
    #[inline]
    fn val(self) -> u8 {
        self
    }
}

/// 寄存器的 32 位模式。
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
#[repr(C)]
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
    /// 取出接收缓冲和发送保持寄存器。
    #[inline]
    pub fn rbr_thr(&self) -> &RBR_THR<R> {
        &self.rbr_thr
    }

    /// 取出中断使能寄存器。
    #[inline]
    pub fn ier(&self) -> &IER<R> {
        &self.ier
    }

    /// 取出中断识别和队列控制寄存器。
    #[inline]
    pub fn iir_fcr(&self) -> &IIR_FCR<R> {
        &self.iir_fcr
    }

    /// 取出线路控制寄存器。
    #[inline]
    pub fn lcr(&self) -> &LCR<R> {
        &self.lcr
    }

    /// 取出调制解调器控制寄存器。
    #[inline]
    pub fn mcr(&self) -> &MCR<R> {
        &self.mcr
    }

    /// 取出线路状态寄存器。
    #[inline]
    pub fn lsr(&self) -> &LSR<R> {
        &self.lsr
    }

    /// 取出调制解调器状态寄存器。
    #[inline]
    pub fn msr(&self) -> &MSR<R> {
        &self.msr
    }

    /// 将分频系数写入锁存器。
    pub fn write_divisor(&self, divisor: u16) {
        let lcr = self.lcr.read();
        self.lcr.write(lcr.enable_dlr_access());
        unsafe {
            self.rbr_thr.0.get().write(R::from(divisor as _));
            self.ier.0.get().write(R::from((divisor >> 8) as _));
        }
        self.lcr.write(lcr);
    }

    /// 从接收队列读取字符到 `buf`，返回读取的字符数。
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
        count
    }

    /// 从 `buf` 写入字符到发送队列，返回写入的字符数。
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
        count
    }
}

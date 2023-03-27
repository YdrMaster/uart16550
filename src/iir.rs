use crate::{Register, IIR_FCR};

impl<R: Register> IIR_FCR<R> {
    /// 读取中断识别位。
    #[inline]
    pub fn read(&self) -> InterruptIdentification {
        InterruptIdentification(unsafe { self.0.get().read_volatile() }.val())
    }
}

/// 中断识别位。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InterruptIdentification(u8);

/// 挂起中断类型。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PendingInterrupt {
    /// 接收线路状态。
    ReceiverLineStatus,
    /// 接收数据可用。
    ReceivedDataAvailable,
    /// 接收数据超时。
    ReceivedDataTimeout,
    /// 发送保持寄存器空。
    TransmitterHoldingRegisterEmpty,
    /// 调制解调器状态。
    ModemStatus,
}

impl InterruptIdentification {
    /// 挂起中断类型。
    #[inline]
    pub const fn pending_interrupts(&self) -> Option<PendingInterrupt> {
        match self.0 & 0b1111 {
            0b0110 => Some(PendingInterrupt::ReceiverLineStatus),
            0b0100 => Some(PendingInterrupt::ReceivedDataAvailable),
            0b1100 => Some(PendingInterrupt::ReceivedDataTimeout),
            0b0010 => Some(PendingInterrupt::TransmitterHoldingRegisterEmpty),
            0b0000 => Some(PendingInterrupt::ModemStatus),
            0b0001 => None,
            _ => unreachable!(),
        }
    }

    /// 是否启用队列。
    #[inline]
    pub const fn fifos_enabled(&self) -> bool {
        self.0 & 0b1100_0000 == 0b1100_0000
    }
}

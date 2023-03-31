use crate::{Register, LSR};

impl<R: Register> LSR<R> {
    /// 读取线路状态。
    #[inline]
    pub fn read(&self) -> LineStatus {
        LineStatus(unsafe { self.0.get().read_volatile() }.val())
    }
}

/// 线路状态。
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct LineStatus(u8);

impl LineStatus {
    const DATA_RDY: u8 = 1 << 0;
    const OVERRUN_ERR: u8 = 1 << 1;
    const PARITY_ERR: u8 = 1 << 2;
    const FRAMING_ERR: u8 = 1 << 3;
    const BREAK_INT: u8 = 1 << 4;
    const THR_EMPTY: u8 = 1 << 5;
    const XMITR_EMPTY: u8 = 1 << 6;
    const RCVR_FIFO_ERR: u8 = 1 << 7;

    /// 如果接收缓冲寄存器中有数据，返回 `true`。
    #[inline]
    pub const fn is_data_ready(&self) -> bool {
        self.0 & Self::DATA_RDY == Self::DATA_RDY
    }

    /// 如果发生溢出错误，返回 `true`。
    #[inline]
    pub const fn is_overrun_error(&self) -> bool {
        self.0 & Self::OVERRUN_ERR == Self::OVERRUN_ERR
    }

    /// 如果发生奇偶校验错误，返回 `true`。
    #[inline]
    pub const fn is_parity_error(&self) -> bool {
        self.0 & Self::PARITY_ERR == Self::PARITY_ERR
    }

    /// 如果发生帧错误，返回 `true`。
    #[inline]
    pub const fn is_framing_error(&self) -> bool {
        self.0 & Self::FRAMING_ERR == Self::FRAMING_ERR
    }

    /// 如果发生断线中断，返回 `true`。
    #[inline]
    pub const fn is_break_condition(&self) -> bool {
        self.0 & Self::BREAK_INT == Self::BREAK_INT
    }

    /// 如果发送队列为空，返回 `true`。
    #[inline]
    pub const fn is_transmitter_fifo_empty(&self) -> bool {
        self.0 & Self::THR_EMPTY == Self::THR_EMPTY
    }

    /// 如果发送寄存器和队列皆为空，返回 `true`。
    #[inline]
    pub const fn is_transmitter_empty(&self) -> bool {
        self.0 & Self::XMITR_EMPTY == Self::XMITR_EMPTY
    }

    /// 如果接收队列中有错误，返回 `true`。
    #[inline]
    pub const fn is_receiver_fifo_error(&self) -> bool {
        self.0 & Self::RCVR_FIFO_ERR == Self::RCVR_FIFO_ERR
    }
}

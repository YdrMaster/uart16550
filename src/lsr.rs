use crate::{Register, LSR};

impl<R: Register> LSR<R> {
    #[inline]
    pub fn read(&self) -> LineStatus {
        LineStatus(unsafe { self.0.get().read_volatile() }.val())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
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

    #[inline]
    pub const fn is_data_ready(&self) -> bool {
        self.0 & Self::DATA_RDY == Self::DATA_RDY
    }

    #[inline]
    pub const fn is_overrun_error(&self) -> bool {
        self.0 & Self::OVERRUN_ERR == Self::OVERRUN_ERR
    }

    #[inline]
    pub const fn is_parity_error(&self) -> bool {
        self.0 & Self::PARITY_ERR == Self::PARITY_ERR
    }

    #[inline]
    pub const fn is_framing_error(&self) -> bool {
        self.0 & Self::FRAMING_ERR == Self::FRAMING_ERR
    }

    #[inline]
    pub const fn is_break_condition(&self) -> bool {
        self.0 & Self::BREAK_INT == Self::BREAK_INT
    }

    #[inline]
    pub const fn is_transmitter_fifo_empty(&self) -> bool {
        self.0 & Self::THR_EMPTY == Self::THR_EMPTY
    }

    #[inline]
    pub const fn is_transmitter_empty(&self) -> bool {
        self.0 & Self::XMITR_EMPTY == Self::XMITR_EMPTY
    }

    #[inline]
    pub const fn is_receiver_fifo_error(&self) -> bool {
        self.0 & Self::RCVR_FIFO_ERR == Self::RCVR_FIFO_ERR
    }
}

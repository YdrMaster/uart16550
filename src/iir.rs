use crate::{Register, IIR_FCR};

impl<R: Register> IIR_FCR<R> {
    #[inline]
    pub fn read(&self) -> InterruptIdentification {
        InterruptIdentification(unsafe { self.0.get().read_volatile() }.val())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InterruptIdentification(u8);

pub enum PendingInterrupt {
    ReceiverLineStatus,
    ReceivedDataAvailable,
    TransmitterHoldingRegisterEmpty,
    ModemStatus,
}

impl InterruptIdentification {
    pub const fn pending_interrupts(&self) -> Option<(PendingInterrupt, bool)> {
        if self.0 & 1 == 1 {
            Some((
                match self.0 & 0b0110 {
                    0b11 => PendingInterrupt::ReceiverLineStatus,
                    0b10 => PendingInterrupt::ReceivedDataAvailable,
                    0b01 => PendingInterrupt::TransmitterHoldingRegisterEmpty,
                    0b00 => PendingInterrupt::ModemStatus,
                    _ => unreachable!(),
                },
                self.0 & 0b1000 == 0b1000,
            ))
        } else {
            None
        }
    }

    #[inline]
    pub const fn fifos_enabled(&self) -> bool {
        self.0 & 0b1100_0000 == 0b1100_0000
    }
}

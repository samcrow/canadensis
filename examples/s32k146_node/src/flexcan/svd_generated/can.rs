#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcr: Mcr,
    ctrl1: Ctrl1,
    timer: Timer,
    _reserved3: [u8; 0x04],
    rxmgmask: Rxmgmask,
    rx14mask: Rx14mask,
    rx15mask: Rx15mask,
    ecr: Ecr,
    esr1: Esr1,
    imask2: Imask2,
    imask1: Imask1,
    iflag2: Iflag2,
    iflag1: Iflag1,
    ctrl2: Ctrl2,
    esr2: Esr2,
    _reserved14: [u8; 0x08],
    crcr: Crcr,
    rxfgmask: Rxfgmask,
    rxfir: Rxfir,
    cbt: Cbt,
    _reserved18: [u8; 0x14],
    imask4: Imask4,
    imask3: Imask3,
    iflag4: Iflag4,
    iflag3: Iflag3,
    _reserved22: [u8; 0x0808],
    rximr: [Rximr; 128],
    _reserved23: [u8; 0x60],
    mecr: Mecr,
    erriar: Erriar,
    erridpr: Erridpr,
    errippr: Errippr,
    rerrar: Rerrar,
    rerrdr: Rerrdr,
    rerrsynr: Rerrsynr,
    errsr: Errsr,
    _reserved31: [u8; 0xf0],
    eprs: Eprs,
    encbt: Encbt,
    edcbt: Edcbt,
    etdc: Etdc,
    fdctrl: Fdctrl,
    fdcbt: Fdcbt,
    fdcrc: Fdcrc,
    erfcr: Erfcr,
    erfier: Erfier,
    erfsr: Erfsr,
    _reserved41: [u8; 0x18],
    hr_time_stamp: [HrTimeStamp; 128],
    _reserved42: [u8; 0x21d0],
    erffel: [Erffel; 128],
}
impl RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x04 - Control 1 Register"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x08 - Free Running Timer"]
    #[inline(always)]
    pub const fn timer(&self) -> &Timer {
        &self.timer
    }
    #[doc = "0x10 - Rx Mailboxes Global Mask Register"]
    #[inline(always)]
    pub const fn rxmgmask(&self) -> &Rxmgmask {
        &self.rxmgmask
    }
    #[doc = "0x14 - Rx 14 Mask Register"]
    #[inline(always)]
    pub const fn rx14mask(&self) -> &Rx14mask {
        &self.rx14mask
    }
    #[doc = "0x18 - Rx 15 Mask Register"]
    #[inline(always)]
    pub const fn rx15mask(&self) -> &Rx15mask {
        &self.rx15mask
    }
    #[doc = "0x1c - Error Counter"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x20 - Error and Status 1 Register"]
    #[inline(always)]
    pub const fn esr1(&self) -> &Esr1 {
        &self.esr1
    }
    #[doc = "0x24 - Interrupt Masks 2 Register"]
    #[inline(always)]
    pub const fn imask2(&self) -> &Imask2 {
        &self.imask2
    }
    #[doc = "0x28 - Interrupt Masks 1 Register"]
    #[inline(always)]
    pub const fn imask1(&self) -> &Imask1 {
        &self.imask1
    }
    #[doc = "0x2c - Interrupt Flags 2 Register"]
    #[inline(always)]
    pub const fn iflag2(&self) -> &Iflag2 {
        &self.iflag2
    }
    #[doc = "0x30 - Interrupt Flags 1 Register"]
    #[inline(always)]
    pub const fn iflag1(&self) -> &Iflag1 {
        &self.iflag1
    }
    #[doc = "0x34 - Control 2 Register"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x38 - Error and Status 2 Register"]
    #[inline(always)]
    pub const fn esr2(&self) -> &Esr2 {
        &self.esr2
    }
    #[doc = "0x44 - CRC Register"]
    #[inline(always)]
    pub const fn crcr(&self) -> &Crcr {
        &self.crcr
    }
    #[doc = "0x48 - Legacy Rx FIFO Global Mask Register"]
    #[inline(always)]
    pub const fn rxfgmask(&self) -> &Rxfgmask {
        &self.rxfgmask
    }
    #[doc = "0x4c - Legacy Rx FIFO Information Register"]
    #[inline(always)]
    pub const fn rxfir(&self) -> &Rxfir {
        &self.rxfir
    }
    #[doc = "0x50 - CAN Bit Timing Register"]
    #[inline(always)]
    pub const fn cbt(&self) -> &Cbt {
        &self.cbt
    }
    #[doc = "0x68 - Interrupt Masks 4 Register"]
    #[inline(always)]
    pub const fn imask4(&self) -> &Imask4 {
        &self.imask4
    }
    #[doc = "0x6c - Interrupt Masks 3 Register"]
    #[inline(always)]
    pub const fn imask3(&self) -> &Imask3 {
        &self.imask3
    }
    #[doc = "0x70 - Interrupt Flags 4 Register"]
    #[inline(always)]
    pub const fn iflag4(&self) -> &Iflag4 {
        &self.iflag4
    }
    #[doc = "0x74 - Interrupt Flags 3 Register"]
    #[inline(always)]
    pub const fn iflag3(&self) -> &Iflag3 {
        &self.iflag3
    }
    #[doc = "0x880..0xa80 - Rx Individual Mask Registers"]
    #[inline(always)]
    pub const fn rximr(&self, n: usize) -> &Rximr {
        &self.rximr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x880..0xa80 - Rx Individual Mask Registers"]
    #[inline(always)]
    pub fn rximr_iter(&self) -> impl Iterator<Item = &Rximr> {
        self.rximr.iter()
    }
    #[doc = "0xae0 - Memory Error Control Register"]
    #[inline(always)]
    pub const fn mecr(&self) -> &Mecr {
        &self.mecr
    }
    #[doc = "0xae4 - Error Injection Address Register"]
    #[inline(always)]
    pub const fn erriar(&self) -> &Erriar {
        &self.erriar
    }
    #[doc = "0xae8 - Error Injection Data Pattern Register"]
    #[inline(always)]
    pub const fn erridpr(&self) -> &Erridpr {
        &self.erridpr
    }
    #[doc = "0xaec - Error Injection Parity Pattern Register"]
    #[inline(always)]
    pub const fn errippr(&self) -> &Errippr {
        &self.errippr
    }
    #[doc = "0xaf0 - Error Report Address Register"]
    #[inline(always)]
    pub const fn rerrar(&self) -> &Rerrar {
        &self.rerrar
    }
    #[doc = "0xaf4 - Error Report Data Register"]
    #[inline(always)]
    pub const fn rerrdr(&self) -> &Rerrdr {
        &self.rerrdr
    }
    #[doc = "0xaf8 - Error Report Syndrome Register"]
    #[inline(always)]
    pub const fn rerrsynr(&self) -> &Rerrsynr {
        &self.rerrsynr
    }
    #[doc = "0xafc - Error Status Register"]
    #[inline(always)]
    pub const fn errsr(&self) -> &Errsr {
        &self.errsr
    }
    #[doc = "0xbf0 - Enhanced CAN Bit Timing Prescalers"]
    #[inline(always)]
    pub const fn eprs(&self) -> &Eprs {
        &self.eprs
    }
    #[doc = "0xbf4 - Enhanced Nominal CAN Bit Timing"]
    #[inline(always)]
    pub const fn encbt(&self) -> &Encbt {
        &self.encbt
    }
    #[doc = "0xbf8 - Enhanced Data Phase CAN bit Timing"]
    #[inline(always)]
    pub const fn edcbt(&self) -> &Edcbt {
        &self.edcbt
    }
    #[doc = "0xbfc - Enhanced Transceiver Delay Compensation"]
    #[inline(always)]
    pub const fn etdc(&self) -> &Etdc {
        &self.etdc
    }
    #[doc = "0xc00 - CAN FD Control Register"]
    #[inline(always)]
    pub const fn fdctrl(&self) -> &Fdctrl {
        &self.fdctrl
    }
    #[doc = "0xc04 - CAN FD Bit Timing Register"]
    #[inline(always)]
    pub const fn fdcbt(&self) -> &Fdcbt {
        &self.fdcbt
    }
    #[doc = "0xc08 - CAN FD CRC Register"]
    #[inline(always)]
    pub const fn fdcrc(&self) -> &Fdcrc {
        &self.fdcrc
    }
    #[doc = "0xc0c - Enhanced Rx FIFO Control Register"]
    #[inline(always)]
    pub const fn erfcr(&self) -> &Erfcr {
        &self.erfcr
    }
    #[doc = "0xc10 - Enhanced Rx FIFO Interrupt Enable Register"]
    #[inline(always)]
    pub const fn erfier(&self) -> &Erfier {
        &self.erfier
    }
    #[doc = "0xc14 - Enhanced Rx FIFO Status Register"]
    #[inline(always)]
    pub const fn erfsr(&self) -> &Erfsr {
        &self.erfsr
    }
    #[doc = "0xc30..0xe30 - High Resolution Time Stamp"]
    #[inline(always)]
    pub const fn hr_time_stamp(&self, n: usize) -> &HrTimeStamp {
        &self.hr_time_stamp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc30..0xe30 - High Resolution Time Stamp"]
    #[inline(always)]
    pub fn hr_time_stamp_iter(&self) -> impl Iterator<Item = &HrTimeStamp> {
        self.hr_time_stamp.iter()
    }
    #[doc = "0x3000..0x3200 - Enhanced Rx FIFO Filter Element"]
    #[inline(always)]
    pub const fn erffel(&self, n: usize) -> &Erffel {
        &self.erffel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3000..0x3200 - Enhanced Rx FIFO Filter Element"]
    #[inline(always)]
    pub fn erffel_iter(&self) -> impl Iterator<Item = &Erffel> {
        self.erffel.iter()
    }
}
#[doc = "MCR (rw) register accessor: Module Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "CTRL1 (rw) register accessor: Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Control 1 Register"]
pub mod ctrl1;
#[doc = "TIMER (rw) register accessor: Free Running Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer`] module"]
#[doc(alias = "TIMER")]
pub type Timer = crate::Reg<timer::TimerSpec>;
#[doc = "Free Running Timer"]
pub mod timer;
#[doc = "RXMGMASK (rw) register accessor: Rx Mailboxes Global Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmgmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmgmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmgmask`] module"]
#[doc(alias = "RXMGMASK")]
pub type Rxmgmask = crate::Reg<rxmgmask::RxmgmaskSpec>;
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod rxmgmask;
#[doc = "RX14MASK (rw) register accessor: Rx 14 Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx14mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx14mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx14mask`] module"]
#[doc(alias = "RX14MASK")]
pub type Rx14mask = crate::Reg<rx14mask::Rx14maskSpec>;
#[doc = "Rx 14 Mask Register"]
pub mod rx14mask;
#[doc = "RX15MASK (rw) register accessor: Rx 15 Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx15mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx15mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx15mask`] module"]
#[doc(alias = "RX15MASK")]
pub type Rx15mask = crate::Reg<rx15mask::Rx15maskSpec>;
#[doc = "Rx 15 Mask Register"]
pub mod rx15mask;
#[doc = "ECR (rw) register accessor: Error Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`] module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "ESR1 (rw) register accessor: Error and Status 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esr1`] module"]
#[doc(alias = "ESR1")]
pub type Esr1 = crate::Reg<esr1::Esr1Spec>;
#[doc = "Error and Status 1 Register"]
pub mod esr1;
#[doc = "IMASK2 (rw) register accessor: Interrupt Masks 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask2`] module"]
#[doc(alias = "IMASK2")]
pub type Imask2 = crate::Reg<imask2::Imask2Spec>;
#[doc = "Interrupt Masks 2 Register"]
pub mod imask2;
#[doc = "IMASK1 (rw) register accessor: Interrupt Masks 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask1`] module"]
#[doc(alias = "IMASK1")]
pub type Imask1 = crate::Reg<imask1::Imask1Spec>;
#[doc = "Interrupt Masks 1 Register"]
pub mod imask1;
#[doc = "IFLAG2 (rw) register accessor: Interrupt Flags 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iflag2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iflag2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iflag2`] module"]
#[doc(alias = "IFLAG2")]
pub type Iflag2 = crate::Reg<iflag2::Iflag2Spec>;
#[doc = "Interrupt Flags 2 Register"]
pub mod iflag2;
#[doc = "IFLAG1 (rw) register accessor: Interrupt Flags 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iflag1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iflag1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iflag1`] module"]
#[doc(alias = "IFLAG1")]
pub type Iflag1 = crate::Reg<iflag1::Iflag1Spec>;
#[doc = "Interrupt Flags 1 Register"]
pub mod iflag1;
#[doc = "CTRL2 (rw) register accessor: Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Control 2 Register"]
pub mod ctrl2;
#[doc = "ESR2 (r) register accessor: Error and Status 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esr2`] module"]
#[doc(alias = "ESR2")]
pub type Esr2 = crate::Reg<esr2::Esr2Spec>;
#[doc = "Error and Status 2 Register"]
pub mod esr2;
#[doc = "CRCR (r) register accessor: CRC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcr`] module"]
#[doc(alias = "CRCR")]
pub type Crcr = crate::Reg<crcr::CrcrSpec>;
#[doc = "CRC Register"]
pub mod crcr;
#[doc = "RXFGMASK (rw) register accessor: Legacy Rx FIFO Global Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfgmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfgmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfgmask`] module"]
#[doc(alias = "RXFGMASK")]
pub type Rxfgmask = crate::Reg<rxfgmask::RxfgmaskSpec>;
#[doc = "Legacy Rx FIFO Global Mask Register"]
pub mod rxfgmask;
#[doc = "RXFIR (r) register accessor: Legacy Rx FIFO Information Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfir`] module"]
#[doc(alias = "RXFIR")]
pub type Rxfir = crate::Reg<rxfir::RxfirSpec>;
#[doc = "Legacy Rx FIFO Information Register"]
pub mod rxfir;
#[doc = "CBT (rw) register accessor: CAN Bit Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cbt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbt`] module"]
#[doc(alias = "CBT")]
pub type Cbt = crate::Reg<cbt::CbtSpec>;
#[doc = "CAN Bit Timing Register"]
pub mod cbt;
#[doc = "IMASK4 (rw) register accessor: Interrupt Masks 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask4`] module"]
#[doc(alias = "IMASK4")]
pub type Imask4 = crate::Reg<imask4::Imask4Spec>;
#[doc = "Interrupt Masks 4 Register"]
pub mod imask4;
#[doc = "IMASK3 (rw) register accessor: Interrupt Masks 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask3`] module"]
#[doc(alias = "IMASK3")]
pub type Imask3 = crate::Reg<imask3::Imask3Spec>;
#[doc = "Interrupt Masks 3 Register"]
pub mod imask3;
#[doc = "IFLAG4 (rw) register accessor: Interrupt Flags 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iflag4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iflag4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iflag4`] module"]
#[doc(alias = "IFLAG4")]
pub type Iflag4 = crate::Reg<iflag4::Iflag4Spec>;
#[doc = "Interrupt Flags 4 Register"]
pub mod iflag4;
#[doc = "IFLAG3 (rw) register accessor: Interrupt Flags 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iflag3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iflag3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iflag3`] module"]
#[doc(alias = "IFLAG3")]
pub type Iflag3 = crate::Reg<iflag3::Iflag3Spec>;
#[doc = "Interrupt Flags 3 Register"]
pub mod iflag3;
#[doc = "RXIMR (rw) register accessor: Rx Individual Mask Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rximr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rximr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rximr`] module"]
#[doc(alias = "RXIMR")]
pub type Rximr = crate::Reg<rximr::RximrSpec>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr;
#[doc = "MECR (rw) register accessor: Memory Error Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mecr`] module"]
#[doc(alias = "MECR")]
pub type Mecr = crate::Reg<mecr::MecrSpec>;
#[doc = "Memory Error Control Register"]
pub mod mecr;
#[doc = "ERRIAR (rw) register accessor: Error Injection Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erriar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erriar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erriar`] module"]
#[doc(alias = "ERRIAR")]
pub type Erriar = crate::Reg<erriar::ErriarSpec>;
#[doc = "Error Injection Address Register"]
pub mod erriar;
#[doc = "ERRIDPR (rw) register accessor: Error Injection Data Pattern Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erridpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erridpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erridpr`] module"]
#[doc(alias = "ERRIDPR")]
pub type Erridpr = crate::Reg<erridpr::ErridprSpec>;
#[doc = "Error Injection Data Pattern Register"]
pub mod erridpr;
#[doc = "ERRIPPR (rw) register accessor: Error Injection Parity Pattern Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errippr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errippr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errippr`] module"]
#[doc(alias = "ERRIPPR")]
pub type Errippr = crate::Reg<errippr::ErripprSpec>;
#[doc = "Error Injection Parity Pattern Register"]
pub mod errippr;
#[doc = "RERRAR (r) register accessor: Error Report Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rerrar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rerrar`] module"]
#[doc(alias = "RERRAR")]
pub type Rerrar = crate::Reg<rerrar::RerrarSpec>;
#[doc = "Error Report Address Register"]
pub mod rerrar;
#[doc = "RERRDR (r) register accessor: Error Report Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rerrdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rerrdr`] module"]
#[doc(alias = "RERRDR")]
pub type Rerrdr = crate::Reg<rerrdr::RerrdrSpec>;
#[doc = "Error Report Data Register"]
pub mod rerrdr;
#[doc = "RERRSYNR (r) register accessor: Error Report Syndrome Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rerrsynr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rerrsynr`] module"]
#[doc(alias = "RERRSYNR")]
pub type Rerrsynr = crate::Reg<rerrsynr::RerrsynrSpec>;
#[doc = "Error Report Syndrome Register"]
pub mod rerrsynr;
#[doc = "ERRSR (rw) register accessor: Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errsr`] module"]
#[doc(alias = "ERRSR")]
pub type Errsr = crate::Reg<errsr::ErrsrSpec>;
#[doc = "Error Status Register"]
pub mod errsr;
#[doc = "EPRS (rw) register accessor: Enhanced CAN Bit Timing Prescalers\n\nYou can [`read`](crate::Reg::read) this register and get [`eprs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eprs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eprs`] module"]
#[doc(alias = "EPRS")]
pub type Eprs = crate::Reg<eprs::EprsSpec>;
#[doc = "Enhanced CAN Bit Timing Prescalers"]
pub mod eprs;
#[doc = "ENCBT (rw) register accessor: Enhanced Nominal CAN Bit Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`encbt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`encbt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@encbt`] module"]
#[doc(alias = "ENCBT")]
pub type Encbt = crate::Reg<encbt::EncbtSpec>;
#[doc = "Enhanced Nominal CAN Bit Timing"]
pub mod encbt;
#[doc = "EDCBT (rw) register accessor: Enhanced Data Phase CAN bit Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`edcbt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edcbt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edcbt`] module"]
#[doc(alias = "EDCBT")]
pub type Edcbt = crate::Reg<edcbt::EdcbtSpec>;
#[doc = "Enhanced Data Phase CAN bit Timing"]
pub mod edcbt;
#[doc = "ETDC (rw) register accessor: Enhanced Transceiver Delay Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`etdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etdc`] module"]
#[doc(alias = "ETDC")]
pub type Etdc = crate::Reg<etdc::EtdcSpec>;
#[doc = "Enhanced Transceiver Delay Compensation"]
pub mod etdc;
#[doc = "FDCTRL (rw) register accessor: CAN FD Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdctrl`] module"]
#[doc(alias = "FDCTRL")]
pub type Fdctrl = crate::Reg<fdctrl::FdctrlSpec>;
#[doc = "CAN FD Control Register"]
pub mod fdctrl;
#[doc = "FDCBT (rw) register accessor: CAN FD Bit Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcbt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcbt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcbt`] module"]
#[doc(alias = "FDCBT")]
pub type Fdcbt = crate::Reg<fdcbt::FdcbtSpec>;
#[doc = "CAN FD Bit Timing Register"]
pub mod fdcbt;
#[doc = "FDCRC (r) register accessor: CAN FD CRC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcrc`] module"]
#[doc(alias = "FDCRC")]
pub type Fdcrc = crate::Reg<fdcrc::FdcrcSpec>;
#[doc = "CAN FD CRC Register"]
pub mod fdcrc;
#[doc = "ERFCR (rw) register accessor: Enhanced Rx FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erfcr`] module"]
#[doc(alias = "ERFCR")]
pub type Erfcr = crate::Reg<erfcr::ErfcrSpec>;
#[doc = "Enhanced Rx FIFO Control Register"]
pub mod erfcr;
#[doc = "ERFIER (rw) register accessor: Enhanced Rx FIFO Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erfier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erfier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erfier`] module"]
#[doc(alias = "ERFIER")]
pub type Erfier = crate::Reg<erfier::ErfierSpec>;
#[doc = "Enhanced Rx FIFO Interrupt Enable Register"]
pub mod erfier;
#[doc = "ERFSR (rw) register accessor: Enhanced Rx FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erfsr`] module"]
#[doc(alias = "ERFSR")]
pub type Erfsr = crate::Reg<erfsr::ErfsrSpec>;
#[doc = "Enhanced Rx FIFO Status Register"]
pub mod erfsr;
#[doc = "HR_TIME_STAMP (rw) register accessor: High Resolution Time Stamp\n\nYou can [`read`](crate::Reg::read) this register and get [`hr_time_stamp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr_time_stamp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hr_time_stamp`] module"]
#[doc(alias = "HR_TIME_STAMP")]
pub type HrTimeStamp = crate::Reg<hr_time_stamp::HrTimeStampSpec>;
#[doc = "High Resolution Time Stamp"]
pub mod hr_time_stamp;
#[doc = "ERFFEL (rw) register accessor: Enhanced Rx FIFO Filter Element\n\nYou can [`read`](crate::Reg::read) this register and get [`erffel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erffel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erffel`] module"]
#[doc(alias = "ERFFEL")]
pub type Erffel = crate::Reg<erffel::ErffelSpec>;
#[doc = "Enhanced Rx FIFO Filter Element"]
pub mod erffel;

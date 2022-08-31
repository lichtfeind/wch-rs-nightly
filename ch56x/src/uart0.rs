#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART0 modem control"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x01 - UART0 interrupt enable"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x02 - UART0 FIFO control"]
    pub fcr: crate::Reg<fcr::FCR_SPEC>,
    #[doc = "0x03 - UART0 line control"]
    pub lcr: crate::Reg<lcr::LCR_SPEC>,
    #[doc = "0x04 - UART0 interrupt identification"]
    pub iir: crate::Reg<iir::IIR_SPEC>,
    #[doc = "0x05 - UART0 line status"]
    pub lsr: crate::Reg<lsr::LSR_SPEC>,
    #[doc = "0x06 - UART0 modem status"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x08 - UART0 receiver buffer, receiving byte _ UART0 transmitter holding, transmittal byte"]
    pub rbr_thr: crate::Reg<rbr_thr::RBR_THR_SPEC>,
    _reserved8: [u8; 0x01],
    #[doc = "0x0a - UART0 receiver FIFO count"]
    pub rfc: crate::Reg<rfc::RFC_SPEC>,
    #[doc = "0x0b - UART0 transmitter FIFO count"]
    pub tfc: crate::Reg<tfc::TFC_SPEC>,
    #[doc = "0x0c - UART0 divisor latch"]
    pub dl: crate::Reg<dl::DL_SPEC>,
    #[doc = "0x0e - UART0 pre-divisor latch byte"]
    pub div: crate::Reg<div::DIV_SPEC>,
    #[doc = "0x0f - UART0 slave address"]
    pub adr: crate::Reg<adr::ADR_SPEC>,
}
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "UART0 modem control"]
pub mod mcr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "UART0 interrupt enable"]
pub mod ier;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "UART0 FIFO control"]
pub mod fcr;
#[doc = "LCR register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "UART0 line control"]
pub mod lcr;
#[doc = "IIR register accessor: an alias for `Reg<IIR_SPEC>`"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "UART0 interrupt identification"]
pub mod iir;
#[doc = "LSR register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "UART0 line status"]
pub mod lsr;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "UART0 modem status"]
pub mod msr;
#[doc = "RBR_THR register accessor: an alias for `Reg<RBR_THR_SPEC>`"]
pub type RBR_THR = crate::Reg<rbr_thr::RBR_THR_SPEC>;
#[doc = "UART0 receiver buffer, receiving byte _ UART0 transmitter holding, transmittal byte"]
pub mod rbr_thr;
#[doc = "RFC register accessor: an alias for `Reg<RFC_SPEC>`"]
pub type RFC = crate::Reg<rfc::RFC_SPEC>;
#[doc = "UART0 receiver FIFO count"]
pub mod rfc;
#[doc = "TFC register accessor: an alias for `Reg<TFC_SPEC>`"]
pub type TFC = crate::Reg<tfc::TFC_SPEC>;
#[doc = "UART0 transmitter FIFO count"]
pub mod tfc;
#[doc = "DL register accessor: an alias for `Reg<DL_SPEC>`"]
pub type DL = crate::Reg<dl::DL_SPEC>;
#[doc = "UART0 divisor latch"]
pub mod dl;
#[doc = "DIV register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "UART0 pre-divisor latch byte"]
pub mod div;
#[doc = "ADR register accessor: an alias for `Reg<ADR_SPEC>`"]
pub type ADR = crate::Reg<adr::ADR_SPEC>;
#[doc = "UART0 slave address"]
pub mod adr;

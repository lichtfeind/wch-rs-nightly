#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Status Register"]
    pub isr1: crate::Reg<isr1::ISR1_SPEC>,
    #[doc = "0x04 - Interrupt Status Register"]
    pub isr2: crate::Reg<isr2::ISR2_SPEC>,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - Interrupt Pending Register"]
    pub ipr1: crate::Reg<ipr1::IPR1_SPEC>,
    #[doc = "0x24 - Interrupt Pending Register"]
    pub ipr2: crate::Reg<ipr2::IPR2_SPEC>,
    _reserved4: [u8; 0x18],
    #[doc = "0x40 - Interrupt Priority Register"]
    pub ithresdr: crate::Reg<ithresdr::ITHRESDR_SPEC>,
    #[doc = "0x44 - Interrupt Fast Address Register"]
    pub fibaddrr: crate::Reg<fibaddrr::FIBADDRR_SPEC>,
    #[doc = "0x48 - Interrupt Config Register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x4c - Interrupt Global Register"]
    pub gisr: crate::Reg<gisr::GISR_SPEC>,
    _reserved8: [u8; 0x10],
    #[doc = "0x60 - Interrupt 0 address Register"]
    pub fifoaddrr0: crate::Reg<fifoaddrr0::FIFOADDRR0_SPEC>,
    #[doc = "0x64 - Interrupt 1 address Register"]
    pub fifoaddrr1: crate::Reg<fifoaddrr1::FIFOADDRR1_SPEC>,
    #[doc = "0x68 - Interrupt 2 address Register"]
    pub fifoaddrr2: crate::Reg<fifoaddrr2::FIFOADDRR2_SPEC>,
    #[doc = "0x6c - Interrupt 3 address Register"]
    pub fifoaddrr3: crate::Reg<fifoaddrr3::FIFOADDRR3_SPEC>,
    _reserved12: [u8; 0x90],
    #[doc = "0x100 - Interrupt Setting Register"]
    pub ienr1: crate::Reg<ienr1::IENR1_SPEC>,
    #[doc = "0x104 - Interrupt Setting Register"]
    pub ienr2: crate::Reg<ienr2::IENR2_SPEC>,
    _reserved14: [u8; 0x78],
    #[doc = "0x180 - Interrupt Clear Register"]
    pub irer1: crate::Reg<irer1::IRER1_SPEC>,
    #[doc = "0x184 - Interrupt Clear Register"]
    pub irer2: crate::Reg<irer2::IRER2_SPEC>,
    _reserved16: [u8; 0x78],
    #[doc = "0x200 - Interrupt Pending Register"]
    pub ipsr1: crate::Reg<ipsr1::IPSR1_SPEC>,
    #[doc = "0x204 - Interrupt Pending Register"]
    pub ipsr2: crate::Reg<ipsr2::IPSR2_SPEC>,
    _reserved18: [u8; 0x78],
    #[doc = "0x280 - Interrupt Pending Clear Register"]
    pub iprr1: crate::Reg<iprr1::IPRR1_SPEC>,
    #[doc = "0x284 - Interrupt Pending Clear Register"]
    pub iprr2: crate::Reg<iprr2::IPRR2_SPEC>,
    _reserved20: [u8; 0x78],
    #[doc = "0x300 - Interrupt ACTIVE Register"]
    pub iactr1: crate::Reg<iactr1::IACTR1_SPEC>,
    #[doc = "0x304 - Interrupt ACTIVE Register"]
    pub iactr2: crate::Reg<iactr2::IACTR2_SPEC>,
    _reserved22: [u8; 0xf8],
    #[doc = "0x400 - Interrupt Priority configuration Register"]
    pub iprior0: crate::Reg<iprior0::IPRIOR0_SPEC>,
    _reserved23: [u8; 0x1c],
    #[doc = "0x420 - Interrupt Priority configuration Register"]
    pub iprior1: crate::Reg<iprior1::IPRIOR1_SPEC>,
    _reserved24: [u8; 0x1c],
    #[doc = "0x440 - Interrupt Priority configuration Register"]
    pub iprior2: crate::Reg<iprior2::IPRIOR2_SPEC>,
    _reserved25: [u8; 0x1c],
    #[doc = "0x460 - Interrupt Priority configuration Register"]
    pub iprior3: crate::Reg<iprior3::IPRIOR3_SPEC>,
    _reserved26: [u8; 0x1c],
    #[doc = "0x480 - Interrupt Priority configuration Register"]
    pub iprior4: crate::Reg<iprior4::IPRIOR4_SPEC>,
    _reserved27: [u8; 0x1c],
    #[doc = "0x4a0 - Interrupt Priority configuration Register"]
    pub iprior5: crate::Reg<iprior5::IPRIOR5_SPEC>,
    _reserved28: [u8; 0x1c],
    #[doc = "0x4c0 - Interrupt Priority configuration Register"]
    pub iprior6: crate::Reg<iprior6::IPRIOR6_SPEC>,
    _reserved29: [u8; 0x1c],
    #[doc = "0x4e0 - Interrupt Priority configuration Register"]
    pub iprior7: crate::Reg<iprior7::IPRIOR7_SPEC>,
    _reserved30: [u8; 0x1c],
    #[doc = "0x500 - Interrupt Priority configuration Register"]
    pub iprior8: crate::Reg<iprior8::IPRIOR8_SPEC>,
    _reserved31: [u8; 0x1c],
    #[doc = "0x520 - Interrupt Priority configuration Register"]
    pub iprior9: crate::Reg<iprior9::IPRIOR9_SPEC>,
    _reserved32: [u8; 0x1c],
    #[doc = "0x540 - Interrupt Priority configuration Register"]
    pub iprior10: crate::Reg<iprior10::IPRIOR10_SPEC>,
    _reserved33: [u8; 0x1c],
    #[doc = "0x560 - Interrupt Priority configuration Register"]
    pub iprior11: crate::Reg<iprior11::IPRIOR11_SPEC>,
    _reserved34: [u8; 0x1c],
    #[doc = "0x580 - Interrupt Priority configuration Register"]
    pub iprior12: crate::Reg<iprior12::IPRIOR12_SPEC>,
    _reserved35: [u8; 0x1c],
    #[doc = "0x5a0 - Interrupt Priority configuration Register"]
    pub iprior13: crate::Reg<iprior13::IPRIOR13_SPEC>,
    _reserved36: [u8; 0x1c],
    #[doc = "0x5c0 - Interrupt Priority configuration Register"]
    pub iprior14: crate::Reg<iprior14::IPRIOR14_SPEC>,
    _reserved37: [u8; 0x1c],
    #[doc = "0x5e0 - Interrupt Priority configuration Register"]
    pub iprior15: crate::Reg<iprior15::IPRIOR15_SPEC>,
    _reserved38: [u8; 0x1c],
    #[doc = "0x600 - Interrupt Priority configuration Register"]
    pub iprior16: crate::Reg<iprior16::IPRIOR16_SPEC>,
    _reserved39: [u8; 0x1c],
    #[doc = "0x620 - Interrupt Priority configuration Register"]
    pub iprior17: crate::Reg<iprior17::IPRIOR17_SPEC>,
    _reserved40: [u8; 0x1c],
    #[doc = "0x640 - Interrupt Priority configuration Register"]
    pub iprior18: crate::Reg<iprior18::IPRIOR18_SPEC>,
    _reserved41: [u8; 0x1c],
    #[doc = "0x660 - Interrupt Priority configuration Register"]
    pub iprior19: crate::Reg<iprior19::IPRIOR19_SPEC>,
    _reserved42: [u8; 0x1c],
    #[doc = "0x680 - Interrupt Priority configuration Register"]
    pub iprior20: crate::Reg<iprior20::IPRIOR20_SPEC>,
    _reserved43: [u8; 0x1c],
    #[doc = "0x6a0 - Interrupt Priority configuration Register"]
    pub iprior21: crate::Reg<iprior21::IPRIOR21_SPEC>,
    _reserved44: [u8; 0x1c],
    #[doc = "0x6c0 - Interrupt Priority configuration Register"]
    pub iprior22: crate::Reg<iprior22::IPRIOR22_SPEC>,
    _reserved45: [u8; 0x1c],
    #[doc = "0x6e0 - Interrupt Priority configuration Register"]
    pub iprior23: crate::Reg<iprior23::IPRIOR23_SPEC>,
    _reserved46: [u8; 0x1c],
    #[doc = "0x700 - Interrupt Priority configuration Register"]
    pub iprior24: crate::Reg<iprior24::IPRIOR24_SPEC>,
    _reserved47: [u8; 0x1c],
    #[doc = "0x720 - Interrupt Priority configuration Register"]
    pub iprior25: crate::Reg<iprior25::IPRIOR25_SPEC>,
    _reserved48: [u8; 0x1c],
    #[doc = "0x740 - Interrupt Priority configuration Register"]
    pub iprior26: crate::Reg<iprior26::IPRIOR26_SPEC>,
    _reserved49: [u8; 0x1c],
    #[doc = "0x760 - Interrupt Priority configuration Register"]
    pub iprior27: crate::Reg<iprior27::IPRIOR27_SPEC>,
    _reserved50: [u8; 0x1c],
    #[doc = "0x780 - Interrupt Priority configuration Register"]
    pub iprior28: crate::Reg<iprior28::IPRIOR28_SPEC>,
    _reserved51: [u8; 0x1c],
    #[doc = "0x7a0 - Interrupt Priority configuration Register"]
    pub iprior29: crate::Reg<iprior29::IPRIOR29_SPEC>,
    _reserved52: [u8; 0x1c],
    #[doc = "0x7c0 - Interrupt Priority configuration Register"]
    pub iprior30: crate::Reg<iprior30::IPRIOR30_SPEC>,
    _reserved53: [u8; 0x1c],
    #[doc = "0x7e0 - Interrupt Priority configuration Register"]
    pub iprior31: crate::Reg<iprior31::IPRIOR31_SPEC>,
    _reserved54: [u8; 0x1c],
    #[doc = "0x800 - Interrupt Priority configuration Register"]
    pub iprior32: crate::Reg<iprior32::IPRIOR32_SPEC>,
    _reserved55: [u8; 0x1c],
    #[doc = "0x820 - Interrupt Priority configuration Register"]
    pub iprior33: crate::Reg<iprior33::IPRIOR33_SPEC>,
    _reserved56: [u8; 0x1c],
    #[doc = "0x840 - Interrupt Priority configuration Register"]
    pub iprior34: crate::Reg<iprior34::IPRIOR34_SPEC>,
    _reserved57: [u8; 0x1c],
    #[doc = "0x860 - Interrupt Priority configuration Register"]
    pub iprior35: crate::Reg<iprior35::IPRIOR35_SPEC>,
    _reserved58: [u8; 0x1c],
    #[doc = "0x880 - Interrupt Priority configuration Register"]
    pub iprior36: crate::Reg<iprior36::IPRIOR36_SPEC>,
    _reserved59: [u8; 0x1c],
    #[doc = "0x8a0 - Interrupt Priority configuration Register"]
    pub iprior37: crate::Reg<iprior37::IPRIOR37_SPEC>,
    _reserved60: [u8; 0x1c],
    #[doc = "0x8c0 - Interrupt Priority configuration Register"]
    pub iprior38: crate::Reg<iprior38::IPRIOR38_SPEC>,
    _reserved61: [u8; 0x1c],
    #[doc = "0x8e0 - Interrupt Priority configuration Register"]
    pub iprior39: crate::Reg<iprior39::IPRIOR39_SPEC>,
    _reserved62: [u8; 0x1c],
    #[doc = "0x900 - Interrupt Priority configuration Register"]
    pub iprior40: crate::Reg<iprior40::IPRIOR40_SPEC>,
    _reserved63: [u8; 0x1c],
    #[doc = "0x920 - Interrupt Priority configuration Register"]
    pub iprior41: crate::Reg<iprior41::IPRIOR41_SPEC>,
    _reserved64: [u8; 0x1c],
    #[doc = "0x940 - Interrupt Priority configuration Register"]
    pub iprior42: crate::Reg<iprior42::IPRIOR42_SPEC>,
    _reserved65: [u8; 0x1c],
    #[doc = "0x960 - Interrupt Priority configuration Register"]
    pub iprior43: crate::Reg<iprior43::IPRIOR43_SPEC>,
    _reserved66: [u8; 0x1c],
    #[doc = "0x980 - Interrupt Priority configuration Register"]
    pub iprior44: crate::Reg<iprior44::IPRIOR44_SPEC>,
    _reserved67: [u8; 0x1c],
    #[doc = "0x9a0 - Interrupt Priority configuration Register"]
    pub iprior45: crate::Reg<iprior45::IPRIOR45_SPEC>,
    _reserved68: [u8; 0x1c],
    #[doc = "0x9c0 - Interrupt Priority configuration Register"]
    pub iprior46: crate::Reg<iprior46::IPRIOR46_SPEC>,
    _reserved69: [u8; 0x1c],
    #[doc = "0x9e0 - Interrupt Priority configuration Register"]
    pub iprior47: crate::Reg<iprior47::IPRIOR47_SPEC>,
    _reserved70: [u8; 0x1c],
    #[doc = "0xa00 - Interrupt Priority configuration Register"]
    pub iprior48: crate::Reg<iprior48::IPRIOR48_SPEC>,
    _reserved71: [u8; 0x1c],
    #[doc = "0xa20 - Interrupt Priority configuration Register"]
    pub iprior49: crate::Reg<iprior49::IPRIOR49_SPEC>,
    _reserved72: [u8; 0x1c],
    #[doc = "0xa40 - Interrupt Priority configuration Register"]
    pub iprior50: crate::Reg<iprior50::IPRIOR50_SPEC>,
    _reserved73: [u8; 0x1c],
    #[doc = "0xa60 - Interrupt Priority configuration Register"]
    pub iprior51: crate::Reg<iprior51::IPRIOR51_SPEC>,
    _reserved74: [u8; 0x1c],
    #[doc = "0xa80 - Interrupt Priority configuration Register"]
    pub iprior52: crate::Reg<iprior52::IPRIOR52_SPEC>,
    _reserved75: [u8; 0x1c],
    #[doc = "0xaa0 - Interrupt Priority configuration Register"]
    pub iprior53: crate::Reg<iprior53::IPRIOR53_SPEC>,
    _reserved76: [u8; 0x2c],
    #[doc = "0xad0 - Interrupt Priority configuration Register"]
    pub iprior54: crate::Reg<iprior54::IPRIOR54_SPEC>,
    _reserved77: [u8; 0x0c],
    #[doc = "0xae0 - Interrupt Priority configuration Register"]
    pub iprior55: crate::Reg<iprior55::IPRIOR55_SPEC>,
    _reserved78: [u8; 0x1c],
    #[doc = "0xb00 - Interrupt Priority configuration Register"]
    pub iprior56: crate::Reg<iprior56::IPRIOR56_SPEC>,
    _reserved79: [u8; 0x1c],
    #[doc = "0xb20 - Interrupt Priority configuration Register"]
    pub iprior57: crate::Reg<iprior57::IPRIOR57_SPEC>,
    _reserved80: [u8; 0x1c],
    #[doc = "0xb40 - Interrupt Priority configuration Register"]
    pub iprior58: crate::Reg<iprior58::IPRIOR58_SPEC>,
    _reserved81: [u8; 0x1c],
    #[doc = "0xb60 - Interrupt Priority configuration Register"]
    pub iprior59: crate::Reg<iprior59::IPRIOR59_SPEC>,
    _reserved82: [u8; 0x1c],
    #[doc = "0xb80 - Interrupt Priority configuration Register"]
    pub iprior60: crate::Reg<iprior60::IPRIOR60_SPEC>,
    _reserved83: [u8; 0x1c],
    #[doc = "0xba0 - Interrupt Priority configuration Register"]
    pub iprior61: crate::Reg<iprior61::IPRIOR61_SPEC>,
    _reserved84: [u8; 0x3c],
    #[doc = "0xbe0 - Interrupt Priority configuration Register"]
    pub iprior62: crate::Reg<iprior62::IPRIOR62_SPEC>,
    _reserved85: [u8; 0x1c],
    #[doc = "0xc00 - Interrupt Priority configuration Register"]
    pub iprior63: crate::Reg<iprior63::IPRIOR63_SPEC>,
    _reserved86: [u8; 0x010c],
    #[doc = "0xd10 - System Control Register"]
    pub sctlr: crate::Reg<sctlr::SCTLR_SPEC>,
}
#[doc = "ISR1 register accessor: an alias for `Reg<ISR1_SPEC>`"]
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr1;
#[doc = "ISR2 register accessor: an alias for `Reg<ISR2_SPEC>`"]
pub type ISR2 = crate::Reg<isr2::ISR2_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr2;
#[doc = "IPR1 register accessor: an alias for `Reg<IPR1_SPEC>`"]
pub type IPR1 = crate::Reg<ipr1::IPR1_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipr1;
#[doc = "IPR2 register accessor: an alias for `Reg<IPR2_SPEC>`"]
pub type IPR2 = crate::Reg<ipr2::IPR2_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipr2;
#[doc = "ITHRESDR register accessor: an alias for `Reg<ITHRESDR_SPEC>`"]
pub type ITHRESDR = crate::Reg<ithresdr::ITHRESDR_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod ithresdr;
#[doc = "FIBADDRR register accessor: an alias for `Reg<FIBADDRR_SPEC>`"]
pub type FIBADDRR = crate::Reg<fibaddrr::FIBADDRR_SPEC>;
#[doc = "Interrupt Fast Address Register"]
pub mod fibaddrr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Interrupt Config Register"]
pub mod cfgr;
#[doc = "GISR register accessor: an alias for `Reg<GISR_SPEC>`"]
pub type GISR = crate::Reg<gisr::GISR_SPEC>;
#[doc = "Interrupt Global Register"]
pub mod gisr;
#[doc = "FIFOADDRR0 register accessor: an alias for `Reg<FIFOADDRR0_SPEC>`"]
pub type FIFOADDRR0 = crate::Reg<fifoaddrr0::FIFOADDRR0_SPEC>;
#[doc = "Interrupt 0 address Register"]
pub mod fifoaddrr0;
#[doc = "FIFOADDRR1 register accessor: an alias for `Reg<FIFOADDRR1_SPEC>`"]
pub type FIFOADDRR1 = crate::Reg<fifoaddrr1::FIFOADDRR1_SPEC>;
#[doc = "Interrupt 1 address Register"]
pub mod fifoaddrr1;
#[doc = "FIFOADDRR2 register accessor: an alias for `Reg<FIFOADDRR2_SPEC>`"]
pub type FIFOADDRR2 = crate::Reg<fifoaddrr2::FIFOADDRR2_SPEC>;
#[doc = "Interrupt 2 address Register"]
pub mod fifoaddrr2;
#[doc = "FIFOADDRR3 register accessor: an alias for `Reg<FIFOADDRR3_SPEC>`"]
pub type FIFOADDRR3 = crate::Reg<fifoaddrr3::FIFOADDRR3_SPEC>;
#[doc = "Interrupt 3 address Register"]
pub mod fifoaddrr3;
#[doc = "IENR1 register accessor: an alias for `Reg<IENR1_SPEC>`"]
pub type IENR1 = crate::Reg<ienr1::IENR1_SPEC>;
#[doc = "Interrupt Setting Register"]
pub mod ienr1;
#[doc = "IENR2 register accessor: an alias for `Reg<IENR2_SPEC>`"]
pub type IENR2 = crate::Reg<ienr2::IENR2_SPEC>;
#[doc = "Interrupt Setting Register"]
pub mod ienr2;
#[doc = "IRER1 register accessor: an alias for `Reg<IRER1_SPEC>`"]
pub type IRER1 = crate::Reg<irer1::IRER1_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod irer1;
#[doc = "IRER2 register accessor: an alias for `Reg<IRER2_SPEC>`"]
pub type IRER2 = crate::Reg<irer2::IRER2_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod irer2;
#[doc = "IPSR1 register accessor: an alias for `Reg<IPSR1_SPEC>`"]
pub type IPSR1 = crate::Reg<ipsr1::IPSR1_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipsr1;
#[doc = "IPSR2 register accessor: an alias for `Reg<IPSR2_SPEC>`"]
pub type IPSR2 = crate::Reg<ipsr2::IPSR2_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ipsr2;
#[doc = "IPRR1 register accessor: an alias for `Reg<IPRR1_SPEC>`"]
pub type IPRR1 = crate::Reg<iprr1::IPRR1_SPEC>;
#[doc = "Interrupt Pending Clear Register"]
pub mod iprr1;
#[doc = "IPRR2 register accessor: an alias for `Reg<IPRR2_SPEC>`"]
pub type IPRR2 = crate::Reg<iprr2::IPRR2_SPEC>;
#[doc = "Interrupt Pending Clear Register"]
pub mod iprr2;
#[doc = "IACTR1 register accessor: an alias for `Reg<IACTR1_SPEC>`"]
pub type IACTR1 = crate::Reg<iactr1::IACTR1_SPEC>;
#[doc = "Interrupt ACTIVE Register"]
pub mod iactr1;
#[doc = "IACTR2 register accessor: an alias for `Reg<IACTR2_SPEC>`"]
pub type IACTR2 = crate::Reg<iactr2::IACTR2_SPEC>;
#[doc = "Interrupt ACTIVE Register"]
pub mod iactr2;
#[doc = "IPRIOR0 register accessor: an alias for `Reg<IPRIOR0_SPEC>`"]
pub type IPRIOR0 = crate::Reg<iprior0::IPRIOR0_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior0;
#[doc = "IPRIOR1 register accessor: an alias for `Reg<IPRIOR1_SPEC>`"]
pub type IPRIOR1 = crate::Reg<iprior1::IPRIOR1_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior1;
#[doc = "IPRIOR2 register accessor: an alias for `Reg<IPRIOR2_SPEC>`"]
pub type IPRIOR2 = crate::Reg<iprior2::IPRIOR2_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior2;
#[doc = "IPRIOR3 register accessor: an alias for `Reg<IPRIOR3_SPEC>`"]
pub type IPRIOR3 = crate::Reg<iprior3::IPRIOR3_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior3;
#[doc = "IPRIOR4 register accessor: an alias for `Reg<IPRIOR4_SPEC>`"]
pub type IPRIOR4 = crate::Reg<iprior4::IPRIOR4_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior4;
#[doc = "IPRIOR5 register accessor: an alias for `Reg<IPRIOR5_SPEC>`"]
pub type IPRIOR5 = crate::Reg<iprior5::IPRIOR5_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior5;
#[doc = "IPRIOR6 register accessor: an alias for `Reg<IPRIOR6_SPEC>`"]
pub type IPRIOR6 = crate::Reg<iprior6::IPRIOR6_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior6;
#[doc = "IPRIOR7 register accessor: an alias for `Reg<IPRIOR7_SPEC>`"]
pub type IPRIOR7 = crate::Reg<iprior7::IPRIOR7_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior7;
#[doc = "IPRIOR8 register accessor: an alias for `Reg<IPRIOR8_SPEC>`"]
pub type IPRIOR8 = crate::Reg<iprior8::IPRIOR8_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior8;
#[doc = "IPRIOR9 register accessor: an alias for `Reg<IPRIOR9_SPEC>`"]
pub type IPRIOR9 = crate::Reg<iprior9::IPRIOR9_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior9;
#[doc = "IPRIOR10 register accessor: an alias for `Reg<IPRIOR10_SPEC>`"]
pub type IPRIOR10 = crate::Reg<iprior10::IPRIOR10_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior10;
#[doc = "IPRIOR11 register accessor: an alias for `Reg<IPRIOR11_SPEC>`"]
pub type IPRIOR11 = crate::Reg<iprior11::IPRIOR11_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior11;
#[doc = "IPRIOR12 register accessor: an alias for `Reg<IPRIOR12_SPEC>`"]
pub type IPRIOR12 = crate::Reg<iprior12::IPRIOR12_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior12;
#[doc = "IPRIOR13 register accessor: an alias for `Reg<IPRIOR13_SPEC>`"]
pub type IPRIOR13 = crate::Reg<iprior13::IPRIOR13_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior13;
#[doc = "IPRIOR14 register accessor: an alias for `Reg<IPRIOR14_SPEC>`"]
pub type IPRIOR14 = crate::Reg<iprior14::IPRIOR14_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior14;
#[doc = "IPRIOR15 register accessor: an alias for `Reg<IPRIOR15_SPEC>`"]
pub type IPRIOR15 = crate::Reg<iprior15::IPRIOR15_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior15;
#[doc = "IPRIOR16 register accessor: an alias for `Reg<IPRIOR16_SPEC>`"]
pub type IPRIOR16 = crate::Reg<iprior16::IPRIOR16_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior16;
#[doc = "IPRIOR17 register accessor: an alias for `Reg<IPRIOR17_SPEC>`"]
pub type IPRIOR17 = crate::Reg<iprior17::IPRIOR17_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior17;
#[doc = "IPRIOR18 register accessor: an alias for `Reg<IPRIOR18_SPEC>`"]
pub type IPRIOR18 = crate::Reg<iprior18::IPRIOR18_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior18;
#[doc = "IPRIOR19 register accessor: an alias for `Reg<IPRIOR19_SPEC>`"]
pub type IPRIOR19 = crate::Reg<iprior19::IPRIOR19_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior19;
#[doc = "IPRIOR20 register accessor: an alias for `Reg<IPRIOR20_SPEC>`"]
pub type IPRIOR20 = crate::Reg<iprior20::IPRIOR20_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior20;
#[doc = "IPRIOR21 register accessor: an alias for `Reg<IPRIOR21_SPEC>`"]
pub type IPRIOR21 = crate::Reg<iprior21::IPRIOR21_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior21;
#[doc = "IPRIOR22 register accessor: an alias for `Reg<IPRIOR22_SPEC>`"]
pub type IPRIOR22 = crate::Reg<iprior22::IPRIOR22_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior22;
#[doc = "IPRIOR23 register accessor: an alias for `Reg<IPRIOR23_SPEC>`"]
pub type IPRIOR23 = crate::Reg<iprior23::IPRIOR23_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior23;
#[doc = "IPRIOR24 register accessor: an alias for `Reg<IPRIOR24_SPEC>`"]
pub type IPRIOR24 = crate::Reg<iprior24::IPRIOR24_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior24;
#[doc = "IPRIOR25 register accessor: an alias for `Reg<IPRIOR25_SPEC>`"]
pub type IPRIOR25 = crate::Reg<iprior25::IPRIOR25_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior25;
#[doc = "IPRIOR26 register accessor: an alias for `Reg<IPRIOR26_SPEC>`"]
pub type IPRIOR26 = crate::Reg<iprior26::IPRIOR26_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior26;
#[doc = "IPRIOR27 register accessor: an alias for `Reg<IPRIOR27_SPEC>`"]
pub type IPRIOR27 = crate::Reg<iprior27::IPRIOR27_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior27;
#[doc = "IPRIOR28 register accessor: an alias for `Reg<IPRIOR28_SPEC>`"]
pub type IPRIOR28 = crate::Reg<iprior28::IPRIOR28_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior28;
#[doc = "IPRIOR29 register accessor: an alias for `Reg<IPRIOR29_SPEC>`"]
pub type IPRIOR29 = crate::Reg<iprior29::IPRIOR29_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior29;
#[doc = "IPRIOR30 register accessor: an alias for `Reg<IPRIOR30_SPEC>`"]
pub type IPRIOR30 = crate::Reg<iprior30::IPRIOR30_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior30;
#[doc = "IPRIOR31 register accessor: an alias for `Reg<IPRIOR31_SPEC>`"]
pub type IPRIOR31 = crate::Reg<iprior31::IPRIOR31_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior31;
#[doc = "IPRIOR32 register accessor: an alias for `Reg<IPRIOR32_SPEC>`"]
pub type IPRIOR32 = crate::Reg<iprior32::IPRIOR32_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior32;
#[doc = "IPRIOR33 register accessor: an alias for `Reg<IPRIOR33_SPEC>`"]
pub type IPRIOR33 = crate::Reg<iprior33::IPRIOR33_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior33;
#[doc = "IPRIOR34 register accessor: an alias for `Reg<IPRIOR34_SPEC>`"]
pub type IPRIOR34 = crate::Reg<iprior34::IPRIOR34_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior34;
#[doc = "IPRIOR35 register accessor: an alias for `Reg<IPRIOR35_SPEC>`"]
pub type IPRIOR35 = crate::Reg<iprior35::IPRIOR35_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior35;
#[doc = "IPRIOR36 register accessor: an alias for `Reg<IPRIOR36_SPEC>`"]
pub type IPRIOR36 = crate::Reg<iprior36::IPRIOR36_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior36;
#[doc = "IPRIOR37 register accessor: an alias for `Reg<IPRIOR37_SPEC>`"]
pub type IPRIOR37 = crate::Reg<iprior37::IPRIOR37_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior37;
#[doc = "IPRIOR38 register accessor: an alias for `Reg<IPRIOR38_SPEC>`"]
pub type IPRIOR38 = crate::Reg<iprior38::IPRIOR38_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior38;
#[doc = "IPRIOR39 register accessor: an alias for `Reg<IPRIOR39_SPEC>`"]
pub type IPRIOR39 = crate::Reg<iprior39::IPRIOR39_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior39;
#[doc = "IPRIOR40 register accessor: an alias for `Reg<IPRIOR40_SPEC>`"]
pub type IPRIOR40 = crate::Reg<iprior40::IPRIOR40_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior40;
#[doc = "IPRIOR41 register accessor: an alias for `Reg<IPRIOR41_SPEC>`"]
pub type IPRIOR41 = crate::Reg<iprior41::IPRIOR41_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior41;
#[doc = "IPRIOR42 register accessor: an alias for `Reg<IPRIOR42_SPEC>`"]
pub type IPRIOR42 = crate::Reg<iprior42::IPRIOR42_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior42;
#[doc = "IPRIOR43 register accessor: an alias for `Reg<IPRIOR43_SPEC>`"]
pub type IPRIOR43 = crate::Reg<iprior43::IPRIOR43_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior43;
#[doc = "IPRIOR44 register accessor: an alias for `Reg<IPRIOR44_SPEC>`"]
pub type IPRIOR44 = crate::Reg<iprior44::IPRIOR44_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior44;
#[doc = "IPRIOR45 register accessor: an alias for `Reg<IPRIOR45_SPEC>`"]
pub type IPRIOR45 = crate::Reg<iprior45::IPRIOR45_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior45;
#[doc = "IPRIOR46 register accessor: an alias for `Reg<IPRIOR46_SPEC>`"]
pub type IPRIOR46 = crate::Reg<iprior46::IPRIOR46_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior46;
#[doc = "IPRIOR47 register accessor: an alias for `Reg<IPRIOR47_SPEC>`"]
pub type IPRIOR47 = crate::Reg<iprior47::IPRIOR47_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior47;
#[doc = "IPRIOR48 register accessor: an alias for `Reg<IPRIOR48_SPEC>`"]
pub type IPRIOR48 = crate::Reg<iprior48::IPRIOR48_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior48;
#[doc = "IPRIOR49 register accessor: an alias for `Reg<IPRIOR49_SPEC>`"]
pub type IPRIOR49 = crate::Reg<iprior49::IPRIOR49_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior49;
#[doc = "IPRIOR50 register accessor: an alias for `Reg<IPRIOR50_SPEC>`"]
pub type IPRIOR50 = crate::Reg<iprior50::IPRIOR50_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior50;
#[doc = "IPRIOR51 register accessor: an alias for `Reg<IPRIOR51_SPEC>`"]
pub type IPRIOR51 = crate::Reg<iprior51::IPRIOR51_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior51;
#[doc = "IPRIOR52 register accessor: an alias for `Reg<IPRIOR52_SPEC>`"]
pub type IPRIOR52 = crate::Reg<iprior52::IPRIOR52_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior52;
#[doc = "IPRIOR53 register accessor: an alias for `Reg<IPRIOR53_SPEC>`"]
pub type IPRIOR53 = crate::Reg<iprior53::IPRIOR53_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior53;
#[doc = "IPRIOR54 register accessor: an alias for `Reg<IPRIOR54_SPEC>`"]
pub type IPRIOR54 = crate::Reg<iprior54::IPRIOR54_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior54;
#[doc = "IPRIOR55 register accessor: an alias for `Reg<IPRIOR55_SPEC>`"]
pub type IPRIOR55 = crate::Reg<iprior55::IPRIOR55_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior55;
#[doc = "IPRIOR56 register accessor: an alias for `Reg<IPRIOR56_SPEC>`"]
pub type IPRIOR56 = crate::Reg<iprior56::IPRIOR56_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior56;
#[doc = "IPRIOR57 register accessor: an alias for `Reg<IPRIOR57_SPEC>`"]
pub type IPRIOR57 = crate::Reg<iprior57::IPRIOR57_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior57;
#[doc = "IPRIOR58 register accessor: an alias for `Reg<IPRIOR58_SPEC>`"]
pub type IPRIOR58 = crate::Reg<iprior58::IPRIOR58_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior58;
#[doc = "IPRIOR59 register accessor: an alias for `Reg<IPRIOR59_SPEC>`"]
pub type IPRIOR59 = crate::Reg<iprior59::IPRIOR59_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior59;
#[doc = "IPRIOR60 register accessor: an alias for `Reg<IPRIOR60_SPEC>`"]
pub type IPRIOR60 = crate::Reg<iprior60::IPRIOR60_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior60;
#[doc = "IPRIOR61 register accessor: an alias for `Reg<IPRIOR61_SPEC>`"]
pub type IPRIOR61 = crate::Reg<iprior61::IPRIOR61_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior61;
#[doc = "IPRIOR62 register accessor: an alias for `Reg<IPRIOR62_SPEC>`"]
pub type IPRIOR62 = crate::Reg<iprior62::IPRIOR62_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior62;
#[doc = "IPRIOR63 register accessor: an alias for `Reg<IPRIOR63_SPEC>`"]
pub type IPRIOR63 = crate::Reg<iprior63::IPRIOR63_SPEC>;
#[doc = "Interrupt Priority configuration Register"]
pub mod iprior63;
#[doc = "SCTLR register accessor: an alias for `Reg<SCTLR_SPEC>`"]
pub type SCTLR = crate::Reg<sctlr::SCTLR_SPEC>;
#[doc = "System Control Register"]
pub mod sctlr;

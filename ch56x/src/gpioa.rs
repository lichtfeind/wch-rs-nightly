#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_pd: [u8; 0x24],
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - direction setting register"]
    #[inline(always)]
    pub fn dir(&self) -> &crate::Reg<dir::DIR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<dir::DIR_SPEC>)
        }
    }
    #[doc = "0x04..0x0c - data input register"]
    #[inline(always)]
    pub fn pin(&self) -> &crate::Reg<pin::PIN_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<pin::PIN_SPEC>)
        }
    }
    #[doc = "0x08..0x10 - data output register"]
    #[inline(always)]
    pub fn out(&self) -> &crate::Reg<out::OUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const crate::Reg<out::OUT_SPEC>)
        }
    }
    #[doc = "0x0c..0x14 - data reset register"]
    #[inline(always)]
    pub fn clr(&self) -> &crate::Reg<clr::CLR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<clr::CLR_SPEC>)
        }
    }
    #[doc = "0x10..0x18 - pull-up enable register"]
    #[inline(always)]
    pub fn pu(&self) -> &crate::Reg<pu::PU_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize) as *const crate::Reg<pu::PU_SPEC>)
        }
    }
    #[doc = "0x14..0x1c - pull-down enable register"]
    #[inline(always)]
    pub fn pd(&self) -> &crate::Reg<pd::PD_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize) as *const crate::Reg<pd::PD_SPEC>)
        }
    }
    #[doc = "0x18..0x20 - drive capability configuration register"]
    #[inline(always)]
    pub fn drv(&self) -> &crate::Reg<drv::DRV_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<drv::DRV_SPEC>)
        }
    }
    #[doc = "0x1c..0x24 - low slope output and Schmitt input register"]
    #[inline(always)]
    pub fn smt(&self) -> &crate::Reg<smt::SMT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<smt::SMT_SPEC>)
        }
    }
}
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "direction setting register"]
pub mod dir;
#[doc = "PIN register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "data input register"]
pub mod pin;
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "data output register"]
pub mod out;
#[doc = "CLR register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "data reset register"]
pub mod clr;
#[doc = "PU register accessor: an alias for `Reg<PU_SPEC>`"]
pub type PU = crate::Reg<pu::PU_SPEC>;
#[doc = "pull-up enable register"]
pub mod pu;
#[doc = "PD register accessor: an alias for `Reg<PD_SPEC>`"]
pub type PD = crate::Reg<pd::PD_SPEC>;
#[doc = "pull-down enable register"]
pub mod pd;
#[doc = "DRV register accessor: an alias for `Reg<DRV_SPEC>`"]
pub type DRV = crate::Reg<drv::DRV_SPEC>;
#[doc = "drive capability configuration register"]
pub mod drv;
#[doc = "SMT register accessor: an alias for `Reg<SMT_SPEC>`"]
pub type SMT = crate::Reg<smt::SMT_SPEC>;
#[doc = "low slope output and Schmitt input register"]
pub mod smt;

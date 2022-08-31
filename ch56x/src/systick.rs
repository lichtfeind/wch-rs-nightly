#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Systick counter control register"]
    pub stk_ctlr: crate::Reg<stk_ctlr::STK_CTLR_SPEC>,
    #[doc = "0x04 - Systick counter low register"]
    pub stk_cntl: crate::Reg<stk_cntl::STK_CNTL_SPEC>,
    #[doc = "0x08 - Systick counter high register"]
    pub stk_cnth: crate::Reg<stk_cnth::STK_CNTH_SPEC>,
    #[doc = "0x0c - Systick compare low register"]
    pub stk_cmplr: crate::Reg<stk_cmplr::STK_CMPLR_SPEC>,
    #[doc = "0x10 - Systick compare high register"]
    pub stk_cmphr: crate::Reg<stk_cmphr::STK_CMPHR_SPEC>,
    #[doc = "0x14 - Systick counter flag"]
    pub stk_cntfg: crate::Reg<stk_cntfg::STK_CNTFG_SPEC>,
}
#[doc = "STK_CTLR register accessor: an alias for `Reg<STK_CTLR_SPEC>`"]
pub type STK_CTLR = crate::Reg<stk_ctlr::STK_CTLR_SPEC>;
#[doc = "Systick counter control register"]
pub mod stk_ctlr;
#[doc = "STK_CNTL register accessor: an alias for `Reg<STK_CNTL_SPEC>`"]
pub type STK_CNTL = crate::Reg<stk_cntl::STK_CNTL_SPEC>;
#[doc = "Systick counter low register"]
pub mod stk_cntl;
#[doc = "STK_CNTH register accessor: an alias for `Reg<STK_CNTH_SPEC>`"]
pub type STK_CNTH = crate::Reg<stk_cnth::STK_CNTH_SPEC>;
#[doc = "Systick counter high register"]
pub mod stk_cnth;
#[doc = "STK_CMPLR register accessor: an alias for `Reg<STK_CMPLR_SPEC>`"]
pub type STK_CMPLR = crate::Reg<stk_cmplr::STK_CMPLR_SPEC>;
#[doc = "Systick compare low register"]
pub mod stk_cmplr;
#[doc = "STK_CMPHR register accessor: an alias for `Reg<STK_CMPHR_SPEC>`"]
pub type STK_CMPHR = crate::Reg<stk_cmphr::STK_CMPHR_SPEC>;
#[doc = "Systick compare high register"]
pub mod stk_cmphr;
#[doc = "STK_CNTFG register accessor: an alias for `Reg<STK_CNTFG_SPEC>`"]
pub type STK_CNTFG = crate::Reg<stk_cntfg::STK_CNTFG_SPEC>;
#[doc = "Systick counter flag"]
pub mod stk_cntfg;

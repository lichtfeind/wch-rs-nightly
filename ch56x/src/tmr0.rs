#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TMR0 mode control"]
    pub ctrl_mod: crate::Reg<ctrl_mod::CTRL_MOD_SPEC>,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - TMR0 interrupt enable"]
    pub inter_en: crate::Reg<inter_en::INTER_EN_SPEC>,
    _reserved2: [u8; 0x03],
    #[doc = "0x06 - TMR0 interrupt flag"]
    pub int_flag: crate::Reg<int_flag::INT_FLAG_SPEC>,
    #[doc = "0x07 - TMR0 FIFO count status"]
    pub fifo_count: crate::Reg<fifo_count::FIFO_COUNT_SPEC>,
    #[doc = "0x08 - TMR0 current count"]
    pub count: crate::Reg<count::COUNT_SPEC>,
    #[doc = "0x0c - TMR0 end count value, only low 26 bit"]
    pub cnt_end: crate::Reg<cnt_end::CNT_END_SPEC>,
    #[doc = "0x10 - TMR0 FIFO register, only low 26 bit"]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
}
#[doc = "CTRL_MOD register accessor: an alias for `Reg<CTRL_MOD_SPEC>`"]
pub type CTRL_MOD = crate::Reg<ctrl_mod::CTRL_MOD_SPEC>;
#[doc = "TMR0 mode control"]
pub mod ctrl_mod;
#[doc = "INTER_EN register accessor: an alias for `Reg<INTER_EN_SPEC>`"]
pub type INTER_EN = crate::Reg<inter_en::INTER_EN_SPEC>;
#[doc = "TMR0 interrupt enable"]
pub mod inter_en;
#[doc = "INT_FLAG register accessor: an alias for `Reg<INT_FLAG_SPEC>`"]
pub type INT_FLAG = crate::Reg<int_flag::INT_FLAG_SPEC>;
#[doc = "TMR0 interrupt flag"]
pub mod int_flag;
#[doc = "FIFO_COUNT register accessor: an alias for `Reg<FIFO_COUNT_SPEC>`"]
pub type FIFO_COUNT = crate::Reg<fifo_count::FIFO_COUNT_SPEC>;
#[doc = "TMR0 FIFO count status"]
pub mod fifo_count;
#[doc = "COUNT register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "TMR0 current count"]
pub mod count;
#[doc = "CNT_END register accessor: an alias for `Reg<CNT_END_SPEC>`"]
pub type CNT_END = crate::Reg<cnt_end::CNT_END_SPEC>;
#[doc = "TMR0 end count value, only low 26 bit"]
pub mod cnt_end;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "TMR0 FIFO register, only low 26 bit"]
pub mod fifo;

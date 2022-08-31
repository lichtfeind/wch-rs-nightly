#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI0 mode control"]
    pub spi0_ctrl_mod: crate::Reg<spi0_ctrl_mod::SPI0_CTRL_MOD_SPEC>,
    #[doc = "0x01 - SPI0 configuration control"]
    pub spi0_ctrl_cfg: crate::Reg<spi0_ctrl_cfg::SPI0_CTRL_CFG_SPEC>,
    #[doc = "0x02 - SPI0 interrupt enable"]
    pub spi0_inter_en: crate::Reg<spi0_inter_en::SPI0_INTER_EN_SPEC>,
    #[doc = "0x03 - SPI0 master clock divisor_ SPI0 slave preset value"]
    pub spi0_clock_div_r8_spi0_slave_pre:
        crate::Reg<spi0_clock_div_r8_spi0_slave_pre::SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE_SPEC>,
    #[doc = "0x04 - SPI0 data buffer"]
    pub spi0_buffer: crate::Reg<spi0_buffer::SPI0_BUFFER_SPEC>,
    #[doc = "0x05 - SPI0 work flag"]
    pub spi0_run_flag: crate::Reg<spi0_run_flag::SPI0_RUN_FLAG_SPEC>,
    #[doc = "0x06 - SPI0 interrupt flag"]
    pub spi0_int_flag: crate::Reg<spi0_int_flag::SPI0_INT_FLAG_SPEC>,
    #[doc = "0x07 - SPI0 FIFO count status"]
    pub spi0_fifo_count: crate::Reg<spi0_fifo_count::SPI0_FIFO_COUNT_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x0c - SPI0 total byte count, only low 12 bit"]
    pub spi0_total_cnt: crate::Reg<spi0_total_cnt::SPI0_TOTAL_CNT_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x10 - SPI0 FIFO register"]
    pub spi0_fifo: crate::Reg<spi0_fifo::SPI0_FIFO_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x13 - SPI0 FIFO count status"]
    pub spi0_fifo_count1: crate::Reg<spi0_fifo_count1::SPI0_FIFO_COUNT1_SPEC>,
    #[doc = "0x14 - SPI0 DMA current address"]
    pub spi0_dma_now: crate::Reg<spi0_dma_now::SPI0_DMA_NOW_SPEC>,
    #[doc = "0x18 - SPI0 DMA begin address"]
    pub spi0_dma_beg: crate::Reg<spi0_dma_beg::SPI0_DMA_BEG_SPEC>,
    #[doc = "0x1c - SPI0 DMA end address"]
    pub spi0_dma_end: crate::Reg<spi0_dma_end::SPI0_DMA_END_SPEC>,
}
#[doc = "SPI0_CTRL_MOD register accessor: an alias for `Reg<SPI0_CTRL_MOD_SPEC>`"]
pub type SPI0_CTRL_MOD = crate::Reg<spi0_ctrl_mod::SPI0_CTRL_MOD_SPEC>;
#[doc = "SPI0 mode control"]
pub mod spi0_ctrl_mod;
#[doc = "SPI0_CTRL_CFG register accessor: an alias for `Reg<SPI0_CTRL_CFG_SPEC>`"]
pub type SPI0_CTRL_CFG = crate::Reg<spi0_ctrl_cfg::SPI0_CTRL_CFG_SPEC>;
#[doc = "SPI0 configuration control"]
pub mod spi0_ctrl_cfg;
#[doc = "SPI0_INTER_EN register accessor: an alias for `Reg<SPI0_INTER_EN_SPEC>`"]
pub type SPI0_INTER_EN = crate::Reg<spi0_inter_en::SPI0_INTER_EN_SPEC>;
#[doc = "SPI0 interrupt enable"]
pub mod spi0_inter_en;
#[doc = "SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE register accessor: an alias for `Reg<SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE_SPEC>`"]
pub type SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE =
    crate::Reg<spi0_clock_div_r8_spi0_slave_pre::SPI0_CLOCK_DIV_R8_SPI0_SLAVE_PRE_SPEC>;
#[doc = "SPI0 master clock divisor_ SPI0 slave preset value"]
pub mod spi0_clock_div_r8_spi0_slave_pre;
#[doc = "SPI0_BUFFER register accessor: an alias for `Reg<SPI0_BUFFER_SPEC>`"]
pub type SPI0_BUFFER = crate::Reg<spi0_buffer::SPI0_BUFFER_SPEC>;
#[doc = "SPI0 data buffer"]
pub mod spi0_buffer;
#[doc = "SPI0_RUN_FLAG register accessor: an alias for `Reg<SPI0_RUN_FLAG_SPEC>`"]
pub type SPI0_RUN_FLAG = crate::Reg<spi0_run_flag::SPI0_RUN_FLAG_SPEC>;
#[doc = "SPI0 work flag"]
pub mod spi0_run_flag;
#[doc = "SPI0_INT_FLAG register accessor: an alias for `Reg<SPI0_INT_FLAG_SPEC>`"]
pub type SPI0_INT_FLAG = crate::Reg<spi0_int_flag::SPI0_INT_FLAG_SPEC>;
#[doc = "SPI0 interrupt flag"]
pub mod spi0_int_flag;
#[doc = "SPI0_FIFO_COUNT register accessor: an alias for `Reg<SPI0_FIFO_COUNT_SPEC>`"]
pub type SPI0_FIFO_COUNT = crate::Reg<spi0_fifo_count::SPI0_FIFO_COUNT_SPEC>;
#[doc = "SPI0 FIFO count status"]
pub mod spi0_fifo_count;
#[doc = "SPI0_TOTAL_CNT register accessor: an alias for `Reg<SPI0_TOTAL_CNT_SPEC>`"]
pub type SPI0_TOTAL_CNT = crate::Reg<spi0_total_cnt::SPI0_TOTAL_CNT_SPEC>;
#[doc = "SPI0 total byte count, only low 12 bit"]
pub mod spi0_total_cnt;
#[doc = "SPI0_FIFO register accessor: an alias for `Reg<SPI0_FIFO_SPEC>`"]
pub type SPI0_FIFO = crate::Reg<spi0_fifo::SPI0_FIFO_SPEC>;
#[doc = "SPI0 FIFO register"]
pub mod spi0_fifo;
#[doc = "SPI0_FIFO_COUNT1 register accessor: an alias for `Reg<SPI0_FIFO_COUNT1_SPEC>`"]
pub type SPI0_FIFO_COUNT1 = crate::Reg<spi0_fifo_count1::SPI0_FIFO_COUNT1_SPEC>;
#[doc = "SPI0 FIFO count status"]
pub mod spi0_fifo_count1;
#[doc = "SPI0_DMA_NOW register accessor: an alias for `Reg<SPI0_DMA_NOW_SPEC>`"]
pub type SPI0_DMA_NOW = crate::Reg<spi0_dma_now::SPI0_DMA_NOW_SPEC>;
#[doc = "SPI0 DMA current address"]
pub mod spi0_dma_now;
#[doc = "SPI0_DMA_BEG register accessor: an alias for `Reg<SPI0_DMA_BEG_SPEC>`"]
pub type SPI0_DMA_BEG = crate::Reg<spi0_dma_beg::SPI0_DMA_BEG_SPEC>;
#[doc = "SPI0 DMA begin address"]
pub mod spi0_dma_beg;
#[doc = "SPI0_DMA_END register accessor: an alias for `Reg<SPI0_DMA_END_SPEC>`"]
pub type SPI0_DMA_END = crate::Reg<spi0_dma_end::SPI0_DMA_END_SPEC>;
#[doc = "SPI0 DMA end address"]
pub mod spi0_dma_end;

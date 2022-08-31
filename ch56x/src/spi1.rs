#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI1 mode control"]
    pub spi1_ctrl_mod: crate::Reg<spi1_ctrl_mod::SPI1_CTRL_MOD_SPEC>,
    #[doc = "0x01 - SPI1 configuration control"]
    pub spi1_ctrl_cfg: crate::Reg<spi1_ctrl_cfg::SPI1_CTRL_CFG_SPEC>,
    #[doc = "0x02 - SPI1 interrupt enable"]
    pub spi1_inter_en: crate::Reg<spi1_inter_en::SPI1_INTER_EN_SPEC>,
    #[doc = "0x03 - SPI1 master clock divisor _ SPI1 slave preset value"]
    pub spi1_clock_div_r8_spi1_slave_pre:
        crate::Reg<spi1_clock_div_r8_spi1_slave_pre::SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRE_SPEC>,
    #[doc = "0x04 - SPI1 data buffer"]
    pub spi1_buffer: crate::Reg<spi1_buffer::SPI1_BUFFER_SPEC>,
    #[doc = "0x05 - SPI1 work flag"]
    pub spi1_run_flag: crate::Reg<spi1_run_flag::SPI1_RUN_FLAG_SPEC>,
    #[doc = "0x06 - SPI1 interrupt flag"]
    pub spi1_int_flag: crate::Reg<spi1_int_flag::SPI1_INT_FLAG_SPEC>,
    #[doc = "0x07 - SPI1 FIFO count status"]
    pub spi1_fifo_count: crate::Reg<spi1_fifo_count::SPI1_FIFO_COUNT_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x0c - SPI1 total byte count, only low 12 bit"]
    pub spi1_total_cnt: crate::Reg<spi1_total_cnt::SPI1_TOTAL_CNT_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x10 - SPI1 FIFO register"]
    pub spi1_fifo: crate::Reg<spi1_fifo::SPI1_FIFO_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x13 - SPI0 FIFO count status"]
    pub spi1_fifo_count1: crate::Reg<spi1_fifo_count1::SPI1_FIFO_COUNT1_SPEC>,
    #[doc = "0x14 - SPI1 DMA current address"]
    pub spi1_dma_now: crate::Reg<spi1_dma_now::SPI1_DMA_NOW_SPEC>,
    #[doc = "0x18 - SPI1 DMA begin address"]
    pub spi1_dma_beg: crate::Reg<spi1_dma_beg::SPI1_DMA_BEG_SPEC>,
    #[doc = "0x1c - SPI1 DMA end address"]
    pub spi1_dma_end: crate::Reg<spi1_dma_end::SPI1_DMA_END_SPEC>,
}
#[doc = "SPI1_CTRL_MOD register accessor: an alias for `Reg<SPI1_CTRL_MOD_SPEC>`"]
pub type SPI1_CTRL_MOD = crate::Reg<spi1_ctrl_mod::SPI1_CTRL_MOD_SPEC>;
#[doc = "SPI1 mode control"]
pub mod spi1_ctrl_mod;
#[doc = "SPI1_CTRL_CFG register accessor: an alias for `Reg<SPI1_CTRL_CFG_SPEC>`"]
pub type SPI1_CTRL_CFG = crate::Reg<spi1_ctrl_cfg::SPI1_CTRL_CFG_SPEC>;
#[doc = "SPI1 configuration control"]
pub mod spi1_ctrl_cfg;
#[doc = "SPI1_INTER_EN register accessor: an alias for `Reg<SPI1_INTER_EN_SPEC>`"]
pub type SPI1_INTER_EN = crate::Reg<spi1_inter_en::SPI1_INTER_EN_SPEC>;
#[doc = "SPI1 interrupt enable"]
pub mod spi1_inter_en;
#[doc = "SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRE register accessor: an alias for `Reg<SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRE_SPEC>`"]
pub type SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRE =
    crate::Reg<spi1_clock_div_r8_spi1_slave_pre::SPI1_CLOCK_DIV_R8_SPI1_SLAVE_PRE_SPEC>;
#[doc = "SPI1 master clock divisor _ SPI1 slave preset value"]
pub mod spi1_clock_div_r8_spi1_slave_pre;
#[doc = "SPI1_BUFFER register accessor: an alias for `Reg<SPI1_BUFFER_SPEC>`"]
pub type SPI1_BUFFER = crate::Reg<spi1_buffer::SPI1_BUFFER_SPEC>;
#[doc = "SPI1 data buffer"]
pub mod spi1_buffer;
#[doc = "SPI1_RUN_FLAG register accessor: an alias for `Reg<SPI1_RUN_FLAG_SPEC>`"]
pub type SPI1_RUN_FLAG = crate::Reg<spi1_run_flag::SPI1_RUN_FLAG_SPEC>;
#[doc = "SPI1 work flag"]
pub mod spi1_run_flag;
#[doc = "SPI1_INT_FLAG register accessor: an alias for `Reg<SPI1_INT_FLAG_SPEC>`"]
pub type SPI1_INT_FLAG = crate::Reg<spi1_int_flag::SPI1_INT_FLAG_SPEC>;
#[doc = "SPI1 interrupt flag"]
pub mod spi1_int_flag;
#[doc = "SPI1_FIFO_COUNT register accessor: an alias for `Reg<SPI1_FIFO_COUNT_SPEC>`"]
pub type SPI1_FIFO_COUNT = crate::Reg<spi1_fifo_count::SPI1_FIFO_COUNT_SPEC>;
#[doc = "SPI1 FIFO count status"]
pub mod spi1_fifo_count;
#[doc = "SPI1_TOTAL_CNT register accessor: an alias for `Reg<SPI1_TOTAL_CNT_SPEC>`"]
pub type SPI1_TOTAL_CNT = crate::Reg<spi1_total_cnt::SPI1_TOTAL_CNT_SPEC>;
#[doc = "SPI1 total byte count, only low 12 bit"]
pub mod spi1_total_cnt;
#[doc = "SPI1_FIFO register accessor: an alias for `Reg<SPI1_FIFO_SPEC>`"]
pub type SPI1_FIFO = crate::Reg<spi1_fifo::SPI1_FIFO_SPEC>;
#[doc = "SPI1 FIFO register"]
pub mod spi1_fifo;
#[doc = "SPI1_FIFO_COUNT1 register accessor: an alias for `Reg<SPI1_FIFO_COUNT1_SPEC>`"]
pub type SPI1_FIFO_COUNT1 = crate::Reg<spi1_fifo_count1::SPI1_FIFO_COUNT1_SPEC>;
#[doc = "SPI0 FIFO count status"]
pub mod spi1_fifo_count1;
#[doc = "SPI1_DMA_NOW register accessor: an alias for `Reg<SPI1_DMA_NOW_SPEC>`"]
pub type SPI1_DMA_NOW = crate::Reg<spi1_dma_now::SPI1_DMA_NOW_SPEC>;
#[doc = "SPI1 DMA current address"]
pub mod spi1_dma_now;
#[doc = "SPI1_DMA_BEG register accessor: an alias for `Reg<SPI1_DMA_BEG_SPEC>`"]
pub type SPI1_DMA_BEG = crate::Reg<spi1_dma_beg::SPI1_DMA_BEG_SPEC>;
#[doc = "SPI1 DMA begin address"]
pub mod spi1_dma_beg;
#[doc = "SPI1_DMA_END register accessor: an alias for `Reg<SPI1_DMA_END_SPEC>`"]
pub type SPI1_DMA_END = crate::Reg<spi1_dma_end::SPI1_DMA_END_SPEC>;
#[doc = "SPI1 DMA end address"]
pub mod spi1_dma_end;

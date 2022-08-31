#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DVP control register0"]
    pub dvp_cr0: crate::Reg<dvp_cr0::DVP_CR0_SPEC>,
    #[doc = "0x01 - DVP control register1"]
    pub dvp_cr1: crate::Reg<dvp_cr1::DVP_CR1_SPEC>,
    #[doc = "0x02 - DVP interrupt enable register"]
    pub dvp_int_en: crate::Reg<dvp_int_en::DVP_INT_EN_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - DVP row number of a frame indicator register"]
    pub dvp_row_num: crate::Reg<dvp_row_num::DVP_ROW_NUM_SPEC>,
    #[doc = "0x06 - DVP row number of a frame indicator register"]
    pub dvp_col_num: crate::Reg<dvp_col_num::DVP_COL_NUM_SPEC>,
    #[doc = "0x08 - DVP dma buffer0 addr"]
    pub dvp_dma_buf0: crate::Reg<dvp_dma_buf0::DVP_DMA_BUF0_SPEC>,
    #[doc = "0x0c - DVP dma buffer1 addr"]
    pub dvp_dma_buf1: crate::Reg<dvp_dma_buf1::DVP_DMA_BUF1_SPEC>,
    _reserved_7_dvp: [u8; 0x04],
    #[doc = "0x14 - DVP row count value"]
    pub dvp_row_cnt: crate::Reg<dvp_row_cnt::DVP_ROW_CNT_SPEC>,
    #[doc = "0x16 - DVP col count value"]
    pub dvp_col_cnt: crate::Reg<dvp_col_cnt::DVP_COL_CNT_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x10 - DVP interrupt flag register"]
    #[inline(always)]
    pub fn dvp_int_flag(&self) -> &crate::Reg<dvp_int_flag::DVP_INT_FLAG_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<dvp_int_flag::DVP_INT_FLAG_SPEC>)
        }
    }
    #[doc = "0x11 - DVP receive fifo status"]
    #[inline(always)]
    pub fn dvp_fifo_st(&self) -> &crate::Reg<dvp_fifo_st::DVP_FIFO_ST_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(17usize)
                as *const crate::Reg<dvp_fifo_st::DVP_FIFO_ST_SPEC>)
        }
    }
}
#[doc = "DVP_CR0 register accessor: an alias for `Reg<DVP_CR0_SPEC>`"]
pub type DVP_CR0 = crate::Reg<dvp_cr0::DVP_CR0_SPEC>;
#[doc = "DVP control register0"]
pub mod dvp_cr0;
#[doc = "DVP_CR1 register accessor: an alias for `Reg<DVP_CR1_SPEC>`"]
pub type DVP_CR1 = crate::Reg<dvp_cr1::DVP_CR1_SPEC>;
#[doc = "DVP control register1"]
pub mod dvp_cr1;
#[doc = "DVP_INT_EN register accessor: an alias for `Reg<DVP_INT_EN_SPEC>`"]
pub type DVP_INT_EN = crate::Reg<dvp_int_en::DVP_INT_EN_SPEC>;
#[doc = "DVP interrupt enable register"]
pub mod dvp_int_en;
#[doc = "DVP_ROW_NUM register accessor: an alias for `Reg<DVP_ROW_NUM_SPEC>`"]
pub type DVP_ROW_NUM = crate::Reg<dvp_row_num::DVP_ROW_NUM_SPEC>;
#[doc = "DVP row number of a frame indicator register"]
pub mod dvp_row_num;
#[doc = "DVP_COL_NUM register accessor: an alias for `Reg<DVP_COL_NUM_SPEC>`"]
pub type DVP_COL_NUM = crate::Reg<dvp_col_num::DVP_COL_NUM_SPEC>;
#[doc = "DVP row number of a frame indicator register"]
pub mod dvp_col_num;
#[doc = "DVP_DMA_BUF0 register accessor: an alias for `Reg<DVP_DMA_BUF0_SPEC>`"]
pub type DVP_DMA_BUF0 = crate::Reg<dvp_dma_buf0::DVP_DMA_BUF0_SPEC>;
#[doc = "DVP dma buffer0 addr"]
pub mod dvp_dma_buf0;
#[doc = "DVP_DMA_BUF1 register accessor: an alias for `Reg<DVP_DMA_BUF1_SPEC>`"]
pub type DVP_DMA_BUF1 = crate::Reg<dvp_dma_buf1::DVP_DMA_BUF1_SPEC>;
#[doc = "DVP dma buffer1 addr"]
pub mod dvp_dma_buf1;
#[doc = "DVP_INT_FLAG register accessor: an alias for `Reg<DVP_INT_FLAG_SPEC>`"]
pub type DVP_INT_FLAG = crate::Reg<dvp_int_flag::DVP_INT_FLAG_SPEC>;
#[doc = "DVP interrupt flag register"]
pub mod dvp_int_flag;
#[doc = "DVP_FIFO_ST register accessor: an alias for `Reg<DVP_FIFO_ST_SPEC>`"]
pub type DVP_FIFO_ST = crate::Reg<dvp_fifo_st::DVP_FIFO_ST_SPEC>;
#[doc = "DVP receive fifo status"]
pub mod dvp_fifo_st;
#[doc = "DVP_ROW_CNT register accessor: an alias for `Reg<DVP_ROW_CNT_SPEC>`"]
pub type DVP_ROW_CNT = crate::Reg<dvp_row_cnt::DVP_ROW_CNT_SPEC>;
#[doc = "DVP row count value"]
pub mod dvp_row_cnt;
#[doc = "DVP_COL_CNT register accessor: an alias for `Reg<DVP_COL_CNT_SPEC>`"]
pub type DVP_COL_CNT = crate::Reg<dvp_col_cnt::DVP_COL_CNT_SPEC>;
#[doc = "DVP col count value"]
pub mod dvp_col_cnt;

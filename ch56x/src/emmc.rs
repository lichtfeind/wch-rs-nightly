#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SD 32bits command argument register"]
    pub emmc_argument: crate::Reg<emmc_argument::EMMC_ARGUMENT_SPEC>,
    #[doc = "0x04 - SD 16bits cmd setting register"]
    pub emmc_cmd_set: crate::Reg<emmc_cmd_set::EMMC_CMD_SET_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - SD 128bits response register, \\[31:0\\]
32bits"]
    pub emmc_response0: crate::Reg<emmc_response0::EMMC_RESPONSE0_SPEC>,
    #[doc = "0x0c - SD 128bits response register, \\[63:32\\]
32bits"]
    pub emmc_response1: crate::Reg<emmc_response1::EMMC_RESPONSE1_SPEC>,
    #[doc = "0x10 - SD 128bits response register, \\[95:64\\]
32bits"]
    pub emmc_response2: crate::Reg<emmc_response2::EMMC_RESPONSE2_SPEC>,
    _reserved_5_emmc: [u8; 0x04],
    #[doc = "0x18 - SD 8bits control register"]
    pub emmc_control: crate::Reg<emmc_control::EMMC_CONTROL_SPEC>,
    _reserved7: [u8; 0x03],
    #[doc = "0x1c - SD 8bits data timeout value"]
    pub emmc_timeout: crate::Reg<emmc_timeout::EMMC_TIMEOUT_SPEC>,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - SD status"]
    pub emmc_status: crate::Reg<emmc_status::EMMC_STATUS_SPEC>,
    #[doc = "0x24 - SD 16bits interrupt flag register"]
    pub emmc_int_fg: crate::Reg<emmc_int_fg::EMMC_INT_FG_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x28 - SD 16bits interrupt enable register"]
    pub emmc_int_en: crate::Reg<emmc_int_en::EMMC_INT_EN_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - SD 16bits DMA start address register when to operate"]
    pub emmc_dma_beg1: crate::Reg<emmc_dma_beg1::EMMC_DMA_BEG1_SPEC>,
    #[doc = "0x30 - SD 32bits data counter, \\[15:0\\]
number of blocks this time will tran/recv, \\[27:16\\]
block sise(byte number) of every block in this time tran/recv"]
    pub emmc_block_cfg: crate::Reg<emmc_block_cfg::EMMC_BLOCK_CFG_SPEC>,
    #[doc = "0x34 - SD TRANSFER MODE register"]
    pub emmc_tran_mode: crate::Reg<emmc_tran_mode::EMMC_TRAN_MODE_SPEC>,
    #[doc = "0x38 - SD clock divider register"]
    pub emmc_clk_div: crate::Reg<emmc_clk_div::EMMC_CLK_DIV_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x3c - SD 16bits DMA start address register when to operate"]
    pub emmc_dma_beg2: crate::Reg<emmc_dma_beg2::EMMC_DMA_BEG2_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x14 - Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\]
32bits"]
    #[inline(always)]
    pub fn emmc_write_cont(&self) -> &crate::Reg<emmc_write_cont::EMMC_WRITE_CONT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<emmc_write_cont::EMMC_WRITE_CONT_SPEC>)
        }
    }
    #[doc = "0x14 - SD 128bits response register, \\[127:96\\]
32bits"]
    #[inline(always)]
    pub fn emmc_response3(&self) -> &crate::Reg<emmc_response3::EMMC_RESPONSE3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<emmc_response3::EMMC_RESPONSE3_SPEC>)
        }
    }
}
#[doc = "EMMC_CLK_DIV register accessor: an alias for `Reg<EMMC_CLK_DIV_SPEC>`"]
pub type EMMC_CLK_DIV = crate::Reg<emmc_clk_div::EMMC_CLK_DIV_SPEC>;
#[doc = "SD clock divider register"]
pub mod emmc_clk_div;
#[doc = "EMMC_ARGUMENT register accessor: an alias for `Reg<EMMC_ARGUMENT_SPEC>`"]
pub type EMMC_ARGUMENT = crate::Reg<emmc_argument::EMMC_ARGUMENT_SPEC>;
#[doc = "SD 32bits command argument register"]
pub mod emmc_argument;
#[doc = "EMMC_CMD_SET register accessor: an alias for `Reg<EMMC_CMD_SET_SPEC>`"]
pub type EMMC_CMD_SET = crate::Reg<emmc_cmd_set::EMMC_CMD_SET_SPEC>;
#[doc = "SD 16bits cmd setting register"]
pub mod emmc_cmd_set;
#[doc = "EMMC_RESPONSE0 register accessor: an alias for `Reg<EMMC_RESPONSE0_SPEC>`"]
pub type EMMC_RESPONSE0 = crate::Reg<emmc_response0::EMMC_RESPONSE0_SPEC>;
#[doc = "SD 128bits response register, \\[31:0\\]
32bits"]
pub mod emmc_response0;
#[doc = "EMMC_RESPONSE1 register accessor: an alias for `Reg<EMMC_RESPONSE1_SPEC>`"]
pub type EMMC_RESPONSE1 = crate::Reg<emmc_response1::EMMC_RESPONSE1_SPEC>;
#[doc = "SD 128bits response register, \\[63:32\\]
32bits"]
pub mod emmc_response1;
#[doc = "EMMC_RESPONSE2 register accessor: an alias for `Reg<EMMC_RESPONSE2_SPEC>`"]
pub type EMMC_RESPONSE2 = crate::Reg<emmc_response2::EMMC_RESPONSE2_SPEC>;
#[doc = "SD 128bits response register, \\[95:64\\]
32bits"]
pub mod emmc_response2;
#[doc = "EMMC_RESPONSE3 register accessor: an alias for `Reg<EMMC_RESPONSE3_SPEC>`"]
pub type EMMC_RESPONSE3 = crate::Reg<emmc_response3::EMMC_RESPONSE3_SPEC>;
#[doc = "SD 128bits response register, \\[127:96\\]
32bits"]
pub mod emmc_response3;
#[doc = "EMMC_WRITE_CONT register accessor: an alias for `Reg<EMMC_WRITE_CONT_SPEC>`"]
pub type EMMC_WRITE_CONT = crate::Reg<emmc_write_cont::EMMC_WRITE_CONT_SPEC>;
#[doc = "Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\]
32bits"]
pub mod emmc_write_cont;
#[doc = "EMMC_CONTROL register accessor: an alias for `Reg<EMMC_CONTROL_SPEC>`"]
pub type EMMC_CONTROL = crate::Reg<emmc_control::EMMC_CONTROL_SPEC>;
#[doc = "SD 8bits control register"]
pub mod emmc_control;
#[doc = "EMMC_TIMEOUT register accessor: an alias for `Reg<EMMC_TIMEOUT_SPEC>`"]
pub type EMMC_TIMEOUT = crate::Reg<emmc_timeout::EMMC_TIMEOUT_SPEC>;
#[doc = "SD 8bits data timeout value"]
pub mod emmc_timeout;
#[doc = "EMMC_STATUS register accessor: an alias for `Reg<EMMC_STATUS_SPEC>`"]
pub type EMMC_STATUS = crate::Reg<emmc_status::EMMC_STATUS_SPEC>;
#[doc = "SD status"]
pub mod emmc_status;
#[doc = "EMMC_INT_FG register accessor: an alias for `Reg<EMMC_INT_FG_SPEC>`"]
pub type EMMC_INT_FG = crate::Reg<emmc_int_fg::EMMC_INT_FG_SPEC>;
#[doc = "SD 16bits interrupt flag register"]
pub mod emmc_int_fg;
#[doc = "EMMC_INT_EN register accessor: an alias for `Reg<EMMC_INT_EN_SPEC>`"]
pub type EMMC_INT_EN = crate::Reg<emmc_int_en::EMMC_INT_EN_SPEC>;
#[doc = "SD 16bits interrupt enable register"]
pub mod emmc_int_en;
#[doc = "EMMC_DMA_BEG1 register accessor: an alias for `Reg<EMMC_DMA_BEG1_SPEC>`"]
pub type EMMC_DMA_BEG1 = crate::Reg<emmc_dma_beg1::EMMC_DMA_BEG1_SPEC>;
#[doc = "SD 16bits DMA start address register when to operate"]
pub mod emmc_dma_beg1;
#[doc = "EMMC_BLOCK_CFG register accessor: an alias for `Reg<EMMC_BLOCK_CFG_SPEC>`"]
pub type EMMC_BLOCK_CFG = crate::Reg<emmc_block_cfg::EMMC_BLOCK_CFG_SPEC>;
#[doc = "SD 32bits data counter, \\[15:0\\]
number of blocks this time will tran/recv, \\[27:16\\]
block sise(byte number) of every block in this time tran/recv"]
pub mod emmc_block_cfg;
#[doc = "EMMC_TRAN_MODE register accessor: an alias for `Reg<EMMC_TRAN_MODE_SPEC>`"]
pub type EMMC_TRAN_MODE = crate::Reg<emmc_tran_mode::EMMC_TRAN_MODE_SPEC>;
#[doc = "SD TRANSFER MODE register"]
pub mod emmc_tran_mode;
#[doc = "EMMC_DMA_BEG2 register accessor: an alias for `Reg<EMMC_DMA_BEG2_SPEC>`"]
pub type EMMC_DMA_BEG2 = crate::Reg<emmc_dma_beg2::EMMC_DMA_BEG2_SPEC>;
#[doc = "SD 16bits DMA start address register when to operate"]
pub mod emmc_dma_beg2;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - parallel if tx or rx cfg"]
    pub hspi_cfg: crate::Reg<hspi_cfg::HSPI_CFG_SPEC>,
    #[doc = "0x01 - parallel if tx or rx control"]
    pub hspi_ctrl: crate::Reg<hspi_ctrl::HSPI_CTRL_SPEC>,
    #[doc = "0x02 - parallel if interrupt enable register"]
    pub hspi_int_en: crate::Reg<hspi_int_en::HSPI_INT_EN_SPEC>,
    #[doc = "0x03 - parallel if aux"]
    pub hspi_aux: crate::Reg<hspi_aux::HSPI_AUX_SPEC>,
    #[doc = "0x04 - parallel if dma tx addr0"]
    pub hspi_tx_addr0: crate::Reg<hspi_tx_addr0::HSPI_TX_ADDR0_SPEC>,
    #[doc = "0x08 - parallel if dma tx addr1"]
    pub hspi_tx_addr1: crate::Reg<hspi_tx_addr1::HSPI_TX_ADDR1_SPEC>,
    #[doc = "0x0c - parallel if dma rx addr0"]
    pub hspi_rx_addr0: crate::Reg<hspi_rx_addr0::HSPI_RX_ADDR0_SPEC>,
    #[doc = "0x10 - parallel if dma rx addr1"]
    pub hspi_rx_addr1: crate::Reg<hspi_rx_addr1::HSPI_RX_ADDR1_SPEC>,
    #[doc = "0x14 - parallel if dma length0"]
    pub hspi_dma_len0: crate::Reg<hspi_dma_len0::HSPI_DMA_LEN0_SPEC>,
    #[doc = "0x16 - parallel if receive length0"]
    pub hspi_rx_len0: crate::Reg<hspi_rx_len0::HSPI_RX_LEN0_SPEC>,
    #[doc = "0x18 - parallel if dma length1"]
    pub hspi_dma_len1: crate::Reg<hspi_dma_len1::HSPI_DMA_LEN1_SPEC>,
    #[doc = "0x1a - parallel if receive length1"]
    pub hspi_rx_len1: crate::Reg<hspi_rx_len1::HSPI_RX_LEN1_SPEC>,
    #[doc = "0x1c - parallel if tx burst config register"]
    pub hspi_burst_cfg: crate::Reg<hspi_burst_cfg::HSPI_BURST_CFG_SPEC>,
    #[doc = "0x1e - parallel if tx burst count"]
    pub hspi_burst_cnt: crate::Reg<hspi_burst_cnt::HSPI_BURST_CNT_SPEC>,
    _reserved14: [u8; 0x01],
    #[doc = "0x20 - parallel if user defined field 0 register"]
    pub hspi_udf0: crate::Reg<hspi_udf0::HSPI_UDF0_SPEC>,
    #[doc = "0x24 - parallel if user defined field 1 register"]
    pub hspi_udf1: crate::Reg<hspi_udf1::HSPI_UDF1_SPEC>,
    #[doc = "0x28 - parallel if interrupt flag"]
    pub hspi_int_flag: crate::Reg<hspi_int_flag::HSPI_INT_FLAG_SPEC>,
    #[doc = "0x29 - parallel rtx status"]
    pub hspi_rtx_status: crate::Reg<hspi_rtx_status::HSPI_RTX_STATUS_SPEC>,
    #[doc = "0x2a - parallel TX sequence ctrl"]
    pub hspi_tx_sc: crate::Reg<hspi_tx_sc::HSPI_TX_SC_SPEC>,
    #[doc = "0x2b - parallel RX sequence ctrl"]
    pub hspi_rx_sc: crate::Reg<hspi_rx_sc::HSPI_RX_SC_SPEC>,
}
#[doc = "HSPI_CFG register accessor: an alias for `Reg<HSPI_CFG_SPEC>`"]
pub type HSPI_CFG = crate::Reg<hspi_cfg::HSPI_CFG_SPEC>;
#[doc = "parallel if tx or rx cfg"]
pub mod hspi_cfg;
#[doc = "HSPI_CTRL register accessor: an alias for `Reg<HSPI_CTRL_SPEC>`"]
pub type HSPI_CTRL = crate::Reg<hspi_ctrl::HSPI_CTRL_SPEC>;
#[doc = "parallel if tx or rx control"]
pub mod hspi_ctrl;
#[doc = "HSPI_INT_EN register accessor: an alias for `Reg<HSPI_INT_EN_SPEC>`"]
pub type HSPI_INT_EN = crate::Reg<hspi_int_en::HSPI_INT_EN_SPEC>;
#[doc = "parallel if interrupt enable register"]
pub mod hspi_int_en;
#[doc = "HSPI_AUX register accessor: an alias for `Reg<HSPI_AUX_SPEC>`"]
pub type HSPI_AUX = crate::Reg<hspi_aux::HSPI_AUX_SPEC>;
#[doc = "parallel if aux"]
pub mod hspi_aux;
#[doc = "HSPI_TX_ADDR0 register accessor: an alias for `Reg<HSPI_TX_ADDR0_SPEC>`"]
pub type HSPI_TX_ADDR0 = crate::Reg<hspi_tx_addr0::HSPI_TX_ADDR0_SPEC>;
#[doc = "parallel if dma tx addr0"]
pub mod hspi_tx_addr0;
#[doc = "HSPI_TX_ADDR1 register accessor: an alias for `Reg<HSPI_TX_ADDR1_SPEC>`"]
pub type HSPI_TX_ADDR1 = crate::Reg<hspi_tx_addr1::HSPI_TX_ADDR1_SPEC>;
#[doc = "parallel if dma tx addr1"]
pub mod hspi_tx_addr1;
#[doc = "HSPI_RX_ADDR0 register accessor: an alias for `Reg<HSPI_RX_ADDR0_SPEC>`"]
pub type HSPI_RX_ADDR0 = crate::Reg<hspi_rx_addr0::HSPI_RX_ADDR0_SPEC>;
#[doc = "parallel if dma rx addr0"]
pub mod hspi_rx_addr0;
#[doc = "HSPI_RX_ADDR1 register accessor: an alias for `Reg<HSPI_RX_ADDR1_SPEC>`"]
pub type HSPI_RX_ADDR1 = crate::Reg<hspi_rx_addr1::HSPI_RX_ADDR1_SPEC>;
#[doc = "parallel if dma rx addr1"]
pub mod hspi_rx_addr1;
#[doc = "HSPI_DMA_LEN0 register accessor: an alias for `Reg<HSPI_DMA_LEN0_SPEC>`"]
pub type HSPI_DMA_LEN0 = crate::Reg<hspi_dma_len0::HSPI_DMA_LEN0_SPEC>;
#[doc = "parallel if dma length0"]
pub mod hspi_dma_len0;
#[doc = "HSPI_RX_LEN0 register accessor: an alias for `Reg<HSPI_RX_LEN0_SPEC>`"]
pub type HSPI_RX_LEN0 = crate::Reg<hspi_rx_len0::HSPI_RX_LEN0_SPEC>;
#[doc = "parallel if receive length0"]
pub mod hspi_rx_len0;
#[doc = "HSPI_DMA_LEN1 register accessor: an alias for `Reg<HSPI_DMA_LEN1_SPEC>`"]
pub type HSPI_DMA_LEN1 = crate::Reg<hspi_dma_len1::HSPI_DMA_LEN1_SPEC>;
#[doc = "parallel if dma length1"]
pub mod hspi_dma_len1;
#[doc = "HSPI_RX_LEN1 register accessor: an alias for `Reg<HSPI_RX_LEN1_SPEC>`"]
pub type HSPI_RX_LEN1 = crate::Reg<hspi_rx_len1::HSPI_RX_LEN1_SPEC>;
#[doc = "parallel if receive length1"]
pub mod hspi_rx_len1;
#[doc = "HSPI_BURST_CFG register accessor: an alias for `Reg<HSPI_BURST_CFG_SPEC>`"]
pub type HSPI_BURST_CFG = crate::Reg<hspi_burst_cfg::HSPI_BURST_CFG_SPEC>;
#[doc = "parallel if tx burst config register"]
pub mod hspi_burst_cfg;
#[doc = "HSPI_BURST_CNT register accessor: an alias for `Reg<HSPI_BURST_CNT_SPEC>`"]
pub type HSPI_BURST_CNT = crate::Reg<hspi_burst_cnt::HSPI_BURST_CNT_SPEC>;
#[doc = "parallel if tx burst count"]
pub mod hspi_burst_cnt;
#[doc = "HSPI_UDF0 register accessor: an alias for `Reg<HSPI_UDF0_SPEC>`"]
pub type HSPI_UDF0 = crate::Reg<hspi_udf0::HSPI_UDF0_SPEC>;
#[doc = "parallel if user defined field 0 register"]
pub mod hspi_udf0;
#[doc = "HSPI_UDF1 register accessor: an alias for `Reg<HSPI_UDF1_SPEC>`"]
pub type HSPI_UDF1 = crate::Reg<hspi_udf1::HSPI_UDF1_SPEC>;
#[doc = "parallel if user defined field 1 register"]
pub mod hspi_udf1;
#[doc = "HSPI_INT_FLAG register accessor: an alias for `Reg<HSPI_INT_FLAG_SPEC>`"]
pub type HSPI_INT_FLAG = crate::Reg<hspi_int_flag::HSPI_INT_FLAG_SPEC>;
#[doc = "parallel if interrupt flag"]
pub mod hspi_int_flag;
#[doc = "HSPI_RTX_STATUS register accessor: an alias for `Reg<HSPI_RTX_STATUS_SPEC>`"]
pub type HSPI_RTX_STATUS = crate::Reg<hspi_rtx_status::HSPI_RTX_STATUS_SPEC>;
#[doc = "parallel rtx status"]
pub mod hspi_rtx_status;
#[doc = "HSPI_TX_SC register accessor: an alias for `Reg<HSPI_TX_SC_SPEC>`"]
pub type HSPI_TX_SC = crate::Reg<hspi_tx_sc::HSPI_TX_SC_SPEC>;
#[doc = "parallel TX sequence ctrl"]
pub mod hspi_tx_sc;
#[doc = "HSPI_RX_SC register accessor: an alias for `Reg<HSPI_RX_SC_SPEC>`"]
pub type HSPI_RX_SC = crate::Reg<hspi_rx_sc::HSPI_RX_SC_SPEC>;
#[doc = "parallel RX sequence ctrl"]
pub mod hspi_rx_sc;

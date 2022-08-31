#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB base control"]
    pub usb_ctrl: crate::Reg<usb_ctrl::USB_CTRL_SPEC>,
    #[doc = "0x01 - USB host control register"]
    pub uhost_ctrl: crate::Reg<uhost_ctrl::UHOST_CTRL_SPEC>,
    #[doc = "0x02 - USB interrupt enable"]
    pub usb_int_en: crate::Reg<usb_int_en::USB_INT_EN_SPEC>,
    #[doc = "0x03 - USB device address"]
    pub usb_dev_ad: crate::Reg<usb_dev_ad::USB_DEV_AD_SPEC>,
    #[doc = "0x04 - USB frame number register"]
    pub usb_frame_no: crate::Reg<usb_frame_no::USB_FRAME_NO_SPEC>,
    #[doc = "0x06 - USB suspend register"]
    pub usb_suspend: crate::Reg<usb_suspend::USB_SUSPEND_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x08 - USB actual speed register"]
    pub usb_spd_type: crate::Reg<usb_spd_type::USB_SPD_TYPE_SPEC>,
    #[doc = "0x09 - USB miscellaneous status"]
    pub usb_mis_st: crate::Reg<usb_mis_st::USB_MIS_ST_SPEC>,
    #[doc = "0x0a - USB interrupt flag"]
    pub usb_int_fg: crate::Reg<usb_int_fg::USB_INT_FG_SPEC>,
    #[doc = "0x0b - USB interrupt status"]
    pub usb_int_st: crate::Reg<usb_int_st::USB_INT_ST_SPEC>,
    #[doc = "0x0c - USB receiving length"]
    pub r6_usb_rx_len: crate::Reg<r6_usb_rx_len::R6_USB_RX_LEN_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x10 - endpoint 1(9) 4(8,12) mode"]
    pub uep4_1_mod: crate::Reg<uep4_1_mod::UEP4_1_MOD_SPEC>,
    #[doc = "0x11 - endpoint 2(10) 3(11) mode and USB host endpoint mode control register"]
    pub uep2_3_mod_r8_uh_ep_mod: crate::Reg<uep2_3_mod_r8_uh_ep_mod::UEP2_3_MOD_R8_UH_EP_MOD_SPEC>,
    #[doc = "0x12 - endpoint 5(13) 6(14) mode"]
    pub uep5_6_mod: crate::Reg<uep5_6_mod::UEP5_6_MOD_SPEC>,
    #[doc = "0x13 - endpoint 7(15) mode"]
    pub uep7_mod: crate::Reg<uep7_mod::UEP7_MOD_SPEC>,
    #[doc = "0x14..0x34 - endpoint %s DMA buffer address"]
    pub uep_rx_dma: [crate::Reg<uep_rx_dma::UEP_RX_DMA_SPEC>; 8],
    #[doc = "0x34..0x50 - endpoint 1 DMA TX buffer address"]
    pub uep_tx_dma: [crate::Reg<uep_tx_dma::UEP_TX_DMA_SPEC>; 7],
    #[doc = "0x50 - endpoint %s receive max length"]
    pub uep0_max_len: crate::Reg<uep_max_len::UEP_MAX_LEN_SPEC>,
    _reserved18: [u8; 0x02],
    #[doc = "0x54 - endpoint %s receive max length"]
    pub uep1_max_len: crate::Reg<uep_max_len::UEP_MAX_LEN_SPEC>,
    _reserved19: [u8; 0x02],
    #[doc = "0x58 - endpoint %s receive max length"]
    pub uep2_max_len_r16_uh_max_len: crate::Reg<uep_max_len::UEP_MAX_LEN_SPEC>,
    _reserved20: [u8; 0x02],
    #[doc = "0x5c - endpoint %s receive max length"]
    pub uep3_max_len: crate::Reg<uep_max_len::UEP_MAX_LEN_SPEC>,
    _reserved21: [u8; 0x02],
    #[doc = "0x60 - endpoint %s receive max length"]
    pub uep4_max_len: crate::Reg<uep_max_len::UEP_MAX_LEN_SPEC>,
    _reserved22: [u8; 0x02],
    #[doc = "0x64 - endpoint %s receive max length"]
    pub uep5_max_len: crate::Reg<uep_max_len::UEP_MAX_LEN_SPEC>,
    _reserved23: [u8; 0x02],
    #[doc = "0x68 - endpoint %s receive max length"]
    pub uep6_max_len: crate::Reg<uep_max_len::UEP_MAX_LEN_SPEC>,
    _reserved24: [u8; 0x02],
    #[doc = "0x6c - endpoint %s receive max length"]
    pub uep7_max_len: crate::Reg<uep_max_len::UEP_MAX_LEN_SPEC>,
    _reserved25: [u8; 0x02],
    #[doc = "0x70 - endpoint 0 transmittal length"]
    pub uep0_t_len: crate::Reg<uep0_t_len::UEP0_T_LEN_SPEC>,
    #[doc = "0x72 - endpoint 0 tx control"]
    pub uep0_tx_ctrl: crate::Reg<uep0_tx_ctrl::UEP0_TX_CTRL_SPEC>,
    #[doc = "0x73 - endpoint 0 rx control"]
    pub uep0_rx_ctrl: crate::Reg<uep0_rx_ctrl::UEP0_RX_CTRL_SPEC>,
    #[doc = "0x74 - endpoint 1 transmittal length"]
    pub uep1_t_len: crate::Reg<uep1_t_len::UEP1_T_LEN_SPEC>,
    #[doc = "0x76 - endpoint 1 tx control"]
    pub uep1_tx_ctrl: crate::Reg<uep1_tx_ctrl::UEP1_TX_CTRL_SPEC>,
    #[doc = "0x77 - endpoint 1 rx control"]
    pub uep1_rx_ctrl: crate::Reg<uep1_rx_ctrl::UEP1_RX_CTRL_SPEC>,
    #[doc = "0x78 - endpoint 2 transmittal length and Set usb host token register"]
    pub uep2_t_len_r16_uh_ep_pid:
        crate::Reg<uep2_t_len_r16_uh_ep_pid::UEP2_T_LEN_R16_UH_EP_PID_SPEC>,
    #[doc = "0x7a - endpoint 2 tx control"]
    pub uep2_tx_ctrl: crate::Reg<uep2_tx_ctrl::UEP2_TX_CTRL_SPEC>,
    #[doc = "0x7b - endpoint 2 rx control and USb host receive endpoint control register"]
    pub uep2_rx_ctrl_r8_uh_rx_ctrl:
        crate::Reg<uep2_rx_ctrl_r8_uh_rx_ctrl::UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>,
    #[doc = "0x7c - endpoint 3 transmittal length and host transmittal endpoint transmittal length"]
    pub uep3_t_len_r16_uh_tx_len:
        crate::Reg<uep3_t_len_r16_uh_tx_len::UEP3_T_LEN_R16_UH_TX_LEN_SPEC>,
    #[doc = "0x7e - endpoint 3 tx control and host transmittal endpoint control"]
    pub uep3_tx_ctrl_r8_uh_tx_ctrl:
        crate::Reg<uep3_tx_ctrl_r8_uh_tx_ctrl::UEP3_TX_CTRL_R8_UH_TX_CTRL_SPEC>,
    #[doc = "0x7f - endpoint 3 rx control"]
    pub uep3_rx_ctrl: crate::Reg<uep3_rx_ctrl::UEP3_RX_CTRL_SPEC>,
    #[doc = "0x80 - endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
    pub uep4_t_len_r16_uh_split_data:
        crate::Reg<uep4_t_len_r16_uh_split_data::UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>,
    #[doc = "0x82 - endpoint 4 tx control"]
    pub uep4_tx_ctrl: crate::Reg<uep4_tx_ctrl::UEP4_TX_CTRL_SPEC>,
    #[doc = "0x83 - endpoint 4 rx control"]
    pub uep4_rx_ctrl: crate::Reg<uep4_rx_ctrl::UEP4_RX_CTRL_SPEC>,
    #[doc = "0x84 - endpoint 5 transmittal length"]
    pub uep5_t_len: crate::Reg<uep5_t_len::UEP5_T_LEN_SPEC>,
    #[doc = "0x86 - endpoint 5 tx control"]
    pub uep5_tx_ctrl: crate::Reg<uep5_tx_ctrl::UEP5_TX_CTRL_SPEC>,
    #[doc = "0x87 - endpoint 5 rx control"]
    pub uep5_rx_ctrl: crate::Reg<uep5_rx_ctrl::UEP5_RX_CTRL_SPEC>,
    #[doc = "0x88 - endpoint 6 transmittal length"]
    pub uep6_t_len: crate::Reg<uep6_t_len::UEP6_T_LEN_SPEC>,
    #[doc = "0x8a - endpoint 6 tx control"]
    pub uep6_tx_ctrl: crate::Reg<uep6_tx_ctrl::UEP6_TX_CTRL_SPEC>,
    #[doc = "0x8b - endpoint 6 rx control"]
    pub uep6_rx_ctrl: crate::Reg<uep6_rx_ctrl::UEP6_RX_CTRL_SPEC>,
    #[doc = "0x8c - endpoint 7 transmittal length"]
    pub uep7_t_len: crate::Reg<uep7_t_len::UEP7_T_LEN_SPEC>,
    #[doc = "0x8e - endpoint 7 tx control"]
    pub uep7_tx_ctrl: crate::Reg<uep7_tx_ctrl::UEP7_TX_CTRL_SPEC>,
    #[doc = "0x8f - endpoint 7 rx control"]
    pub uep7_rx_ctrl: crate::Reg<uep7_rx_ctrl::UEP7_RX_CTRL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x14 - endpoint %s DMA buffer address"]
    #[inline(always)]
    pub fn uep0_rx_dma(&self) -> &crate::Reg<uep_rx_dma::UEP_RX_DMA_SPEC> {
        &self.uep_rx_dma[0]
    }
    #[doc = "0x18 - endpoint %s DMA buffer address"]
    #[inline(always)]
    pub fn uep1_rx_dma(&self) -> &crate::Reg<uep_rx_dma::UEP_RX_DMA_SPEC> {
        &self.uep_rx_dma[1]
    }
    #[doc = "0x1c - endpoint %s DMA buffer address"]
    #[inline(always)]
    pub fn uep2_rx_dma_r32_uh_rx_dma(&self) -> &crate::Reg<uep_rx_dma::UEP_RX_DMA_SPEC> {
        &self.uep_rx_dma[2]
    }
    #[doc = "0x20 - endpoint %s DMA buffer address"]
    #[inline(always)]
    pub fn uep3_rx_dma(&self) -> &crate::Reg<uep_rx_dma::UEP_RX_DMA_SPEC> {
        &self.uep_rx_dma[3]
    }
    #[doc = "0x24 - endpoint %s DMA buffer address"]
    #[inline(always)]
    pub fn uep4_rx_dma(&self) -> &crate::Reg<uep_rx_dma::UEP_RX_DMA_SPEC> {
        &self.uep_rx_dma[4]
    }
    #[doc = "0x28 - endpoint %s DMA buffer address"]
    #[inline(always)]
    pub fn uep5_rx_dma(&self) -> &crate::Reg<uep_rx_dma::UEP_RX_DMA_SPEC> {
        &self.uep_rx_dma[5]
    }
    #[doc = "0x2c - endpoint %s DMA buffer address"]
    #[inline(always)]
    pub fn uep6_rx_dma(&self) -> &crate::Reg<uep_rx_dma::UEP_RX_DMA_SPEC> {
        &self.uep_rx_dma[6]
    }
    #[doc = "0x30 - endpoint %s DMA buffer address"]
    #[inline(always)]
    pub fn uep7_rx_dma(&self) -> &crate::Reg<uep_rx_dma::UEP_RX_DMA_SPEC> {
        &self.uep_rx_dma[7]
    }
    #[doc = "0x34 - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep1_tx_dma(&self) -> &crate::Reg<uep_tx_dma::UEP_TX_DMA_SPEC> {
        &self.uep_tx_dma[0]
    }
    #[doc = "0x38 - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep2_tx_dma(&self) -> &crate::Reg<uep_tx_dma::UEP_TX_DMA_SPEC> {
        &self.uep_tx_dma[1]
    }
    #[doc = "0x3c - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep3_tx_dma_r32_uh_tx_dma(&self) -> &crate::Reg<uep_tx_dma::UEP_TX_DMA_SPEC> {
        &self.uep_tx_dma[2]
    }
    #[doc = "0x40 - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep4_tx_dma(&self) -> &crate::Reg<uep_tx_dma::UEP_TX_DMA_SPEC> {
        &self.uep_tx_dma[3]
    }
    #[doc = "0x44 - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep5_tx_dma(&self) -> &crate::Reg<uep_tx_dma::UEP_TX_DMA_SPEC> {
        &self.uep_tx_dma[4]
    }
    #[doc = "0x48 - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep6_tx_dma(&self) -> &crate::Reg<uep_tx_dma::UEP_TX_DMA_SPEC> {
        &self.uep_tx_dma[5]
    }
    #[doc = "0x4c - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep7_tx_dma(&self) -> &crate::Reg<uep_tx_dma::UEP_TX_DMA_SPEC> {
        &self.uep_tx_dma[6]
    }
}
#[doc = "USB_CTRL register accessor: an alias for `Reg<USB_CTRL_SPEC>`"]
pub type USB_CTRL = crate::Reg<usb_ctrl::USB_CTRL_SPEC>;
#[doc = "USB base control"]
pub mod usb_ctrl;
#[doc = "UHOST_CTRL register accessor: an alias for `Reg<UHOST_CTRL_SPEC>`"]
pub type UHOST_CTRL = crate::Reg<uhost_ctrl::UHOST_CTRL_SPEC>;
#[doc = "USB host control register"]
pub mod uhost_ctrl;
#[doc = "USB_INT_EN register accessor: an alias for `Reg<USB_INT_EN_SPEC>`"]
pub type USB_INT_EN = crate::Reg<usb_int_en::USB_INT_EN_SPEC>;
#[doc = "USB interrupt enable"]
pub mod usb_int_en;
#[doc = "USB_DEV_AD register accessor: an alias for `Reg<USB_DEV_AD_SPEC>`"]
pub type USB_DEV_AD = crate::Reg<usb_dev_ad::USB_DEV_AD_SPEC>;
#[doc = "USB device address"]
pub mod usb_dev_ad;
#[doc = "USB_FRAME_NO register accessor: an alias for `Reg<USB_FRAME_NO_SPEC>`"]
pub type USB_FRAME_NO = crate::Reg<usb_frame_no::USB_FRAME_NO_SPEC>;
#[doc = "USB frame number register"]
pub mod usb_frame_no;
#[doc = "USB_SUSPEND register accessor: an alias for `Reg<USB_SUSPEND_SPEC>`"]
pub type USB_SUSPEND = crate::Reg<usb_suspend::USB_SUSPEND_SPEC>;
#[doc = "USB suspend register"]
pub mod usb_suspend;
#[doc = "USB_SPD_TYPE register accessor: an alias for `Reg<USB_SPD_TYPE_SPEC>`"]
pub type USB_SPD_TYPE = crate::Reg<usb_spd_type::USB_SPD_TYPE_SPEC>;
#[doc = "USB actual speed register"]
pub mod usb_spd_type;
#[doc = "USB_MIS_ST register accessor: an alias for `Reg<USB_MIS_ST_SPEC>`"]
pub type USB_MIS_ST = crate::Reg<usb_mis_st::USB_MIS_ST_SPEC>;
#[doc = "USB miscellaneous status"]
pub mod usb_mis_st;
#[doc = "USB_INT_FG register accessor: an alias for `Reg<USB_INT_FG_SPEC>`"]
pub type USB_INT_FG = crate::Reg<usb_int_fg::USB_INT_FG_SPEC>;
#[doc = "USB interrupt flag"]
pub mod usb_int_fg;
#[doc = "USB_INT_ST register accessor: an alias for `Reg<USB_INT_ST_SPEC>`"]
pub type USB_INT_ST = crate::Reg<usb_int_st::USB_INT_ST_SPEC>;
#[doc = "USB interrupt status"]
pub mod usb_int_st;
#[doc = "R6_USB_RX_LEN register accessor: an alias for `Reg<R6_USB_RX_LEN_SPEC>`"]
pub type R6_USB_RX_LEN = crate::Reg<r6_usb_rx_len::R6_USB_RX_LEN_SPEC>;
#[doc = "USB receiving length"]
pub mod r6_usb_rx_len;
#[doc = "UEP4_1_MOD register accessor: an alias for `Reg<UEP4_1_MOD_SPEC>`"]
pub type UEP4_1_MOD = crate::Reg<uep4_1_mod::UEP4_1_MOD_SPEC>;
#[doc = "endpoint 1(9) 4(8,12) mode"]
pub mod uep4_1_mod;
#[doc = "UEP2_3_MOD_R8_UH_EP_MOD register accessor: an alias for `Reg<UEP2_3_MOD_R8_UH_EP_MOD_SPEC>`"]
pub type UEP2_3_MOD_R8_UH_EP_MOD =
    crate::Reg<uep2_3_mod_r8_uh_ep_mod::UEP2_3_MOD_R8_UH_EP_MOD_SPEC>;
#[doc = "endpoint 2(10) 3(11) mode and USB host endpoint mode control register"]
pub mod uep2_3_mod_r8_uh_ep_mod;
#[doc = "UEP5_6_MOD register accessor: an alias for `Reg<UEP5_6_MOD_SPEC>`"]
pub type UEP5_6_MOD = crate::Reg<uep5_6_mod::UEP5_6_MOD_SPEC>;
#[doc = "endpoint 5(13) 6(14) mode"]
pub mod uep5_6_mod;
#[doc = "UEP7_MOD register accessor: an alias for `Reg<UEP7_MOD_SPEC>`"]
pub type UEP7_MOD = crate::Reg<uep7_mod::UEP7_MOD_SPEC>;
#[doc = "endpoint 7(15) mode"]
pub mod uep7_mod;
#[doc = "UEP_RX_DMA register accessor: an alias for `Reg<UEP_RX_DMA_SPEC>`"]
pub type UEP_RX_DMA = crate::Reg<uep_rx_dma::UEP_RX_DMA_SPEC>;
#[doc = "endpoint %s DMA buffer address"]
pub mod uep_rx_dma;
#[doc = "UEP_TX_DMA register accessor: an alias for `Reg<UEP_TX_DMA_SPEC>`"]
pub type UEP_TX_DMA = crate::Reg<uep_tx_dma::UEP_TX_DMA_SPEC>;
#[doc = "endpoint 1 DMA TX buffer address"]
pub mod uep_tx_dma;
#[doc = "UEP_MAX_LEN register accessor: an alias for `Reg<UEP_MAX_LEN_SPEC>`"]
pub type UEP_MAX_LEN = crate::Reg<uep_max_len::UEP_MAX_LEN_SPEC>;
#[doc = "endpoint %s receive max length"]
pub mod uep_max_len;
#[doc = "UEP0_T_LEN register accessor: an alias for `Reg<UEP0_T_LEN_SPEC>`"]
pub type UEP0_T_LEN = crate::Reg<uep0_t_len::UEP0_T_LEN_SPEC>;
#[doc = "endpoint 0 transmittal length"]
pub mod uep0_t_len;
#[doc = "UEP0_TX_CTRL register accessor: an alias for `Reg<UEP0_TX_CTRL_SPEC>`"]
pub type UEP0_TX_CTRL = crate::Reg<uep0_tx_ctrl::UEP0_TX_CTRL_SPEC>;
#[doc = "endpoint 0 tx control"]
pub mod uep0_tx_ctrl;
#[doc = "UEP0_RX_CTRL register accessor: an alias for `Reg<UEP0_RX_CTRL_SPEC>`"]
pub type UEP0_RX_CTRL = crate::Reg<uep0_rx_ctrl::UEP0_RX_CTRL_SPEC>;
#[doc = "endpoint 0 rx control"]
pub mod uep0_rx_ctrl;
#[doc = "UEP1_T_LEN register accessor: an alias for `Reg<UEP1_T_LEN_SPEC>`"]
pub type UEP1_T_LEN = crate::Reg<uep1_t_len::UEP1_T_LEN_SPEC>;
#[doc = "endpoint 1 transmittal length"]
pub mod uep1_t_len;
#[doc = "UEP1_TX_CTRL register accessor: an alias for `Reg<UEP1_TX_CTRL_SPEC>`"]
pub type UEP1_TX_CTRL = crate::Reg<uep1_tx_ctrl::UEP1_TX_CTRL_SPEC>;
#[doc = "endpoint 1 tx control"]
pub mod uep1_tx_ctrl;
#[doc = "UEP1_RX_CTRL register accessor: an alias for `Reg<UEP1_RX_CTRL_SPEC>`"]
pub type UEP1_RX_CTRL = crate::Reg<uep1_rx_ctrl::UEP1_RX_CTRL_SPEC>;
#[doc = "endpoint 1 rx control"]
pub mod uep1_rx_ctrl;
#[doc = "UEP2_T_LEN_R16_UH_EP_PID register accessor: an alias for `Reg<UEP2_T_LEN_R16_UH_EP_PID_SPEC>`"]
pub type UEP2_T_LEN_R16_UH_EP_PID =
    crate::Reg<uep2_t_len_r16_uh_ep_pid::UEP2_T_LEN_R16_UH_EP_PID_SPEC>;
#[doc = "endpoint 2 transmittal length and Set usb host token register"]
pub mod uep2_t_len_r16_uh_ep_pid;
#[doc = "UEP2_TX_CTRL register accessor: an alias for `Reg<UEP2_TX_CTRL_SPEC>`"]
pub type UEP2_TX_CTRL = crate::Reg<uep2_tx_ctrl::UEP2_TX_CTRL_SPEC>;
#[doc = "endpoint 2 tx control"]
pub mod uep2_tx_ctrl;
#[doc = "UEP2_RX_CTRL_R8_UH_RX_CTRL register accessor: an alias for `Reg<UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>`"]
pub type UEP2_RX_CTRL_R8_UH_RX_CTRL =
    crate::Reg<uep2_rx_ctrl_r8_uh_rx_ctrl::UEP2_RX_CTRL_R8_UH_RX_CTRL_SPEC>;
#[doc = "endpoint 2 rx control and USb host receive endpoint control register"]
pub mod uep2_rx_ctrl_r8_uh_rx_ctrl;
#[doc = "UEP3_T_LEN_R16_UH_TX_LEN register accessor: an alias for `Reg<UEP3_T_LEN_R16_UH_TX_LEN_SPEC>`"]
pub type UEP3_T_LEN_R16_UH_TX_LEN =
    crate::Reg<uep3_t_len_r16_uh_tx_len::UEP3_T_LEN_R16_UH_TX_LEN_SPEC>;
#[doc = "endpoint 3 transmittal length and host transmittal endpoint transmittal length"]
pub mod uep3_t_len_r16_uh_tx_len;
#[doc = "UEP3_TX_CTRL_R8_UH_TX_CTRL register accessor: an alias for `Reg<UEP3_TX_CTRL_R8_UH_TX_CTRL_SPEC>`"]
pub type UEP3_TX_CTRL_R8_UH_TX_CTRL =
    crate::Reg<uep3_tx_ctrl_r8_uh_tx_ctrl::UEP3_TX_CTRL_R8_UH_TX_CTRL_SPEC>;
#[doc = "endpoint 3 tx control and host transmittal endpoint control"]
pub mod uep3_tx_ctrl_r8_uh_tx_ctrl;
#[doc = "UEP3_RX_CTRL register accessor: an alias for `Reg<UEP3_RX_CTRL_SPEC>`"]
pub type UEP3_RX_CTRL = crate::Reg<uep3_rx_ctrl::UEP3_RX_CTRL_SPEC>;
#[doc = "endpoint 3 rx control"]
pub mod uep3_rx_ctrl;
#[doc = "UEP4_T_LEN_R16_UH_SPLIT_DATA register accessor: an alias for `Reg<UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>`"]
pub type UEP4_T_LEN_R16_UH_SPLIT_DATA =
    crate::Reg<uep4_t_len_r16_uh_split_data::UEP4_T_LEN_R16_UH_SPLIT_DATA_SPEC>;
#[doc = "endpoint 4 transmittal length and USB host Tx SPLIT packet data"]
pub mod uep4_t_len_r16_uh_split_data;
#[doc = "UEP4_TX_CTRL register accessor: an alias for `Reg<UEP4_TX_CTRL_SPEC>`"]
pub type UEP4_TX_CTRL = crate::Reg<uep4_tx_ctrl::UEP4_TX_CTRL_SPEC>;
#[doc = "endpoint 4 tx control"]
pub mod uep4_tx_ctrl;
#[doc = "UEP4_RX_CTRL register accessor: an alias for `Reg<UEP4_RX_CTRL_SPEC>`"]
pub type UEP4_RX_CTRL = crate::Reg<uep4_rx_ctrl::UEP4_RX_CTRL_SPEC>;
#[doc = "endpoint 4 rx control"]
pub mod uep4_rx_ctrl;
#[doc = "UEP5_T_LEN register accessor: an alias for `Reg<UEP5_T_LEN_SPEC>`"]
pub type UEP5_T_LEN = crate::Reg<uep5_t_len::UEP5_T_LEN_SPEC>;
#[doc = "endpoint 5 transmittal length"]
pub mod uep5_t_len;
#[doc = "UEP5_TX_CTRL register accessor: an alias for `Reg<UEP5_TX_CTRL_SPEC>`"]
pub type UEP5_TX_CTRL = crate::Reg<uep5_tx_ctrl::UEP5_TX_CTRL_SPEC>;
#[doc = "endpoint 5 tx control"]
pub mod uep5_tx_ctrl;
#[doc = "UEP5_RX_CTRL register accessor: an alias for `Reg<UEP5_RX_CTRL_SPEC>`"]
pub type UEP5_RX_CTRL = crate::Reg<uep5_rx_ctrl::UEP5_RX_CTRL_SPEC>;
#[doc = "endpoint 5 rx control"]
pub mod uep5_rx_ctrl;
#[doc = "UEP6_T_LEN register accessor: an alias for `Reg<UEP6_T_LEN_SPEC>`"]
pub type UEP6_T_LEN = crate::Reg<uep6_t_len::UEP6_T_LEN_SPEC>;
#[doc = "endpoint 6 transmittal length"]
pub mod uep6_t_len;
#[doc = "UEP6_TX_CTRL register accessor: an alias for `Reg<UEP6_TX_CTRL_SPEC>`"]
pub type UEP6_TX_CTRL = crate::Reg<uep6_tx_ctrl::UEP6_TX_CTRL_SPEC>;
#[doc = "endpoint 6 tx control"]
pub mod uep6_tx_ctrl;
#[doc = "UEP6_RX_CTRL register accessor: an alias for `Reg<UEP6_RX_CTRL_SPEC>`"]
pub type UEP6_RX_CTRL = crate::Reg<uep6_rx_ctrl::UEP6_RX_CTRL_SPEC>;
#[doc = "endpoint 6 rx control"]
pub mod uep6_rx_ctrl;
#[doc = "UEP7_T_LEN register accessor: an alias for `Reg<UEP7_T_LEN_SPEC>`"]
pub type UEP7_T_LEN = crate::Reg<uep7_t_len::UEP7_T_LEN_SPEC>;
#[doc = "endpoint 7 transmittal length"]
pub mod uep7_t_len;
#[doc = "UEP7_TX_CTRL register accessor: an alias for `Reg<UEP7_TX_CTRL_SPEC>`"]
pub type UEP7_TX_CTRL = crate::Reg<uep7_tx_ctrl::UEP7_TX_CTRL_SPEC>;
#[doc = "endpoint 7 tx control"]
pub mod uep7_tx_ctrl;
#[doc = "UEP7_RX_CTRL register accessor: an alias for `Reg<UEP7_RX_CTRL_SPEC>`"]
pub type UEP7_RX_CTRL = crate::Reg<uep7_rx_ctrl::UEP7_RX_CTRL_SPEC>;
#[doc = "endpoint 7 rx control"]
pub mod uep7_rx_ctrl;

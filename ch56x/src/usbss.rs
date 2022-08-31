#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub link_cfg: crate::Reg<link_cfg::LINK_CFG_SPEC>,
    #[doc = "0x04 - "]
    pub link_ctrl: crate::Reg<link_ctrl::LINK_CTRL_SPEC>,
    #[doc = "0x08 - "]
    pub link_int_ctrl: crate::Reg<link_int_ctrl::LINK_INT_CTRL_SPEC>,
    #[doc = "0x0c - "]
    pub link_int_flag: crate::Reg<link_int_flag::LINK_INT_FLAG_SPEC>,
    #[doc = "0x10 - "]
    pub link_status: crate::Reg<link_status::LINK_STATUS_SPEC>,
    #[doc = "0x14 - "]
    pub link_hp_buf_ctrl: crate::Reg<link_hp_buf_ctrl::LINK_HP_BUF_CTRL_SPEC>,
    #[doc = "0x18 - "]
    pub link_err_status: crate::Reg<link_err_status::LINK_ERR_STATUS_SPEC>,
    #[doc = "0x1c - "]
    pub link_err_cnt: crate::Reg<link_err_cnt::LINK_ERR_CNT_SPEC>,
    #[doc = "0x20 - "]
    pub usb_control: crate::Reg<usb_control::USB_CONTROL_SPEC>,
    #[doc = "0x24 - "]
    pub usb_status: crate::Reg<usb_status::USB_STATUS_SPEC>,
    #[doc = "0x28 - "]
    pub host_status: crate::Reg<host_status::HOST_STATUS_SPEC>,
    #[doc = "0x2c - "]
    pub usb_rx_len: crate::Reg<usb_rx_len::USB_RX_LEN_SPEC>,
    #[doc = "0x30 - "]
    pub usb_itp: crate::Reg<usb_itp::USB_ITP_SPEC>,
    #[doc = "0x34 - "]
    pub usb_itp_adj: crate::Reg<usb_itp_adj::USB_ITP_ADJ_SPEC>,
    #[doc = "0x38 - "]
    pub link_iso_dly: crate::Reg<link_iso_dly::LINK_ISO_DLY_SPEC>,
    #[doc = "0x3c - "]
    pub link_stream_id: crate::Reg<link_stream_id::LINK_STREAM_ID_SPEC>,
    #[doc = "0x40 - "]
    pub link_route_string: crate::Reg<link_route_string::LINK_ROUTE_STRING_SPEC>,
    #[doc = "0x44 - "]
    pub usb_fc_ctrl: crate::Reg<usb_fc_ctrl::USB_FC_CTRL_SPEC>,
    #[doc = "0x48 - Link Management Packet"]
    pub lmp_tx_data0: crate::Reg<lmp_tx_data0::LMP_TX_DATA0_SPEC>,
    #[doc = "0x4c - Link Management Packet"]
    pub lmp_tx_data1: crate::Reg<lmp_tx_data1::LMP_TX_DATA1_SPEC>,
    #[doc = "0x50 - Link Management Packet"]
    pub lmp_tx_data2: crate::Reg<lmp_tx_data2::LMP_TX_DATA2_SPEC>,
    #[doc = "0x54 - "]
    pub lmp_port_cap: crate::Reg<lmp_port_cap::LMP_PORT_CAP_SPEC>,
    #[doc = "0x58 - "]
    pub lmp_rx_data0: crate::Reg<lmp_rx_data0::LMP_RX_DATA0_SPEC>,
    #[doc = "0x5c - "]
    pub lmp_rx_data1: crate::Reg<lmp_rx_data1::LMP_RX_DATA1_SPEC>,
    #[doc = "0x60 - "]
    pub lmp_rx_data2: crate::Reg<lmp_rx_data2::LMP_RX_DATA2_SPEC>,
    #[doc = "0x64 - "]
    pub tp_rx_data0: crate::Reg<tp_rx_data0::TP_RX_DATA0_SPEC>,
    #[doc = "0x68 - "]
    pub tp_rx_data1: crate::Reg<tp_rx_data1::TP_RX_DATA1_SPEC>,
    #[doc = "0x6c - "]
    pub tp_rx_data2: crate::Reg<tp_rx_data2::TP_RX_DATA2_SPEC>,
    #[doc = "0x70 - "]
    pub uep_cfg: crate::Reg<uep_cfg::UEP_CFG_SPEC>,
    #[doc = "0x74 - "]
    pub uep0_dma: crate::Reg<uep0_dma::UEP0_DMA_SPEC>,
    #[doc = "0x78 - "]
    pub uep0_rx_ctrl: crate::Reg<uep0_rx_ctrl::UEP0_RX_CTRL_SPEC>,
    #[doc = "0x7c - "]
    pub uep0_tx_ctrl: crate::Reg<uep0_tx_ctrl::UEP0_TX_CTRL_SPEC>,
    #[doc = "0x80 - "]
    pub uep1_tx_dma: crate::Reg<uep1_tx_dma::UEP1_TX_DMA_SPEC>,
    #[doc = "0x84 - "]
    pub uep1_rx_dma: crate::Reg<uep1_rx_dma::UEP1_RX_DMA_SPEC>,
    #[doc = "0x88 - "]
    pub uep1_rx_ctrl: crate::Reg<uep1_rx_ctrl::UEP1_RX_CTRL_SPEC>,
    #[doc = "0x8c - "]
    pub uep1_tx_ctrl: crate::Reg<uep1_tx_ctrl::UEP1_TX_CTRL_SPEC>,
    #[doc = "0x90 - "]
    pub uep2_rx_dma: crate::Reg<uep2_rx_dma::UEP2_RX_DMA_SPEC>,
    #[doc = "0x94 - "]
    pub uep2_tx_dma: crate::Reg<uep2_tx_dma::UEP2_TX_DMA_SPEC>,
    #[doc = "0x98 - "]
    pub uep2_rx_ctrl: crate::Reg<uep2_rx_ctrl::UEP2_RX_CTRL_SPEC>,
    #[doc = "0x9c - "]
    pub uep2_tx_ctrl: crate::Reg<uep2_tx_ctrl::UEP2_TX_CTRL_SPEC>,
    #[doc = "0xa0 - "]
    pub uep3_tx_dma: crate::Reg<uep3_tx_dma::UEP3_TX_DMA_SPEC>,
    #[doc = "0xa4 - "]
    pub uep3_rx_dma: crate::Reg<uep3_rx_dma::UEP3_RX_DMA_SPEC>,
    #[doc = "0xa8 - "]
    pub uep3_rx_ctrl: crate::Reg<uep3_rx_ctrl::UEP3_RX_CTRL_SPEC>,
    #[doc = "0xac - "]
    pub uep3_tx_ctrl: crate::Reg<uep3_tx_ctrl::UEP3_TX_CTRL_SPEC>,
    #[doc = "0xb0 - "]
    pub uep4_tx_dma: crate::Reg<uep4_tx_dma::UEP4_TX_DMA_SPEC>,
    #[doc = "0xb4 - "]
    pub uep4_rx_dma: crate::Reg<uep4_rx_dma::UEP4_RX_DMA_SPEC>,
    #[doc = "0xb8 - "]
    pub uep4_rx_ctrl: crate::Reg<uep4_rx_ctrl::UEP4_RX_CTRL_SPEC>,
    #[doc = "0xbc - "]
    pub uep4_tx_ctrl: crate::Reg<uep4_tx_ctrl::UEP4_TX_CTRL_SPEC>,
    #[doc = "0xc0 - "]
    pub uep5_tx_dma: crate::Reg<uep5_tx_dma::UEP5_TX_DMA_SPEC>,
    #[doc = "0xc4 - "]
    pub uep5_rx_dma: crate::Reg<uep5_rx_dma::UEP5_RX_DMA_SPEC>,
    #[doc = "0xc8 - "]
    pub uep5_rx_ctrl: crate::Reg<uep5_rx_ctrl::UEP5_RX_CTRL_SPEC>,
    #[doc = "0xcc - "]
    pub uep5_tx_ctrl: crate::Reg<uep5_tx_ctrl::UEP5_TX_CTRL_SPEC>,
    #[doc = "0xd0 - "]
    pub uep6_tx_dma: crate::Reg<uep6_tx_dma::UEP6_TX_DMA_SPEC>,
    #[doc = "0xd4 - "]
    pub uep6_rx_dma: crate::Reg<uep6_rx_dma::UEP6_RX_DMA_SPEC>,
    #[doc = "0xd8 - "]
    pub uep6_rx_ctrl: crate::Reg<uep6_rx_ctrl::UEP6_RX_CTRL_SPEC>,
    #[doc = "0xdc - "]
    pub uep6_tx_ctrl: crate::Reg<uep6_tx_ctrl::UEP6_TX_CTRL_SPEC>,
    #[doc = "0xe0 - "]
    pub uep7_tx_dma: crate::Reg<uep7_tx_dma::UEP7_TX_DMA_SPEC>,
    #[doc = "0xe4 - "]
    pub uep7_rx_dma: crate::Reg<uep7_rx_dma::UEP7_RX_DMA_SPEC>,
    #[doc = "0xe8 - "]
    pub uep7_rx_ctrl: crate::Reg<uep7_rx_ctrl::UEP7_RX_CTRL_SPEC>,
    #[doc = "0xec - "]
    pub uep7_tx_ctrl: crate::Reg<uep7_tx_ctrl::UEP7_TX_CTRL_SPEC>,
    #[doc = "0xf0 - "]
    pub ux_exit_timer: crate::Reg<ux_exit_timer::UX_EXIT_TIMER_SPEC>,
}
#[doc = "LINK_CFG register accessor: an alias for `Reg<LINK_CFG_SPEC>`"]
pub type LINK_CFG = crate::Reg<link_cfg::LINK_CFG_SPEC>;
#[doc = ""]
pub mod link_cfg;
#[doc = "LINK_CTRL register accessor: an alias for `Reg<LINK_CTRL_SPEC>`"]
pub type LINK_CTRL = crate::Reg<link_ctrl::LINK_CTRL_SPEC>;
#[doc = ""]
pub mod link_ctrl;
#[doc = "LINK_INT_CTRL register accessor: an alias for `Reg<LINK_INT_CTRL_SPEC>`"]
pub type LINK_INT_CTRL = crate::Reg<link_int_ctrl::LINK_INT_CTRL_SPEC>;
#[doc = ""]
pub mod link_int_ctrl;
#[doc = "LINK_INT_FLAG register accessor: an alias for `Reg<LINK_INT_FLAG_SPEC>`"]
pub type LINK_INT_FLAG = crate::Reg<link_int_flag::LINK_INT_FLAG_SPEC>;
#[doc = ""]
pub mod link_int_flag;
#[doc = "LINK_STATUS register accessor: an alias for `Reg<LINK_STATUS_SPEC>`"]
pub type LINK_STATUS = crate::Reg<link_status::LINK_STATUS_SPEC>;
#[doc = ""]
pub mod link_status;
#[doc = "LINK_HP_BUF_CTRL register accessor: an alias for `Reg<LINK_HP_BUF_CTRL_SPEC>`"]
pub type LINK_HP_BUF_CTRL = crate::Reg<link_hp_buf_ctrl::LINK_HP_BUF_CTRL_SPEC>;
#[doc = ""]
pub mod link_hp_buf_ctrl;
#[doc = "LINK_ERR_STATUS register accessor: an alias for `Reg<LINK_ERR_STATUS_SPEC>`"]
pub type LINK_ERR_STATUS = crate::Reg<link_err_status::LINK_ERR_STATUS_SPEC>;
#[doc = ""]
pub mod link_err_status;
#[doc = "LINK_ERR_CNT register accessor: an alias for `Reg<LINK_ERR_CNT_SPEC>`"]
pub type LINK_ERR_CNT = crate::Reg<link_err_cnt::LINK_ERR_CNT_SPEC>;
#[doc = ""]
pub mod link_err_cnt;
#[doc = "USB_CONTROL register accessor: an alias for `Reg<USB_CONTROL_SPEC>`"]
pub type USB_CONTROL = crate::Reg<usb_control::USB_CONTROL_SPEC>;
#[doc = ""]
pub mod usb_control;
#[doc = "USB_STATUS register accessor: an alias for `Reg<USB_STATUS_SPEC>`"]
pub type USB_STATUS = crate::Reg<usb_status::USB_STATUS_SPEC>;
#[doc = ""]
pub mod usb_status;
#[doc = "HOST_STATUS register accessor: an alias for `Reg<HOST_STATUS_SPEC>`"]
pub type HOST_STATUS = crate::Reg<host_status::HOST_STATUS_SPEC>;
#[doc = ""]
pub mod host_status;
#[doc = "USB_RX_LEN register accessor: an alias for `Reg<USB_RX_LEN_SPEC>`"]
pub type USB_RX_LEN = crate::Reg<usb_rx_len::USB_RX_LEN_SPEC>;
#[doc = ""]
pub mod usb_rx_len;
#[doc = "USB_ITP register accessor: an alias for `Reg<USB_ITP_SPEC>`"]
pub type USB_ITP = crate::Reg<usb_itp::USB_ITP_SPEC>;
#[doc = ""]
pub mod usb_itp;
#[doc = "USB_ITP_ADJ register accessor: an alias for `Reg<USB_ITP_ADJ_SPEC>`"]
pub type USB_ITP_ADJ = crate::Reg<usb_itp_adj::USB_ITP_ADJ_SPEC>;
#[doc = ""]
pub mod usb_itp_adj;
#[doc = "LINK_ISO_DLY register accessor: an alias for `Reg<LINK_ISO_DLY_SPEC>`"]
pub type LINK_ISO_DLY = crate::Reg<link_iso_dly::LINK_ISO_DLY_SPEC>;
#[doc = ""]
pub mod link_iso_dly;
#[doc = "LINK_STREAM_ID register accessor: an alias for `Reg<LINK_STREAM_ID_SPEC>`"]
pub type LINK_STREAM_ID = crate::Reg<link_stream_id::LINK_STREAM_ID_SPEC>;
#[doc = ""]
pub mod link_stream_id;
#[doc = "LINK_ROUTE_STRING register accessor: an alias for `Reg<LINK_ROUTE_STRING_SPEC>`"]
pub type LINK_ROUTE_STRING = crate::Reg<link_route_string::LINK_ROUTE_STRING_SPEC>;
#[doc = ""]
pub mod link_route_string;
#[doc = "USB_FC_CTRL register accessor: an alias for `Reg<USB_FC_CTRL_SPEC>`"]
pub type USB_FC_CTRL = crate::Reg<usb_fc_ctrl::USB_FC_CTRL_SPEC>;
#[doc = ""]
pub mod usb_fc_ctrl;
#[doc = "LMP_TX_DATA0 register accessor: an alias for `Reg<LMP_TX_DATA0_SPEC>`"]
pub type LMP_TX_DATA0 = crate::Reg<lmp_tx_data0::LMP_TX_DATA0_SPEC>;
#[doc = "Link Management Packet"]
pub mod lmp_tx_data0;
#[doc = "LMP_TX_DATA1 register accessor: an alias for `Reg<LMP_TX_DATA1_SPEC>`"]
pub type LMP_TX_DATA1 = crate::Reg<lmp_tx_data1::LMP_TX_DATA1_SPEC>;
#[doc = "Link Management Packet"]
pub mod lmp_tx_data1;
#[doc = "LMP_TX_DATA2 register accessor: an alias for `Reg<LMP_TX_DATA2_SPEC>`"]
pub type LMP_TX_DATA2 = crate::Reg<lmp_tx_data2::LMP_TX_DATA2_SPEC>;
#[doc = "Link Management Packet"]
pub mod lmp_tx_data2;
#[doc = "LMP_PORT_CAP register accessor: an alias for `Reg<LMP_PORT_CAP_SPEC>`"]
pub type LMP_PORT_CAP = crate::Reg<lmp_port_cap::LMP_PORT_CAP_SPEC>;
#[doc = ""]
pub mod lmp_port_cap;
#[doc = "LMP_RX_DATA0 register accessor: an alias for `Reg<LMP_RX_DATA0_SPEC>`"]
pub type LMP_RX_DATA0 = crate::Reg<lmp_rx_data0::LMP_RX_DATA0_SPEC>;
#[doc = ""]
pub mod lmp_rx_data0;
#[doc = "LMP_RX_DATA1 register accessor: an alias for `Reg<LMP_RX_DATA1_SPEC>`"]
pub type LMP_RX_DATA1 = crate::Reg<lmp_rx_data1::LMP_RX_DATA1_SPEC>;
#[doc = ""]
pub mod lmp_rx_data1;
#[doc = "LMP_RX_DATA2 register accessor: an alias for `Reg<LMP_RX_DATA2_SPEC>`"]
pub type LMP_RX_DATA2 = crate::Reg<lmp_rx_data2::LMP_RX_DATA2_SPEC>;
#[doc = ""]
pub mod lmp_rx_data2;
#[doc = "TP_RX_DATA0 register accessor: an alias for `Reg<TP_RX_DATA0_SPEC>`"]
pub type TP_RX_DATA0 = crate::Reg<tp_rx_data0::TP_RX_DATA0_SPEC>;
#[doc = ""]
pub mod tp_rx_data0;
#[doc = "TP_RX_DATA1 register accessor: an alias for `Reg<TP_RX_DATA1_SPEC>`"]
pub type TP_RX_DATA1 = crate::Reg<tp_rx_data1::TP_RX_DATA1_SPEC>;
#[doc = ""]
pub mod tp_rx_data1;
#[doc = "TP_RX_DATA2 register accessor: an alias for `Reg<TP_RX_DATA2_SPEC>`"]
pub type TP_RX_DATA2 = crate::Reg<tp_rx_data2::TP_RX_DATA2_SPEC>;
#[doc = ""]
pub mod tp_rx_data2;
#[doc = "UEP_CFG register accessor: an alias for `Reg<UEP_CFG_SPEC>`"]
pub type UEP_CFG = crate::Reg<uep_cfg::UEP_CFG_SPEC>;
#[doc = ""]
pub mod uep_cfg;
#[doc = "UEP0_DMA register accessor: an alias for `Reg<UEP0_DMA_SPEC>`"]
pub type UEP0_DMA = crate::Reg<uep0_dma::UEP0_DMA_SPEC>;
#[doc = ""]
pub mod uep0_dma;
#[doc = "UEP0_RX_CTRL register accessor: an alias for `Reg<UEP0_RX_CTRL_SPEC>`"]
pub type UEP0_RX_CTRL = crate::Reg<uep0_rx_ctrl::UEP0_RX_CTRL_SPEC>;
#[doc = ""]
pub mod uep0_rx_ctrl;
#[doc = "UEP0_TX_CTRL register accessor: an alias for `Reg<UEP0_TX_CTRL_SPEC>`"]
pub type UEP0_TX_CTRL = crate::Reg<uep0_tx_ctrl::UEP0_TX_CTRL_SPEC>;
#[doc = ""]
pub mod uep0_tx_ctrl;
#[doc = "UEP1_TX_DMA register accessor: an alias for `Reg<UEP1_TX_DMA_SPEC>`"]
pub type UEP1_TX_DMA = crate::Reg<uep1_tx_dma::UEP1_TX_DMA_SPEC>;
#[doc = ""]
pub mod uep1_tx_dma;
#[doc = "UEP1_RX_DMA register accessor: an alias for `Reg<UEP1_RX_DMA_SPEC>`"]
pub type UEP1_RX_DMA = crate::Reg<uep1_rx_dma::UEP1_RX_DMA_SPEC>;
#[doc = ""]
pub mod uep1_rx_dma;
#[doc = "UEP1_RX_CTRL register accessor: an alias for `Reg<UEP1_RX_CTRL_SPEC>`"]
pub type UEP1_RX_CTRL = crate::Reg<uep1_rx_ctrl::UEP1_RX_CTRL_SPEC>;
#[doc = ""]
pub mod uep1_rx_ctrl;
#[doc = "UEP1_TX_CTRL register accessor: an alias for `Reg<UEP1_TX_CTRL_SPEC>`"]
pub type UEP1_TX_CTRL = crate::Reg<uep1_tx_ctrl::UEP1_TX_CTRL_SPEC>;
#[doc = ""]
pub mod uep1_tx_ctrl;
#[doc = "UEP2_RX_DMA register accessor: an alias for `Reg<UEP2_RX_DMA_SPEC>`"]
pub type UEP2_RX_DMA = crate::Reg<uep2_rx_dma::UEP2_RX_DMA_SPEC>;
#[doc = ""]
pub mod uep2_rx_dma;
#[doc = "UEP2_TX_DMA register accessor: an alias for `Reg<UEP2_TX_DMA_SPEC>`"]
pub type UEP2_TX_DMA = crate::Reg<uep2_tx_dma::UEP2_TX_DMA_SPEC>;
#[doc = ""]
pub mod uep2_tx_dma;
#[doc = "UEP2_RX_CTRL register accessor: an alias for `Reg<UEP2_RX_CTRL_SPEC>`"]
pub type UEP2_RX_CTRL = crate::Reg<uep2_rx_ctrl::UEP2_RX_CTRL_SPEC>;
#[doc = ""]
pub mod uep2_rx_ctrl;
#[doc = "UEP2_TX_CTRL register accessor: an alias for `Reg<UEP2_TX_CTRL_SPEC>`"]
pub type UEP2_TX_CTRL = crate::Reg<uep2_tx_ctrl::UEP2_TX_CTRL_SPEC>;
#[doc = ""]
pub mod uep2_tx_ctrl;
#[doc = "UEP3_TX_DMA register accessor: an alias for `Reg<UEP3_TX_DMA_SPEC>`"]
pub type UEP3_TX_DMA = crate::Reg<uep3_tx_dma::UEP3_TX_DMA_SPEC>;
#[doc = ""]
pub mod uep3_tx_dma;
#[doc = "UEP3_RX_DMA register accessor: an alias for `Reg<UEP3_RX_DMA_SPEC>`"]
pub type UEP3_RX_DMA = crate::Reg<uep3_rx_dma::UEP3_RX_DMA_SPEC>;
#[doc = ""]
pub mod uep3_rx_dma;
#[doc = "UEP3_RX_CTRL register accessor: an alias for `Reg<UEP3_RX_CTRL_SPEC>`"]
pub type UEP3_RX_CTRL = crate::Reg<uep3_rx_ctrl::UEP3_RX_CTRL_SPEC>;
#[doc = ""]
pub mod uep3_rx_ctrl;
#[doc = "UEP3_TX_CTRL register accessor: an alias for `Reg<UEP3_TX_CTRL_SPEC>`"]
pub type UEP3_TX_CTRL = crate::Reg<uep3_tx_ctrl::UEP3_TX_CTRL_SPEC>;
#[doc = ""]
pub mod uep3_tx_ctrl;
#[doc = "UEP4_TX_DMA register accessor: an alias for `Reg<UEP4_TX_DMA_SPEC>`"]
pub type UEP4_TX_DMA = crate::Reg<uep4_tx_dma::UEP4_TX_DMA_SPEC>;
#[doc = ""]
pub mod uep4_tx_dma;
#[doc = "UEP4_RX_DMA register accessor: an alias for `Reg<UEP4_RX_DMA_SPEC>`"]
pub type UEP4_RX_DMA = crate::Reg<uep4_rx_dma::UEP4_RX_DMA_SPEC>;
#[doc = ""]
pub mod uep4_rx_dma;
#[doc = "UEP4_RX_CTRL register accessor: an alias for `Reg<UEP4_RX_CTRL_SPEC>`"]
pub type UEP4_RX_CTRL = crate::Reg<uep4_rx_ctrl::UEP4_RX_CTRL_SPEC>;
#[doc = ""]
pub mod uep4_rx_ctrl;
#[doc = "UEP4_TX_CTRL register accessor: an alias for `Reg<UEP4_TX_CTRL_SPEC>`"]
pub type UEP4_TX_CTRL = crate::Reg<uep4_tx_ctrl::UEP4_TX_CTRL_SPEC>;
#[doc = ""]
pub mod uep4_tx_ctrl;
#[doc = "UEP5_TX_DMA register accessor: an alias for `Reg<UEP5_TX_DMA_SPEC>`"]
pub type UEP5_TX_DMA = crate::Reg<uep5_tx_dma::UEP5_TX_DMA_SPEC>;
#[doc = ""]
pub mod uep5_tx_dma;
#[doc = "UEP5_RX_DMA register accessor: an alias for `Reg<UEP5_RX_DMA_SPEC>`"]
pub type UEP5_RX_DMA = crate::Reg<uep5_rx_dma::UEP5_RX_DMA_SPEC>;
#[doc = ""]
pub mod uep5_rx_dma;
#[doc = "UEP5_RX_CTRL register accessor: an alias for `Reg<UEP5_RX_CTRL_SPEC>`"]
pub type UEP5_RX_CTRL = crate::Reg<uep5_rx_ctrl::UEP5_RX_CTRL_SPEC>;
#[doc = ""]
pub mod uep5_rx_ctrl;
#[doc = "UEP5_TX_CTRL register accessor: an alias for `Reg<UEP5_TX_CTRL_SPEC>`"]
pub type UEP5_TX_CTRL = crate::Reg<uep5_tx_ctrl::UEP5_TX_CTRL_SPEC>;
#[doc = ""]
pub mod uep5_tx_ctrl;
#[doc = "UEP6_TX_DMA register accessor: an alias for `Reg<UEP6_TX_DMA_SPEC>`"]
pub type UEP6_TX_DMA = crate::Reg<uep6_tx_dma::UEP6_TX_DMA_SPEC>;
#[doc = ""]
pub mod uep6_tx_dma;
#[doc = "UEP6_RX_DMA register accessor: an alias for `Reg<UEP6_RX_DMA_SPEC>`"]
pub type UEP6_RX_DMA = crate::Reg<uep6_rx_dma::UEP6_RX_DMA_SPEC>;
#[doc = ""]
pub mod uep6_rx_dma;
#[doc = "UEP6_RX_CTRL register accessor: an alias for `Reg<UEP6_RX_CTRL_SPEC>`"]
pub type UEP6_RX_CTRL = crate::Reg<uep6_rx_ctrl::UEP6_RX_CTRL_SPEC>;
#[doc = ""]
pub mod uep6_rx_ctrl;
#[doc = "UEP6_TX_CTRL register accessor: an alias for `Reg<UEP6_TX_CTRL_SPEC>`"]
pub type UEP6_TX_CTRL = crate::Reg<uep6_tx_ctrl::UEP6_TX_CTRL_SPEC>;
#[doc = ""]
pub mod uep6_tx_ctrl;
#[doc = "UEP7_TX_DMA register accessor: an alias for `Reg<UEP7_TX_DMA_SPEC>`"]
pub type UEP7_TX_DMA = crate::Reg<uep7_tx_dma::UEP7_TX_DMA_SPEC>;
#[doc = ""]
pub mod uep7_tx_dma;
#[doc = "UEP7_RX_DMA register accessor: an alias for `Reg<UEP7_RX_DMA_SPEC>`"]
pub type UEP7_RX_DMA = crate::Reg<uep7_rx_dma::UEP7_RX_DMA_SPEC>;
#[doc = ""]
pub mod uep7_rx_dma;
#[doc = "UEP7_RX_CTRL register accessor: an alias for `Reg<UEP7_RX_CTRL_SPEC>`"]
pub type UEP7_RX_CTRL = crate::Reg<uep7_rx_ctrl::UEP7_RX_CTRL_SPEC>;
#[doc = ""]
pub mod uep7_rx_ctrl;
#[doc = "UEP7_TX_CTRL register accessor: an alias for `Reg<UEP7_TX_CTRL_SPEC>`"]
pub type UEP7_TX_CTRL = crate::Reg<uep7_tx_ctrl::UEP7_TX_CTRL_SPEC>;
#[doc = ""]
pub mod uep7_tx_ctrl;
#[doc = "UX_EXIT_TIMER register accessor: an alias for `Reg<UX_EXIT_TIMER_SPEC>`"]
pub type UX_EXIT_TIMER = crate::Reg<ux_exit_timer::UX_EXIT_TIMER_SPEC>;
#[doc = ""]
pub mod ux_exit_timer;

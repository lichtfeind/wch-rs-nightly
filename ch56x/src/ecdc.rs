#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ECED AES/SM4 register"]
    pub ecec_ctrl: crate::Reg<ecec_ctrl::ECEC_CTRL_SPEC>,
    #[doc = "0x02 - Interupt enable register"]
    pub ecdc_int_en: crate::Reg<ecdc_int_en::ECDC_INT_EN_SPEC>,
    _reserved2: [u8; 0x03],
    #[doc = "0x06 - Interupt flag register"]
    pub ecdc_int_fg: crate::Reg<ecdc_int_fg::ECDC_INT_FG_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x08 - User key 224-255 register"]
    pub ecdc_key_255t224: crate::Reg<ecdc_key_255t224::ECDC_KEY_255T224_SPEC>,
    #[doc = "0x0c - User key 192-223 register"]
    pub ecdc_key_223t192: crate::Reg<ecdc_key_223t192::ECDC_KEY_223T192_SPEC>,
    #[doc = "0x10 - User key 160-191 register"]
    pub ecdc_key_191t160: crate::Reg<ecdc_key_191t160::ECDC_KEY_191T160_SPEC>,
    #[doc = "0x14 - User key 128-159 register"]
    pub ecdc_key_159t128: crate::Reg<ecdc_key_159t128::ECDC_KEY_159T128_SPEC>,
    #[doc = "0x18 - User key 96-127 register"]
    pub ecdc_key_127t96: crate::Reg<ecdc_key_127t96::ECDC_KEY_127T96_SPEC>,
    #[doc = "0x1c - User key 64-95 register"]
    pub ecdc_key_95t64: crate::Reg<ecdc_key_95t64::ECDC_KEY_95T64_SPEC>,
    #[doc = "0x20 - User key 32-63 register"]
    pub ecdc_key_63t32: crate::Reg<ecdc_key_63t32::ECDC_KEY_63T32_SPEC>,
    #[doc = "0x24 - User key 0-31 register"]
    pub ecdc_key_31t0: crate::Reg<ecdc_key_31t0::ECDC_KEY_31T0_SPEC>,
    #[doc = "0x28 - CTR mode count 96-127 register"]
    pub ecdc_iv_127t96: crate::Reg<ecdc_iv_127t96::ECDC_IV_127T96_SPEC>,
    #[doc = "0x2c - CTR mode count 64-95 register"]
    pub ecdc_iv_95t64: crate::Reg<ecdc_iv_95t64::ECDC_IV_95T64_SPEC>,
    #[doc = "0x30 - CTR mode count 32-63 register"]
    pub ecdc_iv_63t32: crate::Reg<ecdc_iv_63t32::ECDC_IV_63T32_SPEC>,
    #[doc = "0x34 - CTR mode count 0-31 register"]
    pub ecdc_iv_31t0: crate::Reg<ecdc_iv_31t0::ECDC_IV_31T0_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x40 - Single encryption and decryption of original data 96-127 register"]
    pub ecdc_sgsd_127t96: crate::Reg<ecdc_sgsd_127t96::ECDC_SGSD_127T96_SPEC>,
    #[doc = "0x44 - Single encryption and decryption of original data 64-95 register"]
    pub ecdc_sgsd_95t64: crate::Reg<ecdc_sgsd_95t64::ECDC_SGSD_95T64_SPEC>,
    #[doc = "0x48 - Single encryption and decryption of original data 32-63 register"]
    pub ecdc_sgsd_63t32: crate::Reg<ecdc_sgsd_63t32::ECDC_SGSD_63T32_SPEC>,
    #[doc = "0x4c - Single encryption and decryption of original data 0-31 register"]
    pub ecdc_sgsd_31t0: crate::Reg<ecdc_sgsd_31t0::ECDC_SGSD_31T0_SPEC>,
    #[doc = "0x50 - Single encryption and decryption result 96-127 register"]
    pub ecdc_sgrt_127t96: crate::Reg<ecdc_sgrt_127t96::ECDC_SGRT_127T96_SPEC>,
    #[doc = "0x54 - Single encryption and decryption result 64-95 register"]
    pub ecdc_sgrt_95t64: crate::Reg<ecdc_sgrt_95t64::ECDC_SGRT_95T64_SPEC>,
    #[doc = "0x58 - Single encryption and decryption result 0-31 register"]
    pub ecdc_sgrt_63t32: crate::Reg<ecdc_sgrt_63t32::ECDC_SGRT_63T32_SPEC>,
    #[doc = "0x5c - Single encryption and decryption result 0-31 register"]
    pub rb_ecdc_sgrt_31t0: crate::Reg<rb_ecdc_sgrt_31t0::RB_ECDC_SGRT_31T0_SPEC>,
    #[doc = "0x60 - encryption and decryption sram start address register"]
    pub ecdc_sram_addr: crate::Reg<ecdc_sram_addr::ECDC_SRAM_ADDR_SPEC>,
    #[doc = "0x64 - encryption and decryption sram size register"]
    pub ecdc_sram_len: crate::Reg<ecdc_sram_len::ECDC_SRAM_LEN_SPEC>,
}
#[doc = "ECEC_CTRL register accessor: an alias for `Reg<ECEC_CTRL_SPEC>`"]
pub type ECEC_CTRL = crate::Reg<ecec_ctrl::ECEC_CTRL_SPEC>;
#[doc = "ECED AES/SM4 register"]
pub mod ecec_ctrl;
#[doc = "ECDC_INT_EN register accessor: an alias for `Reg<ECDC_INT_EN_SPEC>`"]
pub type ECDC_INT_EN = crate::Reg<ecdc_int_en::ECDC_INT_EN_SPEC>;
#[doc = "Interupt enable register"]
pub mod ecdc_int_en;
#[doc = "ECDC_INT_FG register accessor: an alias for `Reg<ECDC_INT_FG_SPEC>`"]
pub type ECDC_INT_FG = crate::Reg<ecdc_int_fg::ECDC_INT_FG_SPEC>;
#[doc = "Interupt flag register"]
pub mod ecdc_int_fg;
#[doc = "ECDC_KEY_255T224 register accessor: an alias for `Reg<ECDC_KEY_255T224_SPEC>`"]
pub type ECDC_KEY_255T224 = crate::Reg<ecdc_key_255t224::ECDC_KEY_255T224_SPEC>;
#[doc = "User key 224-255 register"]
pub mod ecdc_key_255t224;
#[doc = "ECDC_KEY_223T192 register accessor: an alias for `Reg<ECDC_KEY_223T192_SPEC>`"]
pub type ECDC_KEY_223T192 = crate::Reg<ecdc_key_223t192::ECDC_KEY_223T192_SPEC>;
#[doc = "User key 192-223 register"]
pub mod ecdc_key_223t192;
#[doc = "ECDC_KEY_191T160 register accessor: an alias for `Reg<ECDC_KEY_191T160_SPEC>`"]
pub type ECDC_KEY_191T160 = crate::Reg<ecdc_key_191t160::ECDC_KEY_191T160_SPEC>;
#[doc = "User key 160-191 register"]
pub mod ecdc_key_191t160;
#[doc = "ECDC_KEY_159T128 register accessor: an alias for `Reg<ECDC_KEY_159T128_SPEC>`"]
pub type ECDC_KEY_159T128 = crate::Reg<ecdc_key_159t128::ECDC_KEY_159T128_SPEC>;
#[doc = "User key 128-159 register"]
pub mod ecdc_key_159t128;
#[doc = "ECDC_KEY_127T96 register accessor: an alias for `Reg<ECDC_KEY_127T96_SPEC>`"]
pub type ECDC_KEY_127T96 = crate::Reg<ecdc_key_127t96::ECDC_KEY_127T96_SPEC>;
#[doc = "User key 96-127 register"]
pub mod ecdc_key_127t96;
#[doc = "ECDC_KEY_95T64 register accessor: an alias for `Reg<ECDC_KEY_95T64_SPEC>`"]
pub type ECDC_KEY_95T64 = crate::Reg<ecdc_key_95t64::ECDC_KEY_95T64_SPEC>;
#[doc = "User key 64-95 register"]
pub mod ecdc_key_95t64;
#[doc = "ECDC_KEY_63T32 register accessor: an alias for `Reg<ECDC_KEY_63T32_SPEC>`"]
pub type ECDC_KEY_63T32 = crate::Reg<ecdc_key_63t32::ECDC_KEY_63T32_SPEC>;
#[doc = "User key 32-63 register"]
pub mod ecdc_key_63t32;
#[doc = "ECDC_KEY_31T0 register accessor: an alias for `Reg<ECDC_KEY_31T0_SPEC>`"]
pub type ECDC_KEY_31T0 = crate::Reg<ecdc_key_31t0::ECDC_KEY_31T0_SPEC>;
#[doc = "User key 0-31 register"]
pub mod ecdc_key_31t0;
#[doc = "ECDC_IV_127T96 register accessor: an alias for `Reg<ECDC_IV_127T96_SPEC>`"]
pub type ECDC_IV_127T96 = crate::Reg<ecdc_iv_127t96::ECDC_IV_127T96_SPEC>;
#[doc = "CTR mode count 96-127 register"]
pub mod ecdc_iv_127t96;
#[doc = "ECDC_IV_95T64 register accessor: an alias for `Reg<ECDC_IV_95T64_SPEC>`"]
pub type ECDC_IV_95T64 = crate::Reg<ecdc_iv_95t64::ECDC_IV_95T64_SPEC>;
#[doc = "CTR mode count 64-95 register"]
pub mod ecdc_iv_95t64;
#[doc = "ECDC_IV_63T32 register accessor: an alias for `Reg<ECDC_IV_63T32_SPEC>`"]
pub type ECDC_IV_63T32 = crate::Reg<ecdc_iv_63t32::ECDC_IV_63T32_SPEC>;
#[doc = "CTR mode count 32-63 register"]
pub mod ecdc_iv_63t32;
#[doc = "ECDC_IV_31T0 register accessor: an alias for `Reg<ECDC_IV_31T0_SPEC>`"]
pub type ECDC_IV_31T0 = crate::Reg<ecdc_iv_31t0::ECDC_IV_31T0_SPEC>;
#[doc = "CTR mode count 0-31 register"]
pub mod ecdc_iv_31t0;
#[doc = "ECDC_SGSD_127T96 register accessor: an alias for `Reg<ECDC_SGSD_127T96_SPEC>`"]
pub type ECDC_SGSD_127T96 = crate::Reg<ecdc_sgsd_127t96::ECDC_SGSD_127T96_SPEC>;
#[doc = "Single encryption and decryption of original data 96-127 register"]
pub mod ecdc_sgsd_127t96;
#[doc = "ECDC_SGSD_95T64 register accessor: an alias for `Reg<ECDC_SGSD_95T64_SPEC>`"]
pub type ECDC_SGSD_95T64 = crate::Reg<ecdc_sgsd_95t64::ECDC_SGSD_95T64_SPEC>;
#[doc = "Single encryption and decryption of original data 64-95 register"]
pub mod ecdc_sgsd_95t64;
#[doc = "ECDC_SGSD_63T32 register accessor: an alias for `Reg<ECDC_SGSD_63T32_SPEC>`"]
pub type ECDC_SGSD_63T32 = crate::Reg<ecdc_sgsd_63t32::ECDC_SGSD_63T32_SPEC>;
#[doc = "Single encryption and decryption of original data 32-63 register"]
pub mod ecdc_sgsd_63t32;
#[doc = "ECDC_SGSD_31T0 register accessor: an alias for `Reg<ECDC_SGSD_31T0_SPEC>`"]
pub type ECDC_SGSD_31T0 = crate::Reg<ecdc_sgsd_31t0::ECDC_SGSD_31T0_SPEC>;
#[doc = "Single encryption and decryption of original data 0-31 register"]
pub mod ecdc_sgsd_31t0;
#[doc = "ECDC_SGRT_127T96 register accessor: an alias for `Reg<ECDC_SGRT_127T96_SPEC>`"]
pub type ECDC_SGRT_127T96 = crate::Reg<ecdc_sgrt_127t96::ECDC_SGRT_127T96_SPEC>;
#[doc = "Single encryption and decryption result 96-127 register"]
pub mod ecdc_sgrt_127t96;
#[doc = "ECDC_SGRT_95T64 register accessor: an alias for `Reg<ECDC_SGRT_95T64_SPEC>`"]
pub type ECDC_SGRT_95T64 = crate::Reg<ecdc_sgrt_95t64::ECDC_SGRT_95T64_SPEC>;
#[doc = "Single encryption and decryption result 64-95 register"]
pub mod ecdc_sgrt_95t64;
#[doc = "ECDC_SGRT_63T32 register accessor: an alias for `Reg<ECDC_SGRT_63T32_SPEC>`"]
pub type ECDC_SGRT_63T32 = crate::Reg<ecdc_sgrt_63t32::ECDC_SGRT_63T32_SPEC>;
#[doc = "Single encryption and decryption result 0-31 register"]
pub mod ecdc_sgrt_63t32;
#[doc = "RB_ECDC_SGRT_31T0 register accessor: an alias for `Reg<RB_ECDC_SGRT_31T0_SPEC>`"]
pub type RB_ECDC_SGRT_31T0 = crate::Reg<rb_ecdc_sgrt_31t0::RB_ECDC_SGRT_31T0_SPEC>;
#[doc = "Single encryption and decryption result 0-31 register"]
pub mod rb_ecdc_sgrt_31t0;
#[doc = "ECDC_SRAM_ADDR register accessor: an alias for `Reg<ECDC_SRAM_ADDR_SPEC>`"]
pub type ECDC_SRAM_ADDR = crate::Reg<ecdc_sram_addr::ECDC_SRAM_ADDR_SPEC>;
#[doc = "encryption and decryption sram start address register"]
pub mod ecdc_sram_addr;
#[doc = "ECDC_SRAM_LEN register accessor: an alias for `Reg<ECDC_SRAM_LEN_SPEC>`"]
pub type ECDC_SRAM_LEN = crate::Reg<ecdc_sram_len::ECDC_SRAM_LEN_SPEC>;
#[doc = "encryption and decryption sram size register"]
pub mod ecdc_sram_len;

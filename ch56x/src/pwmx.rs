#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM mode control"]
    pub pwm_ctrl_mod: crate::Reg<pwm_ctrl_mod::PWM_CTRL_MOD_SPEC>,
    #[doc = "0x01 - PWM configuration control"]
    pub pwm_ctrl_cfg: crate::Reg<pwm_ctrl_cfg::PWM_CTRL_CFG_SPEC>,
    #[doc = "0x02 - PWM clock divisor"]
    pub pwm_clock_div: crate::Reg<pwm_clock_div::PWM_CLOCK_DIV_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - PWM data holding"]
    pub pwm0_data: crate::Reg<pwm0_data::PWM0_DATA_SPEC>,
    #[doc = "0x05 - PWM1 data holding"]
    pub pwm1_data: crate::Reg<pwm1_data::PWM1_DATA_SPEC>,
    #[doc = "0x06 - PWM2 data holding"]
    pub pwm2_data: crate::Reg<pwm2_data::PWM2_DATA_SPEC>,
    #[doc = "0x07 - PWM3 data holding"]
    pub pwm3_data: crate::Reg<pwm3_data::PWM3_DATA_SPEC>,
}
#[doc = "PWM_CTRL_MOD register accessor: an alias for `Reg<PWM_CTRL_MOD_SPEC>`"]
pub type PWM_CTRL_MOD = crate::Reg<pwm_ctrl_mod::PWM_CTRL_MOD_SPEC>;
#[doc = "PWM mode control"]
pub mod pwm_ctrl_mod;
#[doc = "PWM_CTRL_CFG register accessor: an alias for `Reg<PWM_CTRL_CFG_SPEC>`"]
pub type PWM_CTRL_CFG = crate::Reg<pwm_ctrl_cfg::PWM_CTRL_CFG_SPEC>;
#[doc = "PWM configuration control"]
pub mod pwm_ctrl_cfg;
#[doc = "PWM_CLOCK_DIV register accessor: an alias for `Reg<PWM_CLOCK_DIV_SPEC>`"]
pub type PWM_CLOCK_DIV = crate::Reg<pwm_clock_div::PWM_CLOCK_DIV_SPEC>;
#[doc = "PWM clock divisor"]
pub mod pwm_clock_div;
#[doc = "PWM0_DATA register accessor: an alias for `Reg<PWM0_DATA_SPEC>`"]
pub type PWM0_DATA = crate::Reg<pwm0_data::PWM0_DATA_SPEC>;
#[doc = "PWM data holding"]
pub mod pwm0_data;
#[doc = "PWM1_DATA register accessor: an alias for `Reg<PWM1_DATA_SPEC>`"]
pub type PWM1_DATA = crate::Reg<pwm1_data::PWM1_DATA_SPEC>;
#[doc = "PWM1 data holding"]
pub mod pwm1_data;
#[doc = "PWM2_DATA register accessor: an alias for `Reg<PWM2_DATA_SPEC>`"]
pub type PWM2_DATA = crate::Reg<pwm2_data::PWM2_DATA_SPEC>;
#[doc = "PWM2 data holding"]
pub mod pwm2_data;
#[doc = "PWM3_DATA register accessor: an alias for `Reg<PWM3_DATA_SPEC>`"]
pub type PWM3_DATA = crate::Reg<pwm3_data::PWM3_DATA_SPEC>;
#[doc = "PWM3 data holding"]
pub mod pwm3_data;

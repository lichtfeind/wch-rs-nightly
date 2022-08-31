#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - safe accessing sign register"]
    pub safe_access_sig: crate::Reg<safe_access_sig::SAFE_ACCESS_SIG_SPEC>,
    #[doc = "0x01 - chip ID register"]
    pub chip_id: crate::Reg<chip_id::CHIP_ID_SPEC>,
    #[doc = "0x02 - safe accessing ID register"]
    pub safe_access_id: crate::Reg<safe_access_id::SAFE_ACCESS_ID_SPEC>,
    #[doc = "0x03 - watch-dog count register"]
    pub wdog_count: crate::Reg<wdog_count::WDOG_COUNT_SPEC>,
    #[doc = "0x04 - flash ROM configuration register"]
    pub glob_rom_cfg: crate::Reg<glob_rom_cfg::GLOB_ROM_CFG_SPEC>,
    #[doc = "0x05 - reset status and boot/debug status"]
    pub rst_boot_stat: crate::Reg<rst_boot_stat::RST_BOOT_STAT_SPEC>,
    #[doc = "0x06 - reset and watch-dog control"]
    pub rst_wdog_ctrl: crate::Reg<rst_wdog_ctrl::RST_WDOG_CTRL_SPEC>,
    #[doc = "0x07 - value keeper during global reset"]
    pub glob_reset_keep: crate::Reg<glob_reset_keep::GLOB_RESET_KEEP_SPEC>,
    #[doc = "0x08 - output clock divider from PLL"]
    pub clk_pll_div: crate::Reg<clk_pll_div::CLK_PLL_DIV_SPEC>,
    _reserved9: [u8; 0x01],
    #[doc = "0x0a - clock control"]
    pub clk_cfg_ctrl: crate::Reg<clk_cfg_ctrl::CLK_CFG_CTRL_SPEC>,
    #[doc = "0x0b - clock mode aux register"]
    pub clk_mod_aux: crate::Reg<clk_mod_aux::CLK_MOD_AUX_SPEC>,
    #[doc = "0x0c - sleep clock off control byte 0"]
    pub slp_clk_off0: crate::Reg<slp_clk_off0::SLP_CLK_OFF0_SPEC>,
    #[doc = "0x0d - sleep clock off control byte 1"]
    pub slp_clk_off1: crate::Reg<slp_clk_off1::SLP_CLK_OFF1_SPEC>,
    #[doc = "0x0e - wake control"]
    pub slp_wake_ctrl: crate::Reg<slp_wake_ctrl::SLP_WAKE_CTRL_SPEC>,
    #[doc = "0x0f - power control"]
    pub slp_power_ctrl: crate::Reg<slp_power_ctrl::SLP_POWER_CTRL_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x12 - alternate pin control"]
    pub pin_alternate: crate::Reg<pin_alternate::PIN_ALTERNATE_SPEC>,
    _reserved16: [u8; 0x09],
    #[doc = "0x1c - GPIO interrupt control"]
    pub gpio_int_flag: crate::Reg<gpio_int_flag::GPIO_INT_FLAG_SPEC>,
    #[doc = "0x1d - GPIO interrupt enable"]
    pub gpio_int_enable: crate::Reg<gpio_int_enable::GPIO_INT_ENABLE_SPEC>,
    #[doc = "0x1e - GPIO interrupt mode"]
    pub gpio_int_mode: crate::Reg<gpio_int_mode::GPIO_INT_MODE_SPEC>,
    #[doc = "0x1f - GPIO interrupt polarity"]
    pub gpio_int_polar: crate::Reg<gpio_int_polar::GPIO_INT_POLAR_SPEC>,
    #[doc = "0x20 - Serdes Analog parameter configuration1"]
    pub serd_ana_cfg1: crate::Reg<serd_ana_cfg1::SERD_ANA_CFG1_SPEC>,
    _reserved21: [u8; 0x02],
    #[doc = "0x24 - Serdes Analog parameter configuration2"]
    pub serd_ana_cfg2: crate::Reg<serd_ana_cfg2::SERD_ANA_CFG2_SPEC>,
    _reserved22: [u8; 0x18],
    #[doc = "0x40 - GPIO PA I/O direction"]
    pub pa_dir: crate::Reg<pa_dir::PA_DIR_SPEC>,
    #[doc = "0x44 - GPIO PA input"]
    pub pa_pin: crate::Reg<pa_pin::PA_PIN_SPEC>,
    #[doc = "0x48 - GPIO PA output"]
    pub pa_out: crate::Reg<pa_out::PA_OUT_SPEC>,
    #[doc = "0x4c - GPIO PA clear output"]
    pub pa_clr: crate::Reg<pa_clr::PA_CLR_SPEC>,
    #[doc = "0x50 - GPIO PA pullup resistance enable"]
    pub pa_pu: crate::Reg<pa_pu::PA_PU_SPEC>,
    #[doc = "0x54 - GPIO PA output open-drain and input pulldown resistance enable"]
    pub pa_pd: crate::Reg<pa_pd::PA_PD_SPEC>,
    #[doc = "0x58 - GPIO PA driving capability"]
    pub pa_drv: crate::Reg<pa_drv::PA_DRV_SPEC>,
    #[doc = "0x5c - GPIO PA output slew rate and input schmitt trigger"]
    pub pa_smt: crate::Reg<pa_smt::PA_SMT_SPEC>,
    #[doc = "0x60 - GPIO PB I/O direction"]
    pub pb_dir: crate::Reg<pb_dir::PB_DIR_SPEC>,
    #[doc = "0x64 - GPIO PB input"]
    pub pb_pin: crate::Reg<pb_pin::PB_PIN_SPEC>,
    #[doc = "0x68 - GPIO PB output"]
    pub pb_out: crate::Reg<pb_out::PB_OUT_SPEC>,
    #[doc = "0x6c - GPIO PB clear output"]
    pub pb_clr: crate::Reg<pb_clr::PB_CLR_SPEC>,
    #[doc = "0x70 - GPIO PB pullup resistance enable"]
    pub pb_pu: crate::Reg<pb_pu::PB_PU_SPEC>,
    #[doc = "0x74 - GPIO PB output open-drain and input pulldown resistance enable"]
    pub pb_pd: crate::Reg<pb_pd::PB_PD_SPEC>,
    #[doc = "0x78 - GPIO PB driving capability"]
    pub pb_drv: crate::Reg<pb_drv::PB_DRV_SPEC>,
    #[doc = "0x7c - GPIO PB output slew rate and input schmitt trigger"]
    pub pb_smt: crate::Reg<pb_smt::PB_SMT_SPEC>,
}
#[doc = "SAFE_ACCESS_SIG register accessor: an alias for `Reg<SAFE_ACCESS_SIG_SPEC>`"]
pub type SAFE_ACCESS_SIG = crate::Reg<safe_access_sig::SAFE_ACCESS_SIG_SPEC>;
#[doc = "safe accessing sign register"]
pub mod safe_access_sig;
#[doc = "CHIP_ID register accessor: an alias for `Reg<CHIP_ID_SPEC>`"]
pub type CHIP_ID = crate::Reg<chip_id::CHIP_ID_SPEC>;
#[doc = "chip ID register"]
pub mod chip_id;
#[doc = "SAFE_ACCESS_ID register accessor: an alias for `Reg<SAFE_ACCESS_ID_SPEC>`"]
pub type SAFE_ACCESS_ID = crate::Reg<safe_access_id::SAFE_ACCESS_ID_SPEC>;
#[doc = "safe accessing ID register"]
pub mod safe_access_id;
#[doc = "WDOG_COUNT register accessor: an alias for `Reg<WDOG_COUNT_SPEC>`"]
pub type WDOG_COUNT = crate::Reg<wdog_count::WDOG_COUNT_SPEC>;
#[doc = "watch-dog count register"]
pub mod wdog_count;
#[doc = "GLOB_ROM_CFG register accessor: an alias for `Reg<GLOB_ROM_CFG_SPEC>`"]
pub type GLOB_ROM_CFG = crate::Reg<glob_rom_cfg::GLOB_ROM_CFG_SPEC>;
#[doc = "flash ROM configuration register"]
pub mod glob_rom_cfg;
#[doc = "RST_BOOT_STAT register accessor: an alias for `Reg<RST_BOOT_STAT_SPEC>`"]
pub type RST_BOOT_STAT = crate::Reg<rst_boot_stat::RST_BOOT_STAT_SPEC>;
#[doc = "reset status and boot/debug status"]
pub mod rst_boot_stat;
#[doc = "RST_WDOG_CTRL register accessor: an alias for `Reg<RST_WDOG_CTRL_SPEC>`"]
pub type RST_WDOG_CTRL = crate::Reg<rst_wdog_ctrl::RST_WDOG_CTRL_SPEC>;
#[doc = "reset and watch-dog control"]
pub mod rst_wdog_ctrl;
#[doc = "GLOB_RESET_KEEP register accessor: an alias for `Reg<GLOB_RESET_KEEP_SPEC>`"]
pub type GLOB_RESET_KEEP = crate::Reg<glob_reset_keep::GLOB_RESET_KEEP_SPEC>;
#[doc = "value keeper during global reset"]
pub mod glob_reset_keep;
#[doc = "CLK_PLL_DIV register accessor: an alias for `Reg<CLK_PLL_DIV_SPEC>`"]
pub type CLK_PLL_DIV = crate::Reg<clk_pll_div::CLK_PLL_DIV_SPEC>;
#[doc = "output clock divider from PLL"]
pub mod clk_pll_div;
#[doc = "CLK_CFG_CTRL register accessor: an alias for `Reg<CLK_CFG_CTRL_SPEC>`"]
pub type CLK_CFG_CTRL = crate::Reg<clk_cfg_ctrl::CLK_CFG_CTRL_SPEC>;
#[doc = "clock control"]
pub mod clk_cfg_ctrl;
#[doc = "CLK_MOD_AUX register accessor: an alias for `Reg<CLK_MOD_AUX_SPEC>`"]
pub type CLK_MOD_AUX = crate::Reg<clk_mod_aux::CLK_MOD_AUX_SPEC>;
#[doc = "clock mode aux register"]
pub mod clk_mod_aux;
#[doc = "SLP_CLK_OFF0 register accessor: an alias for `Reg<SLP_CLK_OFF0_SPEC>`"]
pub type SLP_CLK_OFF0 = crate::Reg<slp_clk_off0::SLP_CLK_OFF0_SPEC>;
#[doc = "sleep clock off control byte 0"]
pub mod slp_clk_off0;
#[doc = "SLP_CLK_OFF1 register accessor: an alias for `Reg<SLP_CLK_OFF1_SPEC>`"]
pub type SLP_CLK_OFF1 = crate::Reg<slp_clk_off1::SLP_CLK_OFF1_SPEC>;
#[doc = "sleep clock off control byte 1"]
pub mod slp_clk_off1;
#[doc = "SLP_WAKE_CTRL register accessor: an alias for `Reg<SLP_WAKE_CTRL_SPEC>`"]
pub type SLP_WAKE_CTRL = crate::Reg<slp_wake_ctrl::SLP_WAKE_CTRL_SPEC>;
#[doc = "wake control"]
pub mod slp_wake_ctrl;
#[doc = "SLP_POWER_CTRL register accessor: an alias for `Reg<SLP_POWER_CTRL_SPEC>`"]
pub type SLP_POWER_CTRL = crate::Reg<slp_power_ctrl::SLP_POWER_CTRL_SPEC>;
#[doc = "power control"]
pub mod slp_power_ctrl;
#[doc = "SERD_ANA_CFG1 register accessor: an alias for `Reg<SERD_ANA_CFG1_SPEC>`"]
pub type SERD_ANA_CFG1 = crate::Reg<serd_ana_cfg1::SERD_ANA_CFG1_SPEC>;
#[doc = "Serdes Analog parameter configuration1"]
pub mod serd_ana_cfg1;
#[doc = "SERD_ANA_CFG2 register accessor: an alias for `Reg<SERD_ANA_CFG2_SPEC>`"]
pub type SERD_ANA_CFG2 = crate::Reg<serd_ana_cfg2::SERD_ANA_CFG2_SPEC>;
#[doc = "Serdes Analog parameter configuration2"]
pub mod serd_ana_cfg2;
#[doc = "GPIO_INT_FLAG register accessor: an alias for `Reg<GPIO_INT_FLAG_SPEC>`"]
pub type GPIO_INT_FLAG = crate::Reg<gpio_int_flag::GPIO_INT_FLAG_SPEC>;
#[doc = "GPIO interrupt control"]
pub mod gpio_int_flag;
#[doc = "GPIO_INT_ENABLE register accessor: an alias for `Reg<GPIO_INT_ENABLE_SPEC>`"]
pub type GPIO_INT_ENABLE = crate::Reg<gpio_int_enable::GPIO_INT_ENABLE_SPEC>;
#[doc = "GPIO interrupt enable"]
pub mod gpio_int_enable;
#[doc = "GPIO_INT_MODE register accessor: an alias for `Reg<GPIO_INT_MODE_SPEC>`"]
pub type GPIO_INT_MODE = crate::Reg<gpio_int_mode::GPIO_INT_MODE_SPEC>;
#[doc = "GPIO interrupt mode"]
pub mod gpio_int_mode;
#[doc = "GPIO_INT_POLAR register accessor: an alias for `Reg<GPIO_INT_POLAR_SPEC>`"]
pub type GPIO_INT_POLAR = crate::Reg<gpio_int_polar::GPIO_INT_POLAR_SPEC>;
#[doc = "GPIO interrupt polarity"]
pub mod gpio_int_polar;
#[doc = "PA_DIR register accessor: an alias for `Reg<PA_DIR_SPEC>`"]
pub type PA_DIR = crate::Reg<pa_dir::PA_DIR_SPEC>;
#[doc = "GPIO PA I/O direction"]
pub mod pa_dir;
#[doc = "PA_PIN register accessor: an alias for `Reg<PA_PIN_SPEC>`"]
pub type PA_PIN = crate::Reg<pa_pin::PA_PIN_SPEC>;
#[doc = "GPIO PA input"]
pub mod pa_pin;
#[doc = "PA_OUT register accessor: an alias for `Reg<PA_OUT_SPEC>`"]
pub type PA_OUT = crate::Reg<pa_out::PA_OUT_SPEC>;
#[doc = "GPIO PA output"]
pub mod pa_out;
#[doc = "PA_CLR register accessor: an alias for `Reg<PA_CLR_SPEC>`"]
pub type PA_CLR = crate::Reg<pa_clr::PA_CLR_SPEC>;
#[doc = "GPIO PA clear output"]
pub mod pa_clr;
#[doc = "PA_PU register accessor: an alias for `Reg<PA_PU_SPEC>`"]
pub type PA_PU = crate::Reg<pa_pu::PA_PU_SPEC>;
#[doc = "GPIO PA pullup resistance enable"]
pub mod pa_pu;
#[doc = "PA_PD register accessor: an alias for `Reg<PA_PD_SPEC>`"]
pub type PA_PD = crate::Reg<pa_pd::PA_PD_SPEC>;
#[doc = "GPIO PA output open-drain and input pulldown resistance enable"]
pub mod pa_pd;
#[doc = "PA_DRV register accessor: an alias for `Reg<PA_DRV_SPEC>`"]
pub type PA_DRV = crate::Reg<pa_drv::PA_DRV_SPEC>;
#[doc = "GPIO PA driving capability"]
pub mod pa_drv;
#[doc = "PA_SMT register accessor: an alias for `Reg<PA_SMT_SPEC>`"]
pub type PA_SMT = crate::Reg<pa_smt::PA_SMT_SPEC>;
#[doc = "GPIO PA output slew rate and input schmitt trigger"]
pub mod pa_smt;
#[doc = "PB_DIR register accessor: an alias for `Reg<PB_DIR_SPEC>`"]
pub type PB_DIR = crate::Reg<pb_dir::PB_DIR_SPEC>;
#[doc = "GPIO PB I/O direction"]
pub mod pb_dir;
#[doc = "PB_PIN register accessor: an alias for `Reg<PB_PIN_SPEC>`"]
pub type PB_PIN = crate::Reg<pb_pin::PB_PIN_SPEC>;
#[doc = "GPIO PB input"]
pub mod pb_pin;
#[doc = "PB_OUT register accessor: an alias for `Reg<PB_OUT_SPEC>`"]
pub type PB_OUT = crate::Reg<pb_out::PB_OUT_SPEC>;
#[doc = "GPIO PB output"]
pub mod pb_out;
#[doc = "PB_CLR register accessor: an alias for `Reg<PB_CLR_SPEC>`"]
pub type PB_CLR = crate::Reg<pb_clr::PB_CLR_SPEC>;
#[doc = "GPIO PB clear output"]
pub mod pb_clr;
#[doc = "PB_PU register accessor: an alias for `Reg<PB_PU_SPEC>`"]
pub type PB_PU = crate::Reg<pb_pu::PB_PU_SPEC>;
#[doc = "GPIO PB pullup resistance enable"]
pub mod pb_pu;
#[doc = "PB_PD register accessor: an alias for `Reg<PB_PD_SPEC>`"]
pub type PB_PD = crate::Reg<pb_pd::PB_PD_SPEC>;
#[doc = "GPIO PB output open-drain and input pulldown resistance enable"]
pub mod pb_pd;
#[doc = "PB_DRV register accessor: an alias for `Reg<PB_DRV_SPEC>`"]
pub type PB_DRV = crate::Reg<pb_drv::PB_DRV_SPEC>;
#[doc = "GPIO PB driving capability"]
pub mod pb_drv;
#[doc = "PB_SMT register accessor: an alias for `Reg<PB_SMT_SPEC>`"]
pub type PB_SMT = crate::Reg<pb_smt::PB_SMT_SPEC>;
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub mod pb_smt;
#[doc = "PIN_ALTERNATE register accessor: an alias for `Reg<PIN_ALTERNATE_SPEC>`"]
pub type PIN_ALTERNATE = crate::Reg<pin_alternate::PIN_ALTERNATE_SPEC>;
#[doc = "alternate pin control"]
pub mod pin_alternate;

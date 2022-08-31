#[doc = "Register `SPI0_RUN_FLAG` reader"]
pub struct R(crate::R<SPI0_RUN_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI0_RUN_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI0_RUN_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI0_RUN_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_SPI_SLV_CMD_ACT` reader - SPI slave command flag"]
pub struct RB_SPI_SLV_CMD_ACT_R(crate::FieldReader<bool, bool>);
impl RB_SPI_SLV_CMD_ACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SPI_SLV_CMD_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SPI_SLV_CMD_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SPI_FIFO_READY` reader - SPI FIFO ready status"]
pub struct RB_SPI_FIFO_READY_R(crate::FieldReader<bool, bool>);
impl RB_SPI_FIFO_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SPI_FIFO_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SPI_FIFO_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SPI_SLV_CS_LOAD` reader - SPI slave chip-select loading status"]
pub struct RB_SPI_SLV_CS_LOAD_R(crate::FieldReader<bool, bool>);
impl RB_SPI_SLV_CS_LOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SPI_SLV_CS_LOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SPI_SLV_CS_LOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SPI_SLV_SELECT` reader - SPI slave selection status"]
pub struct RB_SPI_SLV_SELECT_R(crate::FieldReader<bool, bool>);
impl RB_SPI_SLV_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SPI_SLV_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SPI_SLV_SELECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - SPI slave command flag"]
    #[inline(always)]
    pub fn rb_spi_slv_cmd_act(&self) -> RB_SPI_SLV_CMD_ACT_R {
        RB_SPI_SLV_CMD_ACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI FIFO ready status"]
    #[inline(always)]
    pub fn rb_spi_fifo_ready(&self) -> RB_SPI_FIFO_READY_R {
        RB_SPI_FIFO_READY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI slave chip-select loading status"]
    #[inline(always)]
    pub fn rb_spi_slv_cs_load(&self) -> RB_SPI_SLV_CS_LOAD_R {
        RB_SPI_SLV_CS_LOAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI slave selection status"]
    #[inline(always)]
    pub fn rb_spi_slv_select(&self) -> RB_SPI_SLV_SELECT_R {
        RB_SPI_SLV_SELECT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "SPI0 work flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_run_flag](index.html) module"]
pub struct SPI0_RUN_FLAG_SPEC;
impl crate::RegisterSpec for SPI0_RUN_FLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spi0_run_flag::R](R) reader structure"]
impl crate::Readable for SPI0_RUN_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI0_RUN_FLAG to value 0"]
impl crate::Resettable for SPI0_RUN_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `DVP_FIFO_ST` reader"]
pub struct R(crate::R<DVP_FIFO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_FIFO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_FIFO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_FIFO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_DVP_FIFO_RDY` reader - DVP receive fifo ready"]
pub struct RB_DVP_FIFO_RDY_R(crate::FieldReader<bool, bool>);
impl RB_DVP_FIFO_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_FIFO_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_FIFO_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_FIFO_FULL` reader - DVP receive fifo full"]
pub struct RB_DVP_FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl RB_DVP_FIFO_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_FIFO_OV` reader - DVP receive fifo overflow"]
pub struct RB_DVP_FIFO_OV_R(crate::FieldReader<bool, bool>);
impl RB_DVP_FIFO_OV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_FIFO_OV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_FIFO_OV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_MSK_FIFO_CNT` reader - DVP receive fifo count"]
pub struct RB_DVP_MSK_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl RB_DVP_MSK_FIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_DVP_MSK_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_MSK_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DVP receive fifo ready"]
    #[inline(always)]
    pub fn rb_dvp_fifo_rdy(&self) -> RB_DVP_FIFO_RDY_R {
        RB_DVP_FIFO_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP receive fifo full"]
    #[inline(always)]
    pub fn rb_dvp_fifo_full(&self) -> RB_DVP_FIFO_FULL_R {
        RB_DVP_FIFO_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP receive fifo overflow"]
    #[inline(always)]
    pub fn rb_dvp_fifo_ov(&self) -> RB_DVP_FIFO_OV_R {
        RB_DVP_FIFO_OV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - DVP receive fifo count"]
    #[inline(always)]
    pub fn rb_dvp_msk_fifo_cnt(&self) -> RB_DVP_MSK_FIFO_CNT_R {
        RB_DVP_MSK_FIFO_CNT_R::new(((self.bits >> 4) & 7) as u8)
    }
}
#[doc = "DVP receive fifo status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_fifo_st](index.html) module"]
pub struct DVP_FIFO_ST_SPEC;
impl crate::RegisterSpec for DVP_FIFO_ST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dvp_fifo_st::R](R) reader structure"]
impl crate::Readable for DVP_FIFO_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DVP_FIFO_ST to value 0"]
impl crate::Resettable for DVP_FIFO_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

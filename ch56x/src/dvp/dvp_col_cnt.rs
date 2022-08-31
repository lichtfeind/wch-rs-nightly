#[doc = "Register `DVP_COL_CNT` reader"]
pub struct R(crate::R<DVP_COL_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_COL_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_COL_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_COL_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_DVP_COL_CNT` reader - DVP receive fifo ready"]
pub struct RB_DVP_COL_CNT_R(crate::FieldReader<u16, u16>);
impl RB_DVP_COL_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RB_DVP_COL_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_COL_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - DVP receive fifo ready"]
    #[inline(always)]
    pub fn rb_dvp_col_cnt(&self) -> RB_DVP_COL_CNT_R {
        RB_DVP_COL_CNT_R::new(self.bits)
    }
}
#[doc = "DVP col count value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_col_cnt](index.html) module"]
pub struct DVP_COL_CNT_SPEC;
impl crate::RegisterSpec for DVP_COL_CNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dvp_col_cnt::R](R) reader structure"]
impl crate::Readable for DVP_COL_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DVP_COL_CNT to value 0"]
impl crate::Resettable for DVP_COL_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

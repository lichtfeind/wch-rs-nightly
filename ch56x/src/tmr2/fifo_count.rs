#[doc = "Register `FIFO_COUNT` reader"]
pub struct R(crate::R<FIFO_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_COUNT` reader - TMR FIFO count status"]
pub struct FIFO_COUNT_R(crate::FieldReader<u8, u8>);
impl FIFO_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - TMR FIFO count status"]
    #[inline(always)]
    pub fn fifo_count(&self) -> FIFO_COUNT_R {
        FIFO_COUNT_R::new(self.bits)
    }
}
#[doc = "TMR2 FIFO count status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_count](index.html) module"]
pub struct FIFO_COUNT_SPEC;
impl crate::RegisterSpec for FIFO_COUNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fifo_count::R](R) reader structure"]
impl crate::Readable for FIFO_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFO_COUNT to value 0"]
impl crate::Resettable for FIFO_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

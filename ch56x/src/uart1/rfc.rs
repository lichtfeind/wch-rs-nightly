#[doc = "Register `RFC` reader"]
pub struct R(crate::R<RFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFC` reader - UART receiver FIFO count"]
pub struct RFC_R(crate::FieldReader<u8, u8>);
impl RFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - UART receiver FIFO count"]
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new(self.bits)
    }
}
#[doc = "UART1 receiver FIFO count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfc](index.html) module"]
pub struct RFC_SPEC;
impl crate::RegisterSpec for RFC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rfc::R](R) reader structure"]
impl crate::Readable for RFC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFC to value 0"]
impl crate::Resettable for RFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

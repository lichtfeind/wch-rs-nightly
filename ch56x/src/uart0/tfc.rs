#[doc = "Register `TFC` reader"]
pub struct R(crate::R<TFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFC` reader - UART transmitter FIFO count"]
pub struct TFC_R(crate::FieldReader<u8, u8>);
impl TFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - UART transmitter FIFO count"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(self.bits)
    }
}
#[doc = "UART0 transmitter FIFO count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfc](index.html) module"]
pub struct TFC_SPEC;
impl crate::RegisterSpec for TFC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tfc::R](R) reader structure"]
impl crate::Readable for TFC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TFC to value 0"]
impl crate::Resettable for TFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `SPI0_BUFFER` reader"]
pub struct R(crate::R<SPI0_BUFFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI0_BUFFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI0_BUFFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI0_BUFFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI0_BUFFER` reader - SPI data buffer"]
pub struct SPI0_BUFFER_R(crate::FieldReader<u8, u8>);
impl SPI0_BUFFER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI0_BUFFER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_BUFFER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI data buffer"]
    #[inline(always)]
    pub fn spi0_buffer(&self) -> SPI0_BUFFER_R {
        SPI0_BUFFER_R::new(self.bits)
    }
}
#[doc = "SPI0 data buffer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_buffer](index.html) module"]
pub struct SPI0_BUFFER_SPEC;
impl crate::RegisterSpec for SPI0_BUFFER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spi0_buffer::R](R) reader structure"]
impl crate::Readable for SPI0_BUFFER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI0_BUFFER to value 0"]
impl crate::Resettable for SPI0_BUFFER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

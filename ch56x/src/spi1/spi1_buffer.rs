#[doc = "Register `SPI1_BUFFER` reader"]
pub struct R(crate::R<SPI1_BUFFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI1_BUFFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI1_BUFFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI1_BUFFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI1_BUFFER` reader - SPI data buffer"]
pub struct SPI1_BUFFER_R(crate::FieldReader<u8, u8>);
impl SPI1_BUFFER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI1_BUFFER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1_BUFFER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI data buffer"]
    #[inline(always)]
    pub fn spi1_buffer(&self) -> SPI1_BUFFER_R {
        SPI1_BUFFER_R::new(self.bits)
    }
}
#[doc = "SPI1 data buffer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_buffer](index.html) module"]
pub struct SPI1_BUFFER_SPEC;
impl crate::RegisterSpec for SPI1_BUFFER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spi1_buffer::R](R) reader structure"]
impl crate::Readable for SPI1_BUFFER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI1_BUFFER to value 0"]
impl crate::Resettable for SPI1_BUFFER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

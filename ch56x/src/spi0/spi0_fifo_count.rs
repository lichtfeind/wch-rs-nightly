#[doc = "Register `SPI0_FIFO_COUNT` reader"]
pub struct R(crate::R<SPI0_FIFO_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI0_FIFO_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI0_FIFO_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI0_FIFO_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI0_FIFO_COUNT` writer"]
pub struct W(crate::W<SPI0_FIFO_COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI0_FIFO_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI0_FIFO_COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI0_FIFO_COUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0_FIFO_COUNT` reader - SPI FIFO count status"]
pub struct SPI0_FIFO_COUNT_R(crate::FieldReader<u8, u8>);
impl SPI0_FIFO_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI0_FIFO_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_FIFO_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0_FIFO_COUNT` writer - SPI FIFO count status"]
pub struct SPI0_FIFO_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_FIFO_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI FIFO count status"]
    #[inline(always)]
    pub fn spi0_fifo_count(&self) -> SPI0_FIFO_COUNT_R {
        SPI0_FIFO_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI FIFO count status"]
    #[inline(always)]
    pub fn spi0_fifo_count(&mut self) -> SPI0_FIFO_COUNT_W {
        SPI0_FIFO_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 FIFO count status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_fifo_count](index.html) module"]
pub struct SPI0_FIFO_COUNT_SPEC;
impl crate::RegisterSpec for SPI0_FIFO_COUNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spi0_fifo_count::R](R) reader structure"]
impl crate::Readable for SPI0_FIFO_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi0_fifo_count::W](W) writer structure"]
impl crate::Writable for SPI0_FIFO_COUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI0_FIFO_COUNT to value 0"]
impl crate::Resettable for SPI0_FIFO_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

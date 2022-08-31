#[doc = "Register `SPI0_DMA_NOW` reader"]
pub struct R(crate::R<SPI0_DMA_NOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI0_DMA_NOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI0_DMA_NOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI0_DMA_NOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI0_DMA_NOW` writer"]
pub struct W(crate::W<SPI0_DMA_NOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI0_DMA_NOW_SPEC>;
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
impl From<crate::W<SPI0_DMA_NOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI0_DMA_NOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0_DMA_NOW` reader - SPI DMA current address"]
pub struct SPI0_DMA_NOW_R(crate::FieldReader<u32, u32>);
impl SPI0_DMA_NOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SPI0_DMA_NOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_DMA_NOW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0_DMA_NOW` writer - SPI DMA current address"]
pub struct SPI0_DMA_NOW_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_DMA_NOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - SPI DMA current address"]
    #[inline(always)]
    pub fn spi0_dma_now(&self) -> SPI0_DMA_NOW_R {
        SPI0_DMA_NOW_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - SPI DMA current address"]
    #[inline(always)]
    pub fn spi0_dma_now(&mut self) -> SPI0_DMA_NOW_W {
        SPI0_DMA_NOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 DMA current address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_dma_now](index.html) module"]
pub struct SPI0_DMA_NOW_SPEC;
impl crate::RegisterSpec for SPI0_DMA_NOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi0_dma_now::R](R) reader structure"]
impl crate::Readable for SPI0_DMA_NOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi0_dma_now::W](W) writer structure"]
impl crate::Writable for SPI0_DMA_NOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI0_DMA_NOW to value 0"]
impl crate::Resettable for SPI0_DMA_NOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

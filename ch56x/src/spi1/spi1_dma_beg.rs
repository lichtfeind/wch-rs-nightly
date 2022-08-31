#[doc = "Register `SPI1_DMA_BEG` reader"]
pub struct R(crate::R<SPI1_DMA_BEG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI1_DMA_BEG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI1_DMA_BEG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI1_DMA_BEG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI1_DMA_BEG` writer"]
pub struct W(crate::W<SPI1_DMA_BEG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI1_DMA_BEG_SPEC>;
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
impl From<crate::W<SPI1_DMA_BEG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI1_DMA_BEG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1_DMA_BEG` reader - SPI DMA begin address"]
pub struct SPI1_DMA_BEG_R(crate::FieldReader<u32, u32>);
impl SPI1_DMA_BEG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SPI1_DMA_BEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1_DMA_BEG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1_DMA_BEG` writer - SPI DMA begin address"]
pub struct SPI1_DMA_BEG_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_DMA_BEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - SPI DMA begin address"]
    #[inline(always)]
    pub fn spi1_dma_beg(&self) -> SPI1_DMA_BEG_R {
        SPI1_DMA_BEG_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - SPI DMA begin address"]
    #[inline(always)]
    pub fn spi1_dma_beg(&mut self) -> SPI1_DMA_BEG_W {
        SPI1_DMA_BEG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 DMA begin address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_dma_beg](index.html) module"]
pub struct SPI1_DMA_BEG_SPEC;
impl crate::RegisterSpec for SPI1_DMA_BEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi1_dma_beg::R](R) reader structure"]
impl crate::Readable for SPI1_DMA_BEG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi1_dma_beg::W](W) writer structure"]
impl crate::Writable for SPI1_DMA_BEG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI1_DMA_BEG to value 0"]
impl crate::Resettable for SPI1_DMA_BEG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

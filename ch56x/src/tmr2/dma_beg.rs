#[doc = "Register `DMA_BEG` reader"]
pub struct R(crate::R<DMA_BEG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_BEG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_BEG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_BEG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_BEG` writer"]
pub struct W(crate::W<DMA_BEG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_BEG_SPEC>;
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
impl From<crate::W<DMA_BEG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_BEG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_BEG` reader - TMR2 DMA begin address"]
pub struct DMA_BEG_R(crate::FieldReader<u32, u32>);
impl DMA_BEG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DMA_BEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_BEG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_BEG` writer - TMR2 DMA begin address"]
pub struct DMA_BEG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - TMR2 DMA begin address"]
    #[inline(always)]
    pub fn dma_beg(&self) -> DMA_BEG_R {
        DMA_BEG_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - TMR2 DMA begin address"]
    #[inline(always)]
    pub fn dma_beg(&mut self) -> DMA_BEG_W {
        DMA_BEG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR2 DMA begin address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_beg](index.html) module"]
pub struct DMA_BEG_SPEC;
impl crate::RegisterSpec for DMA_BEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_beg::R](R) reader structure"]
impl crate::Readable for DMA_BEG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_beg::W](W) writer structure"]
impl crate::Writable for DMA_BEG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_BEG to value 0"]
impl crate::Resettable for DMA_BEG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `EMMC_DMA_BEG2` reader"]
pub struct R(crate::R<EMMC_DMA_BEG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_DMA_BEG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_DMA_BEG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_DMA_BEG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_DMA_BEG2` writer"]
pub struct W(crate::W<EMMC_DMA_BEG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_DMA_BEG2_SPEC>;
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
impl From<crate::W<EMMC_DMA_BEG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_DMA_BEG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_DMAAD2_MASK` reader - block DMA start address register"]
pub struct RB_EMMC_DMAAD2_MASK_R(crate::FieldReader<u32, u32>);
impl RB_EMMC_DMAAD2_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RB_EMMC_DMAAD2_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_DMAAD2_MASK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_DMAAD2_MASK` writer - block DMA start address register"]
pub struct RB_EMMC_DMAAD2_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_DMAAD2_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - block DMA start address register"]
    #[inline(always)]
    pub fn rb_emmc_dmaad2_mask(&self) -> RB_EMMC_DMAAD2_MASK_R {
        RB_EMMC_DMAAD2_MASK_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - block DMA start address register"]
    #[inline(always)]
    pub fn rb_emmc_dmaad2_mask(&mut self) -> RB_EMMC_DMAAD2_MASK_W {
        RB_EMMC_DMAAD2_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 16bits DMA start address register when to operate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_dma_beg2](index.html) module"]
pub struct EMMC_DMA_BEG2_SPEC;
impl crate::RegisterSpec for EMMC_DMA_BEG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc_dma_beg2::R](R) reader structure"]
impl crate::Readable for EMMC_DMA_BEG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_dma_beg2::W](W) writer structure"]
impl crate::Writable for EMMC_DMA_BEG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_DMA_BEG2 to value 0"]
impl crate::Resettable for EMMC_DMA_BEG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

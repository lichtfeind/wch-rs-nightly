#[doc = "Register `HSPI_DMA_LEN1` reader"]
pub struct R(crate::R<HSPI_DMA_LEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_DMA_LEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_DMA_LEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_DMA_LEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_DMA_LEN1` writer"]
pub struct W(crate::W<HSPI_DMA_LEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_DMA_LEN1_SPEC>;
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
impl From<crate::W<HSPI_DMA_LEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_DMA_LEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_DMA_LEN1` reader - parallel if dma length1"]
pub struct RB_HSPI_DMA_LEN1_R(crate::FieldReader<u16, u16>);
impl RB_HSPI_DMA_LEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RB_HSPI_DMA_LEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_DMA_LEN1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_DMA_LEN1` writer - parallel if dma length1"]
pub struct RB_HSPI_DMA_LEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_DMA_LEN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u16 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - parallel if dma length1"]
    #[inline(always)]
    pub fn rb_hspi_dma_len1(&self) -> RB_HSPI_DMA_LEN1_R {
        RB_HSPI_DMA_LEN1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - parallel if dma length1"]
    #[inline(always)]
    pub fn rb_hspi_dma_len1(&mut self) -> RB_HSPI_DMA_LEN1_W {
        RB_HSPI_DMA_LEN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if dma length1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_dma_len1](index.html) module"]
pub struct HSPI_DMA_LEN1_SPEC;
impl crate::RegisterSpec for HSPI_DMA_LEN1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hspi_dma_len1::R](R) reader structure"]
impl crate::Readable for HSPI_DMA_LEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_dma_len1::W](W) writer structure"]
impl crate::Writable for HSPI_DMA_LEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_DMA_LEN1 to value 0"]
impl crate::Resettable for HSPI_DMA_LEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

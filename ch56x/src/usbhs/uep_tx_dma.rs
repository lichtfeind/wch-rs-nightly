#[doc = "Register `UEP%s_TX_DMA` reader"]
pub struct R(crate::R<UEP_TX_DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP_TX_DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP_TX_DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP_TX_DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP%s_TX_DMA` writer"]
pub struct W(crate::W<UEP_TX_DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP_TX_DMA_SPEC>;
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
impl From<crate::W<UEP_TX_DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP_TX_DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP1_TX_DMA` reader - endpoint 1 DMA TX buffer address"]
pub struct UEP1_TX_DMA_R(crate::FieldReader<u32, u32>);
impl UEP1_TX_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        UEP1_TX_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UEP1_TX_DMA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UEP1_TX_DMA` writer - endpoint 1 DMA TX buffer address"]
pub struct UEP1_TX_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> UEP1_TX_DMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep1_tx_dma(&self) -> UEP1_TX_DMA_R {
        UEP1_TX_DMA_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - endpoint 1 DMA TX buffer address"]
    #[inline(always)]
    pub fn uep1_tx_dma(&mut self) -> UEP1_TX_DMA_W {
        UEP1_TX_DMA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 1 DMA TX buffer address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep_tx_dma](index.html) module"]
pub struct UEP_TX_DMA_SPEC;
impl crate::RegisterSpec for UEP_TX_DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uep_tx_dma::R](R) reader structure"]
impl crate::Readable for UEP_TX_DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep_tx_dma::W](W) writer structure"]
impl crate::Writable for UEP_TX_DMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP%s_TX_DMA to value 0"]
impl crate::Resettable for UEP_TX_DMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

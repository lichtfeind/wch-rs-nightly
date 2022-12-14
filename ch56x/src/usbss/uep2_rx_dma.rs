#[doc = "Register `UEP2_RX_DMA` reader"]
pub struct R(crate::R<UEP2_RX_DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP2_RX_DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP2_RX_DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP2_RX_DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP2_RX_DMA` writer"]
pub struct W(crate::W<UEP2_RX_DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP2_RX_DMA_SPEC>;
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
impl From<crate::W<UEP2_RX_DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP2_RX_DMA_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep2_rx_dma](index.html) module"]
pub struct UEP2_RX_DMA_SPEC;
impl crate::RegisterSpec for UEP2_RX_DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uep2_rx_dma::R](R) reader structure"]
impl crate::Readable for UEP2_RX_DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep2_rx_dma::W](W) writer structure"]
impl crate::Writable for UEP2_RX_DMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP2_RX_DMA to value 0"]
impl crate::Resettable for UEP2_RX_DMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

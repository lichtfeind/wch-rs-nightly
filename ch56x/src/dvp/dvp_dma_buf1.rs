#[doc = "Register `DVP_DMA_BUF1` reader"]
pub struct R(crate::R<DVP_DMA_BUF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_DMA_BUF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_DMA_BUF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_DMA_BUF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVP_DMA_BUF1` writer"]
pub struct W(crate::W<DVP_DMA_BUF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP_DMA_BUF1_SPEC>;
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
impl From<crate::W<DVP_DMA_BUF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP_DMA_BUF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_DMA_BUF1` reader - the receiving address1 of DMA"]
pub struct RB_DVP_DMA_BUF1_R(crate::FieldReader<u32, u32>);
impl RB_DVP_DMA_BUF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RB_DVP_DMA_BUF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_DMA_BUF1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_DMA_BUF1` writer - the receiving address1 of DMA"]
pub struct RB_DVP_DMA_BUF1_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_DMA_BUF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - the receiving address1 of DMA"]
    #[inline(always)]
    pub fn rb_dvp_dma_buf1(&self) -> RB_DVP_DMA_BUF1_R {
        RB_DVP_DMA_BUF1_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - the receiving address1 of DMA"]
    #[inline(always)]
    pub fn rb_dvp_dma_buf1(&mut self) -> RB_DVP_DMA_BUF1_W {
        RB_DVP_DMA_BUF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DVP dma buffer1 addr\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_dma_buf1](index.html) module"]
pub struct DVP_DMA_BUF1_SPEC;
impl crate::RegisterSpec for DVP_DMA_BUF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp_dma_buf1::R](R) reader structure"]
impl crate::Readable for DVP_DMA_BUF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp_dma_buf1::W](W) writer structure"]
impl crate::Writable for DVP_DMA_BUF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DVP_DMA_BUF1 to value 0"]
impl crate::Resettable for DVP_DMA_BUF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

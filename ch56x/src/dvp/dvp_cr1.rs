#[doc = "Register `DVP_CR1` reader"]
pub struct R(crate::R<DVP_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVP_CR1` writer"]
pub struct W(crate::W<DVP_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP_CR1_SPEC>;
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
impl From<crate::W<DVP_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_DMA_ENABLE` reader - DVP dma enable"]
pub struct RB_DVP_DMA_ENABLE_R(crate::FieldReader<bool, bool>);
impl RB_DVP_DMA_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_DMA_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_DMA_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_DMA_ENABLE` writer - DVP dma enable"]
pub struct RB_DVP_DMA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_DMA_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u8 & 1);
        self.w
    }
}
#[doc = "Field `RB_DVP_ALL_CLR` reader - DVP all clear, high action"]
pub struct RB_DVP_ALL_CLR_R(crate::FieldReader<bool, bool>);
impl RB_DVP_ALL_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_ALL_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_ALL_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_ALL_CLR` writer - DVP all clear, high action"]
pub struct RB_DVP_ALL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_ALL_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u8 & 1) << 1);
        self.w
    }
}
#[doc = "Field `RB_DVP_RCV_CLR` reader - DVP receive logic clear, high action"]
pub struct RB_DVP_RCV_CLR_R(crate::FieldReader<bool, bool>);
impl RB_DVP_RCV_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_RCV_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_RCV_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_RCV_CLR` writer - DVP receive logic clear, high action"]
pub struct RB_DVP_RCV_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_RCV_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u8 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RB_DVP_BUF_TOG` reader - DVP bug toggle by software"]
pub struct RB_DVP_BUF_TOG_R(crate::FieldReader<bool, bool>);
impl RB_DVP_BUF_TOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_BUF_TOG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_BUF_TOG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_BUF_TOG` writer - DVP bug toggle by software"]
pub struct RB_DVP_BUF_TOG_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_BUF_TOG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u8 & 1) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DVP dma enable"]
    #[inline(always)]
    pub fn rb_dvp_dma_enable(&self) -> RB_DVP_DMA_ENABLE_R {
        RB_DVP_DMA_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP all clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_all_clr(&self) -> RB_DVP_ALL_CLR_R {
        RB_DVP_ALL_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP receive logic clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_rcv_clr(&self) -> RB_DVP_RCV_CLR_R {
        RB_DVP_RCV_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP bug toggle by software"]
    #[inline(always)]
    pub fn rb_dvp_buf_tog(&self) -> RB_DVP_BUF_TOG_R {
        RB_DVP_BUF_TOG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP dma enable"]
    #[inline(always)]
    pub fn rb_dvp_dma_enable(&mut self) -> RB_DVP_DMA_ENABLE_W {
        RB_DVP_DMA_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - DVP all clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_all_clr(&mut self) -> RB_DVP_ALL_CLR_W {
        RB_DVP_ALL_CLR_W { w: self }
    }
    #[doc = "Bit 2 - DVP receive logic clear, high action"]
    #[inline(always)]
    pub fn rb_dvp_rcv_clr(&mut self) -> RB_DVP_RCV_CLR_W {
        RB_DVP_RCV_CLR_W { w: self }
    }
    #[doc = "Bit 3 - DVP bug toggle by software"]
    #[inline(always)]
    pub fn rb_dvp_buf_tog(&mut self) -> RB_DVP_BUF_TOG_W {
        RB_DVP_BUF_TOG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DVP control register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_cr1](index.html) module"]
pub struct DVP_CR1_SPEC;
impl crate::RegisterSpec for DVP_CR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dvp_cr1::R](R) reader structure"]
impl crate::Readable for DVP_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp_cr1::W](W) writer structure"]
impl crate::Writable for DVP_CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DVP_CR1 to value 0x06"]
impl crate::Resettable for DVP_CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}

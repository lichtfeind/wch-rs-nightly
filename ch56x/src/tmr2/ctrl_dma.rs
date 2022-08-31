#[doc = "Register `CTRL_DMA` reader"]
pub struct R(crate::R<CTRL_DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_DMA` writer"]
pub struct W(crate::W<CTRL_DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_DMA_SPEC>;
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
impl From<crate::W<CTRL_DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_ENABLE` reader - timer1_2 DMA enable"]
pub struct DMA_ENABLE_R(crate::FieldReader<bool, bool>);
impl DMA_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_ENABLE` writer - timer1_2 DMA enable"]
pub struct DMA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENABLE_W<'a> {
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
#[doc = "Field `DMA_LOOP` reader - timer1_2 DMA address loop enable"]
pub struct DMA_LOOP_R(crate::FieldReader<bool, bool>);
impl DMA_LOOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_LOOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_LOOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_LOOP` writer - timer1_2 DMA address loop enable"]
pub struct DMA_LOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_LOOP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - timer1_2 DMA enable"]
    #[inline(always)]
    pub fn dma_enable(&self) -> DMA_ENABLE_R {
        DMA_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - timer1_2 DMA address loop enable"]
    #[inline(always)]
    pub fn dma_loop(&self) -> DMA_LOOP_R {
        DMA_LOOP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - timer1_2 DMA enable"]
    #[inline(always)]
    pub fn dma_enable(&mut self) -> DMA_ENABLE_W {
        DMA_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - timer1_2 DMA address loop enable"]
    #[inline(always)]
    pub fn dma_loop(&mut self) -> DMA_LOOP_W {
        DMA_LOOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR2 DMA control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_dma](index.html) module"]
pub struct CTRL_DMA_SPEC;
impl crate::RegisterSpec for CTRL_DMA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrl_dma::R](R) reader structure"]
impl crate::Readable for CTRL_DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_dma::W](W) writer structure"]
impl crate::Writable for CTRL_DMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_DMA to value 0"]
impl crate::Resettable for CTRL_DMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

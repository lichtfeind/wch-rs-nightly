#[doc = "Register `INTER_EN` reader"]
pub struct R(crate::R<INTER_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTER_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTER_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTER_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTER_EN` writer"]
pub struct W(crate::W<INTER_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTER_EN_SPEC>;
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
impl From<crate::W<INTER_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTER_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE_CYC_END` reader - enable interrupt for timer capture count timeout or PWM cycle end"]
pub struct IE_CYC_END_R(crate::FieldReader<bool, bool>);
impl IE_CYC_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IE_CYC_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_CYC_END_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE_CYC_END` writer - enable interrupt for timer capture count timeout or PWM cycle end"]
pub struct IE_CYC_END_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_CYC_END_W<'a> {
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
#[doc = "Field `IE_DATA_ACT` reader - enable interrupt for timer capture input action or PWM trigger"]
pub struct IE_DATA_ACT_R(crate::FieldReader<bool, bool>);
impl IE_DATA_ACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IE_DATA_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_DATA_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE_DATA_ACT` writer - enable interrupt for timer capture input action or PWM trigger"]
pub struct IE_DATA_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_DATA_ACT_W<'a> {
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
#[doc = "Field `IE_FIFO_HF` reader - enable interrupt for timer FIFO half (capture fifo >=4 or PWM fifo lower than3)"]
pub struct IE_FIFO_HF_R(crate::FieldReader<bool, bool>);
impl IE_FIFO_HF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IE_FIFO_HF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_FIFO_HF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE_FIFO_HF` writer - enable interrupt for timer FIFO half (capture fifo >=4 or PWM fifo lower than3)"]
pub struct IE_FIFO_HF_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_FIFO_HF_W<'a> {
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
#[doc = "Field `IE_DMA_END` reader - enable interrupt for timer1/2 DMA completion"]
pub struct IE_DMA_END_R(crate::FieldReader<bool, bool>);
impl IE_DMA_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IE_DMA_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_DMA_END_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE_DMA_END` writer - enable interrupt for timer1/2 DMA completion"]
pub struct IE_DMA_END_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_DMA_END_W<'a> {
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
#[doc = "Field `IE_FIFO_OV` reader - enable interrupt for timer FIFO overflow"]
pub struct IE_FIFO_OV_R(crate::FieldReader<bool, bool>);
impl IE_FIFO_OV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IE_FIFO_OV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_FIFO_OV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE_FIFO_OV` writer - enable interrupt for timer FIFO overflow"]
pub struct IE_FIFO_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_FIFO_OV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u8 & 1) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - enable interrupt for timer capture count timeout or PWM cycle end"]
    #[inline(always)]
    pub fn ie_cyc_end(&self) -> IE_CYC_END_R {
        IE_CYC_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for timer capture input action or PWM trigger"]
    #[inline(always)]
    pub fn ie_data_act(&self) -> IE_DATA_ACT_R {
        IE_DATA_ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for timer FIFO half (capture fifo >=4 or PWM fifo lower than3)"]
    #[inline(always)]
    pub fn ie_fifo_hf(&self) -> IE_FIFO_HF_R {
        IE_FIFO_HF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable interrupt for timer1/2 DMA completion"]
    #[inline(always)]
    pub fn ie_dma_end(&self) -> IE_DMA_END_R {
        IE_DMA_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for timer FIFO overflow"]
    #[inline(always)]
    pub fn ie_fifo_ov(&self) -> IE_FIFO_OV_R {
        IE_FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for timer capture count timeout or PWM cycle end"]
    #[inline(always)]
    pub fn ie_cyc_end(&mut self) -> IE_CYC_END_W {
        IE_CYC_END_W { w: self }
    }
    #[doc = "Bit 1 - enable interrupt for timer capture input action or PWM trigger"]
    #[inline(always)]
    pub fn ie_data_act(&mut self) -> IE_DATA_ACT_W {
        IE_DATA_ACT_W { w: self }
    }
    #[doc = "Bit 2 - enable interrupt for timer FIFO half (capture fifo >=4 or PWM fifo lower than3)"]
    #[inline(always)]
    pub fn ie_fifo_hf(&mut self) -> IE_FIFO_HF_W {
        IE_FIFO_HF_W { w: self }
    }
    #[doc = "Bit 3 - enable interrupt for timer1/2 DMA completion"]
    #[inline(always)]
    pub fn ie_dma_end(&mut self) -> IE_DMA_END_W {
        IE_DMA_END_W { w: self }
    }
    #[doc = "Bit 4 - enable interrupt for timer FIFO overflow"]
    #[inline(always)]
    pub fn ie_fifo_ov(&mut self) -> IE_FIFO_OV_W {
        IE_FIFO_OV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR0 interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inter_en](index.html) module"]
pub struct INTER_EN_SPEC;
impl crate::RegisterSpec for INTER_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [inter_en::R](R) reader structure"]
impl crate::Readable for INTER_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inter_en::W](W) writer structure"]
impl crate::Writable for INTER_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTER_EN to value 0"]
impl crate::Resettable for INTER_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

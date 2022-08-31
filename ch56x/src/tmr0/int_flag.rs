#[doc = "Register `INT_FLAG` reader"]
pub struct R(crate::R<INT_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_FLAG` writer"]
pub struct W(crate::W<INT_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_FLAG_SPEC>;
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
impl From<crate::W<INT_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IF_CYC_END` reader - interrupt flag for timer capture count timeout or PWM cycle end"]
pub struct IF_CYC_END_R(crate::FieldReader<bool, bool>);
impl IF_CYC_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IF_CYC_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IF_CYC_END_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IF_CYC_END` writer - interrupt flag for timer capture count timeout or PWM cycle end"]
pub struct IF_CYC_END_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_CYC_END_W<'a> {
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
#[doc = "Field `IF_DATA_ACT` reader - interrupt flag for timer capture input action or PWM trigger"]
pub struct IF_DATA_ACT_R(crate::FieldReader<bool, bool>);
impl IF_DATA_ACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IF_DATA_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IF_DATA_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IF_DATA_ACT` writer - interrupt flag for timer capture input action or PWM trigger"]
pub struct IF_DATA_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_DATA_ACT_W<'a> {
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
#[doc = "Field `IF_FIFO_HF` reader - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
pub struct IF_FIFO_HF_R(crate::FieldReader<bool, bool>);
impl IF_FIFO_HF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IF_FIFO_HF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IF_FIFO_HF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IF_FIFO_HF` writer - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
pub struct IF_FIFO_HF_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_FIFO_HF_W<'a> {
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
#[doc = "Field `IF_DMA_END` reader - interrupt flag for timer1/2 DMA completion"]
pub struct IF_DMA_END_R(crate::FieldReader<bool, bool>);
impl IF_DMA_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IF_DMA_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IF_DMA_END_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IF_DMA_END` writer - interrupt flag for timer1/2 DMA completion"]
pub struct IF_DMA_END_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_DMA_END_W<'a> {
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
#[doc = "Field `IF_FIFO_OV` reader - interrupt flag for timer FIFO overflow"]
pub struct IF_FIFO_OV_R(crate::FieldReader<bool, bool>);
impl IF_FIFO_OV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IF_FIFO_OV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IF_FIFO_OV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IF_FIFO_OV` writer - interrupt flag for timer FIFO overflow"]
pub struct IF_FIFO_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_FIFO_OV_W<'a> {
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
    #[doc = "Bit 0 - interrupt flag for timer capture count timeout or PWM cycle end"]
    #[inline(always)]
    pub fn if_cyc_end(&self) -> IF_CYC_END_R {
        IF_CYC_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt flag for timer capture input action or PWM trigger"]
    #[inline(always)]
    pub fn if_data_act(&self) -> IF_DATA_ACT_R {
        IF_DATA_ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
    #[inline(always)]
    pub fn if_fifo_hf(&self) -> IF_FIFO_HF_R {
        IF_FIFO_HF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt flag for timer1/2 DMA completion"]
    #[inline(always)]
    pub fn if_dma_end(&self) -> IF_DMA_END_R {
        IF_DMA_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - interrupt flag for timer FIFO overflow"]
    #[inline(always)]
    pub fn if_fifo_ov(&self) -> IF_FIFO_OV_R {
        IF_FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt flag for timer capture count timeout or PWM cycle end"]
    #[inline(always)]
    pub fn if_cyc_end(&mut self) -> IF_CYC_END_W {
        IF_CYC_END_W { w: self }
    }
    #[doc = "Bit 1 - interrupt flag for timer capture input action or PWM trigger"]
    #[inline(always)]
    pub fn if_data_act(&mut self) -> IF_DATA_ACT_W {
        IF_DATA_ACT_W { w: self }
    }
    #[doc = "Bit 2 - interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo lower than 3)"]
    #[inline(always)]
    pub fn if_fifo_hf(&mut self) -> IF_FIFO_HF_W {
        IF_FIFO_HF_W { w: self }
    }
    #[doc = "Bit 3 - interrupt flag for timer1/2 DMA completion"]
    #[inline(always)]
    pub fn if_dma_end(&mut self) -> IF_DMA_END_W {
        IF_DMA_END_W { w: self }
    }
    #[doc = "Bit 4 - interrupt flag for timer FIFO overflow"]
    #[inline(always)]
    pub fn if_fifo_ov(&mut self) -> IF_FIFO_OV_W {
        IF_FIFO_OV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR0 interrupt flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_flag](index.html) module"]
pub struct INT_FLAG_SPEC;
impl crate::RegisterSpec for INT_FLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [int_flag::R](R) reader structure"]
impl crate::Readable for INT_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_flag::W](W) writer structure"]
impl crate::Writable for INT_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_FLAG to value 0"]
impl crate::Resettable for INT_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

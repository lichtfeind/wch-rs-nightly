#[doc = "Register `HSPI_INT_FLAG` reader"]
pub struct R(crate::R<HSPI_INT_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_INT_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_INT_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_INT_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_INT_FLAG` writer"]
pub struct W(crate::W<HSPI_INT_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_INT_FLAG_SPEC>;
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
impl From<crate::W<HSPI_INT_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_INT_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_IF_T_DONE` reader - interrupt flag for parallel if transmit done"]
pub struct RB_HSPI_IF_T_DONE_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_IF_T_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_IF_T_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_IF_T_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_IF_T_DONE` writer - interrupt flag for parallel if transmit done"]
pub struct RB_HSPI_IF_T_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_IF_T_DONE_W<'a> {
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
#[doc = "Field `RB_HSPI_IF_R_DONE` reader - interrupt flag for parallel if receive done"]
pub struct RB_HSPI_IF_R_DONE_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_IF_R_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_IF_R_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_IF_R_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_IF_R_DONE` writer - interrupt flag for parallel if receive done"]
pub struct RB_HSPI_IF_R_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_IF_R_DONE_W<'a> {
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
#[doc = "Field `RB_HSPI_IF_FIFO_OV` reader - interrupt flag for parallel if FIFO overflow"]
pub struct RB_HSPI_IF_FIFO_OV_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_IF_FIFO_OV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_IF_FIFO_OV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_IF_FIFO_OV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_IF_FIFO_OV` writer - interrupt flag for parallel if FIFO overflow"]
pub struct RB_HSPI_IF_FIFO_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_IF_FIFO_OV_W<'a> {
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
#[doc = "Field `RB_HSPI_IF_B_DONE` reader - interrupt flag for parallel if tx burst done"]
pub struct RB_HSPI_IF_B_DONE_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_IF_B_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_IF_B_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_IF_B_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_IF_B_DONE` writer - interrupt flag for parallel if tx burst done"]
pub struct RB_HSPI_IF_B_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_IF_B_DONE_W<'a> {
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
    #[doc = "Bit 0 - interrupt flag for parallel if transmit done"]
    #[inline(always)]
    pub fn rb_hspi_if_t_done(&self) -> RB_HSPI_IF_T_DONE_R {
        RB_HSPI_IF_T_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interrupt flag for parallel if receive done"]
    #[inline(always)]
    pub fn rb_hspi_if_r_done(&self) -> RB_HSPI_IF_R_DONE_R {
        RB_HSPI_IF_R_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interrupt flag for parallel if FIFO overflow"]
    #[inline(always)]
    pub fn rb_hspi_if_fifo_ov(&self) -> RB_HSPI_IF_FIFO_OV_R {
        RB_HSPI_IF_FIFO_OV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - interrupt flag for parallel if tx burst done"]
    #[inline(always)]
    pub fn rb_hspi_if_b_done(&self) -> RB_HSPI_IF_B_DONE_R {
        RB_HSPI_IF_B_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt flag for parallel if transmit done"]
    #[inline(always)]
    pub fn rb_hspi_if_t_done(&mut self) -> RB_HSPI_IF_T_DONE_W {
        RB_HSPI_IF_T_DONE_W { w: self }
    }
    #[doc = "Bit 1 - interrupt flag for parallel if receive done"]
    #[inline(always)]
    pub fn rb_hspi_if_r_done(&mut self) -> RB_HSPI_IF_R_DONE_W {
        RB_HSPI_IF_R_DONE_W { w: self }
    }
    #[doc = "Bit 2 - interrupt flag for parallel if FIFO overflow"]
    #[inline(always)]
    pub fn rb_hspi_if_fifo_ov(&mut self) -> RB_HSPI_IF_FIFO_OV_W {
        RB_HSPI_IF_FIFO_OV_W { w: self }
    }
    #[doc = "Bit 3 - interrupt flag for parallel if tx burst done"]
    #[inline(always)]
    pub fn rb_hspi_if_b_done(&mut self) -> RB_HSPI_IF_B_DONE_W {
        RB_HSPI_IF_B_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if interrupt flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_int_flag](index.html) module"]
pub struct HSPI_INT_FLAG_SPEC;
impl crate::RegisterSpec for HSPI_INT_FLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hspi_int_flag::R](R) reader structure"]
impl crate::Readable for HSPI_INT_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_int_flag::W](W) writer structure"]
impl crate::Writable for HSPI_INT_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_INT_FLAG to value 0"]
impl crate::Resettable for HSPI_INT_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

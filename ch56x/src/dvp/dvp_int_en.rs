#[doc = "Register `DVP_INT_EN` reader"]
pub struct R(crate::R<DVP_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVP_INT_EN` writer"]
pub struct W(crate::W<DVP_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP_INT_EN_SPEC>;
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
impl From<crate::W<DVP_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_IE_STR_FRM` reader - DVP frame start interrupt enable"]
pub struct RB_DVP_IE_STR_FRM_R(crate::FieldReader<bool, bool>);
impl RB_DVP_IE_STR_FRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_IE_STR_FRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_IE_STR_FRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_IE_STR_FRM` writer - DVP frame start interrupt enable"]
pub struct RB_DVP_IE_STR_FRM_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_IE_STR_FRM_W<'a> {
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
#[doc = "Field `RB_DVP_IE_ROW_DONE` reader - DVP row received done interrupt enable"]
pub struct RB_DVP_IE_ROW_DONE_R(crate::FieldReader<bool, bool>);
impl RB_DVP_IE_ROW_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_IE_ROW_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_IE_ROW_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_IE_ROW_DONE` writer - DVP row received done interrupt enable"]
pub struct RB_DVP_IE_ROW_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_IE_ROW_DONE_W<'a> {
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
#[doc = "Field `RB_DVP_IE_FRM_DONE` reader - DVP frame received done interrupt enable"]
pub struct RB_DVP_IE_FRM_DONE_R(crate::FieldReader<bool, bool>);
impl RB_DVP_IE_FRM_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_IE_FRM_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_IE_FRM_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_IE_FRM_DONE` writer - DVP frame received done interrupt enable"]
pub struct RB_DVP_IE_FRM_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_IE_FRM_DONE_W<'a> {
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
#[doc = "Field `RB_DVP_IE_FIFO_OV` reader - DVP receive fifo overflow interrupt enable"]
pub struct RB_DVP_IE_FIFO_OV_R(crate::FieldReader<bool, bool>);
impl RB_DVP_IE_FIFO_OV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_IE_FIFO_OV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_IE_FIFO_OV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_IE_FIFO_OV` writer - DVP receive fifo overflow interrupt enable"]
pub struct RB_DVP_IE_FIFO_OV_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_IE_FIFO_OV_W<'a> {
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
#[doc = "Field `RB_DVP_IE_STP_FRM` reader - DVP frame stop interrupt enable"]
pub struct RB_DVP_IE_STP_FRM_R(crate::FieldReader<bool, bool>);
impl RB_DVP_IE_STP_FRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DVP_IE_STP_FRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DVP_IE_STP_FRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DVP_IE_STP_FRM` writer - DVP frame stop interrupt enable"]
pub struct RB_DVP_IE_STP_FRM_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DVP_IE_STP_FRM_W<'a> {
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
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_str_frm(&self) -> RB_DVP_IE_STR_FRM_R {
        RB_DVP_IE_STR_FRM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_row_done(&self) -> RB_DVP_IE_ROW_DONE_R {
        RB_DVP_IE_ROW_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_frm_done(&self) -> RB_DVP_IE_FRM_DONE_R {
        RB_DVP_IE_FRM_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_fifo_ov(&self) -> RB_DVP_IE_FIFO_OV_R {
        RB_DVP_IE_FIFO_OV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DVP frame stop interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_stp_frm(&self) -> RB_DVP_IE_STP_FRM_R {
        RB_DVP_IE_STP_FRM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_str_frm(&mut self) -> RB_DVP_IE_STR_FRM_W {
        RB_DVP_IE_STR_FRM_W { w: self }
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_row_done(&mut self) -> RB_DVP_IE_ROW_DONE_W {
        RB_DVP_IE_ROW_DONE_W { w: self }
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_frm_done(&mut self) -> RB_DVP_IE_FRM_DONE_W {
        RB_DVP_IE_FRM_DONE_W { w: self }
    }
    #[doc = "Bit 3 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_fifo_ov(&mut self) -> RB_DVP_IE_FIFO_OV_W {
        RB_DVP_IE_FIFO_OV_W { w: self }
    }
    #[doc = "Bit 4 - DVP frame stop interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_stp_frm(&mut self) -> RB_DVP_IE_STP_FRM_W {
        RB_DVP_IE_STP_FRM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DVP interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_int_en](index.html) module"]
pub struct DVP_INT_EN_SPEC;
impl crate::RegisterSpec for DVP_INT_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dvp_int_en::R](R) reader structure"]
impl crate::Readable for DVP_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp_int_en::W](W) writer structure"]
impl crate::Writable for DVP_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DVP_INT_EN to value 0"]
impl crate::Resettable for DVP_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

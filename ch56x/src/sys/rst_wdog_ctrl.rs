#[doc = "Register `RST_WDOG_CTRL` reader"]
pub struct R(crate::R<RST_WDOG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_WDOG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_WDOG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_WDOG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_WDOG_CTRL` writer"]
pub struct W(crate::W<RST_WDOG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_WDOG_CTRL_SPEC>;
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
impl From<crate::W<RST_WDOG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_WDOG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SOFTWARE_RESET` reader - global software reset"]
pub struct RB_SOFTWARE_RESET_R(crate::FieldReader<bool, bool>);
impl RB_SOFTWARE_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SOFTWARE_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SOFTWARE_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SOFTWARE_RESET` writer - global software reset"]
pub struct RB_SOFTWARE_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SOFTWARE_RESET_W<'a> {
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
#[doc = "Field `RB_WDOG_RST_EN` reader - enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow"]
pub struct RB_WDOG_RST_EN_R(crate::FieldReader<bool, bool>);
impl RB_WDOG_RST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_WDOG_RST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_WDOG_RST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_WDOG_RST_EN` writer - enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow"]
pub struct RB_WDOG_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_WDOG_RST_EN_W<'a> {
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
#[doc = "Field `RB_WDOG_INT_EN` reader - watch-dog interrupt enable or INT_ID_WDOG interrupt source selection: 0=software interrupt"]
pub struct RB_WDOG_INT_EN_R(crate::FieldReader<bool, bool>);
impl RB_WDOG_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_WDOG_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_WDOG_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_WDOG_INT_EN` writer - watch-dog interrupt enable or INT_ID_WDOG interrupt source selection: 0=software interrupt"]
pub struct RB_WDOG_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_WDOG_INT_EN_W<'a> {
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
#[doc = "Field `RB_WDOG_INT_FLAG` reader - watch-dog timer overflow interrupt flag"]
pub struct RB_WDOG_INT_FLAG_R(crate::FieldReader<bool, bool>);
impl RB_WDOG_INT_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_WDOG_INT_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_WDOG_INT_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_WDOG_INT_FLAG` writer - watch-dog timer overflow interrupt flag"]
pub struct RB_WDOG_INT_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_WDOG_INT_FLAG_W<'a> {
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
    #[doc = "Bit 0 - global software reset"]
    #[inline(always)]
    pub fn rb_software_reset(&self) -> RB_SOFTWARE_RESET_R {
        RB_SOFTWARE_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow"]
    #[inline(always)]
    pub fn rb_wdog_rst_en(&self) -> RB_WDOG_RST_EN_R {
        RB_WDOG_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - watch-dog interrupt enable or INT_ID_WDOG interrupt source selection: 0=software interrupt"]
    #[inline(always)]
    pub fn rb_wdog_int_en(&self) -> RB_WDOG_INT_EN_R {
        RB_WDOG_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - watch-dog timer overflow interrupt flag"]
    #[inline(always)]
    pub fn rb_wdog_int_flag(&self) -> RB_WDOG_INT_FLAG_R {
        RB_WDOG_INT_FLAG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - global software reset"]
    #[inline(always)]
    pub fn rb_software_reset(&mut self) -> RB_SOFTWARE_RESET_W {
        RB_SOFTWARE_RESET_W { w: self }
    }
    #[doc = "Bit 1 - enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow"]
    #[inline(always)]
    pub fn rb_wdog_rst_en(&mut self) -> RB_WDOG_RST_EN_W {
        RB_WDOG_RST_EN_W { w: self }
    }
    #[doc = "Bit 2 - watch-dog interrupt enable or INT_ID_WDOG interrupt source selection: 0=software interrupt"]
    #[inline(always)]
    pub fn rb_wdog_int_en(&mut self) -> RB_WDOG_INT_EN_W {
        RB_WDOG_INT_EN_W { w: self }
    }
    #[doc = "Bit 3 - watch-dog timer overflow interrupt flag"]
    #[inline(always)]
    pub fn rb_wdog_int_flag(&mut self) -> RB_WDOG_INT_FLAG_W {
        RB_WDOG_INT_FLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reset and watch-dog control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_wdog_ctrl](index.html) module"]
pub struct RST_WDOG_CTRL_SPEC;
impl crate::RegisterSpec for RST_WDOG_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rst_wdog_ctrl::R](R) reader structure"]
impl crate::Readable for RST_WDOG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_wdog_ctrl::W](W) writer structure"]
impl crate::Writable for RST_WDOG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_WDOG_CTRL to value 0"]
impl crate::Resettable for RST_WDOG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

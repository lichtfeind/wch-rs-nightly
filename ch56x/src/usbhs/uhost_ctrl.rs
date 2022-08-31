#[doc = "Register `UHOST_CTRL` reader"]
pub struct R(crate::R<UHOST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHOST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHOST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHOST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UHOST_CTRL` writer"]
pub struct W(crate::W<UHOST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHOST_CTRL_SPEC>;
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
impl From<crate::W<UHOST_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHOST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UH_BUS_RESET` reader - USB host send bus reset signal"]
pub struct RB_UH_BUS_RESET_R(crate::FieldReader<bool, bool>);
impl RB_UH_BUS_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_UH_BUS_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UH_BUS_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UH_BUS_RESET` writer - USB host send bus reset signal"]
pub struct RB_UH_BUS_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UH_BUS_RESET_W<'a> {
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
#[doc = "Field `RB_UH_BUS_SUSPEND` reader - USB host send bus suspend signal"]
pub struct RB_UH_BUS_SUSPEND_R(crate::FieldReader<bool, bool>);
impl RB_UH_BUS_SUSPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_UH_BUS_SUSPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UH_BUS_SUSPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UH_BUS_SUSPEND` writer - USB host send bus suspend signal"]
pub struct RB_UH_BUS_SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UH_BUS_SUSPEND_W<'a> {
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
#[doc = "Field `RB_UH_BUS_RESUME` reader - USB host suspend state and wake up device"]
pub struct RB_UH_BUS_RESUME_R(crate::FieldReader<bool, bool>);
impl RB_UH_BUS_RESUME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_UH_BUS_RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UH_BUS_RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UH_BUS_RESUME` writer - USB host suspend state and wake up device"]
pub struct RB_UH_BUS_RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UH_BUS_RESUME_W<'a> {
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
#[doc = "Field `RB_UH_AUTOSOF_EN` reader - Automatically generate sof packet enable control"]
pub struct RB_UH_AUTOSOF_EN_R(crate::FieldReader<bool, bool>);
impl RB_UH_AUTOSOF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_UH_AUTOSOF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UH_AUTOSOF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UH_AUTOSOF_EN` writer - Automatically generate sof packet enable control"]
pub struct RB_UH_AUTOSOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UH_AUTOSOF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u8 & 1) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB host send bus reset signal"]
    #[inline(always)]
    pub fn rb_uh_bus_reset(&self) -> RB_UH_BUS_RESET_R {
        RB_UH_BUS_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB host send bus suspend signal"]
    #[inline(always)]
    pub fn rb_uh_bus_suspend(&self) -> RB_UH_BUS_SUSPEND_R {
        RB_UH_BUS_SUSPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB host suspend state and wake up device"]
    #[inline(always)]
    pub fn rb_uh_bus_resume(&self) -> RB_UH_BUS_RESUME_R {
        RB_UH_BUS_RESUME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Automatically generate sof packet enable control"]
    #[inline(always)]
    pub fn rb_uh_autosof_en(&self) -> RB_UH_AUTOSOF_EN_R {
        RB_UH_AUTOSOF_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB host send bus reset signal"]
    #[inline(always)]
    pub fn rb_uh_bus_reset(&mut self) -> RB_UH_BUS_RESET_W {
        RB_UH_BUS_RESET_W { w: self }
    }
    #[doc = "Bit 1 - USB host send bus suspend signal"]
    #[inline(always)]
    pub fn rb_uh_bus_suspend(&mut self) -> RB_UH_BUS_SUSPEND_W {
        RB_UH_BUS_SUSPEND_W { w: self }
    }
    #[doc = "Bit 2 - USB host suspend state and wake up device"]
    #[inline(always)]
    pub fn rb_uh_bus_resume(&mut self) -> RB_UH_BUS_RESUME_W {
        RB_UH_BUS_RESUME_W { w: self }
    }
    #[doc = "Bit 7 - Automatically generate sof packet enable control"]
    #[inline(always)]
    pub fn rb_uh_autosof_en(&mut self) -> RB_UH_AUTOSOF_EN_W {
        RB_UH_AUTOSOF_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB host control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhost_ctrl](index.html) module"]
pub struct UHOST_CTRL_SPEC;
impl crate::RegisterSpec for UHOST_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uhost_ctrl::R](R) reader structure"]
impl crate::Readable for UHOST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhost_ctrl::W](W) writer structure"]
impl crate::Writable for UHOST_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UHOST_CTRL to value 0"]
impl crate::Resettable for UHOST_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `SCTLR` reader"]
pub struct R(crate::R<SCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTLR` writer"]
pub struct W(crate::W<SCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTLR_SPEC>;
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
impl From<crate::W<SCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEPONEXIT` reader - SLEEPONEXIT"]
pub struct SLEEPONEXIT_R(crate::FieldReader<bool, bool>);
impl SLEEPONEXIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPONEXIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEPONEXIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPONEXIT` writer - SLEEPONEXIT"]
pub struct SLEEPONEXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPONEXIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `SLEEPDEEP` reader - SLEEPDEEP"]
pub struct SLEEPDEEP_R(crate::FieldReader<bool, bool>);
impl SLEEPDEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPDEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEPDEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPDEEP` writer - SLEEPDEEP"]
pub struct SLEEPDEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPDEEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `WFITOWFE` reader - WFITOWFE"]
pub struct WFITOWFE_R(crate::FieldReader<bool, bool>);
impl WFITOWFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WFITOWFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFITOWFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFITOWFE` writer - WFITOWFE"]
pub struct WFITOWFE_W<'a> {
    w: &'a mut W,
}
impl<'a> WFITOWFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `SEVONPEND` reader - SEVONPEND"]
pub struct SEVONPEND_R(crate::FieldReader<bool, bool>);
impl SEVONPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEVONPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEVONPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEVONPEND` writer - SEVONPEND"]
pub struct SEVONPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SEVONPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `SETEVENT` reader - SETEVENT"]
pub struct SETEVENT_R(crate::FieldReader<bool, bool>);
impl SETEVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETEVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETEVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETEVENT` writer - SETEVENT"]
pub struct SETEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SETEVENT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&self) -> WFITOWFE_R {
        WFITOWFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SETEVENT"]
    #[inline(always)]
    pub fn setevent(&self) -> SETEVENT_R {
        SETEVENT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W {
        SLEEPONEXIT_W { w: self }
    }
    #[doc = "Bit 2 - SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W {
        SLEEPDEEP_W { w: self }
    }
    #[doc = "Bit 3 - WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&mut self) -> WFITOWFE_W {
        WFITOWFE_W { w: self }
    }
    #[doc = "Bit 4 - SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SEVONPEND_W {
        SEVONPEND_W { w: self }
    }
    #[doc = "Bit 5 - SETEVENT"]
    #[inline(always)]
    pub fn setevent(&mut self) -> SETEVENT_W {
        SETEVENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctlr](index.html) module"]
pub struct SCTLR_SPEC;
impl crate::RegisterSpec for SCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctlr::R](R) reader structure"]
impl crate::Readable for SCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctlr::W](W) writer structure"]
impl crate::Writable for SCTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCTLR to value 0"]
impl crate::Resettable for SCTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

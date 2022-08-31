#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HWSTKCTRL` reader - HWSTKCTRL"]
pub struct HWSTKCTRL_R(crate::FieldReader<bool, bool>);
impl HWSTKCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HWSTKCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWSTKCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HWSTKCTRL` writer - HWSTKCTRL"]
pub struct HWSTKCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> HWSTKCTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `NESTCTRL` reader - NESTCTRL"]
pub struct NESTCTRL_R(crate::FieldReader<bool, bool>);
impl NESTCTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NESTCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NESTCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NESTCTRL` writer - NESTCTRL"]
pub struct NESTCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> NESTCTRL_W<'a> {
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
#[doc = "Field `NMISET` writer - NMISET"]
pub struct NMISET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMISET_W<'a> {
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
#[doc = "Field `NMIRESET` writer - NMIRESET"]
pub struct NMIRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIRESET_W<'a> {
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
#[doc = "Field `EXCSET` writer - EXCSET"]
pub struct EXCSET_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCSET_W<'a> {
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
#[doc = "Field `EXCRESET` writer - EXCRESET"]
pub struct EXCRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCRESET_W<'a> {
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
#[doc = "Field `PFICRESET` writer - PFICRSET"]
pub struct PFICRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PFICRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `SYSRESET` writer - SYSRESET"]
pub struct SYSRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `KEYCODE` writer - KEYCODE"]
pub struct KEYCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HWSTKCTRL"]
    #[inline(always)]
    pub fn hwstkctrl(&self) -> HWSTKCTRL_R {
        HWSTKCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NESTCTRL"]
    #[inline(always)]
    pub fn nestctrl(&self) -> NESTCTRL_R {
        NESTCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HWSTKCTRL"]
    #[inline(always)]
    pub fn hwstkctrl(&mut self) -> HWSTKCTRL_W {
        HWSTKCTRL_W { w: self }
    }
    #[doc = "Bit 1 - NESTCTRL"]
    #[inline(always)]
    pub fn nestctrl(&mut self) -> NESTCTRL_W {
        NESTCTRL_W { w: self }
    }
    #[doc = "Bit 2 - NMISET"]
    #[inline(always)]
    pub fn nmiset(&mut self) -> NMISET_W {
        NMISET_W { w: self }
    }
    #[doc = "Bit 3 - NMIRESET"]
    #[inline(always)]
    pub fn nmireset(&mut self) -> NMIRESET_W {
        NMIRESET_W { w: self }
    }
    #[doc = "Bit 4 - EXCSET"]
    #[inline(always)]
    pub fn excset(&mut self) -> EXCSET_W {
        EXCSET_W { w: self }
    }
    #[doc = "Bit 5 - EXCRESET"]
    #[inline(always)]
    pub fn excreset(&mut self) -> EXCRESET_W {
        EXCRESET_W { w: self }
    }
    #[doc = "Bit 6 - PFICRSET"]
    #[inline(always)]
    pub fn pficreset(&mut self) -> PFICRESET_W {
        PFICRESET_W { w: self }
    }
    #[doc = "Bit 7 - SYSRESET"]
    #[inline(always)]
    pub fn sysreset(&mut self) -> SYSRESET_W {
        SYSRESET_W { w: self }
    }
    #[doc = "Bits 16:31 - KEYCODE"]
    #[inline(always)]
    pub fn keycode(&mut self) -> KEYCODE_W {
        KEYCODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

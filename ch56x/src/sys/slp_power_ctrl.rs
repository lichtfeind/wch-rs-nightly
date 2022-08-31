#[doc = "Register `SLP_POWER_CTRL` reader"]
pub struct R(crate::R<SLP_POWER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_POWER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_POWER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_POWER_CTRL` writer"]
pub struct W(crate::W<SLP_POWER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_POWER_CTRL_SPEC>;
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
impl From<crate::W<SLP_POWER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_POWER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SLP_USBHS_PWRDN` reader - enable USBHS power down"]
pub struct RB_SLP_USBHS_PWRDN_R(crate::FieldReader<bool, bool>);
impl RB_SLP_USBHS_PWRDN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SLP_USBHS_PWRDN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SLP_USBHS_PWRDN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SLP_USBHS_PWRDN` writer - enable USBHS power down"]
pub struct RB_SLP_USBHS_PWRDN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SLP_USBHS_PWRDN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - enable USBHS power down"]
    #[inline(always)]
    pub fn rb_slp_usbhs_pwrdn(&self) -> RB_SLP_USBHS_PWRDN_R {
        RB_SLP_USBHS_PWRDN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable USBHS power down"]
    #[inline(always)]
    pub fn rb_slp_usbhs_pwrdn(&mut self) -> RB_SLP_USBHS_PWRDN_W {
        RB_SLP_USBHS_PWRDN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_power_ctrl](index.html) module"]
pub struct SLP_POWER_CTRL_SPEC;
impl crate::RegisterSpec for SLP_POWER_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [slp_power_ctrl::R](R) reader structure"]
impl crate::Readable for SLP_POWER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_power_ctrl::W](W) writer structure"]
impl crate::Writable for SLP_POWER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_POWER_CTRL to value 0"]
impl crate::Resettable for SLP_POWER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

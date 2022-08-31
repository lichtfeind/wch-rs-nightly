#[doc = "Register `USB_SUSPEND` reader"]
pub struct R(crate::R<USB_SUSPEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_SUSPEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_SUSPEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_SUSPEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_SUSPEND` writer"]
pub struct W(crate::W<USB_SUSPEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_SUSPEND_SPEC>;
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
impl From<crate::W<USB_SUSPEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_SUSPEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DEV_WAKEUP` reader - Remote wake-up control bit"]
pub struct RB_DEV_WAKEUP_R(crate::FieldReader<bool, bool>);
impl RB_DEV_WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DEV_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DEV_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DEV_WAKEUP` writer - Remote wake-up control bit"]
pub struct RB_DEV_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DEV_WAKEUP_W<'a> {
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
impl R {
    #[doc = "Bit 1 - Remote wake-up control bit"]
    #[inline(always)]
    pub fn rb_dev_wakeup(&self) -> RB_DEV_WAKEUP_R {
        RB_DEV_WAKEUP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Remote wake-up control bit"]
    #[inline(always)]
    pub fn rb_dev_wakeup(&mut self) -> RB_DEV_WAKEUP_W {
        RB_DEV_WAKEUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB suspend register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_suspend](index.html) module"]
pub struct USB_SUSPEND_SPEC;
impl crate::RegisterSpec for USB_SUSPEND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_suspend::R](R) reader structure"]
impl crate::Readable for USB_SUSPEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_suspend::W](W) writer structure"]
impl crate::Writable for USB_SUSPEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_SUSPEND to value 0"]
impl crate::Resettable for USB_SUSPEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

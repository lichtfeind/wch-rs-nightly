#[doc = "Register `USB_DEV_AD` reader"]
pub struct R(crate::R<USB_DEV_AD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_DEV_AD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_DEV_AD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_DEV_AD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_DEV_AD` writer"]
pub struct W(crate::W<USB_DEV_AD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_DEV_AD_SPEC>;
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
impl From<crate::W<USB_DEV_AD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_DEV_AD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_ADDR_MASK` reader - bit mask for USB device address"]
pub struct USB_ADDR_MASK_R(crate::FieldReader<u8, u8>);
impl USB_ADDR_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_ADDR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ADDR_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_ADDR_MASK` writer - bit mask for USB device address"]
pub struct USB_ADDR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ADDR_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u8 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - bit mask for USB device address"]
    #[inline(always)]
    pub fn usb_addr_mask(&self) -> USB_ADDR_MASK_R {
        USB_ADDR_MASK_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - bit mask for USB device address"]
    #[inline(always)]
    pub fn usb_addr_mask(&mut self) -> USB_ADDR_MASK_W {
        USB_ADDR_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB device address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_dev_ad](index.html) module"]
pub struct USB_DEV_AD_SPEC;
impl crate::RegisterSpec for USB_DEV_AD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_dev_ad::R](R) reader structure"]
impl crate::Readable for USB_DEV_AD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_dev_ad::W](W) writer structure"]
impl crate::Writable for USB_DEV_AD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_DEV_AD to value 0"]
impl crate::Resettable for USB_DEV_AD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

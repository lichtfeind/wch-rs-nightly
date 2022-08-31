#[doc = "Register `USB_SPD_TYPE` reader"]
pub struct R(crate::R<USB_SPD_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_SPD_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_SPD_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_SPD_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_USBSPEED_MASK` reader - USB actual speed"]
pub struct RB_USBSPEED_MASK_R(crate::FieldReader<u8, u8>);
impl RB_USBSPEED_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_USBSPEED_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USBSPEED_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - USB actual speed"]
    #[inline(always)]
    pub fn rb_usbspeed_mask(&self) -> RB_USBSPEED_MASK_R {
        RB_USBSPEED_MASK_R::new((self.bits & 3) as u8)
    }
}
#[doc = "USB actual speed register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_spd_type](index.html) module"]
pub struct USB_SPD_TYPE_SPEC;
impl crate::RegisterSpec for USB_SPD_TYPE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_spd_type::R](R) reader structure"]
impl crate::Readable for USB_SPD_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USB_SPD_TYPE to value 0"]
impl crate::Resettable for USB_SPD_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

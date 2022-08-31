#[doc = "Register `USB_FRAME_NO` reader"]
pub struct R(crate::R<USB_FRAME_NO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_FRAME_NO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_FRAME_NO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_FRAME_NO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_FRAME_NO` reader - USB frame number"]
pub struct USB_FRAME_NO_R(crate::FieldReader<u16, u16>);
impl USB_FRAME_NO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        USB_FRAME_NO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_FRAME_NO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - USB frame number"]
    #[inline(always)]
    pub fn usb_frame_no(&self) -> USB_FRAME_NO_R {
        USB_FRAME_NO_R::new(self.bits)
    }
}
#[doc = "USB frame number register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_frame_no](index.html) module"]
pub struct USB_FRAME_NO_SPEC;
impl crate::RegisterSpec for USB_FRAME_NO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usb_frame_no::R](R) reader structure"]
impl crate::Readable for USB_FRAME_NO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USB_FRAME_NO to value 0"]
impl crate::Resettable for USB_FRAME_NO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

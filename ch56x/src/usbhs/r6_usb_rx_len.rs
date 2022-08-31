#[doc = "Register `R6_USB_RX_LEN` reader"]
pub struct R(crate::R<R6_USB_RX_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R6_USB_RX_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R6_USB_RX_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R6_USB_RX_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_RX_LEN` reader - length of received bytes"]
pub struct USB_RX_LEN_R(crate::FieldReader<u16, u16>);
impl USB_RX_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        USB_RX_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_RX_LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - length of received bytes"]
    #[inline(always)]
    pub fn usb_rx_len(&self) -> USB_RX_LEN_R {
        USB_RX_LEN_R::new(self.bits)
    }
}
#[doc = "USB receiving length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r6_usb_rx_len](index.html) module"]
pub struct R6_USB_RX_LEN_SPEC;
impl crate::RegisterSpec for R6_USB_RX_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r6_usb_rx_len::R](R) reader structure"]
impl crate::Readable for R6_USB_RX_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R6_USB_RX_LEN to value 0"]
impl crate::Resettable for R6_USB_RX_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

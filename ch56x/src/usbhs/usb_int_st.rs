#[doc = "Register `USB_INT_ST` reader"]
pub struct R(crate::R<USB_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RB_HOST_RES_MASK_RB_DEV_ENDP_MASK` reader - RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode"]
pub struct RB_HOST_RES_MASK_RB_DEV_ENDP_MASK_R(crate::FieldReader<u8, u8>);
impl RB_HOST_RES_MASK_RB_DEV_ENDP_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_HOST_RES_MASK_RB_DEV_ENDP_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HOST_RES_MASK_RB_DEV_ENDP_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DEV_TOKEN_MASK` reader - RO, bit mask of current token PID code received for USB device mode"]
pub struct RB_DEV_TOKEN_MASK_R(crate::FieldReader<u8, u8>);
impl RB_DEV_TOKEN_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_DEV_TOKEN_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DEV_TOKEN_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_ST_TOGOK` reader - RO, indicate current USB transfer toggle is OK"]
pub struct RB_USB_ST_TOGOK_R(crate::FieldReader<bool, bool>);
impl RB_USB_ST_TOGOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_ST_TOGOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_ST_TOGOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_ST_NAK` reader - RO, indicate current USB transfer is NAK received for USB device mode"]
pub struct RB_USB_ST_NAK_R(crate::FieldReader<bool, bool>);
impl RB_USB_ST_NAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_ST_NAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_ST_NAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode"]
    #[inline(always)]
    pub fn rb_host_res_mask_rb_dev_endp_mask(&self) -> RB_HOST_RES_MASK_RB_DEV_ENDP_MASK_R {
        RB_HOST_RES_MASK_RB_DEV_ENDP_MASK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - RO, bit mask of current token PID code received for USB device mode"]
    #[inline(always)]
    pub fn rb_dev_token_mask(&self) -> RB_DEV_TOKEN_MASK_R {
        RB_DEV_TOKEN_MASK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - RO, indicate current USB transfer toggle is OK"]
    #[inline(always)]
    pub fn rb_usb_st_togok(&self) -> RB_USB_ST_TOGOK_R {
        RB_USB_ST_TOGOK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate current USB transfer is NAK received for USB device mode"]
    #[inline(always)]
    pub fn rb_usb_st_nak(&self) -> RB_USB_ST_NAK_R {
        RB_USB_ST_NAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_int_st](index.html) module"]
pub struct USB_INT_ST_SPEC;
impl crate::RegisterSpec for USB_INT_ST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_int_st::R](R) reader structure"]
impl crate::Readable for USB_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USB_INT_ST to value 0"]
impl crate::Resettable for USB_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

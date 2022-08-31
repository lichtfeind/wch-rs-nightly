#[doc = "Register `USB_INT_EN` reader"]
pub struct R(crate::R<USB_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_INT_EN` writer"]
pub struct W(crate::W<USB_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_INT_EN_SPEC>;
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
impl From<crate::W<USB_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_USB_IE_BUSRST_RB_USB_IE_DETECT` reader - enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode"]
pub struct RB_USB_IE_BUSRST_RB_USB_IE_DETECT_R(crate::FieldReader<bool, bool>);
impl RB_USB_IE_BUSRST_RB_USB_IE_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_IE_BUSRST_RB_USB_IE_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_IE_BUSRST_RB_USB_IE_DETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_IE_BUSRST_RB_USB_IE_DETECT` writer - enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode"]
pub struct RB_USB_IE_BUSRST_RB_USB_IE_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_IE_BUSRST_RB_USB_IE_DETECT_W<'a> {
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
#[doc = "Field `RB_USB_IE_TRANS` reader - enable interrupt for USB transfer completion"]
pub struct RB_USB_IE_TRANS_R(crate::FieldReader<bool, bool>);
impl RB_USB_IE_TRANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_IE_TRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_IE_TRANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_IE_TRANS` writer - enable interrupt for USB transfer completion"]
pub struct RB_USB_IE_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_IE_TRANS_W<'a> {
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
#[doc = "Field `RB_USB_IE_SUSPEND` reader - enable interrupt for USB suspend or resume event"]
pub struct RB_USB_IE_SUSPEND_R(crate::FieldReader<bool, bool>);
impl RB_USB_IE_SUSPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_IE_SUSPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_IE_SUSPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_IE_SUSPEND` writer - enable interrupt for USB suspend or resume event"]
pub struct RB_USB_IE_SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_IE_SUSPEND_W<'a> {
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
#[doc = "Field `RB_USB_IE_SOF` reader - enable interrupt for host SOF timer action for USB host mode"]
pub struct RB_USB_IE_SOF_R(crate::FieldReader<bool, bool>);
impl RB_USB_IE_SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_IE_SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_IE_SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_IE_SOF` writer - enable interrupt for host SOF timer action for USB host mode"]
pub struct RB_USB_IE_SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_IE_SOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u8 & 1) << 3);
        self.w
    }
}
#[doc = "Field `RB_USB_IE_FIFOOV` reader - enable interrupt for FIFO overflow"]
pub struct RB_USB_IE_FIFOOV_R(crate::FieldReader<bool, bool>);
impl RB_USB_IE_FIFOOV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_IE_FIFOOV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_IE_FIFOOV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_IE_FIFOOV` writer - enable interrupt for FIFO overflow"]
pub struct RB_USB_IE_FIFOOV_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_IE_FIFOOV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u8 & 1) << 4);
        self.w
    }
}
#[doc = "Field `RB_USB_IE_SETUPACT` reader - Setup packet end interrupt"]
pub struct RB_USB_IE_SETUPACT_R(crate::FieldReader<bool, bool>);
impl RB_USB_IE_SETUPACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_IE_SETUPACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_IE_SETUPACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_IE_SETUPACT` writer - Setup packet end interrupt"]
pub struct RB_USB_IE_SETUPACT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_IE_SETUPACT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u8 & 1) << 5);
        self.w
    }
}
#[doc = "Field `RB_USB_IE_ISOACT` reader - Synchronous transmission received control token packet interrupt"]
pub struct RB_USB_IE_ISOACT_R(crate::FieldReader<bool, bool>);
impl RB_USB_IE_ISOACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_IE_ISOACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_IE_ISOACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_IE_ISOACT` writer - Synchronous transmission received control token packet interrupt"]
pub struct RB_USB_IE_ISOACT_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_IE_ISOACT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u8 & 1) << 6);
        self.w
    }
}
#[doc = "Field `RB_USB_IE_DEV_NAK` reader - enable interrupt for NAK responded for USB device mode"]
pub struct RB_USB_IE_DEV_NAK_R(crate::FieldReader<bool, bool>);
impl RB_USB_IE_DEV_NAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_IE_DEV_NAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_IE_DEV_NAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_IE_DEV_NAK` writer - enable interrupt for NAK responded for USB device mode"]
pub struct RB_USB_IE_DEV_NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_IE_DEV_NAK_W<'a> {
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
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_busrst_rb_usb_ie_detect(&self) -> RB_USB_IE_BUSRST_RB_USB_IE_DETECT_R {
        RB_USB_IE_BUSRST_RB_USB_IE_DETECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_usb_ie_trans(&self) -> RB_USB_IE_TRANS_R {
        RB_USB_IE_TRANS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_usb_ie_suspend(&self) -> RB_USB_IE_SUSPEND_R {
        RB_USB_IE_SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable interrupt for host SOF timer action for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_sof(&self) -> RB_USB_IE_SOF_R {
        RB_USB_IE_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_usb_ie_fifoov(&self) -> RB_USB_IE_FIFOOV_R {
        RB_USB_IE_FIFOOV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setup packet end interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_setupact(&self) -> RB_USB_IE_SETUPACT_R {
        RB_USB_IE_SETUPACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronous transmission received control token packet interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_isoact(&self) -> RB_USB_IE_ISOACT_R {
        RB_USB_IE_ISOACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_usb_ie_dev_nak(&self) -> RB_USB_IE_DEV_NAK_R {
        RB_USB_IE_DEV_NAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_busrst_rb_usb_ie_detect(&mut self) -> RB_USB_IE_BUSRST_RB_USB_IE_DETECT_W {
        RB_USB_IE_BUSRST_RB_USB_IE_DETECT_W { w: self }
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_usb_ie_trans(&mut self) -> RB_USB_IE_TRANS_W {
        RB_USB_IE_TRANS_W { w: self }
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_usb_ie_suspend(&mut self) -> RB_USB_IE_SUSPEND_W {
        RB_USB_IE_SUSPEND_W { w: self }
    }
    #[doc = "Bit 3 - enable interrupt for host SOF timer action for USB host mode"]
    #[inline(always)]
    pub fn rb_usb_ie_sof(&mut self) -> RB_USB_IE_SOF_W {
        RB_USB_IE_SOF_W { w: self }
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_usb_ie_fifoov(&mut self) -> RB_USB_IE_FIFOOV_W {
        RB_USB_IE_FIFOOV_W { w: self }
    }
    #[doc = "Bit 5 - Setup packet end interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_setupact(&mut self) -> RB_USB_IE_SETUPACT_W {
        RB_USB_IE_SETUPACT_W { w: self }
    }
    #[doc = "Bit 6 - Synchronous transmission received control token packet interrupt"]
    #[inline(always)]
    pub fn rb_usb_ie_isoact(&mut self) -> RB_USB_IE_ISOACT_W {
        RB_USB_IE_ISOACT_W { w: self }
    }
    #[doc = "Bit 7 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_usb_ie_dev_nak(&mut self) -> RB_USB_IE_DEV_NAK_W {
        RB_USB_IE_DEV_NAK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_int_en](index.html) module"]
pub struct USB_INT_EN_SPEC;
impl crate::RegisterSpec for USB_INT_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_int_en::R](R) reader structure"]
impl crate::Readable for USB_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_int_en::W](W) writer structure"]
impl crate::Writable for USB_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_INT_EN to value 0"]
impl crate::Resettable for USB_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

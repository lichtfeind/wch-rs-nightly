#[doc = "Register `USB_CTRL` reader"]
pub struct R(crate::R<USB_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CTRL` writer"]
pub struct W(crate::W<USB_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CTRL_SPEC>;
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
impl From<crate::W<USB_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_USB_DMA_EN` reader - DMA enable and DMA interrupt enable for USB"]
pub struct RB_USB_DMA_EN_R(crate::FieldReader<bool, bool>);
impl RB_USB_DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_DMA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_DMA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_DMA_EN` writer - DMA enable and DMA interrupt enable for USB"]
pub struct RB_USB_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_DMA_EN_W<'a> {
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
#[doc = "Field `RB_USB_CLR_ALL` reader - force clear FIFO and count of USB"]
pub struct RB_USB_CLR_ALL_R(crate::FieldReader<bool, bool>);
impl RB_USB_CLR_ALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_CLR_ALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_CLR_ALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_CLR_ALL` writer - force clear FIFO and count of USB"]
pub struct RB_USB_CLR_ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_CLR_ALL_W<'a> {
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
#[doc = "Field `RB_USB_RESET_SIE` reader - force reset USB SIE, need software clear"]
pub struct RB_USB_RESET_SIE_R(crate::FieldReader<bool, bool>);
impl RB_USB_RESET_SIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_RESET_SIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_RESET_SIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_RESET_SIE` writer - force reset USB SIE, need software clear"]
pub struct RB_USB_RESET_SIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_RESET_SIE_W<'a> {
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
#[doc = "Field `RB_USB_INT_BUSY` reader - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
pub struct RB_USB_INT_BUSY_R(crate::FieldReader<bool, bool>);
impl RB_USB_INT_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_INT_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_INT_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_INT_BUSY` writer - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
pub struct RB_USB_INT_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_INT_BUSY_W<'a> {
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
#[doc = "Field `RB_DEV_PU_EN` reader - USB device enable and internal pullup resistance enable"]
pub struct RB_DEV_PU_EN_R(crate::FieldReader<bool, bool>);
impl RB_DEV_PU_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_DEV_PU_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_DEV_PU_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_DEV_PU_EN` writer - USB device enable and internal pullup resistance enable"]
pub struct RB_DEV_PU_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_DEV_PU_EN_W<'a> {
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
#[doc = "Field `RB_USB_SPTP_MASK` reader - enable USB low speed"]
pub struct RB_USB_SPTP_MASK_R(crate::FieldReader<u8, u8>);
impl RB_USB_SPTP_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_USB_SPTP_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_SPTP_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_SPTP_MASK` writer - enable USB low speed"]
pub struct RB_USB_SPTP_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_SPTP_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 5)) | ((value as u8 & 3) << 5);
        self.w
    }
}
#[doc = "Field `RB_USB_MODE` reader - enable USB host mode: 0=device mode, 1=host mode"]
pub struct RB_USB_MODE_R(crate::FieldReader<bool, bool>);
impl RB_USB_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_USB_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_USB_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_USB_MODE` writer - enable USB host mode: 0=device mode, 1=host mode"]
pub struct RB_USB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_USB_MODE_W<'a> {
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
    #[doc = "Bit 0 - DMA enable and DMA interrupt enable for USB"]
    #[inline(always)]
    pub fn rb_usb_dma_en(&self) -> RB_USB_DMA_EN_R {
        RB_USB_DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    pub fn rb_usb_clr_all(&self) -> RB_USB_CLR_ALL_R {
        RB_USB_CLR_ALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - force reset USB SIE, need software clear"]
    #[inline(always)]
    pub fn rb_usb_reset_sie(&self) -> RB_USB_RESET_SIE_R {
        RB_USB_RESET_SIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
    #[inline(always)]
    pub fn rb_usb_int_busy(&self) -> RB_USB_INT_BUSY_R {
        RB_USB_INT_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    pub fn rb_dev_pu_en(&self) -> RB_DEV_PU_EN_R {
        RB_DEV_PU_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - enable USB low speed"]
    #[inline(always)]
    pub fn rb_usb_sptp_mask(&self) -> RB_USB_SPTP_MASK_R {
        RB_USB_SPTP_MASK_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - enable USB host mode: 0=device mode, 1=host mode"]
    #[inline(always)]
    pub fn rb_usb_mode(&self) -> RB_USB_MODE_R {
        RB_USB_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA enable and DMA interrupt enable for USB"]
    #[inline(always)]
    pub fn rb_usb_dma_en(&mut self) -> RB_USB_DMA_EN_W {
        RB_USB_DMA_EN_W { w: self }
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    pub fn rb_usb_clr_all(&mut self) -> RB_USB_CLR_ALL_W {
        RB_USB_CLR_ALL_W { w: self }
    }
    #[doc = "Bit 2 - force reset USB SIE, need software clear"]
    #[inline(always)]
    pub fn rb_usb_reset_sie(&mut self) -> RB_USB_RESET_SIE_W {
        RB_USB_RESET_SIE_W { w: self }
    }
    #[doc = "Bit 3 - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
    #[inline(always)]
    pub fn rb_usb_int_busy(&mut self) -> RB_USB_INT_BUSY_W {
        RB_USB_INT_BUSY_W { w: self }
    }
    #[doc = "Bit 4 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    pub fn rb_dev_pu_en(&mut self) -> RB_DEV_PU_EN_W {
        RB_DEV_PU_EN_W { w: self }
    }
    #[doc = "Bits 5:6 - enable USB low speed"]
    #[inline(always)]
    pub fn rb_usb_sptp_mask(&mut self) -> RB_USB_SPTP_MASK_W {
        RB_USB_SPTP_MASK_W { w: self }
    }
    #[doc = "Bit 7 - enable USB host mode: 0=device mode, 1=host mode"]
    #[inline(always)]
    pub fn rb_usb_mode(&mut self) -> RB_USB_MODE_W {
        RB_USB_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB base control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ctrl](index.html) module"]
pub struct USB_CTRL_SPEC;
impl crate::RegisterSpec for USB_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_ctrl::R](R) reader structure"]
impl crate::Readable for USB_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_ctrl::W](W) writer structure"]
impl crate::Writable for USB_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_CTRL to value 0x06"]
impl crate::Resettable for USB_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}

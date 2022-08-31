#[doc = "Register `USB_CONTROL` reader"]
pub struct R(crate::R<USB_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CONTROL` writer"]
pub struct W(crate::W<USB_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CONTROL_SPEC>;
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
impl From<crate::W<USB_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_EN` reader - None"]
pub struct DMA_EN_R(crate::FieldReader<bool, bool>);
impl DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_EN` writer - None"]
pub struct DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_EN_W<'a> {
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
#[doc = "Field `USB_ALL_CLR` reader - None"]
pub struct USB_ALL_CLR_R(crate::FieldReader<bool, bool>);
impl USB_ALL_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_ALL_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ALL_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_ALL_CLR` writer - None"]
pub struct USB_ALL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ALL_CLR_W<'a> {
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
#[doc = "Field `USB_FORCE_RST` reader - None"]
pub struct USB_FORCE_RST_R(crate::FieldReader<bool, bool>);
impl USB_FORCE_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_FORCE_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_FORCE_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_FORCE_RST` writer - None"]
pub struct USB_FORCE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_FORCE_RST_W<'a> {
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
#[doc = "Field `INT_BUSY_EN` reader - None"]
pub struct INT_BUSY_EN_R(crate::FieldReader<bool, bool>);
impl INT_BUSY_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_BUSY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_BUSY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_BUSY_EN` writer - None"]
pub struct INT_BUSY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_BUSY_EN_W<'a> {
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
#[doc = "Field `DIR_ABORT` reader - None"]
pub struct DIR_ABORT_R(crate::FieldReader<bool, bool>);
impl DIR_ABORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIR_ABORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_ABORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR_ABORT` writer - None"]
pub struct DIR_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_ABORT_W<'a> {
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
#[doc = "Field `SETUP_FLOW_EN` reader - if enable, data stage/status stage will begin after send ERDY"]
pub struct SETUP_FLOW_EN_R(crate::FieldReader<bool, bool>);
impl SETUP_FLOW_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_FLOW_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_FLOW_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUP_FLOW_EN` writer - if enable, data stage/status stage will begin after send ERDY"]
pub struct SETUP_FLOW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_FLOW_EN_W<'a> {
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
#[doc = "Field `ITP_EN` reader - None"]
pub struct ITP_EN_R(crate::FieldReader<bool, bool>);
impl ITP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITP_EN` writer - None"]
pub struct ITP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ITP_EN_W<'a> {
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
#[doc = "Field `HOST_MODE` reader - None"]
pub struct HOST_MODE_R(crate::FieldReader<bool, bool>);
impl HOST_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_MODE` writer - None"]
pub struct HOST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_MODE_W<'a> {
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
#[doc = "Field `USB_ACT_IE` reader - None"]
pub struct USB_ACT_IE_R(crate::FieldReader<bool, bool>);
impl USB_ACT_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_ACT_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ACT_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_ACT_IE` writer - None"]
pub struct USB_ACT_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ACT_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `USB_LMP_RX_IE` reader - None"]
pub struct USB_LMP_RX_IE_R(crate::FieldReader<bool, bool>);
impl USB_LMP_RX_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_LMP_RX_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_LMP_RX_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_LMP_RX_IE` writer - None"]
pub struct USB_LMP_RX_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_LMP_RX_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `USB_LMP_TX_IE` reader - None"]
pub struct USB_LMP_TX_IE_R(crate::FieldReader<bool, bool>);
impl USB_LMP_TX_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_LMP_TX_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_LMP_TX_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_LMP_TX_IE` writer - None"]
pub struct USB_LMP_TX_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_LMP_TX_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `USB_ITP_IE` reader - None"]
pub struct USB_ITP_IE_R(crate::FieldReader<bool, bool>);
impl USB_ITP_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_ITP_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ITP_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_ITP_IE` writer - None"]
pub struct USB_ITP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ITP_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `USB_OV_IE` reader - None"]
pub struct USB_OV_IE_R(crate::FieldReader<bool, bool>);
impl USB_OV_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_OV_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_OV_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_OV_IE` writer - None"]
pub struct USB_OV_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_OV_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `USB_ERDY_IE` reader - None"]
pub struct USB_ERDY_IE_R(crate::FieldReader<bool, bool>);
impl USB_ERDY_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_ERDY_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ERDY_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_ERDY_IE` writer - None"]
pub struct USB_ERDY_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ERDY_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `ADDR` reader - USB device address"]
pub struct ADDR_R(crate::FieldReader<u8, u8>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - USB device address"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - None"]
    #[inline(always)]
    pub fn usb_all_clr(&self) -> USB_ALL_CLR_R {
        USB_ALL_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn usb_force_rst(&self) -> USB_FORCE_RST_R {
        USB_FORCE_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - None"]
    #[inline(always)]
    pub fn int_busy_en(&self) -> INT_BUSY_EN_R {
        INT_BUSY_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - None"]
    #[inline(always)]
    pub fn dir_abort(&self) -> DIR_ABORT_R {
        DIR_ABORT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - if enable, data stage/status stage will begin after send ERDY"]
    #[inline(always)]
    pub fn setup_flow_en(&self) -> SETUP_FLOW_EN_R {
        SETUP_FLOW_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn itp_en(&self) -> ITP_EN_R {
        ITP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn host_mode(&self) -> HOST_MODE_R {
        HOST_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - None"]
    #[inline(always)]
    pub fn usb_act_ie(&self) -> USB_ACT_IE_R {
        USB_ACT_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - None"]
    #[inline(always)]
    pub fn usb_lmp_rx_ie(&self) -> USB_LMP_RX_IE_R {
        USB_LMP_RX_IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - None"]
    #[inline(always)]
    pub fn usb_lmp_tx_ie(&self) -> USB_LMP_TX_IE_R {
        USB_LMP_TX_IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - None"]
    #[inline(always)]
    pub fn usb_itp_ie(&self) -> USB_ITP_IE_R {
        USB_ITP_IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - None"]
    #[inline(always)]
    pub fn usb_ov_ie(&self) -> USB_OV_IE_R {
        USB_OV_IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - None"]
    #[inline(always)]
    pub fn usb_erdy_ie(&self) -> USB_ERDY_IE_R {
        USB_ERDY_IE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - USB device address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DMA_EN_W {
        DMA_EN_W { w: self }
    }
    #[doc = "Bit 1 - None"]
    #[inline(always)]
    pub fn usb_all_clr(&mut self) -> USB_ALL_CLR_W {
        USB_ALL_CLR_W { w: self }
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn usb_force_rst(&mut self) -> USB_FORCE_RST_W {
        USB_FORCE_RST_W { w: self }
    }
    #[doc = "Bit 3 - None"]
    #[inline(always)]
    pub fn int_busy_en(&mut self) -> INT_BUSY_EN_W {
        INT_BUSY_EN_W { w: self }
    }
    #[doc = "Bit 4 - None"]
    #[inline(always)]
    pub fn dir_abort(&mut self) -> DIR_ABORT_W {
        DIR_ABORT_W { w: self }
    }
    #[doc = "Bit 5 - if enable, data stage/status stage will begin after send ERDY"]
    #[inline(always)]
    pub fn setup_flow_en(&mut self) -> SETUP_FLOW_EN_W {
        SETUP_FLOW_EN_W { w: self }
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn itp_en(&mut self) -> ITP_EN_W {
        ITP_EN_W { w: self }
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn host_mode(&mut self) -> HOST_MODE_W {
        HOST_MODE_W { w: self }
    }
    #[doc = "Bit 16 - None"]
    #[inline(always)]
    pub fn usb_act_ie(&mut self) -> USB_ACT_IE_W {
        USB_ACT_IE_W { w: self }
    }
    #[doc = "Bit 17 - None"]
    #[inline(always)]
    pub fn usb_lmp_rx_ie(&mut self) -> USB_LMP_RX_IE_W {
        USB_LMP_RX_IE_W { w: self }
    }
    #[doc = "Bit 18 - None"]
    #[inline(always)]
    pub fn usb_lmp_tx_ie(&mut self) -> USB_LMP_TX_IE_W {
        USB_LMP_TX_IE_W { w: self }
    }
    #[doc = "Bit 19 - None"]
    #[inline(always)]
    pub fn usb_itp_ie(&mut self) -> USB_ITP_IE_W {
        USB_ITP_IE_W { w: self }
    }
    #[doc = "Bit 20 - None"]
    #[inline(always)]
    pub fn usb_ov_ie(&mut self) -> USB_OV_IE_W {
        USB_OV_IE_W { w: self }
    }
    #[doc = "Bit 21 - None"]
    #[inline(always)]
    pub fn usb_erdy_ie(&mut self) -> USB_ERDY_IE_W {
        USB_ERDY_IE_W { w: self }
    }
    #[doc = "Bits 24:31 - USB device address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_control](index.html) module"]
pub struct USB_CONTROL_SPEC;
impl crate::RegisterSpec for USB_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_control::R](R) reader structure"]
impl crate::Readable for USB_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_control::W](W) writer structure"]
impl crate::Writable for USB_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_CONTROL to value 0"]
impl crate::Resettable for USB_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

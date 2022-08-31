#[doc = "Register `USB_STATUS` reader"]
pub struct R(crate::R<USB_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_STATUS` writer"]
pub struct W(crate::W<USB_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_STATUS_SPEC>;
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
impl From<crate::W<USB_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT_FLAG` reader - None"]
pub struct ACT_FLAG_R(crate::FieldReader<bool, bool>);
impl ACT_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACT_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_FLAG` writer - None"]
pub struct ACT_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_FLAG_W<'a> {
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
#[doc = "Field `LMP_RX_FLAG` reader - None"]
pub struct LMP_RX_FLAG_R(crate::FieldReader<bool, bool>);
impl LMP_RX_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LMP_RX_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LMP_RX_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LMP_RX_FLAG` writer - None"]
pub struct LMP_RX_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> LMP_RX_FLAG_W<'a> {
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
#[doc = "Field `LMP_TX_FLAG` reader - None"]
pub struct LMP_TX_FLAG_R(crate::FieldReader<bool, bool>);
impl LMP_TX_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LMP_TX_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LMP_TX_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LMP_TX_FLAG` writer - None"]
pub struct LMP_TX_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> LMP_TX_FLAG_W<'a> {
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
#[doc = "Field `ITP_RX_FLAG` reader - None"]
pub struct ITP_RX_FLAG_R(crate::FieldReader<bool, bool>);
impl ITP_RX_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ITP_RX_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITP_RX_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITP_RX_FLAG` writer - None"]
pub struct ITP_RX_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> ITP_RX_FLAG_W<'a> {
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
#[doc = "Field `OV_FLAG` reader - None"]
pub struct OV_FLAG_R(crate::FieldReader<bool, bool>);
impl OV_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OV_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OV_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OV_FLAG` writer - None"]
pub struct OV_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> OV_FLAG_W<'a> {
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
#[doc = "Field `USB_ERDY_FLAG` reader - None"]
pub struct USB_ERDY_FLAG_R(crate::FieldReader<bool, bool>);
impl USB_ERDY_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_ERDY_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ERDY_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_ERDY_FLAG` writer - None"]
pub struct USB_ERDY_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ERDY_FLAG_W<'a> {
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
#[doc = "Field `USB_INT_EP` reader - None"]
pub struct USB_INT_EP_R(crate::FieldReader<u8, u8>);
impl USB_INT_EP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_INT_EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_INT_EP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_INT_EP` writer - None"]
pub struct USB_INT_EP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_INT_EP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Field `IN_OUT` reader - None"]
pub struct IN_OUT_R(crate::FieldReader<bool, bool>);
impl IN_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_OUT` writer - None"]
pub struct IN_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "None\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_INT_RES_A {
    #[doc = "0: a"]
    ACK = 0,
    #[doc = "1: a"]
    ERDY = 1,
    #[doc = "2: a"]
    NRDY = 2,
    #[doc = "3: a"]
    STALL = 3,
}
impl From<USB_INT_RES_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_INT_RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_INT_RES` reader - None"]
pub struct USB_INT_RES_R(crate::FieldReader<u8, USB_INT_RES_A>);
impl USB_INT_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_INT_RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_INT_RES_A {
        match self.bits {
            0 => USB_INT_RES_A::ACK,
            1 => USB_INT_RES_A::ERDY,
            2 => USB_INT_RES_A::NRDY,
            3 => USB_INT_RES_A::STALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        **self == USB_INT_RES_A::ACK
    }
    #[doc = "Checks if the value of the field is `ERDY`"]
    #[inline(always)]
    pub fn is_erdy(&self) -> bool {
        **self == USB_INT_RES_A::ERDY
    }
    #[doc = "Checks if the value of the field is `NRDY`"]
    #[inline(always)]
    pub fn is_nrdy(&self) -> bool {
        **self == USB_INT_RES_A::NRDY
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        **self == USB_INT_RES_A::STALL
    }
}
impl core::ops::Deref for USB_INT_RES_R {
    type Target = crate::FieldReader<u8, USB_INT_RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_INT_RES` writer - None"]
pub struct USB_INT_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_INT_RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_INT_RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "a"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(USB_INT_RES_A::ACK)
    }
    #[doc = "a"]
    #[inline(always)]
    pub fn erdy(self) -> &'a mut W {
        self.variant(USB_INT_RES_A::ERDY)
    }
    #[doc = "a"]
    #[inline(always)]
    pub fn nrdy(self) -> &'a mut W {
        self.variant(USB_INT_RES_A::NRDY)
    }
    #[doc = "a"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(USB_INT_RES_A::STALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `USB_NUMP` reader - None"]
pub struct USB_NUMP_R(crate::FieldReader<u8, u8>);
impl USB_NUMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_NUMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_NUMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_NUMP` writer - None"]
pub struct USB_NUMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_NUMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
        self.w
    }
}
#[doc = "Field `SEQ` reader - None"]
pub struct SEQ_R(crate::FieldReader<u8, u8>);
impl SEQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQ` writer - None"]
pub struct SEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | ((value as u32 & 0x1f) << 21);
        self.w
    }
}
#[doc = "Field `TX_LPF` reader - None"]
pub struct TX_LPF_R(crate::FieldReader<bool, bool>);
impl TX_LPF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_LPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LPF` writer - None"]
pub struct TX_LPF_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `STATUS_IS` reader - None"]
pub struct STATUS_IS_R(crate::FieldReader<bool, bool>);
impl STATUS_IS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_IS` writer - None"]
pub struct STATUS_IS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_IS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `SETUP_IS` reader - None"]
pub struct SETUP_IS_R(crate::FieldReader<bool, bool>);
impl SETUP_IS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUP_IS` writer - None"]
pub struct SETUP_IS_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_IS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `EP_INT_FLAG` reader - None"]
pub struct EP_INT_FLAG_R(crate::FieldReader<bool, bool>);
impl EP_INT_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_INT_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_INT_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_INT_FLAG` writer - None"]
pub struct EP_INT_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_INT_FLAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn act_flag(&self) -> ACT_FLAG_R {
        ACT_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - None"]
    #[inline(always)]
    pub fn lmp_rx_flag(&self) -> LMP_RX_FLAG_R {
        LMP_RX_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn lmp_tx_flag(&self) -> LMP_TX_FLAG_R {
        LMP_TX_FLAG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - None"]
    #[inline(always)]
    pub fn itp_rx_flag(&self) -> ITP_RX_FLAG_R {
        ITP_RX_FLAG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - None"]
    #[inline(always)]
    pub fn ov_flag(&self) -> OV_FLAG_R {
        OV_FLAG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - None"]
    #[inline(always)]
    pub fn usb_erdy_flag(&self) -> USB_ERDY_FLAG_R {
        USB_ERDY_FLAG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:10 - None"]
    #[inline(always)]
    pub fn usb_int_ep(&self) -> USB_INT_EP_R {
        USB_INT_EP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - None"]
    #[inline(always)]
    pub fn in_out(&self) -> IN_OUT_R {
        IN_OUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - None"]
    #[inline(always)]
    pub fn usb_int_res(&self) -> USB_INT_RES_R {
        USB_INT_RES_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:22 - None"]
    #[inline(always)]
    pub fn usb_nump(&self) -> USB_NUMP_R {
        USB_NUMP_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - None"]
    #[inline(always)]
    pub fn seq(&self) -> SEQ_R {
        SEQ_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - None"]
    #[inline(always)]
    pub fn tx_lpf(&self) -> TX_LPF_R {
        TX_LPF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - None"]
    #[inline(always)]
    pub fn status_is(&self) -> STATUS_IS_R {
        STATUS_IS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - None"]
    #[inline(always)]
    pub fn setup_is(&self) -> SETUP_IS_R {
        SETUP_IS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - None"]
    #[inline(always)]
    pub fn ep_int_flag(&self) -> EP_INT_FLAG_R {
        EP_INT_FLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn act_flag(&mut self) -> ACT_FLAG_W {
        ACT_FLAG_W { w: self }
    }
    #[doc = "Bit 1 - None"]
    #[inline(always)]
    pub fn lmp_rx_flag(&mut self) -> LMP_RX_FLAG_W {
        LMP_RX_FLAG_W { w: self }
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn lmp_tx_flag(&mut self) -> LMP_TX_FLAG_W {
        LMP_TX_FLAG_W { w: self }
    }
    #[doc = "Bit 3 - None"]
    #[inline(always)]
    pub fn itp_rx_flag(&mut self) -> ITP_RX_FLAG_W {
        ITP_RX_FLAG_W { w: self }
    }
    #[doc = "Bit 4 - None"]
    #[inline(always)]
    pub fn ov_flag(&mut self) -> OV_FLAG_W {
        OV_FLAG_W { w: self }
    }
    #[doc = "Bit 5 - None"]
    #[inline(always)]
    pub fn usb_erdy_flag(&mut self) -> USB_ERDY_FLAG_W {
        USB_ERDY_FLAG_W { w: self }
    }
    #[doc = "Bits 8:10 - None"]
    #[inline(always)]
    pub fn usb_int_ep(&mut self) -> USB_INT_EP_W {
        USB_INT_EP_W { w: self }
    }
    #[doc = "Bit 12 - None"]
    #[inline(always)]
    pub fn in_out(&mut self) -> IN_OUT_W {
        IN_OUT_W { w: self }
    }
    #[doc = "Bits 14:15 - None"]
    #[inline(always)]
    pub fn usb_int_res(&mut self) -> USB_INT_RES_W {
        USB_INT_RES_W { w: self }
    }
    #[doc = "Bits 18:22 - None"]
    #[inline(always)]
    pub fn usb_nump(&mut self) -> USB_NUMP_W {
        USB_NUMP_W { w: self }
    }
    #[doc = "Bits 21:25 - None"]
    #[inline(always)]
    pub fn seq(&mut self) -> SEQ_W {
        SEQ_W { w: self }
    }
    #[doc = "Bit 28 - None"]
    #[inline(always)]
    pub fn tx_lpf(&mut self) -> TX_LPF_W {
        TX_LPF_W { w: self }
    }
    #[doc = "Bit 29 - None"]
    #[inline(always)]
    pub fn status_is(&mut self) -> STATUS_IS_W {
        STATUS_IS_W { w: self }
    }
    #[doc = "Bit 30 - None"]
    #[inline(always)]
    pub fn setup_is(&mut self) -> SETUP_IS_W {
        SETUP_IS_W { w: self }
    }
    #[doc = "Bit 31 - None"]
    #[inline(always)]
    pub fn ep_int_flag(&mut self) -> EP_INT_FLAG_W {
        EP_INT_FLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_status](index.html) module"]
pub struct USB_STATUS_SPEC;
impl crate::RegisterSpec for USB_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_status::R](R) reader structure"]
impl crate::Readable for USB_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_status::W](W) writer structure"]
impl crate::Writable for USB_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_STATUS to value 0"]
impl crate::Resettable for USB_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

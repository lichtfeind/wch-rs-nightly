#[doc = "Register `LINK_STATUS` reader"]
pub struct R(crate::R<LINK_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINK_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINK_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINK_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINK_STATUS` writer"]
pub struct W(crate::W<LINK_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINK_STATUS_SPEC>;
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
impl From<crate::W<LINK_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINK_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINK_PRESENT` reader - None"]
pub struct LINK_PRESENT_R(crate::FieldReader<bool, bool>);
impl LINK_PRESENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_PRESENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_PRESENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_PRESENT` writer - None"]
pub struct LINK_PRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_PRESENT_W<'a> {
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
#[doc = "Field `RX_WARM_RESET` reader - None"]
pub struct RX_WARM_RESET_R(crate::FieldReader<bool, bool>);
impl RX_WARM_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_WARM_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WARM_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_WARM_RESET` writer - None"]
pub struct RX_WARM_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WARM_RESET_W<'a> {
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
#[doc = "Field `LINK_BUSY` reader - None"]
pub struct LINK_BUSY_R(crate::FieldReader<bool, bool>);
impl LINK_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_BUSY` writer - None"]
pub struct LINK_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_BUSY_W<'a> {
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
#[doc = "Field `LINK_READY` reader - None"]
pub struct LINK_READY_R(crate::FieldReader<bool, bool>);
impl LINK_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_READY` writer - None"]
pub struct LINK_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_READY_W<'a> {
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
#[doc = "Field `LINK_TXEQ` reader - None"]
pub struct LINK_TXEQ_R(crate::FieldReader<bool, bool>);
impl LINK_TXEQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_TXEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_TXEQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_TXEQ` writer - None"]
pub struct LINK_TXEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_TXEQ_W<'a> {
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
#[doc = "Field `LINK_RX_DETECT` reader - None"]
pub struct LINK_RX_DETECT_R(crate::FieldReader<bool, bool>);
impl LINK_RX_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_RX_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_RX_DETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_RX_DETECT` writer - None"]
pub struct LINK_RX_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_RX_DETECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `LINK_U0` reader - None"]
pub struct LINK_U0_R(crate::FieldReader<bool, bool>);
impl LINK_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_U0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_U0` writer - None"]
pub struct LINK_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_U0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `LINK_U1` reader - None"]
pub struct LINK_U1_R(crate::FieldReader<bool, bool>);
impl LINK_U1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_U1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_U1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_U1` writer - None"]
pub struct LINK_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_U1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `LINK_U2` reader - None"]
pub struct LINK_U2_R(crate::FieldReader<bool, bool>);
impl LINK_U2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_U2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_U2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_U2` writer - None"]
pub struct LINK_U2_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_U2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `LINK_U3` reader - None"]
pub struct LINK_U3_R(crate::FieldReader<bool, bool>);
impl LINK_U3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_U3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_U3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_U3` writer - None"]
pub struct LINK_U3_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_U3_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `LINK_DISABLED` reader - None"]
pub struct LINK_DISABLED_R(crate::FieldReader<bool, bool>);
impl LINK_DISABLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_DISABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_DISABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_DISABLED` writer - None"]
pub struct LINK_DISABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_DISABLED_W<'a> {
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
#[doc = "Field `LINK_INACT` reader - None"]
pub struct LINK_INACT_R(crate::FieldReader<bool, bool>);
impl LINK_INACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_INACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_INACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_INACT` writer - None"]
pub struct LINK_INACT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_INACT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `LINK_LOOPBACK` reader - None"]
pub struct LINK_LOOPBACK_R(crate::FieldReader<bool, bool>);
impl LINK_LOOPBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_LOOPBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_LOOPBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_LOOPBACK` writer - None"]
pub struct LINK_LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_LOOPBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `LINK_COMPLIANCE` reader - None"]
pub struct LINK_COMPLIANCE_R(crate::FieldReader<bool, bool>);
impl LINK_COMPLIANCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_COMPLIANCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_COMPLIANCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_COMPLIANCE` writer - None"]
pub struct LINK_COMPLIANCE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_COMPLIANCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `RX_Ux_EXIT_REQ` reader - None"]
pub struct RX_UX_EXIT_REQ_R(crate::FieldReader<bool, bool>);
impl RX_UX_EXIT_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_UX_EXIT_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_UX_EXIT_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_Ux_EXIT_REQ` writer - None"]
pub struct RX_UX_EXIT_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_UX_EXIT_REQ_W<'a> {
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
impl R {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn link_present(&self) -> LINK_PRESENT_R {
        LINK_PRESENT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - None"]
    #[inline(always)]
    pub fn rx_warm_reset(&self) -> RX_WARM_RESET_R {
        RX_WARM_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn link_busy(&self) -> LINK_BUSY_R {
        LINK_BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - None"]
    #[inline(always)]
    pub fn link_ready(&self) -> LINK_READY_R {
        LINK_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn link_txeq(&self) -> LINK_TXEQ_R {
        LINK_TXEQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - None"]
    #[inline(always)]
    pub fn link_rx_detect(&self) -> LINK_RX_DETECT_R {
        LINK_RX_DETECT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - None"]
    #[inline(always)]
    pub fn link_u0(&self) -> LINK_U0_R {
        LINK_U0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - None"]
    #[inline(always)]
    pub fn link_u1(&self) -> LINK_U1_R {
        LINK_U1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - None"]
    #[inline(always)]
    pub fn link_u2(&self) -> LINK_U2_R {
        LINK_U2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - None"]
    #[inline(always)]
    pub fn link_u3(&self) -> LINK_U3_R {
        LINK_U3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - None"]
    #[inline(always)]
    pub fn link_disabled(&self) -> LINK_DISABLED_R {
        LINK_DISABLED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - None"]
    #[inline(always)]
    pub fn link_inact(&self) -> LINK_INACT_R {
        LINK_INACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - None"]
    #[inline(always)]
    pub fn link_loopback(&self) -> LINK_LOOPBACK_R {
        LINK_LOOPBACK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - None"]
    #[inline(always)]
    pub fn link_compliance(&self) -> LINK_COMPLIANCE_R {
        LINK_COMPLIANCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - None"]
    #[inline(always)]
    pub fn rx_ux_exit_req(&self) -> RX_UX_EXIT_REQ_R {
        RX_UX_EXIT_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn link_present(&mut self) -> LINK_PRESENT_W {
        LINK_PRESENT_W { w: self }
    }
    #[doc = "Bit 1 - None"]
    #[inline(always)]
    pub fn rx_warm_reset(&mut self) -> RX_WARM_RESET_W {
        RX_WARM_RESET_W { w: self }
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn link_busy(&mut self) -> LINK_BUSY_W {
        LINK_BUSY_W { w: self }
    }
    #[doc = "Bit 3 - None"]
    #[inline(always)]
    pub fn link_ready(&mut self) -> LINK_READY_W {
        LINK_READY_W { w: self }
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn link_txeq(&mut self) -> LINK_TXEQ_W {
        LINK_TXEQ_W { w: self }
    }
    #[doc = "Bit 7 - None"]
    #[inline(always)]
    pub fn link_rx_detect(&mut self) -> LINK_RX_DETECT_W {
        LINK_RX_DETECT_W { w: self }
    }
    #[doc = "Bit 8 - None"]
    #[inline(always)]
    pub fn link_u0(&mut self) -> LINK_U0_W {
        LINK_U0_W { w: self }
    }
    #[doc = "Bit 9 - None"]
    #[inline(always)]
    pub fn link_u1(&mut self) -> LINK_U1_W {
        LINK_U1_W { w: self }
    }
    #[doc = "Bit 10 - None"]
    #[inline(always)]
    pub fn link_u2(&mut self) -> LINK_U2_W {
        LINK_U2_W { w: self }
    }
    #[doc = "Bit 11 - None"]
    #[inline(always)]
    pub fn link_u3(&mut self) -> LINK_U3_W {
        LINK_U3_W { w: self }
    }
    #[doc = "Bit 12 - None"]
    #[inline(always)]
    pub fn link_disabled(&mut self) -> LINK_DISABLED_W {
        LINK_DISABLED_W { w: self }
    }
    #[doc = "Bit 13 - None"]
    #[inline(always)]
    pub fn link_inact(&mut self) -> LINK_INACT_W {
        LINK_INACT_W { w: self }
    }
    #[doc = "Bit 14 - None"]
    #[inline(always)]
    pub fn link_loopback(&mut self) -> LINK_LOOPBACK_W {
        LINK_LOOPBACK_W { w: self }
    }
    #[doc = "Bit 15 - None"]
    #[inline(always)]
    pub fn link_compliance(&mut self) -> LINK_COMPLIANCE_W {
        LINK_COMPLIANCE_W { w: self }
    }
    #[doc = "Bit 16 - None"]
    #[inline(always)]
    pub fn rx_ux_exit_req(&mut self) -> RX_UX_EXIT_REQ_W {
        RX_UX_EXIT_REQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [link_status](index.html) module"]
pub struct LINK_STATUS_SPEC;
impl crate::RegisterSpec for LINK_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [link_status::R](R) reader structure"]
impl crate::Readable for LINK_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [link_status::W](W) writer structure"]
impl crate::Writable for LINK_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINK_STATUS to value 0"]
impl crate::Resettable for LINK_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

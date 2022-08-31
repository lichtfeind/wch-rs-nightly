#[doc = "Register `LINK_INT_FLAG` reader"]
pub struct R(crate::R<LINK_INT_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINK_INT_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINK_INT_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINK_INT_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINK_INT_FLAG` writer"]
pub struct W(crate::W<LINK_INT_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINK_INT_FLAG_SPEC>;
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
impl From<crate::W<LINK_INT_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINK_INT_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINK_RDY` reader - None"]
pub struct LINK_RDY_R(crate::FieldReader<bool, bool>);
impl LINK_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_RDY` writer - None"]
pub struct LINK_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_RDY_W<'a> {
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
#[doc = "Field `LINK_RECOV` reader - None"]
pub struct LINK_RECOV_R(crate::FieldReader<bool, bool>);
impl LINK_RECOV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_RECOV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_RECOV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_RECOV` writer - None"]
pub struct LINK_RECOV_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_RECOV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `LINK_DISABLE` reader - None"]
pub struct LINK_DISABLE_R(crate::FieldReader<bool, bool>);
impl LINK_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_DISABLE` writer - None"]
pub struct LINK_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_DISABLE_W<'a> {
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
#[doc = "Field `LINK_GO_U3` reader - None"]
pub struct LINK_GO_U3_R(crate::FieldReader<bool, bool>);
impl LINK_GO_U3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_GO_U3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_GO_U3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_GO_U3` writer - None"]
pub struct LINK_GO_U3_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_GO_U3_W<'a> {
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
#[doc = "Field `LINK_GO_U2` reader - None"]
pub struct LINK_GO_U2_R(crate::FieldReader<bool, bool>);
impl LINK_GO_U2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_GO_U2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_GO_U2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_GO_U2` writer - None"]
pub struct LINK_GO_U2_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_GO_U2_W<'a> {
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
#[doc = "Field `LINK_GO_U1` reader - None"]
pub struct LINK_GO_U1_R(crate::FieldReader<bool, bool>);
impl LINK_GO_U1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_GO_U1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_GO_U1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_GO_U1` writer - None"]
pub struct LINK_GO_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_GO_U1_W<'a> {
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
#[doc = "Field `LINK_GO_U0` reader - None"]
pub struct LINK_GO_U0_R(crate::FieldReader<bool, bool>);
impl LINK_GO_U0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_GO_U0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_GO_U0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_GO_U0` writer - None"]
pub struct LINK_GO_U0_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_GO_U0_W<'a> {
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
#[doc = "Field `LINK_U3_WAKE` reader - None"]
pub struct LINK_U3_WAKE_R(crate::FieldReader<bool, bool>);
impl LINK_U3_WAKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_U3_WAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_U3_WAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_U3_WAKE` writer - None"]
pub struct LINK_U3_WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_U3_WAKE_W<'a> {
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
#[doc = "Field `LINK_Ux_REJECT` reader - None"]
pub struct LINK_UX_REJECT_R(crate::FieldReader<bool, bool>);
impl LINK_UX_REJECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_UX_REJECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_UX_REJECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_Ux_REJECT` writer - None"]
pub struct LINK_UX_REJECT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_UX_REJECT_W<'a> {
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
#[doc = "Field `TERM_PRESENT` reader - None"]
pub struct TERM_PRESENT_R(crate::FieldReader<bool, bool>);
impl TERM_PRESENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TERM_PRESENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERM_PRESENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERM_PRESENT` writer - None"]
pub struct TERM_PRESENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TERM_PRESENT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `LINK_Ux_EXIT` reader - None"]
pub struct LINK_UX_EXIT_R(crate::FieldReader<bool, bool>);
impl LINK_UX_EXIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_UX_EXIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_UX_EXIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_Ux_EXIT` writer - None"]
pub struct LINK_UX_EXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_UX_EXIT_W<'a> {
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
#[doc = "Field `WARM_RESET` reader - None"]
pub struct WARM_RESET_R(crate::FieldReader<bool, bool>);
impl WARM_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WARM_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WARM_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WARM_RESET` writer - None"]
pub struct WARM_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> WARM_RESET_W<'a> {
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
#[doc = "Field `U3_WAKEUP` reader - None"]
pub struct U3_WAKEUP_R(crate::FieldReader<bool, bool>);
impl U3_WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        U3_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U3_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U3_WAKEUP` writer - None"]
pub struct U3_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> U3_WAKEUP_W<'a> {
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
#[doc = "Field `HOT_RESET` reader - None"]
pub struct HOT_RESET_R(crate::FieldReader<bool, bool>);
impl HOT_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOT_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOT_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOT_RESET` writer - None"]
pub struct HOT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> HOT_RESET_W<'a> {
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
#[doc = "Field `LINK_RX_DET` reader - None"]
pub struct LINK_RX_DET_R(crate::FieldReader<bool, bool>);
impl LINK_RX_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_RX_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_RX_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_RX_DET` writer - None"]
pub struct LINK_RX_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_RX_DET_W<'a> {
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
impl R {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn link_rdy(&self) -> LINK_RDY_R {
        LINK_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - None"]
    #[inline(always)]
    pub fn link_recov(&self) -> LINK_RECOV_R {
        LINK_RECOV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn link_inact(&self) -> LINK_INACT_R {
        LINK_INACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - None"]
    #[inline(always)]
    pub fn link_disable(&self) -> LINK_DISABLE_R {
        LINK_DISABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - None"]
    #[inline(always)]
    pub fn link_go_u3(&self) -> LINK_GO_U3_R {
        LINK_GO_U3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - None"]
    #[inline(always)]
    pub fn link_go_u2(&self) -> LINK_GO_U2_R {
        LINK_GO_U2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn link_go_u1(&self) -> LINK_GO_U1_R {
        LINK_GO_U1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - None"]
    #[inline(always)]
    pub fn link_go_u0(&self) -> LINK_GO_U0_R {
        LINK_GO_U0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - None"]
    #[inline(always)]
    pub fn link_u3_wake(&self) -> LINK_U3_WAKE_R {
        LINK_U3_WAKE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - None"]
    #[inline(always)]
    pub fn link_ux_reject(&self) -> LINK_UX_REJECT_R {
        LINK_UX_REJECT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - None"]
    #[inline(always)]
    pub fn term_present(&self) -> TERM_PRESENT_R {
        TERM_PRESENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - None"]
    #[inline(always)]
    pub fn link_txeq(&self) -> LINK_TXEQ_R {
        LINK_TXEQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - None"]
    #[inline(always)]
    pub fn link_ux_exit(&self) -> LINK_UX_EXIT_R {
        LINK_UX_EXIT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - None"]
    #[inline(always)]
    pub fn warm_reset(&self) -> WARM_RESET_R {
        WARM_RESET_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - None"]
    #[inline(always)]
    pub fn u3_wakeup(&self) -> U3_WAKEUP_R {
        U3_WAKEUP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - None"]
    #[inline(always)]
    pub fn hot_reset(&self) -> HOT_RESET_R {
        HOT_RESET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - None"]
    #[inline(always)]
    pub fn link_rx_det(&self) -> LINK_RX_DET_R {
        LINK_RX_DET_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn link_rdy(&mut self) -> LINK_RDY_W {
        LINK_RDY_W { w: self }
    }
    #[doc = "Bit 1 - None"]
    #[inline(always)]
    pub fn link_recov(&mut self) -> LINK_RECOV_W {
        LINK_RECOV_W { w: self }
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn link_inact(&mut self) -> LINK_INACT_W {
        LINK_INACT_W { w: self }
    }
    #[doc = "Bit 3 - None"]
    #[inline(always)]
    pub fn link_disable(&mut self) -> LINK_DISABLE_W {
        LINK_DISABLE_W { w: self }
    }
    #[doc = "Bit 4 - None"]
    #[inline(always)]
    pub fn link_go_u3(&mut self) -> LINK_GO_U3_W {
        LINK_GO_U3_W { w: self }
    }
    #[doc = "Bit 5 - None"]
    #[inline(always)]
    pub fn link_go_u2(&mut self) -> LINK_GO_U2_W {
        LINK_GO_U2_W { w: self }
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn link_go_u1(&mut self) -> LINK_GO_U1_W {
        LINK_GO_U1_W { w: self }
    }
    #[doc = "Bit 7 - None"]
    #[inline(always)]
    pub fn link_go_u0(&mut self) -> LINK_GO_U0_W {
        LINK_GO_U0_W { w: self }
    }
    #[doc = "Bit 8 - None"]
    #[inline(always)]
    pub fn link_u3_wake(&mut self) -> LINK_U3_WAKE_W {
        LINK_U3_WAKE_W { w: self }
    }
    #[doc = "Bit 9 - None"]
    #[inline(always)]
    pub fn link_ux_reject(&mut self) -> LINK_UX_REJECT_W {
        LINK_UX_REJECT_W { w: self }
    }
    #[doc = "Bit 10 - None"]
    #[inline(always)]
    pub fn term_present(&mut self) -> TERM_PRESENT_W {
        TERM_PRESENT_W { w: self }
    }
    #[doc = "Bit 11 - None"]
    #[inline(always)]
    pub fn link_txeq(&mut self) -> LINK_TXEQ_W {
        LINK_TXEQ_W { w: self }
    }
    #[doc = "Bit 12 - None"]
    #[inline(always)]
    pub fn link_ux_exit(&mut self) -> LINK_UX_EXIT_W {
        LINK_UX_EXIT_W { w: self }
    }
    #[doc = "Bit 13 - None"]
    #[inline(always)]
    pub fn warm_reset(&mut self) -> WARM_RESET_W {
        WARM_RESET_W { w: self }
    }
    #[doc = "Bit 14 - None"]
    #[inline(always)]
    pub fn u3_wakeup(&mut self) -> U3_WAKEUP_W {
        U3_WAKEUP_W { w: self }
    }
    #[doc = "Bit 15 - None"]
    #[inline(always)]
    pub fn hot_reset(&mut self) -> HOT_RESET_W {
        HOT_RESET_W { w: self }
    }
    #[doc = "Bit 20 - None"]
    #[inline(always)]
    pub fn link_rx_det(&mut self) -> LINK_RX_DET_W {
        LINK_RX_DET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [link_int_flag](index.html) module"]
pub struct LINK_INT_FLAG_SPEC;
impl crate::RegisterSpec for LINK_INT_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [link_int_flag::R](R) reader structure"]
impl crate::Readable for LINK_INT_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [link_int_flag::W](W) writer structure"]
impl crate::Writable for LINK_INT_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINK_INT_FLAG to value 0"]
impl crate::Resettable for LINK_INT_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

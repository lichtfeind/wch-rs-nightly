#[doc = "Register `LINK_CTRL` reader"]
pub struct R(crate::R<LINK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINK_CTRL` writer"]
pub struct W(crate::W<LINK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINK_CTRL_SPEC>;
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
impl From<crate::W<LINK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "None\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POWER_MODE_A {
    #[doc = "0: a"]
    MODE0 = 0,
    #[doc = "1: a"]
    MODE1 = 1,
    #[doc = "2: a"]
    MODE2 = 2,
    #[doc = "3: a"]
    MODE3 = 3,
}
impl From<POWER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: POWER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POWER_MODE` reader - None"]
pub struct POWER_MODE_R(crate::FieldReader<u8, POWER_MODE_A>);
impl POWER_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POWER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWER_MODE_A {
        match self.bits {
            0 => POWER_MODE_A::MODE0,
            1 => POWER_MODE_A::MODE1,
            2 => POWER_MODE_A::MODE2,
            3 => POWER_MODE_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        **self == POWER_MODE_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        **self == POWER_MODE_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        **self == POWER_MODE_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        **self == POWER_MODE_A::MODE3
    }
}
impl core::ops::Deref for POWER_MODE_R {
    type Target = crate::FieldReader<u8, POWER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_MODE` writer - None"]
pub struct POWER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POWER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "a"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(POWER_MODE_A::MODE0)
    }
    #[doc = "a"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(POWER_MODE_A::MODE1)
    }
    #[doc = "a"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(POWER_MODE_A::MODE2)
    }
    #[doc = "a"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(POWER_MODE_A::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `GO_DISABLED` reader - None"]
pub struct GO_DISABLED_R(crate::FieldReader<bool, bool>);
impl GO_DISABLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GO_DISABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GO_DISABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GO_DISABLED` writer - None"]
pub struct GO_DISABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> GO_DISABLED_W<'a> {
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
#[doc = "Field `GO_INACTIVE` reader - None"]
pub struct GO_INACTIVE_R(crate::FieldReader<bool, bool>);
impl GO_INACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GO_INACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GO_INACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GO_INACTIVE` writer - None"]
pub struct GO_INACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> GO_INACTIVE_W<'a> {
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
#[doc = "Field `GO_RECOVERY` reader - None"]
pub struct GO_RECOVERY_R(crate::FieldReader<bool, bool>);
impl GO_RECOVERY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GO_RECOVERY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GO_RECOVERY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GO_RECOVERY` writer - None"]
pub struct GO_RECOVERY_W<'a> {
    w: &'a mut W,
}
impl<'a> GO_RECOVERY_W<'a> {
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
#[doc = "Field `GO_RX_DET` reader - None"]
pub struct GO_RX_DET_R(crate::FieldReader<bool, bool>);
impl GO_RX_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GO_RX_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GO_RX_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GO_RX_DET` writer - None"]
pub struct GO_RX_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> GO_RX_DET_W<'a> {
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
#[doc = "Field `TX_WARM_RESET` reader - None"]
pub struct TX_WARM_RESET_R(crate::FieldReader<bool, bool>);
impl TX_WARM_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_WARM_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WARM_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WARM_RESET` writer - None"]
pub struct TX_WARM_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WARM_RESET_W<'a> {
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
#[doc = "Field `TX_UX_EXIT` reader - None"]
pub struct TX_UX_EXIT_R(crate::FieldReader<bool, bool>);
impl TX_UX_EXIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_UX_EXIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_UX_EXIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_UX_EXIT` writer - None"]
pub struct TX_UX_EXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_UX_EXIT_W<'a> {
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
#[doc = "Field `POLLING_EN` reader - None"]
pub struct POLLING_EN_R(crate::FieldReader<bool, bool>);
impl POLLING_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POLLING_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLLING_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLLING_EN` writer - None"]
pub struct POLLING_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLING_EN_W<'a> {
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
#[doc = "Field `TX_LGO_U1` reader - None"]
pub struct TX_LGO_U1_R(crate::FieldReader<bool, bool>);
impl TX_LGO_U1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_LGO_U1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LGO_U1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LGO_U1` writer - None"]
pub struct TX_LGO_U1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LGO_U1_W<'a> {
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
#[doc = "Field `TX_LGO_U2` reader - None"]
pub struct TX_LGO_U2_R(crate::FieldReader<bool, bool>);
impl TX_LGO_U2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_LGO_U2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LGO_U2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LGO_U2` writer - None"]
pub struct TX_LGO_U2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LGO_U2_W<'a> {
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
#[doc = "Field `TX_LGO_U3` reader - None"]
pub struct TX_LGO_U3_R(crate::FieldReader<bool, bool>);
impl TX_LGO_U3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_LGO_U3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LGO_U3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LGO_U3` writer - None"]
pub struct TX_LGO_U3_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LGO_U3_W<'a> {
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
#[doc = "Field `TX_HOT_RESET` reader - None"]
pub struct TX_HOT_RESET_R(crate::FieldReader<bool, bool>);
impl TX_HOT_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_HOT_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_HOT_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_HOT_RESET` writer - None"]
pub struct TX_HOT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_HOT_RESET_W<'a> {
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
    #[doc = "Bits 0:1 - None"]
    #[inline(always)]
    pub fn power_mode(&self) -> POWER_MODE_R {
        POWER_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - None"]
    #[inline(always)]
    pub fn go_disabled(&self) -> GO_DISABLED_R {
        GO_DISABLED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - None"]
    #[inline(always)]
    pub fn go_inactive(&self) -> GO_INACTIVE_R {
        GO_INACTIVE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn go_recovery(&self) -> GO_RECOVERY_R {
        GO_RECOVERY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - None"]
    #[inline(always)]
    pub fn go_rx_det(&self) -> GO_RX_DET_R {
        GO_RX_DET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - None"]
    #[inline(always)]
    pub fn tx_warm_reset(&self) -> TX_WARM_RESET_R {
        TX_WARM_RESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - None"]
    #[inline(always)]
    pub fn tx_ux_exit(&self) -> TX_UX_EXIT_R {
        TX_UX_EXIT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - None"]
    #[inline(always)]
    pub fn polling_en(&self) -> POLLING_EN_R {
        POLLING_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - None"]
    #[inline(always)]
    pub fn tx_lgo_u1(&self) -> TX_LGO_U1_R {
        TX_LGO_U1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - None"]
    #[inline(always)]
    pub fn tx_lgo_u2(&self) -> TX_LGO_U2_R {
        TX_LGO_U2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - None"]
    #[inline(always)]
    pub fn tx_lgo_u3(&self) -> TX_LGO_U3_R {
        TX_LGO_U3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - None"]
    #[inline(always)]
    pub fn tx_hot_reset(&self) -> TX_HOT_RESET_R {
        TX_HOT_RESET_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - None"]
    #[inline(always)]
    pub fn power_mode(&mut self) -> POWER_MODE_W {
        POWER_MODE_W { w: self }
    }
    #[doc = "Bit 4 - None"]
    #[inline(always)]
    pub fn go_disabled(&mut self) -> GO_DISABLED_W {
        GO_DISABLED_W { w: self }
    }
    #[doc = "Bit 5 - None"]
    #[inline(always)]
    pub fn go_inactive(&mut self) -> GO_INACTIVE_W {
        GO_INACTIVE_W { w: self }
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn go_recovery(&mut self) -> GO_RECOVERY_W {
        GO_RECOVERY_W { w: self }
    }
    #[doc = "Bit 7 - None"]
    #[inline(always)]
    pub fn go_rx_det(&mut self) -> GO_RX_DET_W {
        GO_RX_DET_W { w: self }
    }
    #[doc = "Bit 8 - None"]
    #[inline(always)]
    pub fn tx_warm_reset(&mut self) -> TX_WARM_RESET_W {
        TX_WARM_RESET_W { w: self }
    }
    #[doc = "Bit 9 - None"]
    #[inline(always)]
    pub fn tx_ux_exit(&mut self) -> TX_UX_EXIT_W {
        TX_UX_EXIT_W { w: self }
    }
    #[doc = "Bit 12 - None"]
    #[inline(always)]
    pub fn polling_en(&mut self) -> POLLING_EN_W {
        POLLING_EN_W { w: self }
    }
    #[doc = "Bit 13 - None"]
    #[inline(always)]
    pub fn tx_lgo_u1(&mut self) -> TX_LGO_U1_W {
        TX_LGO_U1_W { w: self }
    }
    #[doc = "Bit 14 - None"]
    #[inline(always)]
    pub fn tx_lgo_u2(&mut self) -> TX_LGO_U2_W {
        TX_LGO_U2_W { w: self }
    }
    #[doc = "Bit 15 - None"]
    #[inline(always)]
    pub fn tx_lgo_u3(&mut self) -> TX_LGO_U3_W {
        TX_LGO_U3_W { w: self }
    }
    #[doc = "Bit 16 - None"]
    #[inline(always)]
    pub fn tx_hot_reset(&mut self) -> TX_HOT_RESET_W {
        TX_HOT_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [link_ctrl](index.html) module"]
pub struct LINK_CTRL_SPEC;
impl crate::RegisterSpec for LINK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [link_ctrl::R](R) reader structure"]
impl crate::Readable for LINK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [link_ctrl::W](W) writer structure"]
impl crate::Writable for LINK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINK_CTRL to value 0"]
impl crate::Resettable for LINK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

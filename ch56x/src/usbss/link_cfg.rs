#[doc = "Register `LINK_CFG` reader"]
pub struct R(crate::R<LINK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINK_CFG` writer"]
pub struct W(crate::W<LINK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINK_CFG_SPEC>;
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
impl From<crate::W<LINK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOWN_FLAG` reader - None"]
pub struct DOWN_FLAG_R(crate::FieldReader<bool, bool>);
impl DOWN_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOWN_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOWN_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOWN_FLAG` writer - None"]
pub struct DOWN_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN_FLAG_W<'a> {
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
#[doc = "Field `TERM_EN` reader - None"]
pub struct TERM_EN_R(crate::FieldReader<bool, bool>);
impl TERM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TERM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERM_EN` writer - None"]
pub struct TERM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TERM_EN_W<'a> {
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
#[doc = "Field `DEVICE_PHY` reader - None"]
pub struct DEVICE_PHY_R(crate::FieldReader<bool, bool>);
impl DEVICE_PHY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEVICE_PHY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICE_PHY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVICE_PHY` writer - None"]
pub struct DEVICE_PHY_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICE_PHY_W<'a> {
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
#[doc = "Field `PIPE_RESET` reader - None"]
pub struct PIPE_RESET_R(crate::FieldReader<bool, bool>);
impl PIPE_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIPE_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIPE_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIPE_RESET` writer - None"]
pub struct PIPE_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PIPE_RESET_W<'a> {
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
#[doc = "Field `COMPLIANCE_EN` reader - None"]
pub struct COMPLIANCE_EN_R(crate::FieldReader<bool, bool>);
impl COMPLIANCE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPLIANCE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPLIANCE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPLIANCE_EN` writer - None"]
pub struct COMPLIANCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPLIANCE_EN_W<'a> {
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
#[doc = "Field `LFPS_RX_PD` reader - None"]
pub struct LFPS_RX_PD_R(crate::FieldReader<bool, bool>);
impl LFPS_RX_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LFPS_RX_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFPS_RX_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFPS_RX_PD` writer - None"]
pub struct LFPS_RX_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> LFPS_RX_PD_W<'a> {
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
#[doc = "Field `CFG_EQ_EN` reader - None"]
pub struct CFG_EQ_EN_R(crate::FieldReader<bool, bool>);
impl CFG_EQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_EQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_EQ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG_EQ_EN` writer - None"]
pub struct CFG_EQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_EQ_EN_W<'a> {
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
#[doc = "Field `TX_SWING` reader - None"]
pub struct TX_SWING_R(crate::FieldReader<bool, bool>);
impl TX_SWING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_SWING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SWING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SWING` writer - None"]
pub struct TX_SWING_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SWING_W<'a> {
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
#[doc = "Field `DEEMPH_CFG` reader - None"]
pub struct DEEMPH_CFG_R(crate::FieldReader<bool, bool>);
impl DEEMPH_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEEMPH_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEMPH_CFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEMPH_CFG` writer - None"]
pub struct DEEMPH_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEMPH_CFG_W<'a> {
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
#[doc = "Field `LPM_EN` reader - None"]
pub struct LPM_EN_R(crate::FieldReader<bool, bool>);
impl LPM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_EN` writer - None"]
pub struct LPM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_EN_W<'a> {
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
#[doc = "Field `U1_ALLOW` reader - None"]
pub struct U1_ALLOW_R(crate::FieldReader<bool, bool>);
impl U1_ALLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        U1_ALLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1_ALLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1_ALLOW` writer - None"]
pub struct U1_ALLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_ALLOW_W<'a> {
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
#[doc = "Field `U2_ALLOW` reader - None"]
pub struct U2_ALLOW_R(crate::FieldReader<bool, bool>);
impl U2_ALLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        U2_ALLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2_ALLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2_ALLOW` writer - None"]
pub struct U2_ALLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> U2_ALLOW_W<'a> {
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
#[doc = "Field `HP_PENDING` reader - None"]
pub struct HP_PENDING_R(crate::FieldReader<bool, bool>);
impl HP_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HP_PENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HP_PENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HP_PENDING` writer - None"]
pub struct HP_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> HP_PENDING_W<'a> {
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
    pub fn down_flag(&self) -> DOWN_FLAG_R {
        DOWN_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - None"]
    #[inline(always)]
    pub fn term_en(&self) -> TERM_EN_R {
        TERM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn device_phy(&self) -> DEVICE_PHY_R {
        DEVICE_PHY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - None"]
    #[inline(always)]
    pub fn pipe_reset(&self) -> PIPE_RESET_R {
        PIPE_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - None"]
    #[inline(always)]
    pub fn compliance_en(&self) -> COMPLIANCE_EN_R {
        COMPLIANCE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - None"]
    #[inline(always)]
    pub fn lfps_rx_pd(&self) -> LFPS_RX_PD_R {
        LFPS_RX_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn cfg_eq_en(&self) -> CFG_EQ_EN_R {
        CFG_EQ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - None"]
    #[inline(always)]
    pub fn tx_swing(&self) -> TX_SWING_R {
        TX_SWING_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - None"]
    #[inline(always)]
    pub fn deemph_cfg(&self) -> DEEMPH_CFG_R {
        DEEMPH_CFG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - None"]
    #[inline(always)]
    pub fn lpm_en(&self) -> LPM_EN_R {
        LPM_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - None"]
    #[inline(always)]
    pub fn u1_allow(&self) -> U1_ALLOW_R {
        U1_ALLOW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - None"]
    #[inline(always)]
    pub fn u2_allow(&self) -> U2_ALLOW_R {
        U2_ALLOW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - None"]
    #[inline(always)]
    pub fn hp_pending(&self) -> HP_PENDING_R {
        HP_PENDING_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - None"]
    #[inline(always)]
    pub fn down_flag(&mut self) -> DOWN_FLAG_W {
        DOWN_FLAG_W { w: self }
    }
    #[doc = "Bit 1 - None"]
    #[inline(always)]
    pub fn term_en(&mut self) -> TERM_EN_W {
        TERM_EN_W { w: self }
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn device_phy(&mut self) -> DEVICE_PHY_W {
        DEVICE_PHY_W { w: self }
    }
    #[doc = "Bit 3 - None"]
    #[inline(always)]
    pub fn pipe_reset(&mut self) -> PIPE_RESET_W {
        PIPE_RESET_W { w: self }
    }
    #[doc = "Bit 4 - None"]
    #[inline(always)]
    pub fn compliance_en(&mut self) -> COMPLIANCE_EN_W {
        COMPLIANCE_EN_W { w: self }
    }
    #[doc = "Bit 5 - None"]
    #[inline(always)]
    pub fn lfps_rx_pd(&mut self) -> LFPS_RX_PD_W {
        LFPS_RX_PD_W { w: self }
    }
    #[doc = "Bit 6 - None"]
    #[inline(always)]
    pub fn cfg_eq_en(&mut self) -> CFG_EQ_EN_W {
        CFG_EQ_EN_W { w: self }
    }
    #[doc = "Bit 7 - None"]
    #[inline(always)]
    pub fn tx_swing(&mut self) -> TX_SWING_W {
        TX_SWING_W { w: self }
    }
    #[doc = "Bit 8 - None"]
    #[inline(always)]
    pub fn deemph_cfg(&mut self) -> DEEMPH_CFG_W {
        DEEMPH_CFG_W { w: self }
    }
    #[doc = "Bit 12 - None"]
    #[inline(always)]
    pub fn lpm_en(&mut self) -> LPM_EN_W {
        LPM_EN_W { w: self }
    }
    #[doc = "Bit 16 - None"]
    #[inline(always)]
    pub fn u1_allow(&mut self) -> U1_ALLOW_W {
        U1_ALLOW_W { w: self }
    }
    #[doc = "Bit 17 - None"]
    #[inline(always)]
    pub fn u2_allow(&mut self) -> U2_ALLOW_W {
        U2_ALLOW_W { w: self }
    }
    #[doc = "Bit 20 - None"]
    #[inline(always)]
    pub fn hp_pending(&mut self) -> HP_PENDING_W {
        HP_PENDING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [link_cfg](index.html) module"]
pub struct LINK_CFG_SPEC;
impl crate::RegisterSpec for LINK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [link_cfg::R](R) reader structure"]
impl crate::Readable for LINK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [link_cfg::W](W) writer structure"]
impl crate::Writable for LINK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINK_CFG to value 0"]
impl crate::Resettable for LINK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

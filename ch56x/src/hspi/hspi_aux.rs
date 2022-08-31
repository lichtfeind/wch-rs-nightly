#[doc = "Register `HSPI_AUX` reader"]
pub struct R(crate::R<HSPI_AUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_AUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_AUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_AUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_AUX` writer"]
pub struct W(crate::W<HSPI_AUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_AUX_SPEC>;
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
impl From<crate::W<HSPI_AUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_AUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_TCK_MOD` reader - parallel if tx clk polar control"]
pub struct RB_HSPI_TCK_MOD_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_TCK_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_TCK_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_TCK_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_TCK_MOD` writer - parallel if tx clk polar control"]
pub struct RB_HSPI_TCK_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_TCK_MOD_W<'a> {
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
#[doc = "Field `RB_HSPI_RCK_MOD` reader - parallel if rx clk polar control"]
pub struct RB_HSPI_RCK_MOD_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_RCK_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_RCK_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_RCK_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_RCK_MOD` writer - parallel if rx clk polar control"]
pub struct RB_HSPI_RCK_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_RCK_MOD_W<'a> {
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
#[doc = "Field `RB_HSPI_ACK_TX_MOD` reader - parallel if tx ack mode cfg"]
pub struct RB_HSPI_ACK_TX_MOD_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_ACK_TX_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_ACK_TX_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_ACK_TX_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_ACK_TX_MOD` writer - parallel if tx ack mode cfg"]
pub struct RB_HSPI_ACK_TX_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_ACK_TX_MOD_W<'a> {
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
#[doc = "Field `RB_HSPI_ACK_CNT_SEL` reader - delay time of parallel if send ack when receive done"]
pub struct RB_HSPI_ACK_CNT_SEL_R(crate::FieldReader<u8, u8>);
impl RB_HSPI_ACK_CNT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_HSPI_ACK_CNT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_ACK_CNT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_ACK_CNT_SEL` writer - delay time of parallel if send ack when receive done"]
pub struct RB_HSPI_ACK_CNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_ACK_CNT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 3)) | ((value as u8 & 3) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - parallel if tx clk polar control"]
    #[inline(always)]
    pub fn rb_hspi_tck_mod(&self) -> RB_HSPI_TCK_MOD_R {
        RB_HSPI_TCK_MOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - parallel if rx clk polar control"]
    #[inline(always)]
    pub fn rb_hspi_rck_mod(&self) -> RB_HSPI_RCK_MOD_R {
        RB_HSPI_RCK_MOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - parallel if tx ack mode cfg"]
    #[inline(always)]
    pub fn rb_hspi_ack_tx_mod(&self) -> RB_HSPI_ACK_TX_MOD_R {
        RB_HSPI_ACK_TX_MOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - delay time of parallel if send ack when receive done"]
    #[inline(always)]
    pub fn rb_hspi_ack_cnt_sel(&self) -> RB_HSPI_ACK_CNT_SEL_R {
        RB_HSPI_ACK_CNT_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - parallel if tx clk polar control"]
    #[inline(always)]
    pub fn rb_hspi_tck_mod(&mut self) -> RB_HSPI_TCK_MOD_W {
        RB_HSPI_TCK_MOD_W { w: self }
    }
    #[doc = "Bit 1 - parallel if rx clk polar control"]
    #[inline(always)]
    pub fn rb_hspi_rck_mod(&mut self) -> RB_HSPI_RCK_MOD_W {
        RB_HSPI_RCK_MOD_W { w: self }
    }
    #[doc = "Bit 2 - parallel if tx ack mode cfg"]
    #[inline(always)]
    pub fn rb_hspi_ack_tx_mod(&mut self) -> RB_HSPI_ACK_TX_MOD_W {
        RB_HSPI_ACK_TX_MOD_W { w: self }
    }
    #[doc = "Bits 3:4 - delay time of parallel if send ack when receive done"]
    #[inline(always)]
    pub fn rb_hspi_ack_cnt_sel(&mut self) -> RB_HSPI_ACK_CNT_SEL_W {
        RB_HSPI_ACK_CNT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if aux\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_aux](index.html) module"]
pub struct HSPI_AUX_SPEC;
impl crate::RegisterSpec for HSPI_AUX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hspi_aux::R](R) reader structure"]
impl crate::Readable for HSPI_AUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_aux::W](W) writer structure"]
impl crate::Writable for HSPI_AUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_AUX to value 0"]
impl crate::Resettable for HSPI_AUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

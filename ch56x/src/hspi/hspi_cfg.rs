#[doc = "Register `HSPI_CFG` reader"]
pub struct R(crate::R<HSPI_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_CFG` writer"]
pub struct W(crate::W<HSPI_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_CFG_SPEC>;
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
impl From<crate::W<HSPI_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_MODE` reader - parallel if mode"]
pub struct RB_HSPI_MODE_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_MODE` writer - parallel if mode"]
pub struct RB_HSPI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_MODE_W<'a> {
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
#[doc = "Field `RB_HSPI_DUALDMA` reader - parallel if dualdma mode enable"]
pub struct RB_HSPI_DUALDMA_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_DUALDMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_DUALDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_DUALDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_DUALDMA` writer - parallel if dualdma mode enable"]
pub struct RB_HSPI_DUALDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_DUALDMA_W<'a> {
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
#[doc = "Field `RB_HSPI_MSK_SIZE` reader - parallel if data mode"]
pub struct RB_HSPI_MSK_SIZE_R(crate::FieldReader<u8, u8>);
impl RB_HSPI_MSK_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_HSPI_MSK_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_MSK_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_MSK_SIZE` writer - parallel if data mode"]
pub struct RB_HSPI_MSK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_MSK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u8 & 3) << 2);
        self.w
    }
}
#[doc = "Field `RB_HSPI_TX_TOG_EN` reader - parallel if tx addr toggle enable"]
pub struct RB_HSPI_TX_TOG_EN_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_TX_TOG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_TX_TOG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_TX_TOG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_TX_TOG_EN` writer - parallel if tx addr toggle enable"]
pub struct RB_HSPI_TX_TOG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_TX_TOG_EN_W<'a> {
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
#[doc = "Field `RB_HSPI_RX_TOG_EN` reader - parallel if rx addr toggle enable"]
pub struct RB_HSPI_RX_TOG_EN_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_RX_TOG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_RX_TOG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_RX_TOG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_RX_TOG_EN` writer - parallel if rx addr toggle enable"]
pub struct RB_HSPI_RX_TOG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_RX_TOG_EN_W<'a> {
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
#[doc = "Field `RB_HSPI_HW_ACK` reader - parallel if tx ack by hardware"]
pub struct RB_HSPI_HW_ACK_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_HW_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_HW_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_HW_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_HW_ACK` writer - parallel if tx ack by hardware"]
pub struct RB_HSPI_HW_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_HW_ACK_W<'a> {
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
    #[doc = "Bit 0 - parallel if mode"]
    #[inline(always)]
    pub fn rb_hspi_mode(&self) -> RB_HSPI_MODE_R {
        RB_HSPI_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - parallel if dualdma mode enable"]
    #[inline(always)]
    pub fn rb_hspi_dualdma(&self) -> RB_HSPI_DUALDMA_R {
        RB_HSPI_DUALDMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - parallel if data mode"]
    #[inline(always)]
    pub fn rb_hspi_msk_size(&self) -> RB_HSPI_MSK_SIZE_R {
        RB_HSPI_MSK_SIZE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - parallel if tx addr toggle enable"]
    #[inline(always)]
    pub fn rb_hspi_tx_tog_en(&self) -> RB_HSPI_TX_TOG_EN_R {
        RB_HSPI_TX_TOG_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - parallel if rx addr toggle enable"]
    #[inline(always)]
    pub fn rb_hspi_rx_tog_en(&self) -> RB_HSPI_RX_TOG_EN_R {
        RB_HSPI_RX_TOG_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - parallel if tx ack by hardware"]
    #[inline(always)]
    pub fn rb_hspi_hw_ack(&self) -> RB_HSPI_HW_ACK_R {
        RB_HSPI_HW_ACK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - parallel if mode"]
    #[inline(always)]
    pub fn rb_hspi_mode(&mut self) -> RB_HSPI_MODE_W {
        RB_HSPI_MODE_W { w: self }
    }
    #[doc = "Bit 1 - parallel if dualdma mode enable"]
    #[inline(always)]
    pub fn rb_hspi_dualdma(&mut self) -> RB_HSPI_DUALDMA_W {
        RB_HSPI_DUALDMA_W { w: self }
    }
    #[doc = "Bits 2:3 - parallel if data mode"]
    #[inline(always)]
    pub fn rb_hspi_msk_size(&mut self) -> RB_HSPI_MSK_SIZE_W {
        RB_HSPI_MSK_SIZE_W { w: self }
    }
    #[doc = "Bit 5 - parallel if tx addr toggle enable"]
    #[inline(always)]
    pub fn rb_hspi_tx_tog_en(&mut self) -> RB_HSPI_TX_TOG_EN_W {
        RB_HSPI_TX_TOG_EN_W { w: self }
    }
    #[doc = "Bit 6 - parallel if rx addr toggle enable"]
    #[inline(always)]
    pub fn rb_hspi_rx_tog_en(&mut self) -> RB_HSPI_RX_TOG_EN_W {
        RB_HSPI_RX_TOG_EN_W { w: self }
    }
    #[doc = "Bit 7 - parallel if tx ack by hardware"]
    #[inline(always)]
    pub fn rb_hspi_hw_ack(&mut self) -> RB_HSPI_HW_ACK_W {
        RB_HSPI_HW_ACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if tx or rx cfg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_cfg](index.html) module"]
pub struct HSPI_CFG_SPEC;
impl crate::RegisterSpec for HSPI_CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hspi_cfg::R](R) reader structure"]
impl crate::Readable for HSPI_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_cfg::W](W) writer structure"]
impl crate::Writable for HSPI_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_CFG to value 0x82"]
impl crate::Resettable for HSPI_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x82
    }
}

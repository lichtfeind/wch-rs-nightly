#[doc = "Register `UEP7_MOD` reader"]
pub struct R(crate::R<UEP7_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP7_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP7_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP7_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP7_MOD` writer"]
pub struct W(crate::W<UEP7_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP7_MOD_SPEC>;
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
impl From<crate::W<UEP7_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP7_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UEP7_BUF_MOD` reader - buffer mode of USB endpoint 7(15)"]
pub struct RB_UEP7_BUF_MOD_R(crate::FieldReader<bool, bool>);
impl RB_UEP7_BUF_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_UEP7_BUF_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UEP7_BUF_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UEP7_BUF_MOD` writer - buffer mode of USB endpoint 7(15)"]
pub struct RB_UEP7_BUF_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UEP7_BUF_MOD_W<'a> {
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
#[doc = "Field `RB_UEP7_TX_EN` reader - enable USB endpoint 7(15) transmittal (IN)"]
pub struct RB_UEP7_TX_EN_R(crate::FieldReader<bool, bool>);
impl RB_UEP7_TX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_UEP7_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UEP7_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UEP7_TX_EN` writer - enable USB endpoint 7(15) transmittal (IN)"]
pub struct RB_UEP7_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UEP7_TX_EN_W<'a> {
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
#[doc = "Field `RB_UEP7_RX_EN` reader - enable USB endpoint 7(15) receiving (OUT)"]
pub struct RB_UEP7_RX_EN_R(crate::FieldReader<bool, bool>);
impl RB_UEP7_RX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_UEP7_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_UEP7_RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_UEP7_RX_EN` writer - enable USB endpoint 7(15) receiving (OUT)"]
pub struct RB_UEP7_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_UEP7_RX_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - buffer mode of USB endpoint 7(15)"]
    #[inline(always)]
    pub fn rb_uep7_buf_mod(&self) -> RB_UEP7_BUF_MOD_R {
        RB_UEP7_BUF_MOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 7(15) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep7_tx_en(&self) -> RB_UEP7_TX_EN_R {
        RB_UEP7_TX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable USB endpoint 7(15) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep7_rx_en(&self) -> RB_UEP7_RX_EN_R {
        RB_UEP7_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - buffer mode of USB endpoint 7(15)"]
    #[inline(always)]
    pub fn rb_uep7_buf_mod(&mut self) -> RB_UEP7_BUF_MOD_W {
        RB_UEP7_BUF_MOD_W { w: self }
    }
    #[doc = "Bit 2 - enable USB endpoint 7(15) transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep7_tx_en(&mut self) -> RB_UEP7_TX_EN_W {
        RB_UEP7_TX_EN_W { w: self }
    }
    #[doc = "Bit 3 - enable USB endpoint 7(15) receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep7_rx_en(&mut self) -> RB_UEP7_RX_EN_W {
        RB_UEP7_RX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 7(15) mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep7_mod](index.html) module"]
pub struct UEP7_MOD_SPEC;
impl crate::RegisterSpec for UEP7_MOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uep7_mod::R](R) reader structure"]
impl crate::Readable for UEP7_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep7_mod::W](W) writer structure"]
impl crate::Writable for UEP7_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP7_MOD to value 0"]
impl crate::Resettable for UEP7_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

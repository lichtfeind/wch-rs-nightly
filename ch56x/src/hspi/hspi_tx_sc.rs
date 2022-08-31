#[doc = "Register `HSPI_TX_SC` reader"]
pub struct R(crate::R<HSPI_TX_SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_TX_SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_TX_SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_TX_SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_TX_SC` writer"]
pub struct W(crate::W<HSPI_TX_SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_TX_SC_SPEC>;
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
impl From<crate::W<HSPI_TX_SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_TX_SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_TX_NUM` reader - parallel if tx sequence num"]
pub struct RB_HSPI_TX_NUM_R(crate::FieldReader<u8, u8>);
impl RB_HSPI_TX_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_HSPI_TX_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_TX_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_TX_NUM` writer - parallel if tx sequence num"]
pub struct RB_HSPI_TX_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_TX_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Field `RB_HSPI_TX_TOG` reader - parallel if tx addr toggle flag"]
pub struct RB_HSPI_TX_TOG_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_TX_TOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_TX_TOG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_TX_TOG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_TX_TOG` writer - parallel if tx addr toggle flag"]
pub struct RB_HSPI_TX_TOG_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_TX_TOG_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - parallel if tx sequence num"]
    #[inline(always)]
    pub fn rb_hspi_tx_num(&self) -> RB_HSPI_TX_NUM_R {
        RB_HSPI_TX_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - parallel if tx addr toggle flag"]
    #[inline(always)]
    pub fn rb_hspi_tx_tog(&self) -> RB_HSPI_TX_TOG_R {
        RB_HSPI_TX_TOG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - parallel if tx sequence num"]
    #[inline(always)]
    pub fn rb_hspi_tx_num(&mut self) -> RB_HSPI_TX_NUM_W {
        RB_HSPI_TX_NUM_W { w: self }
    }
    #[doc = "Bit 4 - parallel if tx addr toggle flag"]
    #[inline(always)]
    pub fn rb_hspi_tx_tog(&mut self) -> RB_HSPI_TX_TOG_W {
        RB_HSPI_TX_TOG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel TX sequence ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_tx_sc](index.html) module"]
pub struct HSPI_TX_SC_SPEC;
impl crate::RegisterSpec for HSPI_TX_SC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hspi_tx_sc::R](R) reader structure"]
impl crate::Readable for HSPI_TX_SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_tx_sc::W](W) writer structure"]
impl crate::Writable for HSPI_TX_SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_TX_SC to value 0"]
impl crate::Resettable for HSPI_TX_SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

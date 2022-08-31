#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_EN` reader - UART FIFO enable"]
pub struct FIFO_EN_R(crate::FieldReader<bool, bool>);
impl FIFO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_EN` writer - UART FIFO enable"]
pub struct FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_EN_W<'a> {
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
#[doc = "Field `RX_FIFO_CLR` reader - clear UART receiver FIFO, high action, auto clear"]
pub struct RX_FIFO_CLR_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_CLR` writer - clear UART receiver FIFO, high action, auto clear"]
pub struct RX_FIFO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CLR_W<'a> {
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
#[doc = "Field `TX_FIFO_CLR` reader - clear UART transmitter FIFO, high action, auto clear"]
pub struct TX_FIFO_CLR_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FIFO_CLR` writer - clear UART transmitter FIFO, high action, auto clear"]
pub struct TX_FIFO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_CLR_W<'a> {
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
#[doc = "Field `FIFO_TRIG` reader - UART receiver FIFO trigger level"]
pub struct FIFO_TRIG_R(crate::FieldReader<u8, u8>);
impl FIFO_TRIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_TRIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_TRIG` writer - UART receiver FIFO trigger level"]
pub struct FIFO_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_TRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u8 & 3) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - UART FIFO enable"]
    #[inline(always)]
    pub fn fifo_en(&self) -> FIFO_EN_R {
        FIFO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clear UART receiver FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn rx_fifo_clr(&self) -> RX_FIFO_CLR_R {
        RX_FIFO_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clear UART transmitter FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn tx_fifo_clr(&self) -> TX_FIFO_CLR_R {
        TX_FIFO_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 6:7 - UART receiver FIFO trigger level"]
    #[inline(always)]
    pub fn fifo_trig(&self) -> FIFO_TRIG_R {
        FIFO_TRIG_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UART FIFO enable"]
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FIFO_EN_W {
        FIFO_EN_W { w: self }
    }
    #[doc = "Bit 1 - clear UART receiver FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RX_FIFO_CLR_W {
        RX_FIFO_CLR_W { w: self }
    }
    #[doc = "Bit 2 - clear UART transmitter FIFO, high action, auto clear"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TX_FIFO_CLR_W {
        TX_FIFO_CLR_W { w: self }
    }
    #[doc = "Bits 6:7 - UART receiver FIFO trigger level"]
    #[inline(always)]
    pub fn fifo_trig(&mut self) -> FIFO_TRIG_W {
        FIFO_TRIG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART1 FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

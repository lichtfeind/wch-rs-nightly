#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECV_RDY` reader - UART interrupt enable for receiver data ready"]
pub struct RECV_RDY_R(crate::FieldReader<bool, bool>);
impl RECV_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RECV_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECV_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECV_RDY` writer - UART interrupt enable for receiver data ready"]
pub struct RECV_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RECV_RDY_W<'a> {
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
#[doc = "Field `THR_EMPTY` reader - UART interrupt enable for THR empty"]
pub struct THR_EMPTY_R(crate::FieldReader<bool, bool>);
impl THR_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THR_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THR_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THR_EMPTY` writer - UART interrupt enable for THR empty"]
pub struct THR_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_EMPTY_W<'a> {
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
#[doc = "Field `LINE_STAT` reader - UART interrupt enable for receiver line status"]
pub struct LINE_STAT_R(crate::FieldReader<bool, bool>);
impl LINE_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINE_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_STAT` writer - UART interrupt enable for receiver line status"]
pub struct LINE_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_STAT_W<'a> {
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
#[doc = "Field `TXD_EN` reader - UART TXD pin enable"]
pub struct TXD_EN_R(crate::FieldReader<bool, bool>);
impl TXD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXD_EN` writer - UART TXD pin enable"]
pub struct TXD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_EN_W<'a> {
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
#[doc = "Field `RESET` reader - UART software reset control, high action, auto clear"]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - UART software reset control, high action, auto clear"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
    #[doc = "Bit 0 - UART interrupt enable for receiver data ready"]
    #[inline(always)]
    pub fn recv_rdy(&self) -> RECV_RDY_R {
        RECV_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART interrupt enable for THR empty"]
    #[inline(always)]
    pub fn thr_empty(&self) -> THR_EMPTY_R {
        THR_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART interrupt enable for receiver line status"]
    #[inline(always)]
    pub fn line_stat(&self) -> LINE_STAT_R {
        LINE_STAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - UART TXD pin enable"]
    #[inline(always)]
    pub fn txd_en(&self) -> TXD_EN_R {
        TXD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART software reset control, high action, auto clear"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART interrupt enable for receiver data ready"]
    #[inline(always)]
    pub fn recv_rdy(&mut self) -> RECV_RDY_W {
        RECV_RDY_W { w: self }
    }
    #[doc = "Bit 1 - UART interrupt enable for THR empty"]
    #[inline(always)]
    pub fn thr_empty(&mut self) -> THR_EMPTY_W {
        THR_EMPTY_W { w: self }
    }
    #[doc = "Bit 2 - UART interrupt enable for receiver line status"]
    #[inline(always)]
    pub fn line_stat(&mut self) -> LINE_STAT_W {
        LINE_STAT_W { w: self }
    }
    #[doc = "Bit 6 - UART TXD pin enable"]
    #[inline(always)]
    pub fn txd_en(&mut self) -> TXD_EN_W {
        TXD_EN_W { w: self }
    }
    #[doc = "Bit 7 - UART software reset control, high action, auto clear"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART2 interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

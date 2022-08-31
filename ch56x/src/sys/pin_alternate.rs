#[doc = "Register `PIN_ALTERNATE` reader"]
pub struct R(crate::R<PIN_ALTERNATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_ALTERNATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_ALTERNATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_ALTERNATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN_ALTERNATE` writer"]
pub struct W(crate::W<PIN_ALTERNATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_ALTERNATE_SPEC>;
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
impl From<crate::W<PIN_ALTERNATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_ALTERNATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_PIN_MII` reader - ETH mii interface selection"]
pub struct RB_PIN_MII_R(crate::FieldReader<bool, bool>);
impl RB_PIN_MII_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_PIN_MII_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_PIN_MII_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_PIN_MII` writer - ETH mii interface selection"]
pub struct RB_PIN_MII_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_PIN_MII_W<'a> {
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
#[doc = "Field `RB_PIN_TMR1` reader - TMR1 alternate pin enable"]
pub struct RB_PIN_TMR1_R(crate::FieldReader<bool, bool>);
impl RB_PIN_TMR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_PIN_TMR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_PIN_TMR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_PIN_TMR1` writer - TMR1 alternate pin enable"]
pub struct RB_PIN_TMR1_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_PIN_TMR1_W<'a> {
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
#[doc = "Field `RB_PIN_TMR2` reader - TMR2 alternate pin enable"]
pub struct RB_PIN_TMR2_R(crate::FieldReader<bool, bool>);
impl RB_PIN_TMR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_PIN_TMR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_PIN_TMR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_PIN_TMR2` writer - TMR2 alternate pin enable"]
pub struct RB_PIN_TMR2_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_PIN_TMR2_W<'a> {
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
#[doc = "Field `RB_PIN_UART0` reader - RXD0/TXD0 alternate pin enable"]
pub struct RB_PIN_UART0_R(crate::FieldReader<bool, bool>);
impl RB_PIN_UART0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_PIN_UART0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_PIN_UART0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_PIN_UART0` writer - RXD0/TXD0 alternate pin enable"]
pub struct RB_PIN_UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_PIN_UART0_W<'a> {
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
    #[doc = "Bit 0 - ETH mii interface selection"]
    #[inline(always)]
    pub fn rb_pin_mii(&self) -> RB_PIN_MII_R {
        RB_PIN_MII_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TMR1 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr1(&self) -> RB_PIN_TMR1_R {
        RB_PIN_TMR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TMR2 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr2(&self) -> RB_PIN_TMR2_R {
        RB_PIN_TMR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RXD0/TXD0 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_uart0(&self) -> RB_PIN_UART0_R {
        RB_PIN_UART0_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETH mii interface selection"]
    #[inline(always)]
    pub fn rb_pin_mii(&mut self) -> RB_PIN_MII_W {
        RB_PIN_MII_W { w: self }
    }
    #[doc = "Bit 1 - TMR1 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr1(&mut self) -> RB_PIN_TMR1_W {
        RB_PIN_TMR1_W { w: self }
    }
    #[doc = "Bit 2 - TMR2 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_tmr2(&mut self) -> RB_PIN_TMR2_W {
        RB_PIN_TMR2_W { w: self }
    }
    #[doc = "Bit 4 - RXD0/TXD0 alternate pin enable"]
    #[inline(always)]
    pub fn rb_pin_uart0(&mut self) -> RB_PIN_UART0_W {
        RB_PIN_UART0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "alternate pin control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin_alternate](index.html) module"]
pub struct PIN_ALTERNATE_SPEC;
impl crate::RegisterSpec for PIN_ALTERNATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pin_alternate::R](R) reader structure"]
impl crate::Readable for PIN_ALTERNATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin_alternate::W](W) writer structure"]
impl crate::Writable for PIN_ALTERNATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN_ALTERNATE to value 0"]
impl crate::Resettable for PIN_ALTERNATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

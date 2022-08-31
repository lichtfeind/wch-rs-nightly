#[doc = "Register `CTRL_MOD` reader"]
pub struct R(crate::R<CTRL_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_MOD` writer"]
pub struct W(crate::W<CTRL_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_MOD_SPEC>;
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
impl From<crate::W<CTRL_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE_IN` reader - timer in mode"]
pub struct MODE_IN_R(crate::FieldReader<bool, bool>);
impl MODE_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_IN` writer - timer in mode"]
pub struct MODE_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_IN_W<'a> {
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
#[doc = "Field `ALL_CLEAR` reader - force clear timer FIFO and count"]
pub struct ALL_CLEAR_R(crate::FieldReader<bool, bool>);
impl ALL_CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALL_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALL_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALL_CLEAR` writer - force clear timer FIFO and count"]
pub struct ALL_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_CLEAR_W<'a> {
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
#[doc = "Field `COUNT_EN` reader - timer count enable"]
pub struct COUNT_EN_R(crate::FieldReader<bool, bool>);
impl COUNT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COUNT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT_EN` writer - timer count enable"]
pub struct COUNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_EN_W<'a> {
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
#[doc = "Field `OUT_EN` reader - timer output enable"]
pub struct OUT_EN_R(crate::FieldReader<bool, bool>);
impl OUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EN` writer - timer output enable"]
pub struct OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_W<'a> {
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
#[doc = "Field `OUT_POLAR_CAP_COUNT` reader - timer PWM output polarity _ Count sub-mode"]
pub struct OUT_POLAR_CAP_COUNT_R(crate::FieldReader<bool, bool>);
impl OUT_POLAR_CAP_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_POLAR_CAP_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_POLAR_CAP_COUNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_POLAR_CAP_COUNT` writer - timer PWM output polarity _ Count sub-mode"]
pub struct OUT_POLAR_CAP_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_POLAR_CAP_COUNT_W<'a> {
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
#[doc = "Field ` PWM_REPEAT_CAP_EDGE` reader - timer PWM repeat mode _ timer capture edge mode"]
pub struct PWM_REPEAT_CAP_EDGE_R(crate::FieldReader<u8, u8>);
impl PWM_REPEAT_CAP_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWM_REPEAT_CAP_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_REPEAT_CAP_EDGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field ` PWM_REPEAT_CAP_EDGE` writer - timer PWM repeat mode _ timer capture edge mode"]
pub struct PWM_REPEAT_CAP_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_REPEAT_CAP_EDGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u8 & 3) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - timer in mode"]
    #[inline(always)]
    pub fn mode_in(&self) -> MODE_IN_R {
        MODE_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear timer FIFO and count"]
    #[inline(always)]
    pub fn all_clear(&self) -> ALL_CLEAR_R {
        ALL_CLEAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - timer count enable"]
    #[inline(always)]
    pub fn count_en(&self) -> COUNT_EN_R {
        COUNT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - timer output enable"]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - timer PWM output polarity _ Count sub-mode"]
    #[inline(always)]
    pub fn out_polar_cap_count(&self) -> OUT_POLAR_CAP_COUNT_R {
        OUT_POLAR_CAP_COUNT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - timer PWM repeat mode _ timer capture edge mode"]
    #[inline(always)]
    pub fn pwm_repeat_cap_edge(&self) -> PWM_REPEAT_CAP_EDGE_R {
        PWM_REPEAT_CAP_EDGE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - timer in mode"]
    #[inline(always)]
    pub fn mode_in(&mut self) -> MODE_IN_W {
        MODE_IN_W { w: self }
    }
    #[doc = "Bit 1 - force clear timer FIFO and count"]
    #[inline(always)]
    pub fn all_clear(&mut self) -> ALL_CLEAR_W {
        ALL_CLEAR_W { w: self }
    }
    #[doc = "Bit 2 - timer count enable"]
    #[inline(always)]
    pub fn count_en(&mut self) -> COUNT_EN_W {
        COUNT_EN_W { w: self }
    }
    #[doc = "Bit 3 - timer output enable"]
    #[inline(always)]
    pub fn out_en(&mut self) -> OUT_EN_W {
        OUT_EN_W { w: self }
    }
    #[doc = "Bit 4 - timer PWM output polarity _ Count sub-mode"]
    #[inline(always)]
    pub fn out_polar_cap_count(&mut self) -> OUT_POLAR_CAP_COUNT_W {
        OUT_POLAR_CAP_COUNT_W { w: self }
    }
    #[doc = "Bits 6:7 - timer PWM repeat mode _ timer capture edge mode"]
    #[inline(always)]
    pub fn pwm_repeat_cap_edge(&mut self) -> PWM_REPEAT_CAP_EDGE_W {
        PWM_REPEAT_CAP_EDGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR0 mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_mod](index.html) module"]
pub struct CTRL_MOD_SPEC;
impl crate::RegisterSpec for CTRL_MOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrl_mod::R](R) reader structure"]
impl crate::Readable for CTRL_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_mod::W](W) writer structure"]
impl crate::Writable for CTRL_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_MOD to value 0x02"]
impl crate::Resettable for CTRL_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}

#[doc = "Register `PWM_CLOCK_DIV` reader"]
pub struct R(crate::R<PWM_CLOCK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CLOCK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CLOCK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CLOCK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CLOCK_DIV` writer"]
pub struct W(crate::W<PWM_CLOCK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CLOCK_DIV_SPEC>;
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
impl From<crate::W<PWM_CLOCK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CLOCK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_CLOCK_DIV` reader - PWM clock divisor"]
pub struct PWM_CLOCK_DIV_R(crate::FieldReader<u8, u8>);
impl PWM_CLOCK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWM_CLOCK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_CLOCK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM_CLOCK_DIV` writer - PWM clock divisor"]
pub struct PWM_CLOCK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_CLOCK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PWM clock divisor"]
    #[inline(always)]
    pub fn pwm_clock_div(&self) -> PWM_CLOCK_DIV_R {
        PWM_CLOCK_DIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - PWM clock divisor"]
    #[inline(always)]
    pub fn pwm_clock_div(&mut self) -> PWM_CLOCK_DIV_W {
        PWM_CLOCK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM clock divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_clock_div](index.html) module"]
pub struct PWM_CLOCK_DIV_SPEC;
impl crate::RegisterSpec for PWM_CLOCK_DIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pwm_clock_div::R](R) reader structure"]
impl crate::Readable for PWM_CLOCK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_clock_div::W](W) writer structure"]
impl crate::Writable for PWM_CLOCK_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CLOCK_DIV to value 0"]
impl crate::Resettable for PWM_CLOCK_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

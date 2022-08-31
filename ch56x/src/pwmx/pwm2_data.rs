#[doc = "Register `PWM2_DATA` reader"]
pub struct R(crate::R<PWM2_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM2_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM2_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM2_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM2_DATA` writer"]
pub struct W(crate::W<PWM2_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM2_DATA_SPEC>;
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
impl From<crate::W<PWM2_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM2_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM2_DATA` reader - PWM2 data holding"]
pub struct PWM2_DATA_R(crate::FieldReader<u8, u8>);
impl PWM2_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWM2_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM2_DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM2_DATA` writer - PWM2 data holding"]
pub struct PWM2_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM2_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PWM2 data holding"]
    #[inline(always)]
    pub fn pwm2_data(&self) -> PWM2_DATA_R {
        PWM2_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - PWM2 data holding"]
    #[inline(always)]
    pub fn pwm2_data(&mut self) -> PWM2_DATA_W {
        PWM2_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM2 data holding\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm2_data](index.html) module"]
pub struct PWM2_DATA_SPEC;
impl crate::RegisterSpec for PWM2_DATA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pwm2_data::R](R) reader structure"]
impl crate::Readable for PWM2_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm2_data::W](W) writer structure"]
impl crate::Writable for PWM2_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM2_DATA to value 0"]
impl crate::Resettable for PWM2_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

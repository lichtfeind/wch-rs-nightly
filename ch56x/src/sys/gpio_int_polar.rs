#[doc = "Register `GPIO_INT_POLAR` reader"]
pub struct R(crate::R<GPIO_INT_POLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_POLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_POLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_POLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_POLAR` writer"]
pub struct W(crate::W<GPIO_INT_POLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_POLAR_SPEC>;
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
impl From<crate::W<GPIO_INT_POLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_POLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_GPIO_PA2_IP` reader - PA2 pin interrupt mode"]
pub struct RB_GPIO_PA2_IP_R(crate::FieldReader<bool, bool>);
impl RB_GPIO_PA2_IP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_GPIO_PA2_IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_GPIO_PA2_IP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_GPIO_PA2_IP` writer - PA2 pin interrupt mode"]
pub struct RB_GPIO_PA2_IP_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_GPIO_PA2_IP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PA2 pin interrupt mode"]
    #[inline(always)]
    pub fn rb_gpio_pa2_ip(&self) -> RB_GPIO_PA2_IP_R {
        RB_GPIO_PA2_IP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PA2 pin interrupt mode"]
    #[inline(always)]
    pub fn rb_gpio_pa2_ip(&mut self) -> RB_GPIO_PA2_IP_W {
        RB_GPIO_PA2_IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_polar](index.html) module"]
pub struct GPIO_INT_POLAR_SPEC;
impl crate::RegisterSpec for GPIO_INT_POLAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gpio_int_polar::R](R) reader structure"]
impl crate::Readable for GPIO_INT_POLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_polar::W](W) writer structure"]
impl crate::Writable for GPIO_INT_POLAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_POLAR to value 0"]
impl crate::Resettable for GPIO_INT_POLAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

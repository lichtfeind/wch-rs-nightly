#[doc = "Register `ADR` reader"]
pub struct R(crate::R<ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADR` writer"]
pub struct W(crate::W<ADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADR_SPEC>;
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
impl From<crate::W<ADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADR` reader - UART0 slave address"]
pub struct ADR_R(crate::FieldReader<u8, u8>);
impl ADR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADR` writer - UART0 slave address"]
pub struct ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - UART0 slave address"]
    #[inline(always)]
    pub fn adr(&self) -> ADR_R {
        ADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART0 slave address"]
    #[inline(always)]
    pub fn adr(&mut self) -> ADR_W {
        ADR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0 slave address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adr](index.html) module"]
pub struct ADR_SPEC;
impl crate::RegisterSpec for ADR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adr::R](R) reader structure"]
impl crate::Readable for ADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adr::W](W) writer structure"]
impl crate::Writable for ADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADR to value 0xff"]
impl crate::Resettable for ADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}

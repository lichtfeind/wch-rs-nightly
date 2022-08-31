#[doc = "Register `DIV` reader"]
pub struct R(crate::R<DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV` writer"]
pub struct W(crate::W<DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_SPEC>;
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
impl From<crate::W<DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - UART pre-divisor latch byte"]
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - UART pre-divisor latch byte"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - UART pre-divisor latch byte"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART pre-divisor latch byte"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART3 pre-divisor latch byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div](index.html) module"]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [div::R](R) reader structure"]
impl crate::Readable for DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div::W](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `DL` reader"]
pub struct R(crate::R<DL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DL` writer"]
pub struct W(crate::W<DL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DL_SPEC>;
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
impl From<crate::W<DL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DL` reader - UART divisor latch"]
pub struct DL_R(crate::FieldReader<u16, u16>);
impl DL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DL` writer - UART divisor latch"]
pub struct DL_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - UART divisor latch"]
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART divisor latch"]
    #[inline(always)]
    pub fn dl(&mut self) -> DL_W {
        DL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0 divisor latch\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dl](index.html) module"]
pub struct DL_SPEC;
impl crate::RegisterSpec for DL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dl::R](R) reader structure"]
impl crate::Readable for DL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dl::W](W) writer structure"]
impl crate::Writable for DL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DL to value 0"]
impl crate::Resettable for DL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

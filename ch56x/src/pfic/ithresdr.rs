#[doc = "Register `ITHRESDR` reader"]
pub struct R(crate::R<ITHRESDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITHRESDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITHRESDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITHRESDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITHRESDR` writer"]
pub struct W(crate::W<ITHRESDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITHRESDR_SPEC>;
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
impl From<crate::W<ITHRESDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITHRESDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRESHOLD` reader - THRESHOLD"]
pub struct THRESHOLD_R(crate::FieldReader<u8, u8>);
impl THRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRESHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRESHOLD` writer - THRESHOLD"]
pub struct THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - THRESHOLD"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - THRESHOLD"]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W {
        THRESHOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ithresdr](index.html) module"]
pub struct ITHRESDR_SPEC;
impl crate::RegisterSpec for ITHRESDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ithresdr::R](R) reader structure"]
impl crate::Readable for ITHRESDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ithresdr::W](W) writer structure"]
impl crate::Writable for ITHRESDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ITHRESDR to value 0"]
impl crate::Resettable for ITHRESDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

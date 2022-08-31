#[doc = "Register `GLOB_RESET_KEEP` reader"]
pub struct R(crate::R<GLOB_RESET_KEEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOB_RESET_KEEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOB_RESET_KEEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOB_RESET_KEEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOB_RESET_KEEP` writer"]
pub struct W(crate::W<GLOB_RESET_KEEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOB_RESET_KEEP_SPEC>;
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
impl From<crate::W<GLOB_RESET_KEEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOB_RESET_KEEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLOB_RESET_KEEP` reader - value keeper during global reset"]
pub struct GLOB_RESET_KEEP_R(crate::FieldReader<u8, u8>);
impl GLOB_RESET_KEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GLOB_RESET_KEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLOB_RESET_KEEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLOB_RESET_KEEP` writer - value keeper during global reset"]
pub struct GLOB_RESET_KEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> GLOB_RESET_KEEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - value keeper during global reset"]
    #[inline(always)]
    pub fn glob_reset_keep(&self) -> GLOB_RESET_KEEP_R {
        GLOB_RESET_KEEP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - value keeper during global reset"]
    #[inline(always)]
    pub fn glob_reset_keep(&mut self) -> GLOB_RESET_KEEP_W {
        GLOB_RESET_KEEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "value keeper during global reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glob_reset_keep](index.html) module"]
pub struct GLOB_RESET_KEEP_SPEC;
impl crate::RegisterSpec for GLOB_RESET_KEEP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [glob_reset_keep::R](R) reader structure"]
impl crate::Readable for GLOB_RESET_KEEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glob_reset_keep::W](W) writer structure"]
impl crate::Writable for GLOB_RESET_KEEP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLOB_RESET_KEEP to value 0"]
impl crate::Resettable for GLOB_RESET_KEEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

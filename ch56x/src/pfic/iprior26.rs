#[doc = "Register `IPRIOR26` reader"]
pub struct R(crate::R<IPRIOR26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRIOR26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRIOR26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRIOR26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPRIOR26` writer"]
pub struct W(crate::W<IPRIOR26_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPRIOR26_SPEC>;
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
impl From<crate::W<IPRIOR26_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPRIOR26_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPRIOR` reader - IPRIOR26"]
pub struct IPRIOR_R(crate::FieldReader<u32, u32>);
impl IPRIOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IPRIOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPRIOR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPRIOR` writer - IPRIOR26"]
pub struct IPRIOR_W<'a> {
    w: &'a mut W,
}
impl<'a> IPRIOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IPRIOR26"]
    #[inline(always)]
    pub fn iprior(&self) -> IPRIOR_R {
        IPRIOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IPRIOR26"]
    #[inline(always)]
    pub fn iprior(&mut self) -> IPRIOR_W {
        IPRIOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprior26](index.html) module"]
pub struct IPRIOR26_SPEC;
impl crate::RegisterSpec for IPRIOR26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iprior26::R](R) reader structure"]
impl crate::Readable for IPRIOR26_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iprior26::W](W) writer structure"]
impl crate::Writable for IPRIOR26_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPRIOR26 to value 0"]
impl crate::Resettable for IPRIOR26_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

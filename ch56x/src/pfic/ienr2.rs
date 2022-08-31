#[doc = "Register `IENR2` reader"]
pub struct R(crate::R<IENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IENR2` writer"]
pub struct W(crate::W<IENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IENR2_SPEC>;
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
impl From<crate::W<IENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN` reader - INTEN"]
pub struct INTEN_R(crate::FieldReader<u32, u32>);
impl INTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN` writer - INTEN"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienr2](index.html) module"]
pub struct IENR2_SPEC;
impl crate::RegisterSpec for IENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ienr2::R](R) reader structure"]
impl crate::Readable for IENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ienr2::W](W) writer structure"]
impl crate::Writable for IENR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IENR2 to value 0"]
impl crate::Resettable for IENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `IRER1` reader"]
pub struct R(crate::R<IRER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRER1` writer"]
pub struct W(crate::W<IRER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRER1_SPEC>;
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
impl From<crate::W<IRER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTRESET` reader - INTRESET"]
pub struct INTRESET_R(crate::FieldReader<u32, u32>);
impl INTRESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INTRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTRESET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTRESET` writer - INTRESET"]
pub struct INTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> INTRESET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | ((value as u32 & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - INTRESET"]
    #[inline(always)]
    pub fn intreset(&self) -> INTRESET_R {
        INTRESET_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:31 - INTRESET"]
    #[inline(always)]
    pub fn intreset(&mut self) -> INTRESET_W {
        INTRESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irer1](index.html) module"]
pub struct IRER1_SPEC;
impl crate::RegisterSpec for IRER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irer1::R](R) reader structure"]
impl crate::Readable for IRER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irer1::W](W) writer structure"]
impl crate::Writable for IRER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRER1 to value 0"]
impl crate::Resettable for IRER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `IPRR2` reader"]
pub struct R(crate::R<IPRR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPRR2` writer"]
pub struct W(crate::W<IPRR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPRR2_SPEC>;
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
impl From<crate::W<IPRR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPRR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PENDRESET` reader - PENDRESET"]
pub struct PENDRESET_R(crate::FieldReader<u32, u32>);
impl PENDRESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PENDRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDRESET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDRESET` writer - PENDRESET"]
pub struct PENDRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDRESET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | (value as u32 & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&self) -> PENDRESET_R {
        PENDRESET_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&mut self) -> PENDRESET_W {
        PENDRESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Pending Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprr2](index.html) module"]
pub struct IPRR2_SPEC;
impl crate::RegisterSpec for IPRR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iprr2::R](R) reader structure"]
impl crate::Readable for IPRR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iprr2::W](W) writer structure"]
impl crate::Writable for IPRR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPRR2 to value 0"]
impl crate::Resettable for IPRR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

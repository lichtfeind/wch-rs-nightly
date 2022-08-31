#[doc = "Register `STK_CNTL` reader"]
pub struct R(crate::R<STK_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STK_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STK_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STK_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STK_CNTL` writer"]
pub struct W(crate::W<STK_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STK_CNTL_SPEC>;
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
impl From<crate::W<STK_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STK_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTL` reader - CNTL"]
pub struct CNTL_R(crate::FieldReader<u32, u32>);
impl CNTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CNTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTL` writer - CNTL"]
pub struct CNTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CNTL"]
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTL"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CNTL_W {
        CNTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Systick counter low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_cntl](index.html) module"]
pub struct STK_CNTL_SPEC;
impl crate::RegisterSpec for STK_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stk_cntl::R](R) reader structure"]
impl crate::Readable for STK_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stk_cntl::W](W) writer structure"]
impl crate::Writable for STK_CNTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STK_CNTL to value 0"]
impl crate::Resettable for STK_CNTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

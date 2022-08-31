#[doc = "Register `STK_CNTH` reader"]
pub struct R(crate::R<STK_CNTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STK_CNTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STK_CNTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STK_CNTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STK_CNTH` writer"]
pub struct W(crate::W<STK_CNTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STK_CNTH_SPEC>;
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
impl From<crate::W<STK_CNTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STK_CNTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTH` reader - CNTH"]
pub struct CNTH_R(crate::FieldReader<u32, u32>);
impl CNTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CNTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTH` writer - CNTH"]
pub struct CNTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CNTH"]
    #[inline(always)]
    pub fn cnth(&self) -> CNTH_R {
        CNTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTH"]
    #[inline(always)]
    pub fn cnth(&mut self) -> CNTH_W {
        CNTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Systick counter high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_cnth](index.html) module"]
pub struct STK_CNTH_SPEC;
impl crate::RegisterSpec for STK_CNTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stk_cnth::R](R) reader structure"]
impl crate::Readable for STK_CNTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stk_cnth::W](W) writer structure"]
impl crate::Writable for STK_CNTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STK_CNTH to value 0"]
impl crate::Resettable for STK_CNTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

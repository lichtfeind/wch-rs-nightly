#[doc = "Register `WDOG_COUNT` reader"]
pub struct R(crate::R<WDOG_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOG_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOG_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOG_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOG_COUNT` writer"]
pub struct W(crate::W<WDOG_COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOG_COUNT_SPEC>;
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
impl From<crate::W<WDOG_COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOG_COUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOG_COUNT` reader - watch-dog count"]
pub struct WDOG_COUNT_R(crate::FieldReader<u8, u8>);
impl WDOG_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOG_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOG_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG_COUNT` writer - watch-dog count"]
pub struct WDOG_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - watch-dog count"]
    #[inline(always)]
    pub fn wdog_count(&self) -> WDOG_COUNT_R {
        WDOG_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - watch-dog count"]
    #[inline(always)]
    pub fn wdog_count(&mut self) -> WDOG_COUNT_W {
        WDOG_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watch-dog count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdog_count](index.html) module"]
pub struct WDOG_COUNT_SPEC;
impl crate::RegisterSpec for WDOG_COUNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wdog_count::R](R) reader structure"]
impl crate::Readable for WDOG_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdog_count::W](W) writer structure"]
impl crate::Writable for WDOG_COUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOG_COUNT to value 0"]
impl crate::Resettable for WDOG_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

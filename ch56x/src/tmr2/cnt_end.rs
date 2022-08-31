#[doc = "Register `CNT_END` reader"]
pub struct R(crate::R<CNT_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT_END` writer"]
pub struct W(crate::W<CNT_END_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_END_SPEC>;
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
impl From<crate::W<CNT_END_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_END_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_END` reader - TMR current count"]
pub struct CNT_END_R(crate::FieldReader<u32, u32>);
impl CNT_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CNT_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_END_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_END` writer - TMR current count"]
pub struct CNT_END_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TMR current count"]
    #[inline(always)]
    pub fn cnt_end(&self) -> CNT_END_R {
        CNT_END_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TMR current count"]
    #[inline(always)]
    pub fn cnt_end(&mut self) -> CNT_END_W {
        CNT_END_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR2 end count value, only low 26 bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_end](index.html) module"]
pub struct CNT_END_SPEC;
impl crate::RegisterSpec for CNT_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt_end::R](R) reader structure"]
impl crate::Readable for CNT_END_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt_end::W](W) writer structure"]
impl crate::Writable for CNT_END_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT_END to value 0"]
impl crate::Resettable for CNT_END_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

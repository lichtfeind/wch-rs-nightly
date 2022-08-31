#[doc = "Register `UEP6_T_LEN` reader"]
pub struct R(crate::R<UEP6_T_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP6_T_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP6_T_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP6_T_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP6_T_LEN` writer"]
pub struct W(crate::W<UEP6_T_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP6_T_LEN_SPEC>;
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
impl From<crate::W<UEP6_T_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP6_T_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP6_T_LEN` reader - endpoint 6 transmittal length"]
pub struct UEP6_T_LEN_R(crate::FieldReader<u16, u16>);
impl UEP6_T_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UEP6_T_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UEP6_T_LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UEP6_T_LEN` writer - endpoint 6 transmittal length"]
pub struct UEP6_T_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UEP6_T_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - endpoint 6 transmittal length"]
    #[inline(always)]
    pub fn uep6_t_len(&self) -> UEP6_T_LEN_R {
        UEP6_T_LEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - endpoint 6 transmittal length"]
    #[inline(always)]
    pub fn uep6_t_len(&mut self) -> UEP6_T_LEN_W {
        UEP6_T_LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 6 transmittal length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep6_t_len](index.html) module"]
pub struct UEP6_T_LEN_SPEC;
impl crate::RegisterSpec for UEP6_T_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uep6_t_len::R](R) reader structure"]
impl crate::Readable for UEP6_T_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep6_t_len::W](W) writer structure"]
impl crate::Writable for UEP6_T_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP6_T_LEN to value 0"]
impl crate::Resettable for UEP6_T_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

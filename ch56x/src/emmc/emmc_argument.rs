#[doc = "Register `EMMC_ARGUMENT` reader"]
pub struct R(crate::R<EMMC_ARGUMENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_ARGUMENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_ARGUMENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_ARGUMENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_ARGUMENT` writer"]
pub struct W(crate::W<EMMC_ARGUMENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_ARGUMENT_SPEC>;
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
impl From<crate::W<EMMC_ARGUMENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_ARGUMENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMMC_ARGUMENT` reader - 32 bit command parameter register"]
pub struct EMMC_ARGUMENT_R(crate::FieldReader<u32, u32>);
impl EMMC_ARGUMENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EMMC_ARGUMENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMMC_ARGUMENT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMMC_ARGUMENT` writer - 32 bit command parameter register"]
pub struct EMMC_ARGUMENT_W<'a> {
    w: &'a mut W,
}
impl<'a> EMMC_ARGUMENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 32 bit command parameter register"]
    #[inline(always)]
    pub fn emmc_argument(&self) -> EMMC_ARGUMENT_R {
        EMMC_ARGUMENT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bit command parameter register"]
    #[inline(always)]
    pub fn emmc_argument(&mut self) -> EMMC_ARGUMENT_W {
        EMMC_ARGUMENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 32bits command argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_argument](index.html) module"]
pub struct EMMC_ARGUMENT_SPEC;
impl crate::RegisterSpec for EMMC_ARGUMENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc_argument::R](R) reader structure"]
impl crate::Readable for EMMC_ARGUMENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_argument::W](W) writer structure"]
impl crate::Writable for EMMC_ARGUMENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_ARGUMENT to value 0"]
impl crate::Resettable for EMMC_ARGUMENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

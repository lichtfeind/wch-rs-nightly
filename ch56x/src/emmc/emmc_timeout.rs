#[doc = "Register `EMMC_TIMEOUT` reader"]
pub struct R(crate::R<EMMC_TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_TIMEOUT` writer"]
pub struct W(crate::W<EMMC_TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_TIMEOUT_SPEC>;
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
impl From<crate::W<EMMC_TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_TOCNT_MASK` reader - response /data timeout configuration"]
pub struct RB_EMMC_TOCNT_MASK_R(crate::FieldReader<u8, u8>);
impl RB_EMMC_TOCNT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_EMMC_TOCNT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_TOCNT_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_TOCNT_MASK` writer - response /data timeout configuration"]
pub struct RB_EMMC_TOCNT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_TOCNT_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - response /data timeout configuration"]
    #[inline(always)]
    pub fn rb_emmc_tocnt_mask(&self) -> RB_EMMC_TOCNT_MASK_R {
        RB_EMMC_TOCNT_MASK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - response /data timeout configuration"]
    #[inline(always)]
    pub fn rb_emmc_tocnt_mask(&mut self) -> RB_EMMC_TOCNT_MASK_W {
        RB_EMMC_TOCNT_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 8bits data timeout value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_timeout](index.html) module"]
pub struct EMMC_TIMEOUT_SPEC;
impl crate::RegisterSpec for EMMC_TIMEOUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [emmc_timeout::R](R) reader structure"]
impl crate::Readable for EMMC_TIMEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_timeout::W](W) writer structure"]
impl crate::Writable for EMMC_TIMEOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_TIMEOUT to value 0x0c"]
impl crate::Resettable for EMMC_TIMEOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}

#[doc = "Register `EMMC_RESPONSE2` reader"]
pub struct R(crate::R<EMMC_RESPONSE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_RESPONSE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_RESPONSE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_RESPONSE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EMMC_RESPONSE2` reader - response parameter register"]
pub struct EMMC_RESPONSE2_R(crate::FieldReader<u32, u32>);
impl EMMC_RESPONSE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EMMC_RESPONSE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMMC_RESPONSE2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - response parameter register"]
    #[inline(always)]
    pub fn emmc_response2(&self) -> EMMC_RESPONSE2_R {
        EMMC_RESPONSE2_R::new(self.bits)
    }
}
#[doc = "SD 128bits response register, \\[95:64\\]
32bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_response2](index.html) module"]
pub struct EMMC_RESPONSE2_SPEC;
impl crate::RegisterSpec for EMMC_RESPONSE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc_response2::R](R) reader structure"]
impl crate::Readable for EMMC_RESPONSE2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EMMC_RESPONSE2 to value 0"]
impl crate::Resettable for EMMC_RESPONSE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

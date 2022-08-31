#[doc = "Register `EMMC_RESPONSE0` reader"]
pub struct R(crate::R<EMMC_RESPONSE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_RESPONSE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_RESPONSE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_RESPONSE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EMMC_RESPONSE0` reader - response parameter register"]
pub struct EMMC_RESPONSE0_R(crate::FieldReader<u32, u32>);
impl EMMC_RESPONSE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EMMC_RESPONSE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMMC_RESPONSE0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - response parameter register"]
    #[inline(always)]
    pub fn emmc_response0(&self) -> EMMC_RESPONSE0_R {
        EMMC_RESPONSE0_R::new(self.bits)
    }
}
#[doc = "SD 128bits response register, \\[31:0\\]
32bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_response0](index.html) module"]
pub struct EMMC_RESPONSE0_SPEC;
impl crate::RegisterSpec for EMMC_RESPONSE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc_response0::R](R) reader structure"]
impl crate::Readable for EMMC_RESPONSE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EMMC_RESPONSE0 to value 0"]
impl crate::Resettable for EMMC_RESPONSE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

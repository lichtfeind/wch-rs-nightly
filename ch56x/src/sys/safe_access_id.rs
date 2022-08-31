#[doc = "Register `SAFE_ACCESS_ID` reader"]
pub struct R(crate::R<SAFE_ACCESS_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAFE_ACCESS_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAFE_ACCESS_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAFE_ACCESS_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAFE_ACCESS_ID` reader - safe accessing ID"]
pub struct SAFE_ACCESS_ID_R(crate::FieldReader<u8, u8>);
impl SAFE_ACCESS_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAFE_ACCESS_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAFE_ACCESS_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - safe accessing ID"]
    #[inline(always)]
    pub fn safe_access_id(&self) -> SAFE_ACCESS_ID_R {
        SAFE_ACCESS_ID_R::new(self.bits)
    }
}
#[doc = "safe accessing ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [safe_access_id](index.html) module"]
pub struct SAFE_ACCESS_ID_SPEC;
impl crate::RegisterSpec for SAFE_ACCESS_ID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [safe_access_id::R](R) reader structure"]
impl crate::Readable for SAFE_ACCESS_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAFE_ACCESS_ID to value 0x02"]
impl crate::Resettable for SAFE_ACCESS_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}

#[doc = "Register `CHIP_ID` reader"]
pub struct R(crate::R<CHIP_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIP_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIP_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIP_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHIP_ID` reader - chip ID"]
pub struct CHIP_ID_R(crate::FieldReader<u8, u8>);
impl CHIP_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHIP_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - chip ID"]
    #[inline(always)]
    pub fn chip_id(&self) -> CHIP_ID_R {
        CHIP_ID_R::new(self.bits)
    }
}
#[doc = "chip ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chip_id](index.html) module"]
pub struct CHIP_ID_SPEC;
impl crate::RegisterSpec for CHIP_ID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chip_id::R](R) reader structure"]
impl crate::Readable for CHIP_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHIP_ID to value 0x69"]
impl crate::Resettable for CHIP_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x69
    }
}

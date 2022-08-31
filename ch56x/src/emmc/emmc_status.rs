#[doc = "Register `EMMC_STATUS` reader"]
pub struct R(crate::R<EMMC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MASK_BLOCK_NUM` reader - the number of blocks successfully transmitted in the current multi-block transmission"]
pub struct MASK_BLOCK_NUM_R(crate::FieldReader<u16, u16>);
impl MASK_BLOCK_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MASK_BLOCK_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_BLOCK_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_CMDSTA` reader - indicate cmd line is high level now"]
pub struct RB_EMMC_CMDSTA_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_CMDSTA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_CMDSTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_CMDSTA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_DAT0STA` reader - indicate dat\\[0\\]
line is high level now"]
pub struct RB_EMMC_DAT0STA_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_DAT0STA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_DAT0STA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_DAT0STA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - the number of blocks successfully transmitted in the current multi-block transmission"]
    #[inline(always)]
    pub fn mask_block_num(&self) -> MASK_BLOCK_NUM_R {
        MASK_BLOCK_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - indicate cmd line is high level now"]
    #[inline(always)]
    pub fn rb_emmc_cmdsta(&self) -> RB_EMMC_CMDSTA_R {
        RB_EMMC_CMDSTA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - indicate dat\\[0\\]
line is high level now"]
    #[inline(always)]
    pub fn rb_emmc_dat0sta(&self) -> RB_EMMC_DAT0STA_R {
        RB_EMMC_DAT0STA_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "SD status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_status](index.html) module"]
pub struct EMMC_STATUS_SPEC;
impl crate::RegisterSpec for EMMC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc_status::R](R) reader structure"]
impl crate::Readable for EMMC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EMMC_STATUS to value 0"]
impl crate::Resettable for EMMC_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

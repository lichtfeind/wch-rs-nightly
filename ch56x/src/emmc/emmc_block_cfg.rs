#[doc = "Register `EMMC_BLOCK_CFG` reader"]
pub struct R(crate::R<EMMC_BLOCK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_BLOCK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_BLOCK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_BLOCK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_BLOCK_CFG` writer"]
pub struct W(crate::W<EMMC_BLOCK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_BLOCK_CFG_SPEC>;
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
impl From<crate::W<EMMC_BLOCK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_BLOCK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_BKNUM_MASK` reader - the number of blocks to be transferred"]
pub struct RB_EMMC_BKNUM_MASK_R(crate::FieldReader<u16, u16>);
impl RB_EMMC_BKNUM_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RB_EMMC_BKNUM_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_BKNUM_MASK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_BKNUM_MASK` writer - the number of blocks to be transferred"]
pub struct RB_EMMC_BKNUM_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_BKNUM_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `RB_EMMC_BKSIZE_MASK` reader - single block transfer size"]
pub struct RB_EMMC_BKSIZE_MASK_R(crate::FieldReader<u16, u16>);
impl RB_EMMC_BKSIZE_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RB_EMMC_BKSIZE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_BKSIZE_MASK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_BKSIZE_MASK` writer - single block transfer size"]
pub struct RB_EMMC_BKSIZE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_BKSIZE_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - the number of blocks to be transferred"]
    #[inline(always)]
    pub fn rb_emmc_bknum_mask(&self) -> RB_EMMC_BKNUM_MASK_R {
        RB_EMMC_BKNUM_MASK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - single block transfer size"]
    #[inline(always)]
    pub fn rb_emmc_bksize_mask(&self) -> RB_EMMC_BKSIZE_MASK_R {
        RB_EMMC_BKSIZE_MASK_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of blocks to be transferred"]
    #[inline(always)]
    pub fn rb_emmc_bknum_mask(&mut self) -> RB_EMMC_BKNUM_MASK_W {
        RB_EMMC_BKNUM_MASK_W { w: self }
    }
    #[doc = "Bits 16:27 - single block transfer size"]
    #[inline(always)]
    pub fn rb_emmc_bksize_mask(&mut self) -> RB_EMMC_BKSIZE_MASK_W {
        RB_EMMC_BKSIZE_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 32bits data counter, \\[15:0\\]
number of blocks this time will tran/recv, \\[27:16\\]
block sise(byte number) of every block in this time tran/recv\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_block_cfg](index.html) module"]
pub struct EMMC_BLOCK_CFG_SPEC;
impl crate::RegisterSpec for EMMC_BLOCK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc_block_cfg::R](R) reader structure"]
impl crate::Readable for EMMC_BLOCK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_block_cfg::W](W) writer structure"]
impl crate::Writable for EMMC_BLOCK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_BLOCK_CFG to value 0"]
impl crate::Resettable for EMMC_BLOCK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

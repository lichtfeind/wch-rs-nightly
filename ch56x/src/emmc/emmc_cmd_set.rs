#[doc = "Register `EMMC_CMD_SET` reader"]
pub struct R(crate::R<EMMC_CMD_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_CMD_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_CMD_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_CMD_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_CMD_SET` writer"]
pub struct W(crate::W<EMMC_CMD_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_CMD_SET_SPEC>;
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
impl From<crate::W<EMMC_CMD_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_CMD_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_CMDIDX_MASK` reader - the index number of the currently sent command"]
pub struct RB_EMMC_CMDIDX_MASK_R(crate::FieldReader<u8, u8>);
impl RB_EMMC_CMDIDX_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_EMMC_CMDIDX_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_CMDIDX_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_CMDIDX_MASK` writer - the index number of the currently sent command"]
pub struct RB_EMMC_CMDIDX_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_CMDIDX_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u16 & 0x3f);
        self.w
    }
}
#[doc = "Field `RB_EMMC_RPTY_MASK` reader - current respone type"]
pub struct RB_EMMC_RPTY_MASK_R(crate::FieldReader<u8, u8>);
impl RB_EMMC_RPTY_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_EMMC_RPTY_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_RPTY_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_RPTY_MASK` writer - current respone type"]
pub struct RB_EMMC_RPTY_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_RPTY_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u16 & 3) << 8);
        self.w
    }
}
#[doc = "Field `RB_EMMC_CKCRC` reader - check the response CRC"]
pub struct RB_EMMC_CKCRC_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_CKCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_CKCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_CKCRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_CKCRC` writer - check the response CRC"]
pub struct RB_EMMC_CKCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_CKCRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u16 & 1) << 10);
        self.w
    }
}
#[doc = "Field `RB_EMMC_CKIDX` reader - check the response command index"]
pub struct RB_EMMC_CKIDX_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_CKIDX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_CKIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_CKIDX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_CKIDX` writer - check the response command index"]
pub struct RB_EMMC_CKIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_CKIDX_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u16 & 1) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - the index number of the currently sent command"]
    #[inline(always)]
    pub fn rb_emmc_cmdidx_mask(&self) -> RB_EMMC_CMDIDX_MASK_R {
        RB_EMMC_CMDIDX_MASK_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - current respone type"]
    #[inline(always)]
    pub fn rb_emmc_rpty_mask(&self) -> RB_EMMC_RPTY_MASK_R {
        RB_EMMC_RPTY_MASK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - check the response CRC"]
    #[inline(always)]
    pub fn rb_emmc_ckcrc(&self) -> RB_EMMC_CKCRC_R {
        RB_EMMC_CKCRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - check the response command index"]
    #[inline(always)]
    pub fn rb_emmc_ckidx(&self) -> RB_EMMC_CKIDX_R {
        RB_EMMC_CKIDX_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - the index number of the currently sent command"]
    #[inline(always)]
    pub fn rb_emmc_cmdidx_mask(&mut self) -> RB_EMMC_CMDIDX_MASK_W {
        RB_EMMC_CMDIDX_MASK_W { w: self }
    }
    #[doc = "Bits 8:9 - current respone type"]
    #[inline(always)]
    pub fn rb_emmc_rpty_mask(&mut self) -> RB_EMMC_RPTY_MASK_W {
        RB_EMMC_RPTY_MASK_W { w: self }
    }
    #[doc = "Bit 10 - check the response CRC"]
    #[inline(always)]
    pub fn rb_emmc_ckcrc(&mut self) -> RB_EMMC_CKCRC_W {
        RB_EMMC_CKCRC_W { w: self }
    }
    #[doc = "Bit 11 - check the response command index"]
    #[inline(always)]
    pub fn rb_emmc_ckidx(&mut self) -> RB_EMMC_CKIDX_W {
        RB_EMMC_CKIDX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD 16bits cmd setting register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_cmd_set](index.html) module"]
pub struct EMMC_CMD_SET_SPEC;
impl crate::RegisterSpec for EMMC_CMD_SET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [emmc_cmd_set::R](R) reader structure"]
impl crate::Readable for EMMC_CMD_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_cmd_set::W](W) writer structure"]
impl crate::Writable for EMMC_CMD_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_CMD_SET to value 0"]
impl crate::Resettable for EMMC_CMD_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

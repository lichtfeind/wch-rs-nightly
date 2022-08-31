#[doc = "Register `GLOB_ROM_CFG` reader"]
pub struct R(crate::R<GLOB_ROM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOB_ROM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOB_ROM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOB_ROM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOB_ROM_CFG` writer"]
pub struct W(crate::W<GLOB_ROM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOB_ROM_CFG_SPEC>;
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
impl From<crate::W<GLOB_ROM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOB_ROM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_ROM_EXT_RE` reader - enable flash ROM being read by external programmer"]
pub struct RB_ROM_EXT_RE_R(crate::FieldReader<bool, bool>);
impl RB_ROM_EXT_RE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ROM_EXT_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ROM_EXT_RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ROM_EXT_RE` writer - enable flash ROM being read by external programmer"]
pub struct RB_ROM_EXT_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ROM_EXT_RE_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u8 & 1);
        self.w
    }
}
#[doc = "Field `RB_CODE_RAM_WE` reader - enable code RAM being write"]
pub struct RB_CODE_RAM_WE_R(crate::FieldReader<bool, bool>);
impl RB_CODE_RAM_WE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_CODE_RAM_WE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_CODE_RAM_WE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_CODE_RAM_WE` writer - enable code RAM being write"]
pub struct RB_CODE_RAM_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_CODE_RAM_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u8 & 1) << 1);
        self.w
    }
}
#[doc = "Field `RB_ROM_DATA_WE` reader - enable flash ROM data area being erase/write"]
pub struct RB_ROM_DATA_WE_R(crate::FieldReader<bool, bool>);
impl RB_ROM_DATA_WE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ROM_DATA_WE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ROM_DATA_WE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ROM_DATA_WE` writer - enable flash ROM data area being erase/write"]
pub struct RB_ROM_DATA_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ROM_DATA_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u8 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RB_ROM_CODE_WE` reader - enable flash ROM code and data area being erase or write"]
pub struct RB_ROM_CODE_WE_R(crate::FieldReader<bool, bool>);
impl RB_ROM_CODE_WE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ROM_CODE_WE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ROM_CODE_WE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ROM_CODE_WE` writer - enable flash ROM code and data area being erase or write"]
pub struct RB_ROM_CODE_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ROM_CODE_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u8 & 1) << 3);
        self.w
    }
}
#[doc = "Field `RB_ROM_CODE_OFS` reader - Config the start offset address of user code in Flash"]
pub struct RB_ROM_CODE_OFS_R(crate::FieldReader<bool, bool>);
impl RB_ROM_CODE_OFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ROM_CODE_OFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ROM_CODE_OFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ROM_CODE_OFS` writer - Config the start offset address of user code in Flash"]
pub struct RB_ROM_CODE_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ROM_CODE_OFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u8 & 1) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - enable flash ROM being read by external programmer"]
    #[inline(always)]
    pub fn rb_rom_ext_re(&self) -> RB_ROM_EXT_RE_R {
        RB_ROM_EXT_RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable code RAM being write"]
    #[inline(always)]
    pub fn rb_code_ram_we(&self) -> RB_CODE_RAM_WE_R {
        RB_CODE_RAM_WE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable flash ROM data area being erase/write"]
    #[inline(always)]
    pub fn rb_rom_data_we(&self) -> RB_ROM_DATA_WE_R {
        RB_ROM_DATA_WE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable flash ROM code and data area being erase or write"]
    #[inline(always)]
    pub fn rb_rom_code_we(&self) -> RB_ROM_CODE_WE_R {
        RB_ROM_CODE_WE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Config the start offset address of user code in Flash"]
    #[inline(always)]
    pub fn rb_rom_code_ofs(&self) -> RB_ROM_CODE_OFS_R {
        RB_ROM_CODE_OFS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable flash ROM being read by external programmer"]
    #[inline(always)]
    pub fn rb_rom_ext_re(&mut self) -> RB_ROM_EXT_RE_W {
        RB_ROM_EXT_RE_W { w: self }
    }
    #[doc = "Bit 1 - enable code RAM being write"]
    #[inline(always)]
    pub fn rb_code_ram_we(&mut self) -> RB_CODE_RAM_WE_W {
        RB_CODE_RAM_WE_W { w: self }
    }
    #[doc = "Bit 2 - enable flash ROM data area being erase/write"]
    #[inline(always)]
    pub fn rb_rom_data_we(&mut self) -> RB_ROM_DATA_WE_W {
        RB_ROM_DATA_WE_W { w: self }
    }
    #[doc = "Bit 3 - enable flash ROM code and data area being erase or write"]
    #[inline(always)]
    pub fn rb_rom_code_we(&mut self) -> RB_ROM_CODE_WE_W {
        RB_ROM_CODE_WE_W { w: self }
    }
    #[doc = "Bit 4 - Config the start offset address of user code in Flash"]
    #[inline(always)]
    pub fn rb_rom_code_ofs(&mut self) -> RB_ROM_CODE_OFS_W {
        RB_ROM_CODE_OFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "flash ROM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glob_rom_cfg](index.html) module"]
pub struct GLOB_ROM_CFG_SPEC;
impl crate::RegisterSpec for GLOB_ROM_CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [glob_rom_cfg::R](R) reader structure"]
impl crate::Readable for GLOB_ROM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glob_rom_cfg::W](W) writer structure"]
impl crate::Writable for GLOB_ROM_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLOB_ROM_CFG to value 0x80"]
impl crate::Resettable for GLOB_ROM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}

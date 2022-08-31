#[doc = "Register `ECEC_CTRL` reader"]
pub struct R(crate::R<ECEC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECEC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECEC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECEC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECEC_CTRL` writer"]
pub struct W(crate::W<ECEC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECEC_CTRL_SPEC>;
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
impl From<crate::W<ECEC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECEC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_ECDC_KEYEX_EN` reader - enable key expansion"]
pub struct RB_ECDC_KEYEX_EN_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_KEYEX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_KEYEX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_KEYEX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_KEYEX_EN` writer - enable key expansion"]
pub struct RB_ECDC_KEYEX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_KEYEX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
#[doc = "Field `RB_ECDC_RDPERI_EN` reader - when write data to dma"]
pub struct RB_ECDC_RDPERI_EN_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_RDPERI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_RDPERI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_RDPERI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_RDPERI_EN` writer - when write data to dma"]
pub struct RB_ECDC_RDPERI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_RDPERI_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `RB_ECDC_WRPERI_EN` reader - when read data from dma"]
pub struct RB_ECDC_WRPERI_EN_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_WRPERI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_WRPERI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_WRPERI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_WRPERI_EN` writer - when read data from dma"]
pub struct RB_ECDC_WRPERI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_WRPERI_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RB_ECDC_MODE_SEL` reader - ECDC mode select"]
pub struct RB_ECDC_MODE_SEL_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_MODE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_MODE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_MODE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_MODE_SEL` writer - ECDC mode select"]
pub struct RB_ECDC_MODE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_MODE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `RB_ECDC_CLKDIV_MASK` reader - Clock divide factor"]
pub struct RB_ECDC_CLKDIV_MASK_R(crate::FieldReader<u8, u8>);
impl RB_ECDC_CLKDIV_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_ECDC_CLKDIV_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_CLKDIV_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_CLKDIV_MASK` writer - Clock divide factor"]
pub struct RB_ECDC_CLKDIV_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_CLKDIV_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u16 & 7) << 4);
        self.w
    }
}
#[doc = "Field `RB_ECDC_WRSRAM_EN` reader - module dma enable"]
pub struct RB_ECDC_WRSRAM_EN_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_WRSRAM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_WRSRAM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_WRSRAM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_WRSRAM_EN` writer - module dma enable"]
pub struct RB_ECDC_WRSRAM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_WRSRAM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `RB_ECDC_ALGRM_MOD` reader - Encryption and decryption algorithm mode selection"]
pub struct RB_ECDC_ALGRM_MOD_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_ALGRM_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_ALGRM_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_ALGRM_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_ALGRM_MOD` writer - Encryption and decryption algorithm mode selection"]
pub struct RB_ECDC_ALGRM_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_ALGRM_MOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u16 & 1) << 8);
        self.w
    }
}
#[doc = "Field `RB_ECDC_CIPHER_MOD` reader - Block cipher mode selection"]
pub struct RB_ECDC_CIPHER_MOD_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_CIPHER_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_CIPHER_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_CIPHER_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_CIPHER_MOD` writer - Block cipher mode selection"]
pub struct RB_ECDC_CIPHER_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_CIPHER_MOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u16 & 1) << 9);
        self.w
    }
}
#[doc = "Field `RB_ECDC_KLEN_MASK` reader - Key length setting"]
pub struct RB_ECDC_KLEN_MASK_R(crate::FieldReader<u8, u8>);
impl RB_ECDC_KLEN_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_ECDC_KLEN_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_KLEN_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_KLEN_MASK` writer - Key length setting"]
pub struct RB_ECDC_KLEN_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_KLEN_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u16 & 3) << 10);
        self.w
    }
}
#[doc = "Field `RB_ECDC_DAT_MOD` reader - source data and result data is bit endian"]
pub struct RB_ECDC_DAT_MOD_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_DAT_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_DAT_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_DAT_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_DAT_MOD` writer - source data and result data is bit endian"]
pub struct RB_ECDC_DAT_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_DAT_MOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u16 & 1) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - enable key expansion"]
    #[inline(always)]
    pub fn rb_ecdc_keyex_en(&self) -> RB_ECDC_KEYEX_EN_R {
        RB_ECDC_KEYEX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when write data to dma"]
    #[inline(always)]
    pub fn rb_ecdc_rdperi_en(&self) -> RB_ECDC_RDPERI_EN_R {
        RB_ECDC_RDPERI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - when read data from dma"]
    #[inline(always)]
    pub fn rb_ecdc_wrperi_en(&self) -> RB_ECDC_WRPERI_EN_R {
        RB_ECDC_WRPERI_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECDC mode select"]
    #[inline(always)]
    pub fn rb_ecdc_mode_sel(&self) -> RB_ECDC_MODE_SEL_R {
        RB_ECDC_MODE_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Clock divide factor"]
    #[inline(always)]
    pub fn rb_ecdc_clkdiv_mask(&self) -> RB_ECDC_CLKDIV_MASK_R {
        RB_ECDC_CLKDIV_MASK_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - module dma enable"]
    #[inline(always)]
    pub fn rb_ecdc_wrsram_en(&self) -> RB_ECDC_WRSRAM_EN_R {
        RB_ECDC_WRSRAM_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Encryption and decryption algorithm mode selection"]
    #[inline(always)]
    pub fn rb_ecdc_algrm_mod(&self) -> RB_ECDC_ALGRM_MOD_R {
        RB_ECDC_ALGRM_MOD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Block cipher mode selection"]
    #[inline(always)]
    pub fn rb_ecdc_cipher_mod(&self) -> RB_ECDC_CIPHER_MOD_R {
        RB_ECDC_CIPHER_MOD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Key length setting"]
    #[inline(always)]
    pub fn rb_ecdc_klen_mask(&self) -> RB_ECDC_KLEN_MASK_R {
        RB_ECDC_KLEN_MASK_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - source data and result data is bit endian"]
    #[inline(always)]
    pub fn rb_ecdc_dat_mod(&self) -> RB_ECDC_DAT_MOD_R {
        RB_ECDC_DAT_MOD_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable key expansion"]
    #[inline(always)]
    pub fn rb_ecdc_keyex_en(&mut self) -> RB_ECDC_KEYEX_EN_W {
        RB_ECDC_KEYEX_EN_W { w: self }
    }
    #[doc = "Bit 1 - when write data to dma"]
    #[inline(always)]
    pub fn rb_ecdc_rdperi_en(&mut self) -> RB_ECDC_RDPERI_EN_W {
        RB_ECDC_RDPERI_EN_W { w: self }
    }
    #[doc = "Bit 2 - when read data from dma"]
    #[inline(always)]
    pub fn rb_ecdc_wrperi_en(&mut self) -> RB_ECDC_WRPERI_EN_W {
        RB_ECDC_WRPERI_EN_W { w: self }
    }
    #[doc = "Bit 3 - ECDC mode select"]
    #[inline(always)]
    pub fn rb_ecdc_mode_sel(&mut self) -> RB_ECDC_MODE_SEL_W {
        RB_ECDC_MODE_SEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Clock divide factor"]
    #[inline(always)]
    pub fn rb_ecdc_clkdiv_mask(&mut self) -> RB_ECDC_CLKDIV_MASK_W {
        RB_ECDC_CLKDIV_MASK_W { w: self }
    }
    #[doc = "Bit 7 - module dma enable"]
    #[inline(always)]
    pub fn rb_ecdc_wrsram_en(&mut self) -> RB_ECDC_WRSRAM_EN_W {
        RB_ECDC_WRSRAM_EN_W { w: self }
    }
    #[doc = "Bit 8 - Encryption and decryption algorithm mode selection"]
    #[inline(always)]
    pub fn rb_ecdc_algrm_mod(&mut self) -> RB_ECDC_ALGRM_MOD_W {
        RB_ECDC_ALGRM_MOD_W { w: self }
    }
    #[doc = "Bit 9 - Block cipher mode selection"]
    #[inline(always)]
    pub fn rb_ecdc_cipher_mod(&mut self) -> RB_ECDC_CIPHER_MOD_W {
        RB_ECDC_CIPHER_MOD_W { w: self }
    }
    #[doc = "Bits 10:11 - Key length setting"]
    #[inline(always)]
    pub fn rb_ecdc_klen_mask(&mut self) -> RB_ECDC_KLEN_MASK_W {
        RB_ECDC_KLEN_MASK_W { w: self }
    }
    #[doc = "Bit 13 - source data and result data is bit endian"]
    #[inline(always)]
    pub fn rb_ecdc_dat_mod(&mut self) -> RB_ECDC_DAT_MOD_W {
        RB_ECDC_DAT_MOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECED AES/SM4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecec_ctrl](index.html) module"]
pub struct ECEC_CTRL_SPEC;
impl crate::RegisterSpec for ECEC_CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ecec_ctrl::R](R) reader structure"]
impl crate::Readable for ECEC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecec_ctrl::W](W) writer structure"]
impl crate::Writable for ECEC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECEC_CTRL to value 0x20"]
impl crate::Resettable for ECEC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}

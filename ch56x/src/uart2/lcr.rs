#[doc = "Register `LCR` reader"]
pub struct R(crate::R<LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR` writer"]
pub struct W(crate::W<LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_SPEC>;
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
impl From<crate::W<LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WORD_SZ` reader - UART word bit length"]
pub struct WORD_SZ_R(crate::FieldReader<u8, u8>);
impl WORD_SZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WORD_SZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WORD_SZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WORD_SZ` writer - UART word bit length"]
pub struct WORD_SZ_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_SZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u8 & 3);
        self.w
    }
}
#[doc = "Field `STOP_BIT` reader - UART stop bit length"]
pub struct STOP_BIT_R(crate::FieldReader<bool, bool>);
impl STOP_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_BIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_BIT` writer - UART stop bit length"]
pub struct STOP_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_BIT_W<'a> {
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
#[doc = "Field `PAR_EN` reader - UART parity enable"]
pub struct PAR_EN_R(crate::FieldReader<bool, bool>);
impl PAR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAR_EN` writer - UART parity enable"]
pub struct PAR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAR_EN_W<'a> {
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
#[doc = "Field `PAR_MOD` reader - UART parity mode"]
pub struct PAR_MOD_R(crate::FieldReader<u8, u8>);
impl PAR_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAR_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAR_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAR_MOD` writer - UART parity mode"]
pub struct PAR_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAR_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u8 & 3) << 4);
        self.w
    }
}
#[doc = "Field `BREAK_EN` reader - UART break control enable"]
pub struct BREAK_EN_R(crate::FieldReader<bool, bool>);
impl BREAK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BREAK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREAK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREAK_EN` writer - UART break control enable"]
pub struct BREAK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BREAK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u8 & 1) << 6);
        self.w
    }
}
#[doc = "Field `DLAB_RB_LCR_GP_BIT` reader - UART reserved bit _ UART general purpose bit"]
pub struct DLAB_RB_LCR_GP_BIT_R(crate::FieldReader<bool, bool>);
impl DLAB_RB_LCR_GP_BIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLAB_RB_LCR_GP_BIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLAB_RB_LCR_GP_BIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLAB_RB_LCR_GP_BIT` writer - UART reserved bit _ UART general purpose bit"]
pub struct DLAB_RB_LCR_GP_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAB_RB_LCR_GP_BIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u8 & 1) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - UART word bit length"]
    #[inline(always)]
    pub fn word_sz(&self) -> WORD_SZ_R {
        WORD_SZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - UART stop bit length"]
    #[inline(always)]
    pub fn stop_bit(&self) -> STOP_BIT_R {
        STOP_BIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART parity enable"]
    #[inline(always)]
    pub fn par_en(&self) -> PAR_EN_R {
        PAR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - UART parity mode"]
    #[inline(always)]
    pub fn par_mod(&self) -> PAR_MOD_R {
        PAR_MOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - UART break control enable"]
    #[inline(always)]
    pub fn break_en(&self) -> BREAK_EN_R {
        BREAK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART reserved bit _ UART general purpose bit"]
    #[inline(always)]
    pub fn dlab_rb_lcr_gp_bit(&self) -> DLAB_RB_LCR_GP_BIT_R {
        DLAB_RB_LCR_GP_BIT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART word bit length"]
    #[inline(always)]
    pub fn word_sz(&mut self) -> WORD_SZ_W {
        WORD_SZ_W { w: self }
    }
    #[doc = "Bit 2 - UART stop bit length"]
    #[inline(always)]
    pub fn stop_bit(&mut self) -> STOP_BIT_W {
        STOP_BIT_W { w: self }
    }
    #[doc = "Bit 3 - UART parity enable"]
    #[inline(always)]
    pub fn par_en(&mut self) -> PAR_EN_W {
        PAR_EN_W { w: self }
    }
    #[doc = "Bits 4:5 - UART parity mode"]
    #[inline(always)]
    pub fn par_mod(&mut self) -> PAR_MOD_W {
        PAR_MOD_W { w: self }
    }
    #[doc = "Bit 6 - UART break control enable"]
    #[inline(always)]
    pub fn break_en(&mut self) -> BREAK_EN_W {
        BREAK_EN_W { w: self }
    }
    #[doc = "Bit 7 - UART reserved bit _ UART general purpose bit"]
    #[inline(always)]
    pub fn dlab_rb_lcr_gp_bit(&mut self) -> DLAB_RB_LCR_GP_BIT_W {
        DLAB_RB_LCR_GP_BIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART2 line control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr](index.html) module"]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcr::R](R) reader structure"]
impl crate::Readable for LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr::W](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

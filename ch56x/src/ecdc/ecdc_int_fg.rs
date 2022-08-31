#[doc = "Register `ECDC_INT_FG` reader"]
pub struct R(crate::R<ECDC_INT_FG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECDC_INT_FG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECDC_INT_FG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECDC_INT_FG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECDC_INT_FG` writer"]
pub struct W(crate::W<ECDC_INT_FG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECDC_INT_FG_SPEC>;
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
impl From<crate::W<ECDC_INT_FG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECDC_INT_FG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_ECDC_IF_EKDONE` reader - Key extension completion interrupt flag"]
pub struct RB_ECDC_IF_EKDONE_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_IF_EKDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_IF_EKDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_IF_EKDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_IF_EKDONE` writer - Key extension completion interrupt flag"]
pub struct RB_ECDC_IF_EKDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_IF_EKDONE_W<'a> {
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
#[doc = "Field `RB_ECDC_IF_SINGLE` reader - Single encryption and decryption completion interrupt flag"]
pub struct RB_ECDC_IF_SINGLE_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_IF_SINGLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_IF_SINGLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_IF_SINGLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_IF_SINGLE` writer - Single encryption and decryption completion interrupt flag"]
pub struct RB_ECDC_IF_SINGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_IF_SINGLE_W<'a> {
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
#[doc = "Field `RB_ECDC_IF_WRSRAM` reader - Memory to memory encryption and decryption completion interrupt flag"]
pub struct RB_ECDC_IF_WRSRAM_R(crate::FieldReader<bool, bool>);
impl RB_ECDC_IF_WRSRAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_ECDC_IF_WRSRAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_ECDC_IF_WRSRAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_ECDC_IF_WRSRAM` writer - Memory to memory encryption and decryption completion interrupt flag"]
pub struct RB_ECDC_IF_WRSRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_ECDC_IF_WRSRAM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Key extension completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_ekdone(&self) -> RB_ECDC_IF_EKDONE_R {
        RB_ECDC_IF_EKDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single encryption and decryption completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_single(&self) -> RB_ECDC_IF_SINGLE_R {
        RB_ECDC_IF_SINGLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory to memory encryption and decryption completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_wrsram(&self) -> RB_ECDC_IF_WRSRAM_R {
        RB_ECDC_IF_WRSRAM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key extension completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_ekdone(&mut self) -> RB_ECDC_IF_EKDONE_W {
        RB_ECDC_IF_EKDONE_W { w: self }
    }
    #[doc = "Bit 1 - Single encryption and decryption completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_single(&mut self) -> RB_ECDC_IF_SINGLE_W {
        RB_ECDC_IF_SINGLE_W { w: self }
    }
    #[doc = "Bit 2 - Memory to memory encryption and decryption completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_wrsram(&mut self) -> RB_ECDC_IF_WRSRAM_W {
        RB_ECDC_IF_WRSRAM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecdc_int_fg](index.html) module"]
pub struct ECDC_INT_FG_SPEC;
impl crate::RegisterSpec for ECDC_INT_FG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ecdc_int_fg::R](R) reader structure"]
impl crate::Readable for ECDC_INT_FG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecdc_int_fg::W](W) writer structure"]
impl crate::Writable for ECDC_INT_FG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECDC_INT_FG to value 0"]
impl crate::Resettable for ECDC_INT_FG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

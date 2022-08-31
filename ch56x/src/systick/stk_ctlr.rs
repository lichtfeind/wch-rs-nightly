#[doc = "Register `STK_CTLR` reader"]
pub struct R(crate::R<STK_CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STK_CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STK_CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STK_CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STK_CTLR` writer"]
pub struct W(crate::W<STK_CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STK_CTLR_SPEC>;
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
impl From<crate::W<STK_CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STK_CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STE` reader - Systick counter enable"]
pub struct STE_R(crate::FieldReader<bool, bool>);
impl STE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STE` writer - Systick counter enable"]
pub struct STE_W<'a> {
    w: &'a mut W,
}
impl<'a> STE_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `STIE` reader - Systick counter interrupt enable"]
pub struct STIE_R(crate::FieldReader<bool, bool>);
impl STIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIE` writer - Systick counter interrupt enable"]
pub struct STIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `STCLK` reader - System counter clock Source selection"]
pub struct STCLK_R(crate::FieldReader<bool, bool>);
impl STCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STCLK` writer - System counter clock Source selection"]
pub struct STCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> STCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `STRELOAD` reader - System counter reload control"]
pub struct STRELOAD_R(crate::FieldReader<bool, bool>);
impl STRELOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STRELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRELOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRELOAD` writer - System counter reload control"]
pub struct STRELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> STRELOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Systick counter enable"]
    #[inline(always)]
    pub fn ste(&self) -> STE_R {
        STE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Systick counter interrupt enable"]
    #[inline(always)]
    pub fn stie(&self) -> STIE_R {
        STIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System counter clock Source selection"]
    #[inline(always)]
    pub fn stclk(&self) -> STCLK_R {
        STCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - System counter reload control"]
    #[inline(always)]
    pub fn streload(&self) -> STRELOAD_R {
        STRELOAD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Systick counter enable"]
    #[inline(always)]
    pub fn ste(&mut self) -> STE_W {
        STE_W { w: self }
    }
    #[doc = "Bit 1 - Systick counter interrupt enable"]
    #[inline(always)]
    pub fn stie(&mut self) -> STIE_W {
        STIE_W { w: self }
    }
    #[doc = "Bit 2 - System counter clock Source selection"]
    #[inline(always)]
    pub fn stclk(&mut self) -> STCLK_W {
        STCLK_W { w: self }
    }
    #[doc = "Bit 8 - System counter reload control"]
    #[inline(always)]
    pub fn streload(&mut self) -> STRELOAD_W {
        STRELOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Systick counter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_ctlr](index.html) module"]
pub struct STK_CTLR_SPEC;
impl crate::RegisterSpec for STK_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stk_ctlr::R](R) reader structure"]
impl crate::Readable for STK_CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stk_ctlr::W](W) writer structure"]
impl crate::Writable for STK_CTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STK_CTLR to value 0"]
impl crate::Resettable for STK_CTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

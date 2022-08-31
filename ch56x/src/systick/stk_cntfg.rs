#[doc = "Register `STK_CNTFG` reader"]
pub struct R(crate::R<STK_CNTFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STK_CNTFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STK_CNTFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STK_CNTFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STK_CNTFG` writer"]
pub struct W(crate::W<STK_CNTFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STK_CNTFG_SPEC>;
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
impl From<crate::W<STK_CNTFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STK_CNTFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWIE` reader - System soft interrupt enable"]
pub struct SWIE_R(crate::FieldReader<bool, bool>);
impl SWIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWIE` writer - System soft interrupt enable"]
pub struct SWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIE_W<'a> {
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
#[doc = "Field `CNTIF` reader - Systick counter clear zero flag"]
pub struct CNTIF_R(crate::FieldReader<bool, bool>);
impl CNTIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CNTIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTIF` writer - Systick counter clear zero flag"]
pub struct CNTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTIF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - System soft interrupt enable"]
    #[inline(always)]
    pub fn swie(&self) -> SWIE_R {
        SWIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Systick counter clear zero flag"]
    #[inline(always)]
    pub fn cntif(&self) -> CNTIF_R {
        CNTIF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System soft interrupt enable"]
    #[inline(always)]
    pub fn swie(&mut self) -> SWIE_W {
        SWIE_W { w: self }
    }
    #[doc = "Bit 1 - Systick counter clear zero flag"]
    #[inline(always)]
    pub fn cntif(&mut self) -> CNTIF_W {
        CNTIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Systick counter flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_cntfg](index.html) module"]
pub struct STK_CNTFG_SPEC;
impl crate::RegisterSpec for STK_CNTFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stk_cntfg::R](R) reader structure"]
impl crate::Readable for STK_CNTFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stk_cntfg::W](W) writer structure"]
impl crate::Writable for STK_CNTFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STK_CNTFG to value 0"]
impl crate::Resettable for STK_CNTFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `HSPI_BURST_CFG` reader"]
pub struct R(crate::R<HSPI_BURST_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSPI_BURST_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSPI_BURST_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSPI_BURST_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSPI_BURST_CFG` writer"]
pub struct W(crate::W<HSPI_BURST_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSPI_BURST_CFG_SPEC>;
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
impl From<crate::W<HSPI_BURST_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSPI_BURST_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_HSPI_BURST_EN` reader - burst transmit enable"]
pub struct RB_HSPI_BURST_EN_R(crate::FieldReader<bool, bool>);
impl RB_HSPI_BURST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_HSPI_BURST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_BURST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_BURST_EN` writer - burst transmit enable"]
pub struct RB_HSPI_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_BURST_EN_W<'a> {
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
#[doc = "Field `RB_HSPI_BURST_LEN` reader - burst transmit length"]
pub struct RB_HSPI_BURST_LEN_R(crate::FieldReader<u8, u8>);
impl RB_HSPI_BURST_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_HSPI_BURST_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_HSPI_BURST_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_HSPI_BURST_LEN` writer - burst transmit length"]
pub struct RB_HSPI_BURST_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_HSPI_BURST_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - burst transmit enable"]
    #[inline(always)]
    pub fn rb_hspi_burst_en(&self) -> RB_HSPI_BURST_EN_R {
        RB_HSPI_BURST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - burst transmit length"]
    #[inline(always)]
    pub fn rb_hspi_burst_len(&self) -> RB_HSPI_BURST_LEN_R {
        RB_HSPI_BURST_LEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - burst transmit enable"]
    #[inline(always)]
    pub fn rb_hspi_burst_en(&mut self) -> RB_HSPI_BURST_EN_W {
        RB_HSPI_BURST_EN_W { w: self }
    }
    #[doc = "Bits 8:15 - burst transmit length"]
    #[inline(always)]
    pub fn rb_hspi_burst_len(&mut self) -> RB_HSPI_BURST_LEN_W {
        RB_HSPI_BURST_LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "parallel if tx burst config register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hspi_burst_cfg](index.html) module"]
pub struct HSPI_BURST_CFG_SPEC;
impl crate::RegisterSpec for HSPI_BURST_CFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hspi_burst_cfg::R](R) reader structure"]
impl crate::Readable for HSPI_BURST_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hspi_burst_cfg::W](W) writer structure"]
impl crate::Writable for HSPI_BURST_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSPI_BURST_CFG to value 0"]
impl crate::Resettable for HSPI_BURST_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

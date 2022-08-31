#[doc = "Register `SERD_ANA_CFG1` reader"]
pub struct R(crate::R<SERD_ANA_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERD_ANA_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERD_ANA_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERD_ANA_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SERD_ANA_CFG1` writer"]
pub struct W(crate::W<SERD_ANA_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SERD_ANA_CFG1_SPEC>;
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
impl From<crate::W<SERD_ANA_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SERD_ANA_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SERD_PLL_CFG` reader - SerDes PHY internal configuration bit"]
pub struct RB_SERD_PLL_CFG_R(crate::FieldReader<u8, u8>);
impl RB_SERD_PLL_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_SERD_PLL_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SERD_PLL_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SERD_PLL_CFG` writer - SerDes PHY internal configuration bit"]
pub struct RB_SERD_PLL_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SERD_PLL_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `RB_SERD_30M_SEL` reader - SerDes PHY reference clock source seletion"]
pub struct RB_SERD_30M_SEL_R(crate::FieldReader<bool, bool>);
impl RB_SERD_30M_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SERD_30M_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SERD_30M_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SERD_30M_SEL` writer - SerDes PHY reference clock source seletion"]
pub struct RB_SERD_30M_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SERD_30M_SEL_W<'a> {
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
#[doc = "Field `RB_SERD_DN_SEL` reader - Enable SerDes PHY GXM test pin"]
pub struct RB_SERD_DN_SEL_R(crate::FieldReader<bool, bool>);
impl RB_SERD_DN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SERD_DN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SERD_DN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SERD_DN_SEL` writer - Enable SerDes PHY GXM test pin"]
pub struct RB_SERD_DN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SERD_DN_SEL_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - SerDes PHY internal configuration bit"]
    #[inline(always)]
    pub fn rb_serd_pll_cfg(&self) -> RB_SERD_PLL_CFG_R {
        RB_SERD_PLL_CFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - SerDes PHY reference clock source seletion"]
    #[inline(always)]
    pub fn rb_serd_30m_sel(&self) -> RB_SERD_30M_SEL_R {
        RB_SERD_30M_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable SerDes PHY GXM test pin"]
    #[inline(always)]
    pub fn rb_serd_dn_sel(&self) -> RB_SERD_DN_SEL_R {
        RB_SERD_DN_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - SerDes PHY internal configuration bit"]
    #[inline(always)]
    pub fn rb_serd_pll_cfg(&mut self) -> RB_SERD_PLL_CFG_W {
        RB_SERD_PLL_CFG_W { w: self }
    }
    #[doc = "Bit 8 - SerDes PHY reference clock source seletion"]
    #[inline(always)]
    pub fn rb_serd_30m_sel(&mut self) -> RB_SERD_30M_SEL_W {
        RB_SERD_30M_SEL_W { w: self }
    }
    #[doc = "Bit 9 - Enable SerDes PHY GXM test pin"]
    #[inline(always)]
    pub fn rb_serd_dn_sel(&mut self) -> RB_SERD_DN_SEL_W {
        RB_SERD_DN_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serdes Analog parameter configuration1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serd_ana_cfg1](index.html) module"]
pub struct SERD_ANA_CFG1_SPEC;
impl crate::RegisterSpec for SERD_ANA_CFG1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [serd_ana_cfg1::R](R) reader structure"]
impl crate::Readable for SERD_ANA_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [serd_ana_cfg1::W](W) writer structure"]
impl crate::Writable for SERD_ANA_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SERD_ANA_CFG1 to value 0x5a"]
impl crate::Resettable for SERD_ANA_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5a
    }
}

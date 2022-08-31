#[doc = "Register `CLK_MOD_AUX` reader"]
pub struct R(crate::R<CLK_MOD_AUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_MOD_AUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_MOD_AUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_MOD_AUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_MOD_AUX` writer"]
pub struct W(crate::W<CLK_MOD_AUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_MOD_AUX_SPEC>;
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
impl From<crate::W<CLK_MOD_AUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_MOD_AUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_INT_125M_EN` reader - clock from USB_PHY PCLK(125MHz)"]
pub struct RB_INT_125M_EN_R(crate::FieldReader<bool, bool>);
impl RB_INT_125M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_INT_125M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_INT_125M_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_INT_125M_EN` writer - clock from USB_PHY PCLK(125MHz)"]
pub struct RB_INT_125M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_INT_125M_EN_W<'a> {
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
#[doc = "Field `RB_EXT_125M_EN` reader - clock from pin_PA\\[16\\]"]
pub struct RB_EXT_125M_EN_R(crate::FieldReader<bool, bool>);
impl RB_EXT_125M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EXT_125M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EXT_125M_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EXT_125M_EN` writer - clock from pin_PA\\[16\\]"]
pub struct RB_EXT_125M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EXT_125M_EN_W<'a> {
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
#[doc = "Field `RB_MCO_SEL_MSK` reader - MCO output selection"]
pub struct RB_MCO_SEL_MSK_R(crate::FieldReader<u8, u8>);
impl RB_MCO_SEL_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_MCO_SEL_MSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_MCO_SEL_MSK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_MCO_SEL_MSK` writer - MCO output selection"]
pub struct RB_MCO_SEL_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_MCO_SEL_MSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u8 & 3) << 2);
        self.w
    }
}
#[doc = "Field `RB_MCO_EN` reader - MCO output enable"]
pub struct RB_MCO_EN_R(crate::FieldReader<bool, bool>);
impl RB_MCO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_MCO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_MCO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_MCO_EN` writer - MCO output enable"]
pub struct RB_MCO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_MCO_EN_W<'a> {
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
    #[doc = "Bit 0 - clock from USB_PHY PCLK(125MHz)"]
    #[inline(always)]
    pub fn rb_int_125m_en(&self) -> RB_INT_125M_EN_R {
        RB_INT_125M_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock from pin_PA\\[16\\]"]
    #[inline(always)]
    pub fn rb_ext_125m_en(&self) -> RB_EXT_125M_EN_R {
        RB_EXT_125M_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MCO output selection"]
    #[inline(always)]
    pub fn rb_mco_sel_msk(&self) -> RB_MCO_SEL_MSK_R {
        RB_MCO_SEL_MSK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - MCO output enable"]
    #[inline(always)]
    pub fn rb_mco_en(&self) -> RB_MCO_EN_R {
        RB_MCO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clock from USB_PHY PCLK(125MHz)"]
    #[inline(always)]
    pub fn rb_int_125m_en(&mut self) -> RB_INT_125M_EN_W {
        RB_INT_125M_EN_W { w: self }
    }
    #[doc = "Bit 1 - clock from pin_PA\\[16\\]"]
    #[inline(always)]
    pub fn rb_ext_125m_en(&mut self) -> RB_EXT_125M_EN_W {
        RB_EXT_125M_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - MCO output selection"]
    #[inline(always)]
    pub fn rb_mco_sel_msk(&mut self) -> RB_MCO_SEL_MSK_W {
        RB_MCO_SEL_MSK_W { w: self }
    }
    #[doc = "Bit 4 - MCO output enable"]
    #[inline(always)]
    pub fn rb_mco_en(&mut self) -> RB_MCO_EN_W {
        RB_MCO_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock mode aux register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_mod_aux](index.html) module"]
pub struct CLK_MOD_AUX_SPEC;
impl crate::RegisterSpec for CLK_MOD_AUX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clk_mod_aux::R](R) reader structure"]
impl crate::Readable for CLK_MOD_AUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_mod_aux::W](W) writer structure"]
impl crate::Writable for CLK_MOD_AUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_MOD_AUX to value 0"]
impl crate::Resettable for CLK_MOD_AUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

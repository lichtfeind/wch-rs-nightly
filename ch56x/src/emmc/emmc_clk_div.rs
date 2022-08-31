#[doc = "Register `EMMC_CLK_DIV` reader"]
pub struct R(crate::R<EMMC_CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_CLK_DIV` writer"]
pub struct W(crate::W<EMMC_CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_CLK_DIV_SPEC>;
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
impl From<crate::W<EMMC_CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_EMMC_DIV_MASK` reader - clk div"]
pub struct RB_EMMC_DIV_MASK_R(crate::FieldReader<u8, u8>);
impl RB_EMMC_DIV_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_EMMC_DIV_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_DIV_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_DIV_MASK` writer - clk div"]
pub struct RB_EMMC_DIV_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_DIV_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u16 & 0x1f);
        self.w
    }
}
#[doc = "Field `RB_EMMC_CLKOE` reader - chip output sdclk oe"]
pub struct RB_EMMC_CLKOE_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_CLKOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_CLKOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_CLKOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_CLKOE` writer - chip output sdclk oe"]
pub struct RB_EMMC_CLKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_CLKOE_W<'a> {
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
#[doc = "Field `RB_EMMC_CLKMode` reader - EMMC clock frequency mode selection bit"]
pub struct RB_EMMC_CLKMODE_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_CLKMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_CLKMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_CLKMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_CLKMode` writer - EMMC clock frequency mode selection bit"]
pub struct RB_EMMC_CLKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_CLKMODE_W<'a> {
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
#[doc = "Field `RB_EMMC_PHASEINV` reader - invert chip output sdclk phase"]
pub struct RB_EMMC_PHASEINV_R(crate::FieldReader<bool, bool>);
impl RB_EMMC_PHASEINV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_EMMC_PHASEINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_EMMC_PHASEINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_EMMC_PHASEINV` writer - invert chip output sdclk phase"]
pub struct RB_EMMC_PHASEINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_EMMC_PHASEINV_W<'a> {
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
impl R {
    #[doc = "Bits 0:4 - clk div"]
    #[inline(always)]
    pub fn rb_emmc_div_mask(&self) -> RB_EMMC_DIV_MASK_R {
        RB_EMMC_DIV_MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - chip output sdclk oe"]
    #[inline(always)]
    pub fn rb_emmc_clkoe(&self) -> RB_EMMC_CLKOE_R {
        RB_EMMC_CLKOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EMMC clock frequency mode selection bit"]
    #[inline(always)]
    pub fn rb_emmc_clkmode(&self) -> RB_EMMC_CLKMODE_R {
        RB_EMMC_CLKMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - invert chip output sdclk phase"]
    #[inline(always)]
    pub fn rb_emmc_phaseinv(&self) -> RB_EMMC_PHASEINV_R {
        RB_EMMC_PHASEINV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk div"]
    #[inline(always)]
    pub fn rb_emmc_div_mask(&mut self) -> RB_EMMC_DIV_MASK_W {
        RB_EMMC_DIV_MASK_W { w: self }
    }
    #[doc = "Bit 8 - chip output sdclk oe"]
    #[inline(always)]
    pub fn rb_emmc_clkoe(&mut self) -> RB_EMMC_CLKOE_W {
        RB_EMMC_CLKOE_W { w: self }
    }
    #[doc = "Bit 9 - EMMC clock frequency mode selection bit"]
    #[inline(always)]
    pub fn rb_emmc_clkmode(&mut self) -> RB_EMMC_CLKMODE_W {
        RB_EMMC_CLKMODE_W { w: self }
    }
    #[doc = "Bit 10 - invert chip output sdclk phase"]
    #[inline(always)]
    pub fn rb_emmc_phaseinv(&mut self) -> RB_EMMC_PHASEINV_W {
        RB_EMMC_PHASEINV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_clk_div](index.html) module"]
pub struct EMMC_CLK_DIV_SPEC;
impl crate::RegisterSpec for EMMC_CLK_DIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [emmc_clk_div::R](R) reader structure"]
impl crate::Readable for EMMC_CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_clk_div::W](W) writer structure"]
impl crate::Writable for EMMC_CLK_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_CLK_DIV to value 0x0213"]
impl crate::Resettable for EMMC_CLK_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0213
    }
}

#[doc = "Register `CLK_CFG_CTRL` reader"]
pub struct R(crate::R<CLK_CFG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CFG_CTRL` writer"]
pub struct W(crate::W<CLK_CFG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG_CTRL_SPEC>;
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
impl From<crate::W<CLK_CFG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_SLEEP` reader - PLL sleep control"]
pub struct PLL_SLEEP_R(crate::FieldReader<bool, bool>);
impl PLL_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_SLEEP` writer - PLL sleep control"]
pub struct PLL_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_SLEEP_W<'a> {
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
#[doc = "Field `SEL_PLL` reader - clock source selection"]
pub struct SEL_PLL_R(crate::FieldReader<bool, bool>);
impl SEL_PLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEL_PLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_PLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_PLL` writer - clock source selection"]
pub struct SEL_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_PLL_W<'a> {
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
#[doc = "Must be set to 0b01 or config does not take\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_A {
    #[doc = "1: Must be set to 0b01 or config does not take"]
    KEY = 1,
}
impl From<LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOCK` reader - Must be set to 0b01 or config does not take"]
pub struct LOCK_R(crate::FieldReader<u8, LOCK_A>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_A> {
        match self.bits {
            1 => Some(LOCK_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        **self == LOCK_A::KEY
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<u8, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - Must be set to 0b01 or config does not take"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Must be set to 0b01 or config does not take"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(LOCK_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u8 & 3) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PLL sleep control"]
    #[inline(always)]
    pub fn pll_sleep(&self) -> PLL_SLEEP_R {
        PLL_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock source selection"]
    #[inline(always)]
    pub fn sel_pll(&self) -> SEL_PLL_R {
        SEL_PLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Must be set to 0b01 or config does not take"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PLL sleep control"]
    #[inline(always)]
    pub fn pll_sleep(&mut self) -> PLL_SLEEP_W {
        PLL_SLEEP_W { w: self }
    }
    #[doc = "Bit 1 - clock source selection"]
    #[inline(always)]
    pub fn sel_pll(&mut self) -> SEL_PLL_W {
        SEL_PLL_W { w: self }
    }
    #[doc = "Bits 6:7 - Must be set to 0b01 or config does not take"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg_ctrl](index.html) module"]
pub struct CLK_CFG_CTRL_SPEC;
impl crate::RegisterSpec for CLK_CFG_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clk_cfg_ctrl::R](R) reader structure"]
impl crate::Readable for CLK_CFG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg_ctrl::W](W) writer structure"]
impl crate::Writable for CLK_CFG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CFG_CTRL to value 0x80"]
impl crate::Resettable for CLK_CFG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}

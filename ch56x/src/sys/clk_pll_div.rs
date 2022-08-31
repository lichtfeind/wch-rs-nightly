#[doc = "Register `CLK_PLL_DIV` reader"]
pub struct R(crate::R<CLK_PLL_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PLL_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_PLL_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_PLL_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PLL_DIV` writer"]
pub struct W(crate::W<CLK_PLL_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PLL_DIV_SPEC>;
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
impl From<crate::W<CLK_PLL_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_PLL_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - output clock divider from PLL"]
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - output clock divider from PLL"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - output clock divider from PLL"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - output clock divider from PLL"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "output clock divider from PLL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pll_div](index.html) module"]
pub struct CLK_PLL_DIV_SPEC;
impl crate::RegisterSpec for CLK_PLL_DIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clk_pll_div::R](R) reader structure"]
impl crate::Readable for CLK_PLL_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_pll_div::W](W) writer structure"]
impl crate::Writable for CLK_PLL_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PLL_DIV to value 0x42"]
impl crate::Resettable for CLK_PLL_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x42
    }
}

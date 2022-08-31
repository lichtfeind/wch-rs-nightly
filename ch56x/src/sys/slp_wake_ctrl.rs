#[doc = "Register `SLP_WAKE_CTRL` reader"]
pub struct R(crate::R<SLP_WAKE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_WAKE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_WAKE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_WAKE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_WAKE_CTRL` writer"]
pub struct W(crate::W<SLP_WAKE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_WAKE_CTRL_SPEC>;
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
impl From<crate::W<SLP_WAKE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_WAKE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SLP_USBHS_WAKE` reader - enable USBHS waking"]
pub struct RB_SLP_USBHS_WAKE_R(crate::FieldReader<bool, bool>);
impl RB_SLP_USBHS_WAKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SLP_USBHS_WAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SLP_USBHS_WAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SLP_USBHS_WAKE` writer - enable USBHS waking"]
pub struct RB_SLP_USBHS_WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SLP_USBHS_WAKE_W<'a> {
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
#[doc = "Field `RB_SLP_USBSS_WAKE` reader - enable USBSS waking"]
pub struct RB_SLP_USBSS_WAKE_R(crate::FieldReader<bool, bool>);
impl RB_SLP_USBSS_WAKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SLP_USBSS_WAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SLP_USBSS_WAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SLP_USBSS_WAKE` writer - enable USBSS waking"]
pub struct RB_SLP_USBSS_WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SLP_USBSS_WAKE_W<'a> {
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
#[doc = "Field `RB_SLP_CLK_ETH` reader - sleep ETH clock"]
pub struct RB_SLP_CLK_ETH_R(crate::FieldReader<bool, bool>);
impl RB_SLP_CLK_ETH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SLP_CLK_ETH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SLP_CLK_ETH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SLP_CLK_ETH` writer - sleep ETH clock"]
pub struct RB_SLP_CLK_ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SLP_CLK_ETH_W<'a> {
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
#[doc = "Field `RB_SLP_CLK_ECDC` reader - sleep ECDC clock"]
pub struct RB_SLP_CLK_ECDC_R(crate::FieldReader<bool, bool>);
impl RB_SLP_CLK_ECDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SLP_CLK_ECDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SLP_CLK_ECDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SLP_CLK_ECDC` writer - sleep ECDC clock"]
pub struct RB_SLP_CLK_ECDC_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SLP_CLK_ECDC_W<'a> {
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
#[doc = "Field `RB_SLP_GPIO_WAKE` reader - enable GPIO waking"]
pub struct RB_SLP_GPIO_WAKE_R(crate::FieldReader<bool, bool>);
impl RB_SLP_GPIO_WAKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SLP_GPIO_WAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SLP_GPIO_WAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SLP_GPIO_WAKE` writer - enable GPIO waking"]
pub struct RB_SLP_GPIO_WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SLP_GPIO_WAKE_W<'a> {
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
#[doc = "Field `RB_SLP_ETH_WAKE` reader - enable Eth waking"]
pub struct RB_SLP_ETH_WAKE_R(crate::FieldReader<bool, bool>);
impl RB_SLP_ETH_WAKE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RB_SLP_ETH_WAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SLP_ETH_WAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SLP_ETH_WAKE` writer - enable Eth waking"]
pub struct RB_SLP_ETH_WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SLP_ETH_WAKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u8 & 1) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - enable USBHS waking"]
    #[inline(always)]
    pub fn rb_slp_usbhs_wake(&self) -> RB_SLP_USBHS_WAKE_R {
        RB_SLP_USBHS_WAKE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable USBSS waking"]
    #[inline(always)]
    pub fn rb_slp_usbss_wake(&self) -> RB_SLP_USBSS_WAKE_R {
        RB_SLP_USBSS_WAKE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - sleep ETH clock"]
    #[inline(always)]
    pub fn rb_slp_clk_eth(&self) -> RB_SLP_CLK_ETH_R {
        RB_SLP_CLK_ETH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sleep ECDC clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ecdc(&self) -> RB_SLP_CLK_ECDC_R {
        RB_SLP_CLK_ECDC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable GPIO waking"]
    #[inline(always)]
    pub fn rb_slp_gpio_wake(&self) -> RB_SLP_GPIO_WAKE_R {
        RB_SLP_GPIO_WAKE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable Eth waking"]
    #[inline(always)]
    pub fn rb_slp_eth_wake(&self) -> RB_SLP_ETH_WAKE_R {
        RB_SLP_ETH_WAKE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable USBHS waking"]
    #[inline(always)]
    pub fn rb_slp_usbhs_wake(&mut self) -> RB_SLP_USBHS_WAKE_W {
        RB_SLP_USBHS_WAKE_W { w: self }
    }
    #[doc = "Bit 1 - enable USBSS waking"]
    #[inline(always)]
    pub fn rb_slp_usbss_wake(&mut self) -> RB_SLP_USBSS_WAKE_W {
        RB_SLP_USBSS_WAKE_W { w: self }
    }
    #[doc = "Bit 2 - sleep ETH clock"]
    #[inline(always)]
    pub fn rb_slp_clk_eth(&mut self) -> RB_SLP_CLK_ETH_W {
        RB_SLP_CLK_ETH_W { w: self }
    }
    #[doc = "Bit 3 - sleep ECDC clock"]
    #[inline(always)]
    pub fn rb_slp_clk_ecdc(&mut self) -> RB_SLP_CLK_ECDC_W {
        RB_SLP_CLK_ECDC_W { w: self }
    }
    #[doc = "Bit 4 - enable GPIO waking"]
    #[inline(always)]
    pub fn rb_slp_gpio_wake(&mut self) -> RB_SLP_GPIO_WAKE_W {
        RB_SLP_GPIO_WAKE_W { w: self }
    }
    #[doc = "Bit 5 - enable Eth waking"]
    #[inline(always)]
    pub fn rb_slp_eth_wake(&mut self) -> RB_SLP_ETH_WAKE_W {
        RB_SLP_ETH_WAKE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wake control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wake_ctrl](index.html) module"]
pub struct SLP_WAKE_CTRL_SPEC;
impl crate::RegisterSpec for SLP_WAKE_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [slp_wake_ctrl::R](R) reader structure"]
impl crate::Readable for SLP_WAKE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_wake_ctrl::W](W) writer structure"]
impl crate::Writable for SLP_WAKE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_WAKE_CTRL to value 0"]
impl crate::Resettable for SLP_WAKE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

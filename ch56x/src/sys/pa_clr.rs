#[doc = "Register `PA_CLR` writer"]
pub struct W(crate::W<PA_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_CLR_SPEC>;
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
impl From<crate::W<PA_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO PA clear output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR0_AW {
    #[doc = "1: 1: Reset pin to 0"]
    RESET = 1,
}
impl From<CLR0_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR0` writer - GPIO PA clear output"]
pub struct CLR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR0_AW::RESET)
    }
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
#[doc = "GPIO PA clear output"]
pub type CLR1_AW = CLR0_AW;
#[doc = "Field `CLR1` writer - GPIO PA clear output"]
pub struct CLR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR1_AW::RESET)
    }
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
#[doc = "GPIO PA clear output"]
pub type CLR2_AW = CLR0_AW;
#[doc = "Field `CLR2` writer - GPIO PA clear output"]
pub struct CLR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR2_AW::RESET)
    }
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
#[doc = "GPIO PA clear output"]
pub type CLR3_AW = CLR0_AW;
#[doc = "Field `CLR3` writer - GPIO PA clear output"]
pub struct CLR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR3_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR4_AW = CLR0_AW;
#[doc = "Field `CLR4` writer - GPIO PA clear output"]
pub struct CLR4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR4_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR5_AW = CLR0_AW;
#[doc = "Field `CLR5` writer - GPIO PA clear output"]
pub struct CLR5_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR5_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR6_AW = CLR0_AW;
#[doc = "Field `CLR6` writer - GPIO PA clear output"]
pub struct CLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR6_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR7_AW = CLR0_AW;
#[doc = "Field `CLR7` writer - GPIO PA clear output"]
pub struct CLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR7_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR8_AW = CLR0_AW;
#[doc = "Field `CLR8` writer - GPIO PA clear output"]
pub struct CLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR8_AW::RESET)
    }
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
#[doc = "GPIO PA clear output"]
pub type CLR9_AW = CLR0_AW;
#[doc = "Field `CLR9` writer - GPIO PA clear output"]
pub struct CLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR9_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR9_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR10_AW = CLR0_AW;
#[doc = "Field `CLR10` writer - GPIO PA clear output"]
pub struct CLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR10_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR10_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR11_AW = CLR0_AW;
#[doc = "Field `CLR11` writer - GPIO PA clear output"]
pub struct CLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR11_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR11_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR12_AW = CLR0_AW;
#[doc = "Field `CLR12` writer - GPIO PA clear output"]
pub struct CLR12_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR12_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR12_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR13_AW = CLR0_AW;
#[doc = "Field `CLR13` writer - GPIO PA clear output"]
pub struct CLR13_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR13_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR13_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR14_AW = CLR0_AW;
#[doc = "Field `CLR14` writer - GPIO PA clear output"]
pub struct CLR14_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR14_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR14_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR15_AW = CLR0_AW;
#[doc = "Field `CLR15` writer - GPIO PA clear output"]
pub struct CLR15_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR15_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR15_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR16_AW = CLR0_AW;
#[doc = "Field `CLR16` writer - GPIO PA clear output"]
pub struct CLR16_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR16_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR16_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR17_AW = CLR0_AW;
#[doc = "Field `CLR17` writer - GPIO PA clear output"]
pub struct CLR17_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR17_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR17_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR18_AW = CLR0_AW;
#[doc = "Field `CLR18` writer - GPIO PA clear output"]
pub struct CLR18_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR18_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR18_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR19_AW = CLR0_AW;
#[doc = "Field `CLR19` writer - GPIO PA clear output"]
pub struct CLR19_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR19_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR19_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR20_AW = CLR0_AW;
#[doc = "Field `CLR20` writer - GPIO PA clear output"]
pub struct CLR20_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR20_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR20_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR21_AW = CLR0_AW;
#[doc = "Field `CLR21` writer - GPIO PA clear output"]
pub struct CLR21_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR21_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR21_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR22_AW = CLR0_AW;
#[doc = "Field `CLR22` writer - GPIO PA clear output"]
pub struct CLR22_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR22_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR22_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "GPIO PA clear output"]
pub type CLR23_AW = CLR0_AW;
#[doc = "Field `CLR23` writer - GPIO PA clear output"]
pub struct CLR23_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR23_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Reset pin to 0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CLR23_AW::RESET)
    }
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr0(&mut self) -> CLR0_W {
        CLR0_W { w: self }
    }
    #[doc = "Bit 1 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr1(&mut self) -> CLR1_W {
        CLR1_W { w: self }
    }
    #[doc = "Bit 2 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr2(&mut self) -> CLR2_W {
        CLR2_W { w: self }
    }
    #[doc = "Bit 3 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr3(&mut self) -> CLR3_W {
        CLR3_W { w: self }
    }
    #[doc = "Bit 4 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr4(&mut self) -> CLR4_W {
        CLR4_W { w: self }
    }
    #[doc = "Bit 5 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr5(&mut self) -> CLR5_W {
        CLR5_W { w: self }
    }
    #[doc = "Bit 6 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr6(&mut self) -> CLR6_W {
        CLR6_W { w: self }
    }
    #[doc = "Bit 7 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr7(&mut self) -> CLR7_W {
        CLR7_W { w: self }
    }
    #[doc = "Bit 8 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr8(&mut self) -> CLR8_W {
        CLR8_W { w: self }
    }
    #[doc = "Bit 9 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr9(&mut self) -> CLR9_W {
        CLR9_W { w: self }
    }
    #[doc = "Bit 10 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr10(&mut self) -> CLR10_W {
        CLR10_W { w: self }
    }
    #[doc = "Bit 11 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr11(&mut self) -> CLR11_W {
        CLR11_W { w: self }
    }
    #[doc = "Bit 12 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr12(&mut self) -> CLR12_W {
        CLR12_W { w: self }
    }
    #[doc = "Bit 13 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr13(&mut self) -> CLR13_W {
        CLR13_W { w: self }
    }
    #[doc = "Bit 14 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr14(&mut self) -> CLR14_W {
        CLR14_W { w: self }
    }
    #[doc = "Bit 15 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr15(&mut self) -> CLR15_W {
        CLR15_W { w: self }
    }
    #[doc = "Bit 16 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr16(&mut self) -> CLR16_W {
        CLR16_W { w: self }
    }
    #[doc = "Bit 17 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr17(&mut self) -> CLR17_W {
        CLR17_W { w: self }
    }
    #[doc = "Bit 18 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr18(&mut self) -> CLR18_W {
        CLR18_W { w: self }
    }
    #[doc = "Bit 19 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr19(&mut self) -> CLR19_W {
        CLR19_W { w: self }
    }
    #[doc = "Bit 20 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr20(&mut self) -> CLR20_W {
        CLR20_W { w: self }
    }
    #[doc = "Bit 21 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr21(&mut self) -> CLR21_W {
        CLR21_W { w: self }
    }
    #[doc = "Bit 22 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr22(&mut self) -> CLR22_W {
        CLR22_W { w: self }
    }
    #[doc = "Bit 23 - GPIO PA clear output"]
    #[inline(always)]
    pub fn clr23(&mut self) -> CLR23_W {
        CLR23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO PA clear output\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_clr](index.html) module"]
pub struct PA_CLR_SPEC;
impl crate::RegisterSpec for PA_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pa_clr::W](W) writer structure"]
impl crate::Writable for PA_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PA_CLR to value 0"]
impl crate::Resettable for PA_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

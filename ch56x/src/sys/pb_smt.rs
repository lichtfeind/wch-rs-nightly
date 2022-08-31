#[doc = "Register `PB_SMT` reader"]
pub struct R(crate::R<PB_SMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_SMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_SMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_SMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB_SMT` writer"]
pub struct W(crate::W<PB_SMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_SMT_SPEC>;
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
impl From<crate::W<PB_SMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_SMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO PB output slew rate and input schmitt trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMT0_A {
    #[doc = "0: 0: Low slope output disabled / Schmitt trigger input disabled"]
    NORMAL_FAST = 0,
    #[doc = "1: 1: Low slope output enabled / Schmitt trigger input enabled"]
    SCHMITT_SLOW = 1,
}
impl From<SMT0_A> for bool {
    #[inline(always)]
    fn from(variant: SMT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMT0` reader - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT0_R(crate::FieldReader<bool, SMT0_A>);
impl SMT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMT0_A {
        match self.bits {
            false => SMT0_A::NORMAL_FAST,
            true => SMT0_A::SCHMITT_SLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_FAST`"]
    #[inline(always)]
    pub fn is_normal_fast(&self) -> bool {
        **self == SMT0_A::NORMAL_FAST
    }
    #[doc = "Checks if the value of the field is `SCHMITT_SLOW`"]
    #[inline(always)]
    pub fn is_schmitt_slow(&self) -> bool {
        **self == SMT0_A::SCHMITT_SLOW
    }
}
impl core::ops::Deref for SMT0_R {
    type Target = crate::FieldReader<bool, SMT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMT0` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT0_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT0_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT1_A = SMT0_A;
#[doc = "Field `SMT1` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT1_R = SMT0_R;
#[doc = "Field `SMT1` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT1_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT1_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT2_A = SMT0_A;
#[doc = "Field `SMT2` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT2_R = SMT0_R;
#[doc = "Field `SMT2` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT2_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT2_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT3_A = SMT0_A;
#[doc = "Field `SMT3` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT3_R = SMT0_R;
#[doc = "Field `SMT3` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT3_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT3_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT4_A = SMT0_A;
#[doc = "Field `SMT4` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT4_R = SMT0_R;
#[doc = "Field `SMT4` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT4_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT4_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT4_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT5_A = SMT0_A;
#[doc = "Field `SMT5` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT5_R = SMT0_R;
#[doc = "Field `SMT5` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT5_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT5_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT5_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT6_A = SMT0_A;
#[doc = "Field `SMT6` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT6_R = SMT0_R;
#[doc = "Field `SMT6` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT6_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT6_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT6_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT7_A = SMT0_A;
#[doc = "Field `SMT7` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT7_R = SMT0_R;
#[doc = "Field `SMT7` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT7_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT7_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT7_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT8_A = SMT0_A;
#[doc = "Field `SMT8` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT8_R = SMT0_R;
#[doc = "Field `SMT8` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT8_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT8_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT8_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT9_A = SMT0_A;
#[doc = "Field `SMT9` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT9_R = SMT0_R;
#[doc = "Field `SMT9` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT9_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT9_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT9_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT10_A = SMT0_A;
#[doc = "Field `SMT10` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT10_R = SMT0_R;
#[doc = "Field `SMT10` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT10_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT10_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT11_A = SMT0_A;
#[doc = "Field `SMT11` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT11_R = SMT0_R;
#[doc = "Field `SMT11` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT11_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT11_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT12_A = SMT0_A;
#[doc = "Field `SMT12` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT12_R = SMT0_R;
#[doc = "Field `SMT12` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT12_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT12_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT13_A = SMT0_A;
#[doc = "Field `SMT13` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT13_R = SMT0_R;
#[doc = "Field `SMT13` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT13_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT13_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT14_A = SMT0_A;
#[doc = "Field `SMT14` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT14_R = SMT0_R;
#[doc = "Field `SMT14` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT14_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT14_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT15_A = SMT0_A;
#[doc = "Field `SMT15` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT15_R = SMT0_R;
#[doc = "Field `SMT15` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT15_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT15_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT15_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT16_A = SMT0_A;
#[doc = "Field `SMT16` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT16_R = SMT0_R;
#[doc = "Field `SMT16` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT16_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT16_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT16_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT17_A = SMT0_A;
#[doc = "Field `SMT17` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT17_R = SMT0_R;
#[doc = "Field `SMT17` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT17_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT17_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT17_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT18_A = SMT0_A;
#[doc = "Field `SMT18` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT18_R = SMT0_R;
#[doc = "Field `SMT18` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT18_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT18_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT18_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT19_A = SMT0_A;
#[doc = "Field `SMT19` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT19_R = SMT0_R;
#[doc = "Field `SMT19` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT19_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT19_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT19_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT20_A = SMT0_A;
#[doc = "Field `SMT20` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT20_R = SMT0_R;
#[doc = "Field `SMT20` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT20_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT20_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT20_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT21_A = SMT0_A;
#[doc = "Field `SMT21` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT21_R = SMT0_R;
#[doc = "Field `SMT21` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT21_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT21_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT21_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT22_A = SMT0_A;
#[doc = "Field `SMT22` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT22_R = SMT0_R;
#[doc = "Field `SMT22` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT22_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT22_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT22_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT23_A = SMT0_A;
#[doc = "Field `SMT23` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT23_R = SMT0_R;
#[doc = "Field `SMT23` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT23_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT23_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT23_A::SCHMITT_SLOW)
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
#[doc = "GPIO PB output slew rate and input schmitt trigger"]
pub type SMT24_A = SMT0_A;
#[doc = "Field `SMT24` reader - GPIO PB output slew rate and input schmitt trigger"]
pub type SMT24_R = SMT0_R;
#[doc = "Field `SMT24` writer - GPIO PB output slew rate and input schmitt trigger"]
pub struct SMT24_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Low slope output disabled / Schmitt trigger input disabled"]
    #[inline(always)]
    pub fn normal_fast(self) -> &'a mut W {
        self.variant(SMT24_A::NORMAL_FAST)
    }
    #[doc = "1: Low slope output enabled / Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn schmitt_slow(self) -> &'a mut W {
        self.variant(SMT24_A::SCHMITT_SLOW)
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt0(&self) -> SMT0_R {
        SMT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt1(&self) -> SMT1_R {
        SMT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt2(&self) -> SMT2_R {
        SMT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt3(&self) -> SMT3_R {
        SMT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt4(&self) -> SMT4_R {
        SMT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt5(&self) -> SMT5_R {
        SMT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt6(&self) -> SMT6_R {
        SMT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt7(&self) -> SMT7_R {
        SMT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt8(&self) -> SMT8_R {
        SMT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt9(&self) -> SMT9_R {
        SMT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt10(&self) -> SMT10_R {
        SMT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt11(&self) -> SMT11_R {
        SMT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt12(&self) -> SMT12_R {
        SMT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt13(&self) -> SMT13_R {
        SMT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt14(&self) -> SMT14_R {
        SMT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt15(&self) -> SMT15_R {
        SMT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt16(&self) -> SMT16_R {
        SMT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt17(&self) -> SMT17_R {
        SMT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt18(&self) -> SMT18_R {
        SMT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt19(&self) -> SMT19_R {
        SMT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt20(&self) -> SMT20_R {
        SMT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt21(&self) -> SMT21_R {
        SMT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt22(&self) -> SMT22_R {
        SMT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt23(&self) -> SMT23_R {
        SMT23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt24(&self) -> SMT24_R {
        SMT24_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt0(&mut self) -> SMT0_W {
        SMT0_W { w: self }
    }
    #[doc = "Bit 1 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt1(&mut self) -> SMT1_W {
        SMT1_W { w: self }
    }
    #[doc = "Bit 2 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt2(&mut self) -> SMT2_W {
        SMT2_W { w: self }
    }
    #[doc = "Bit 3 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt3(&mut self) -> SMT3_W {
        SMT3_W { w: self }
    }
    #[doc = "Bit 4 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt4(&mut self) -> SMT4_W {
        SMT4_W { w: self }
    }
    #[doc = "Bit 5 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt5(&mut self) -> SMT5_W {
        SMT5_W { w: self }
    }
    #[doc = "Bit 6 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt6(&mut self) -> SMT6_W {
        SMT6_W { w: self }
    }
    #[doc = "Bit 7 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt7(&mut self) -> SMT7_W {
        SMT7_W { w: self }
    }
    #[doc = "Bit 8 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt8(&mut self) -> SMT8_W {
        SMT8_W { w: self }
    }
    #[doc = "Bit 9 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt9(&mut self) -> SMT9_W {
        SMT9_W { w: self }
    }
    #[doc = "Bit 10 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt10(&mut self) -> SMT10_W {
        SMT10_W { w: self }
    }
    #[doc = "Bit 11 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt11(&mut self) -> SMT11_W {
        SMT11_W { w: self }
    }
    #[doc = "Bit 12 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt12(&mut self) -> SMT12_W {
        SMT12_W { w: self }
    }
    #[doc = "Bit 13 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt13(&mut self) -> SMT13_W {
        SMT13_W { w: self }
    }
    #[doc = "Bit 14 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt14(&mut self) -> SMT14_W {
        SMT14_W { w: self }
    }
    #[doc = "Bit 15 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt15(&mut self) -> SMT15_W {
        SMT15_W { w: self }
    }
    #[doc = "Bit 16 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt16(&mut self) -> SMT16_W {
        SMT16_W { w: self }
    }
    #[doc = "Bit 17 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt17(&mut self) -> SMT17_W {
        SMT17_W { w: self }
    }
    #[doc = "Bit 18 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt18(&mut self) -> SMT18_W {
        SMT18_W { w: self }
    }
    #[doc = "Bit 19 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt19(&mut self) -> SMT19_W {
        SMT19_W { w: self }
    }
    #[doc = "Bit 20 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt20(&mut self) -> SMT20_W {
        SMT20_W { w: self }
    }
    #[doc = "Bit 21 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt21(&mut self) -> SMT21_W {
        SMT21_W { w: self }
    }
    #[doc = "Bit 22 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt22(&mut self) -> SMT22_W {
        SMT22_W { w: self }
    }
    #[doc = "Bit 23 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt23(&mut self) -> SMT23_W {
        SMT23_W { w: self }
    }
    #[doc = "Bit 24 - GPIO PB output slew rate and input schmitt trigger"]
    #[inline(always)]
    pub fn smt24(&mut self) -> SMT24_W {
        SMT24_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO PB output slew rate and input schmitt trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_smt](index.html) module"]
pub struct PB_SMT_SPEC;
impl crate::RegisterSpec for PB_SMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_smt::R](R) reader structure"]
impl crate::Readable for PB_SMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_smt::W](W) writer structure"]
impl crate::Writable for PB_SMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PB_SMT to value 0"]
impl crate::Resettable for PB_SMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

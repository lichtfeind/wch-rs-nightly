#[doc = "Register `PA_OUT` reader"]
pub struct R(crate::R<PA_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PA_OUT` writer"]
pub struct W(crate::W<PA_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_OUT_SPEC>;
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
impl From<crate::W<PA_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO PA output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT0_A {
    #[doc = "1: 1: Set pin to 1"]
    SET = 1,
    #[doc = "0: 0: Set pin to 0"]
    CLEAR = 0,
}
impl From<OUT0_A> for bool {
    #[inline(always)]
    fn from(variant: OUT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT0` reader - GPIO PA output"]
pub struct OUT0_R(crate::FieldReader<bool, OUT0_A>);
impl OUT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT0_A {
        match self.bits {
            true => OUT0_A::SET,
            false => OUT0_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == OUT0_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == OUT0_A::CLEAR
    }
}
impl core::ops::Deref for OUT0_R {
    type Target = crate::FieldReader<bool, OUT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT0` writer - GPIO PA output"]
pub struct OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT0_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT0_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT1_A = OUT0_A;
#[doc = "Field `OUT1` reader - GPIO PA output"]
pub type OUT1_R = OUT0_R;
#[doc = "Field `OUT1` writer - GPIO PA output"]
pub struct OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT1_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT1_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT2_A = OUT0_A;
#[doc = "Field `OUT2` reader - GPIO PA output"]
pub type OUT2_R = OUT0_R;
#[doc = "Field `OUT2` writer - GPIO PA output"]
pub struct OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT2_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT2_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT3_A = OUT0_A;
#[doc = "Field `OUT3` reader - GPIO PA output"]
pub type OUT3_R = OUT0_R;
#[doc = "Field `OUT3` writer - GPIO PA output"]
pub struct OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT3_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT3_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT4_A = OUT0_A;
#[doc = "Field `OUT4` reader - GPIO PA output"]
pub type OUT4_R = OUT0_R;
#[doc = "Field `OUT4` writer - GPIO PA output"]
pub struct OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT4_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT4_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT5_A = OUT0_A;
#[doc = "Field `OUT5` reader - GPIO PA output"]
pub type OUT5_R = OUT0_R;
#[doc = "Field `OUT5` writer - GPIO PA output"]
pub struct OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT5_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT5_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT6_A = OUT0_A;
#[doc = "Field `OUT6` reader - GPIO PA output"]
pub type OUT6_R = OUT0_R;
#[doc = "Field `OUT6` writer - GPIO PA output"]
pub struct OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT6_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT6_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT7_A = OUT0_A;
#[doc = "Field `OUT7` reader - GPIO PA output"]
pub type OUT7_R = OUT0_R;
#[doc = "Field `OUT7` writer - GPIO PA output"]
pub struct OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT7_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT7_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT8_A = OUT0_A;
#[doc = "Field `OUT8` reader - GPIO PA output"]
pub type OUT8_R = OUT0_R;
#[doc = "Field `OUT8` writer - GPIO PA output"]
pub struct OUT8_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT8_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT8_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT9_A = OUT0_A;
#[doc = "Field `OUT9` reader - GPIO PA output"]
pub type OUT9_R = OUT0_R;
#[doc = "Field `OUT9` writer - GPIO PA output"]
pub struct OUT9_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT9_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT9_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT10_A = OUT0_A;
#[doc = "Field `OUT10` reader - GPIO PA output"]
pub type OUT10_R = OUT0_R;
#[doc = "Field `OUT10` writer - GPIO PA output"]
pub struct OUT10_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT10_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT10_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT11_A = OUT0_A;
#[doc = "Field `OUT11` reader - GPIO PA output"]
pub type OUT11_R = OUT0_R;
#[doc = "Field `OUT11` writer - GPIO PA output"]
pub struct OUT11_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT11_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT11_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT12_A = OUT0_A;
#[doc = "Field `OUT12` reader - GPIO PA output"]
pub type OUT12_R = OUT0_R;
#[doc = "Field `OUT12` writer - GPIO PA output"]
pub struct OUT12_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT12_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT12_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT13_A = OUT0_A;
#[doc = "Field `OUT13` reader - GPIO PA output"]
pub type OUT13_R = OUT0_R;
#[doc = "Field `OUT13` writer - GPIO PA output"]
pub struct OUT13_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT13_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT13_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT14_A = OUT0_A;
#[doc = "Field `OUT14` reader - GPIO PA output"]
pub type OUT14_R = OUT0_R;
#[doc = "Field `OUT14` writer - GPIO PA output"]
pub struct OUT14_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT14_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT14_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT15_A = OUT0_A;
#[doc = "Field `OUT15` reader - GPIO PA output"]
pub type OUT15_R = OUT0_R;
#[doc = "Field `OUT15` writer - GPIO PA output"]
pub struct OUT15_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT15_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT15_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT16_A = OUT0_A;
#[doc = "Field `OUT16` reader - GPIO PA output"]
pub type OUT16_R = OUT0_R;
#[doc = "Field `OUT16` writer - GPIO PA output"]
pub struct OUT16_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT16_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT16_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT17_A = OUT0_A;
#[doc = "Field `OUT17` reader - GPIO PA output"]
pub type OUT17_R = OUT0_R;
#[doc = "Field `OUT17` writer - GPIO PA output"]
pub struct OUT17_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT17_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT17_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT18_A = OUT0_A;
#[doc = "Field `OUT18` reader - GPIO PA output"]
pub type OUT18_R = OUT0_R;
#[doc = "Field `OUT18` writer - GPIO PA output"]
pub struct OUT18_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT18_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT18_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT19_A = OUT0_A;
#[doc = "Field `OUT19` reader - GPIO PA output"]
pub type OUT19_R = OUT0_R;
#[doc = "Field `OUT19` writer - GPIO PA output"]
pub struct OUT19_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT19_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT19_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT20_A = OUT0_A;
#[doc = "Field `OUT20` reader - GPIO PA output"]
pub type OUT20_R = OUT0_R;
#[doc = "Field `OUT20` writer - GPIO PA output"]
pub struct OUT20_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT20_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT20_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT21_A = OUT0_A;
#[doc = "Field `OUT21` reader - GPIO PA output"]
pub type OUT21_R = OUT0_R;
#[doc = "Field `OUT21` writer - GPIO PA output"]
pub struct OUT21_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT21_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT21_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT22_A = OUT0_A;
#[doc = "Field `OUT22` reader - GPIO PA output"]
pub type OUT22_R = OUT0_R;
#[doc = "Field `OUT22` writer - GPIO PA output"]
pub struct OUT22_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT22_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT22_A::CLEAR)
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
#[doc = "GPIO PA output"]
pub type OUT23_A = OUT0_A;
#[doc = "Field `OUT23` reader - GPIO PA output"]
pub type OUT23_R = OUT0_R;
#[doc = "Field `OUT23` writer - GPIO PA output"]
pub struct OUT23_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1: Set pin to 1"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OUT23_A::SET)
    }
    #[doc = "0: Set pin to 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OUT23_A::CLEAR)
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
impl R {
    #[doc = "Bit 0 - GPIO PA output"]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO PA output"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO PA output"]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO PA output"]
    #[inline(always)]
    pub fn out3(&self) -> OUT3_R {
        OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO PA output"]
    #[inline(always)]
    pub fn out4(&self) -> OUT4_R {
        OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO PA output"]
    #[inline(always)]
    pub fn out5(&self) -> OUT5_R {
        OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO PA output"]
    #[inline(always)]
    pub fn out6(&self) -> OUT6_R {
        OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO PA output"]
    #[inline(always)]
    pub fn out7(&self) -> OUT7_R {
        OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO PA output"]
    #[inline(always)]
    pub fn out8(&self) -> OUT8_R {
        OUT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO PA output"]
    #[inline(always)]
    pub fn out9(&self) -> OUT9_R {
        OUT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO PA output"]
    #[inline(always)]
    pub fn out10(&self) -> OUT10_R {
        OUT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO PA output"]
    #[inline(always)]
    pub fn out11(&self) -> OUT11_R {
        OUT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO PA output"]
    #[inline(always)]
    pub fn out12(&self) -> OUT12_R {
        OUT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO PA output"]
    #[inline(always)]
    pub fn out13(&self) -> OUT13_R {
        OUT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO PA output"]
    #[inline(always)]
    pub fn out14(&self) -> OUT14_R {
        OUT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO PA output"]
    #[inline(always)]
    pub fn out15(&self) -> OUT15_R {
        OUT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO PA output"]
    #[inline(always)]
    pub fn out16(&self) -> OUT16_R {
        OUT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO PA output"]
    #[inline(always)]
    pub fn out17(&self) -> OUT17_R {
        OUT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO PA output"]
    #[inline(always)]
    pub fn out18(&self) -> OUT18_R {
        OUT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO PA output"]
    #[inline(always)]
    pub fn out19(&self) -> OUT19_R {
        OUT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO PA output"]
    #[inline(always)]
    pub fn out20(&self) -> OUT20_R {
        OUT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO PA output"]
    #[inline(always)]
    pub fn out21(&self) -> OUT21_R {
        OUT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO PA output"]
    #[inline(always)]
    pub fn out22(&self) -> OUT22_R {
        OUT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO PA output"]
    #[inline(always)]
    pub fn out23(&self) -> OUT23_R {
        OUT23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO PA output"]
    #[inline(always)]
    pub fn out0(&mut self) -> OUT0_W {
        OUT0_W { w: self }
    }
    #[doc = "Bit 1 - GPIO PA output"]
    #[inline(always)]
    pub fn out1(&mut self) -> OUT1_W {
        OUT1_W { w: self }
    }
    #[doc = "Bit 2 - GPIO PA output"]
    #[inline(always)]
    pub fn out2(&mut self) -> OUT2_W {
        OUT2_W { w: self }
    }
    #[doc = "Bit 3 - GPIO PA output"]
    #[inline(always)]
    pub fn out3(&mut self) -> OUT3_W {
        OUT3_W { w: self }
    }
    #[doc = "Bit 4 - GPIO PA output"]
    #[inline(always)]
    pub fn out4(&mut self) -> OUT4_W {
        OUT4_W { w: self }
    }
    #[doc = "Bit 5 - GPIO PA output"]
    #[inline(always)]
    pub fn out5(&mut self) -> OUT5_W {
        OUT5_W { w: self }
    }
    #[doc = "Bit 6 - GPIO PA output"]
    #[inline(always)]
    pub fn out6(&mut self) -> OUT6_W {
        OUT6_W { w: self }
    }
    #[doc = "Bit 7 - GPIO PA output"]
    #[inline(always)]
    pub fn out7(&mut self) -> OUT7_W {
        OUT7_W { w: self }
    }
    #[doc = "Bit 8 - GPIO PA output"]
    #[inline(always)]
    pub fn out8(&mut self) -> OUT8_W {
        OUT8_W { w: self }
    }
    #[doc = "Bit 9 - GPIO PA output"]
    #[inline(always)]
    pub fn out9(&mut self) -> OUT9_W {
        OUT9_W { w: self }
    }
    #[doc = "Bit 10 - GPIO PA output"]
    #[inline(always)]
    pub fn out10(&mut self) -> OUT10_W {
        OUT10_W { w: self }
    }
    #[doc = "Bit 11 - GPIO PA output"]
    #[inline(always)]
    pub fn out11(&mut self) -> OUT11_W {
        OUT11_W { w: self }
    }
    #[doc = "Bit 12 - GPIO PA output"]
    #[inline(always)]
    pub fn out12(&mut self) -> OUT12_W {
        OUT12_W { w: self }
    }
    #[doc = "Bit 13 - GPIO PA output"]
    #[inline(always)]
    pub fn out13(&mut self) -> OUT13_W {
        OUT13_W { w: self }
    }
    #[doc = "Bit 14 - GPIO PA output"]
    #[inline(always)]
    pub fn out14(&mut self) -> OUT14_W {
        OUT14_W { w: self }
    }
    #[doc = "Bit 15 - GPIO PA output"]
    #[inline(always)]
    pub fn out15(&mut self) -> OUT15_W {
        OUT15_W { w: self }
    }
    #[doc = "Bit 16 - GPIO PA output"]
    #[inline(always)]
    pub fn out16(&mut self) -> OUT16_W {
        OUT16_W { w: self }
    }
    #[doc = "Bit 17 - GPIO PA output"]
    #[inline(always)]
    pub fn out17(&mut self) -> OUT17_W {
        OUT17_W { w: self }
    }
    #[doc = "Bit 18 - GPIO PA output"]
    #[inline(always)]
    pub fn out18(&mut self) -> OUT18_W {
        OUT18_W { w: self }
    }
    #[doc = "Bit 19 - GPIO PA output"]
    #[inline(always)]
    pub fn out19(&mut self) -> OUT19_W {
        OUT19_W { w: self }
    }
    #[doc = "Bit 20 - GPIO PA output"]
    #[inline(always)]
    pub fn out20(&mut self) -> OUT20_W {
        OUT20_W { w: self }
    }
    #[doc = "Bit 21 - GPIO PA output"]
    #[inline(always)]
    pub fn out21(&mut self) -> OUT21_W {
        OUT21_W { w: self }
    }
    #[doc = "Bit 22 - GPIO PA output"]
    #[inline(always)]
    pub fn out22(&mut self) -> OUT22_W {
        OUT22_W { w: self }
    }
    #[doc = "Bit 23 - GPIO PA output"]
    #[inline(always)]
    pub fn out23(&mut self) -> OUT23_W {
        OUT23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO PA output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_out](index.html) module"]
pub struct PA_OUT_SPEC;
impl crate::RegisterSpec for PA_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_out::R](R) reader structure"]
impl crate::Readable for PA_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_out::W](W) writer structure"]
impl crate::Writable for PA_OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PA_OUT to value 0"]
impl crate::Resettable for PA_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

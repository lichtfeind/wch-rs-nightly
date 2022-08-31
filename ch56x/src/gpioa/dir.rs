#[doc = "Register `DIR` reader"]
pub struct R(crate::R<DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIR` writer"]
pub struct W(crate::W<DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIR_SPEC>;
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
impl From<crate::W<DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set GPIO direction for GPIO 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR0_A {
    #[doc = "0: 0: Input"]
    INPUT = 0,
    #[doc = "1: 1: Output"]
    OUTPUT = 1,
}
impl From<DIR0_A> for bool {
    #[inline(always)]
    fn from(variant: DIR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR0` reader - Set GPIO direction for GPIO 0"]
pub struct DIR0_R(crate::FieldReader<bool, DIR0_A>);
impl DIR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR0_A {
        match self.bits {
            false => DIR0_A::INPUT,
            true => DIR0_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == DIR0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == DIR0_A::OUTPUT
    }
}
impl core::ops::Deref for DIR0_R {
    type Target = crate::FieldReader<bool, DIR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR0` writer - Set GPIO direction for GPIO 0"]
pub struct DIR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR0_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR0_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !1) | (value as u64 & 1);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 1"]
pub type DIR1_A = DIR0_A;
#[doc = "Field `DIR1` reader - Set GPIO direction for GPIO 1"]
pub type DIR1_R = DIR0_R;
#[doc = "Field `DIR1` writer - Set GPIO direction for GPIO 1"]
pub struct DIR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR1_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR1_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u64 & 1) << 1);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 2"]
pub type DIR2_A = DIR0_A;
#[doc = "Field `DIR2` reader - Set GPIO direction for GPIO 2"]
pub type DIR2_R = DIR0_R;
#[doc = "Field `DIR2` writer - Set GPIO direction for GPIO 2"]
pub struct DIR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR2_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR2_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u64 & 1) << 2);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 3"]
pub type DIR3_A = DIR0_A;
#[doc = "Field `DIR3` reader - Set GPIO direction for GPIO 3"]
pub type DIR3_R = DIR0_R;
#[doc = "Field `DIR3` writer - Set GPIO direction for GPIO 3"]
pub struct DIR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR3_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR3_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u64 & 1) << 3);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 4"]
pub type DIR4_A = DIR0_A;
#[doc = "Field `DIR4` reader - Set GPIO direction for GPIO 4"]
pub type DIR4_R = DIR0_R;
#[doc = "Field `DIR4` writer - Set GPIO direction for GPIO 4"]
pub struct DIR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR4_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR4_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u64 & 1) << 4);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 5"]
pub type DIR5_A = DIR0_A;
#[doc = "Field `DIR5` reader - Set GPIO direction for GPIO 5"]
pub type DIR5_R = DIR0_R;
#[doc = "Field `DIR5` writer - Set GPIO direction for GPIO 5"]
pub struct DIR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR5_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR5_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u64 & 1) << 5);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 6"]
pub type DIR6_A = DIR0_A;
#[doc = "Field `DIR6` reader - Set GPIO direction for GPIO 6"]
pub type DIR6_R = DIR0_R;
#[doc = "Field `DIR6` writer - Set GPIO direction for GPIO 6"]
pub struct DIR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR6_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR6_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u64 & 1) << 6);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 7"]
pub type DIR7_A = DIR0_A;
#[doc = "Field `DIR7` reader - Set GPIO direction for GPIO 7"]
pub type DIR7_R = DIR0_R;
#[doc = "Field `DIR7` writer - Set GPIO direction for GPIO 7"]
pub struct DIR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR7_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR7_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u64 & 1) << 7);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 8"]
pub type DIR8_A = DIR0_A;
#[doc = "Field `DIR8` reader - Set GPIO direction for GPIO 8"]
pub type DIR8_R = DIR0_R;
#[doc = "Field `DIR8` writer - Set GPIO direction for GPIO 8"]
pub struct DIR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR8_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR8_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u64 & 1) << 8);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 9"]
pub type DIR9_A = DIR0_A;
#[doc = "Field `DIR9` reader - Set GPIO direction for GPIO 9"]
pub type DIR9_R = DIR0_R;
#[doc = "Field `DIR9` writer - Set GPIO direction for GPIO 9"]
pub struct DIR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR9_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR9_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u64 & 1) << 9);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 10"]
pub type DIR10_A = DIR0_A;
#[doc = "Field `DIR10` reader - Set GPIO direction for GPIO 10"]
pub type DIR10_R = DIR0_R;
#[doc = "Field `DIR10` writer - Set GPIO direction for GPIO 10"]
pub struct DIR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR10_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR10_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u64 & 1) << 10);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 11"]
pub type DIR11_A = DIR0_A;
#[doc = "Field `DIR11` reader - Set GPIO direction for GPIO 11"]
pub type DIR11_R = DIR0_R;
#[doc = "Field `DIR11` writer - Set GPIO direction for GPIO 11"]
pub struct DIR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR11_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR11_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u64 & 1) << 11);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 12"]
pub type DIR12_A = DIR0_A;
#[doc = "Field `DIR12` reader - Set GPIO direction for GPIO 12"]
pub type DIR12_R = DIR0_R;
#[doc = "Field `DIR12` writer - Set GPIO direction for GPIO 12"]
pub struct DIR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR12_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR12_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u64 & 1) << 12);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 13"]
pub type DIR13_A = DIR0_A;
#[doc = "Field `DIR13` reader - Set GPIO direction for GPIO 13"]
pub type DIR13_R = DIR0_R;
#[doc = "Field `DIR13` writer - Set GPIO direction for GPIO 13"]
pub struct DIR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR13_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR13_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u64 & 1) << 13);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 14"]
pub type DIR14_A = DIR0_A;
#[doc = "Field `DIR14` reader - Set GPIO direction for GPIO 14"]
pub type DIR14_R = DIR0_R;
#[doc = "Field `DIR14` writer - Set GPIO direction for GPIO 14"]
pub struct DIR14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR14_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR14_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u64 & 1) << 14);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 15"]
pub type DIR15_A = DIR0_A;
#[doc = "Field `DIR15` reader - Set GPIO direction for GPIO 15"]
pub type DIR15_R = DIR0_R;
#[doc = "Field `DIR15` writer - Set GPIO direction for GPIO 15"]
pub struct DIR15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR15_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR15_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u64 & 1) << 15);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 16"]
pub type DIR16_A = DIR0_A;
#[doc = "Field `DIR16` reader - Set GPIO direction for GPIO 16"]
pub type DIR16_R = DIR0_R;
#[doc = "Field `DIR16` writer - Set GPIO direction for GPIO 16"]
pub struct DIR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR16_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR16_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u64 & 1) << 16);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 17"]
pub type DIR17_A = DIR0_A;
#[doc = "Field `DIR17` reader - Set GPIO direction for GPIO 17"]
pub type DIR17_R = DIR0_R;
#[doc = "Field `DIR17` writer - Set GPIO direction for GPIO 17"]
pub struct DIR17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR17_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR17_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u64 & 1) << 17);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 18"]
pub type DIR18_A = DIR0_A;
#[doc = "Field `DIR18` reader - Set GPIO direction for GPIO 18"]
pub type DIR18_R = DIR0_R;
#[doc = "Field `DIR18` writer - Set GPIO direction for GPIO 18"]
pub struct DIR18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR18_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR18_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u64 & 1) << 18);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 19"]
pub type DIR19_A = DIR0_A;
#[doc = "Field `DIR19` reader - Set GPIO direction for GPIO 19"]
pub type DIR19_R = DIR0_R;
#[doc = "Field `DIR19` writer - Set GPIO direction for GPIO 19"]
pub struct DIR19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR19_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR19_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u64 & 1) << 19);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 20"]
pub type DIR20_A = DIR0_A;
#[doc = "Field `DIR20` reader - Set GPIO direction for GPIO 20"]
pub type DIR20_R = DIR0_R;
#[doc = "Field `DIR20` writer - Set GPIO direction for GPIO 20"]
pub struct DIR20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR20_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR20_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u64 & 1) << 20);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 21"]
pub type DIR21_A = DIR0_A;
#[doc = "Field `DIR21` reader - Set GPIO direction for GPIO 21"]
pub type DIR21_R = DIR0_R;
#[doc = "Field `DIR21` writer - Set GPIO direction for GPIO 21"]
pub struct DIR21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR21_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR21_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u64 & 1) << 21);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 22"]
pub type DIR22_A = DIR0_A;
#[doc = "Field `DIR22` reader - Set GPIO direction for GPIO 22"]
pub type DIR22_R = DIR0_R;
#[doc = "Field `DIR22` writer - Set GPIO direction for GPIO 22"]
pub struct DIR22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR22_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR22_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u64 & 1) << 22);
        self.w
    }
}
#[doc = "Set GPIO direction for GPIO 23"]
pub type DIR23_A = DIR0_A;
#[doc = "Field `DIR23` reader - Set GPIO direction for GPIO 23"]
pub type DIR23_R = DIR0_R;
#[doc = "Field `DIR23` writer - Set GPIO direction for GPIO 23"]
pub struct DIR23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR23_A::INPUT)
    }
    #[doc = "1: Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR23_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u64 & 1) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set GPIO direction for GPIO 0"]
    #[inline(always)]
    pub fn dir0(&self) -> DIR0_R {
        DIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set GPIO direction for GPIO 1"]
    #[inline(always)]
    pub fn dir1(&self) -> DIR1_R {
        DIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set GPIO direction for GPIO 2"]
    #[inline(always)]
    pub fn dir2(&self) -> DIR2_R {
        DIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set GPIO direction for GPIO 3"]
    #[inline(always)]
    pub fn dir3(&self) -> DIR3_R {
        DIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set GPIO direction for GPIO 4"]
    #[inline(always)]
    pub fn dir4(&self) -> DIR4_R {
        DIR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set GPIO direction for GPIO 5"]
    #[inline(always)]
    pub fn dir5(&self) -> DIR5_R {
        DIR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set GPIO direction for GPIO 6"]
    #[inline(always)]
    pub fn dir6(&self) -> DIR6_R {
        DIR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set GPIO direction for GPIO 7"]
    #[inline(always)]
    pub fn dir7(&self) -> DIR7_R {
        DIR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set GPIO direction for GPIO 8"]
    #[inline(always)]
    pub fn dir8(&self) -> DIR8_R {
        DIR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set GPIO direction for GPIO 9"]
    #[inline(always)]
    pub fn dir9(&self) -> DIR9_R {
        DIR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set GPIO direction for GPIO 10"]
    #[inline(always)]
    pub fn dir10(&self) -> DIR10_R {
        DIR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set GPIO direction for GPIO 11"]
    #[inline(always)]
    pub fn dir11(&self) -> DIR11_R {
        DIR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set GPIO direction for GPIO 12"]
    #[inline(always)]
    pub fn dir12(&self) -> DIR12_R {
        DIR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set GPIO direction for GPIO 13"]
    #[inline(always)]
    pub fn dir13(&self) -> DIR13_R {
        DIR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set GPIO direction for GPIO 14"]
    #[inline(always)]
    pub fn dir14(&self) -> DIR14_R {
        DIR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set GPIO direction for GPIO 15"]
    #[inline(always)]
    pub fn dir15(&self) -> DIR15_R {
        DIR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set GPIO direction for GPIO 16"]
    #[inline(always)]
    pub fn dir16(&self) -> DIR16_R {
        DIR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set GPIO direction for GPIO 17"]
    #[inline(always)]
    pub fn dir17(&self) -> DIR17_R {
        DIR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set GPIO direction for GPIO 18"]
    #[inline(always)]
    pub fn dir18(&self) -> DIR18_R {
        DIR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set GPIO direction for GPIO 19"]
    #[inline(always)]
    pub fn dir19(&self) -> DIR19_R {
        DIR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set GPIO direction for GPIO 20"]
    #[inline(always)]
    pub fn dir20(&self) -> DIR20_R {
        DIR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set GPIO direction for GPIO 21"]
    #[inline(always)]
    pub fn dir21(&self) -> DIR21_R {
        DIR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set GPIO direction for GPIO 22"]
    #[inline(always)]
    pub fn dir22(&self) -> DIR22_R {
        DIR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set GPIO direction for GPIO 23"]
    #[inline(always)]
    pub fn dir23(&self) -> DIR23_R {
        DIR23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set GPIO direction for GPIO 0"]
    #[inline(always)]
    pub fn dir0(&mut self) -> DIR0_W {
        DIR0_W { w: self }
    }
    #[doc = "Bit 1 - Set GPIO direction for GPIO 1"]
    #[inline(always)]
    pub fn dir1(&mut self) -> DIR1_W {
        DIR1_W { w: self }
    }
    #[doc = "Bit 2 - Set GPIO direction for GPIO 2"]
    #[inline(always)]
    pub fn dir2(&mut self) -> DIR2_W {
        DIR2_W { w: self }
    }
    #[doc = "Bit 3 - Set GPIO direction for GPIO 3"]
    #[inline(always)]
    pub fn dir3(&mut self) -> DIR3_W {
        DIR3_W { w: self }
    }
    #[doc = "Bit 4 - Set GPIO direction for GPIO 4"]
    #[inline(always)]
    pub fn dir4(&mut self) -> DIR4_W {
        DIR4_W { w: self }
    }
    #[doc = "Bit 5 - Set GPIO direction for GPIO 5"]
    #[inline(always)]
    pub fn dir5(&mut self) -> DIR5_W {
        DIR5_W { w: self }
    }
    #[doc = "Bit 6 - Set GPIO direction for GPIO 6"]
    #[inline(always)]
    pub fn dir6(&mut self) -> DIR6_W {
        DIR6_W { w: self }
    }
    #[doc = "Bit 7 - Set GPIO direction for GPIO 7"]
    #[inline(always)]
    pub fn dir7(&mut self) -> DIR7_W {
        DIR7_W { w: self }
    }
    #[doc = "Bit 8 - Set GPIO direction for GPIO 8"]
    #[inline(always)]
    pub fn dir8(&mut self) -> DIR8_W {
        DIR8_W { w: self }
    }
    #[doc = "Bit 9 - Set GPIO direction for GPIO 9"]
    #[inline(always)]
    pub fn dir9(&mut self) -> DIR9_W {
        DIR9_W { w: self }
    }
    #[doc = "Bit 10 - Set GPIO direction for GPIO 10"]
    #[inline(always)]
    pub fn dir10(&mut self) -> DIR10_W {
        DIR10_W { w: self }
    }
    #[doc = "Bit 11 - Set GPIO direction for GPIO 11"]
    #[inline(always)]
    pub fn dir11(&mut self) -> DIR11_W {
        DIR11_W { w: self }
    }
    #[doc = "Bit 12 - Set GPIO direction for GPIO 12"]
    #[inline(always)]
    pub fn dir12(&mut self) -> DIR12_W {
        DIR12_W { w: self }
    }
    #[doc = "Bit 13 - Set GPIO direction for GPIO 13"]
    #[inline(always)]
    pub fn dir13(&mut self) -> DIR13_W {
        DIR13_W { w: self }
    }
    #[doc = "Bit 14 - Set GPIO direction for GPIO 14"]
    #[inline(always)]
    pub fn dir14(&mut self) -> DIR14_W {
        DIR14_W { w: self }
    }
    #[doc = "Bit 15 - Set GPIO direction for GPIO 15"]
    #[inline(always)]
    pub fn dir15(&mut self) -> DIR15_W {
        DIR15_W { w: self }
    }
    #[doc = "Bit 16 - Set GPIO direction for GPIO 16"]
    #[inline(always)]
    pub fn dir16(&mut self) -> DIR16_W {
        DIR16_W { w: self }
    }
    #[doc = "Bit 17 - Set GPIO direction for GPIO 17"]
    #[inline(always)]
    pub fn dir17(&mut self) -> DIR17_W {
        DIR17_W { w: self }
    }
    #[doc = "Bit 18 - Set GPIO direction for GPIO 18"]
    #[inline(always)]
    pub fn dir18(&mut self) -> DIR18_W {
        DIR18_W { w: self }
    }
    #[doc = "Bit 19 - Set GPIO direction for GPIO 19"]
    #[inline(always)]
    pub fn dir19(&mut self) -> DIR19_W {
        DIR19_W { w: self }
    }
    #[doc = "Bit 20 - Set GPIO direction for GPIO 20"]
    #[inline(always)]
    pub fn dir20(&mut self) -> DIR20_W {
        DIR20_W { w: self }
    }
    #[doc = "Bit 21 - Set GPIO direction for GPIO 21"]
    #[inline(always)]
    pub fn dir21(&mut self) -> DIR21_W {
        DIR21_W { w: self }
    }
    #[doc = "Bit 22 - Set GPIO direction for GPIO 22"]
    #[inline(always)]
    pub fn dir22(&mut self) -> DIR22_W {
        DIR22_W { w: self }
    }
    #[doc = "Bit 23 - Set GPIO direction for GPIO 23"]
    #[inline(always)]
    pub fn dir23(&mut self) -> DIR23_W {
        DIR23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "direction setting register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](index.html) module"]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [dir::R](R) reader structure"]
impl crate::Readable for DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dir::W](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

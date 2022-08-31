#[doc = "Register `PB_PU` reader"]
pub struct R(crate::R<PB_PU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PB_PU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PB_PU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PB_PU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PB_PU` writer"]
pub struct W(crate::W<PB_PU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PB_PU_SPEC>;
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
impl From<crate::W<PB_PU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PB_PU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU0` reader - GPIO PB pullup resistance enable"]
pub struct PU0_R(crate::FieldReader<bool, bool>);
impl PU0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU0` writer - GPIO PB pullup resistance enable"]
pub struct PU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PU0_W<'a> {
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
#[doc = "Field `PU1` reader - GPIO PB pullup resistance enable"]
pub struct PU1_R(crate::FieldReader<bool, bool>);
impl PU1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU1` writer - GPIO PB pullup resistance enable"]
pub struct PU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PU1_W<'a> {
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
#[doc = "Field `PU2` reader - GPIO PB pullup resistance enable"]
pub struct PU2_R(crate::FieldReader<bool, bool>);
impl PU2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU2` writer - GPIO PB pullup resistance enable"]
pub struct PU2_W<'a> {
    w: &'a mut W,
}
impl<'a> PU2_W<'a> {
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
#[doc = "Field `PU3` reader - GPIO PB pullup resistance enable"]
pub struct PU3_R(crate::FieldReader<bool, bool>);
impl PU3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU3` writer - GPIO PB pullup resistance enable"]
pub struct PU3_W<'a> {
    w: &'a mut W,
}
impl<'a> PU3_W<'a> {
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
#[doc = "Field `PU4` reader - GPIO PB pullup resistance enable"]
pub struct PU4_R(crate::FieldReader<bool, bool>);
impl PU4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU4` writer - GPIO PB pullup resistance enable"]
pub struct PU4_W<'a> {
    w: &'a mut W,
}
impl<'a> PU4_W<'a> {
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
#[doc = "Field `PU5` reader - GPIO PB pullup resistance enable"]
pub struct PU5_R(crate::FieldReader<bool, bool>);
impl PU5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU5` writer - GPIO PB pullup resistance enable"]
pub struct PU5_W<'a> {
    w: &'a mut W,
}
impl<'a> PU5_W<'a> {
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
#[doc = "Field `PU6` reader - GPIO PB pullup resistance enable"]
pub struct PU6_R(crate::FieldReader<bool, bool>);
impl PU6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU6` writer - GPIO PB pullup resistance enable"]
pub struct PU6_W<'a> {
    w: &'a mut W,
}
impl<'a> PU6_W<'a> {
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
#[doc = "Field `PU7` reader - GPIO PB pullup resistance enable"]
pub struct PU7_R(crate::FieldReader<bool, bool>);
impl PU7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU7` writer - GPIO PB pullup resistance enable"]
pub struct PU7_W<'a> {
    w: &'a mut W,
}
impl<'a> PU7_W<'a> {
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
#[doc = "Field `PU8` reader - GPIO PB pullup resistance enable"]
pub struct PU8_R(crate::FieldReader<bool, bool>);
impl PU8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU8` writer - GPIO PB pullup resistance enable"]
pub struct PU8_W<'a> {
    w: &'a mut W,
}
impl<'a> PU8_W<'a> {
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
#[doc = "Field `PU9` reader - GPIO PB pullup resistance enable"]
pub struct PU9_R(crate::FieldReader<bool, bool>);
impl PU9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU9` writer - GPIO PB pullup resistance enable"]
pub struct PU9_W<'a> {
    w: &'a mut W,
}
impl<'a> PU9_W<'a> {
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
#[doc = "Field `PU10` reader - GPIO PB pullup resistance enable"]
pub struct PU10_R(crate::FieldReader<bool, bool>);
impl PU10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU10` writer - GPIO PB pullup resistance enable"]
pub struct PU10_W<'a> {
    w: &'a mut W,
}
impl<'a> PU10_W<'a> {
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
#[doc = "Field `PU11` reader - GPIO PB pullup resistance enable"]
pub struct PU11_R(crate::FieldReader<bool, bool>);
impl PU11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU11` writer - GPIO PB pullup resistance enable"]
pub struct PU11_W<'a> {
    w: &'a mut W,
}
impl<'a> PU11_W<'a> {
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
#[doc = "Field `PU12` reader - GPIO PB pullup resistance enable"]
pub struct PU12_R(crate::FieldReader<bool, bool>);
impl PU12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU12` writer - GPIO PB pullup resistance enable"]
pub struct PU12_W<'a> {
    w: &'a mut W,
}
impl<'a> PU12_W<'a> {
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
#[doc = "Field `PU13` reader - GPIO PB pullup resistance enable"]
pub struct PU13_R(crate::FieldReader<bool, bool>);
impl PU13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU13` writer - GPIO PB pullup resistance enable"]
pub struct PU13_W<'a> {
    w: &'a mut W,
}
impl<'a> PU13_W<'a> {
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
#[doc = "Field `PU14` reader - GPIO PB pullup resistance enable"]
pub struct PU14_R(crate::FieldReader<bool, bool>);
impl PU14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU14` writer - GPIO PB pullup resistance enable"]
pub struct PU14_W<'a> {
    w: &'a mut W,
}
impl<'a> PU14_W<'a> {
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
#[doc = "Field `PU15` reader - GPIO PB pullup resistance enable"]
pub struct PU15_R(crate::FieldReader<bool, bool>);
impl PU15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU15` writer - GPIO PB pullup resistance enable"]
pub struct PU15_W<'a> {
    w: &'a mut W,
}
impl<'a> PU15_W<'a> {
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
#[doc = "Field `PU16` reader - GPIO PB pullup resistance enable"]
pub struct PU16_R(crate::FieldReader<bool, bool>);
impl PU16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU16` writer - GPIO PB pullup resistance enable"]
pub struct PU16_W<'a> {
    w: &'a mut W,
}
impl<'a> PU16_W<'a> {
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
#[doc = "Field `PU17` reader - GPIO PB pullup resistance enable"]
pub struct PU17_R(crate::FieldReader<bool, bool>);
impl PU17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU17` writer - GPIO PB pullup resistance enable"]
pub struct PU17_W<'a> {
    w: &'a mut W,
}
impl<'a> PU17_W<'a> {
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
#[doc = "Field `PU18` reader - GPIO PB pullup resistance enable"]
pub struct PU18_R(crate::FieldReader<bool, bool>);
impl PU18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU18` writer - GPIO PB pullup resistance enable"]
pub struct PU18_W<'a> {
    w: &'a mut W,
}
impl<'a> PU18_W<'a> {
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
#[doc = "Field `PU19` reader - GPIO PB pullup resistance enable"]
pub struct PU19_R(crate::FieldReader<bool, bool>);
impl PU19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU19` writer - GPIO PB pullup resistance enable"]
pub struct PU19_W<'a> {
    w: &'a mut W,
}
impl<'a> PU19_W<'a> {
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
#[doc = "Field `PU20` reader - GPIO PB pullup resistance enable"]
pub struct PU20_R(crate::FieldReader<bool, bool>);
impl PU20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU20` writer - GPIO PB pullup resistance enable"]
pub struct PU20_W<'a> {
    w: &'a mut W,
}
impl<'a> PU20_W<'a> {
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
#[doc = "Field `PU21` reader - GPIO PB pullup resistance enable"]
pub struct PU21_R(crate::FieldReader<bool, bool>);
impl PU21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU21` writer - GPIO PB pullup resistance enable"]
pub struct PU21_W<'a> {
    w: &'a mut W,
}
impl<'a> PU21_W<'a> {
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
#[doc = "Field `PU22` reader - GPIO PB pullup resistance enable"]
pub struct PU22_R(crate::FieldReader<bool, bool>);
impl PU22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU22` writer - GPIO PB pullup resistance enable"]
pub struct PU22_W<'a> {
    w: &'a mut W,
}
impl<'a> PU22_W<'a> {
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
#[doc = "Field `PU23` reader - GPIO PB pullup resistance enable"]
pub struct PU23_R(crate::FieldReader<bool, bool>);
impl PU23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU23` writer - GPIO PB pullup resistance enable"]
pub struct PU23_W<'a> {
    w: &'a mut W,
}
impl<'a> PU23_W<'a> {
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
#[doc = "Field `PU24` reader - GPIO PB pullup resistance enable"]
pub struct PU24_R(crate::FieldReader<bool, bool>);
impl PU24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU24` writer - GPIO PB pullup resistance enable"]
pub struct PU24_W<'a> {
    w: &'a mut W,
}
impl<'a> PU24_W<'a> {
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
    #[doc = "Bit 0 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu12(&self) -> PU12_R {
        PU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu16(&self) -> PU16_R {
        PU16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu17(&self) -> PU17_R {
        PU17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu18(&self) -> PU18_R {
        PU18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu19(&self) -> PU19_R {
        PU19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu20(&self) -> PU20_R {
        PU20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu21(&self) -> PU21_R {
        PU21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu22(&self) -> PU22_R {
        PU22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu23(&self) -> PU23_R {
        PU23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu24(&self) -> PU24_R {
        PU24_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W {
        PU0_W { w: self }
    }
    #[doc = "Bit 1 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W {
        PU1_W { w: self }
    }
    #[doc = "Bit 2 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W {
        PU2_W { w: self }
    }
    #[doc = "Bit 3 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W {
        PU3_W { w: self }
    }
    #[doc = "Bit 4 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W {
        PU4_W { w: self }
    }
    #[doc = "Bit 5 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu5(&mut self) -> PU5_W {
        PU5_W { w: self }
    }
    #[doc = "Bit 6 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W {
        PU6_W { w: self }
    }
    #[doc = "Bit 7 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu7(&mut self) -> PU7_W {
        PU7_W { w: self }
    }
    #[doc = "Bit 8 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu8(&mut self) -> PU8_W {
        PU8_W { w: self }
    }
    #[doc = "Bit 9 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu9(&mut self) -> PU9_W {
        PU9_W { w: self }
    }
    #[doc = "Bit 10 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu10(&mut self) -> PU10_W {
        PU10_W { w: self }
    }
    #[doc = "Bit 11 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu11(&mut self) -> PU11_W {
        PU11_W { w: self }
    }
    #[doc = "Bit 12 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu12(&mut self) -> PU12_W {
        PU12_W { w: self }
    }
    #[doc = "Bit 13 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu13(&mut self) -> PU13_W {
        PU13_W { w: self }
    }
    #[doc = "Bit 14 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu14(&mut self) -> PU14_W {
        PU14_W { w: self }
    }
    #[doc = "Bit 15 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu15(&mut self) -> PU15_W {
        PU15_W { w: self }
    }
    #[doc = "Bit 16 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu16(&mut self) -> PU16_W {
        PU16_W { w: self }
    }
    #[doc = "Bit 17 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu17(&mut self) -> PU17_W {
        PU17_W { w: self }
    }
    #[doc = "Bit 18 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu18(&mut self) -> PU18_W {
        PU18_W { w: self }
    }
    #[doc = "Bit 19 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu19(&mut self) -> PU19_W {
        PU19_W { w: self }
    }
    #[doc = "Bit 20 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu20(&mut self) -> PU20_W {
        PU20_W { w: self }
    }
    #[doc = "Bit 21 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu21(&mut self) -> PU21_W {
        PU21_W { w: self }
    }
    #[doc = "Bit 22 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu22(&mut self) -> PU22_W {
        PU22_W { w: self }
    }
    #[doc = "Bit 23 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu23(&mut self) -> PU23_W {
        PU23_W { w: self }
    }
    #[doc = "Bit 24 - GPIO PB pullup resistance enable"]
    #[inline(always)]
    pub fn pu24(&mut self) -> PU24_W {
        PU24_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO PB pullup resistance enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_pu](index.html) module"]
pub struct PB_PU_SPEC;
impl crate::RegisterSpec for PB_PU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pb_pu::R](R) reader structure"]
impl crate::Readable for PB_PU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pb_pu::W](W) writer structure"]
impl crate::Writable for PB_PU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PB_PU to value 0"]
impl crate::Resettable for PB_PU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

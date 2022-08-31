#[doc = "Register `PD` reader"]
pub struct R(crate::R<PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD` writer"]
pub struct W(crate::W<PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_SPEC>;
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
impl From<crate::W<PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD0` reader - Pull-down enabled"]
pub struct PD0_R(crate::FieldReader<bool, bool>);
impl PD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD0` writer - Pull-down enabled"]
pub struct PD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD0_W<'a> {
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
#[doc = "Field `PD1` reader - Pull-down enabled"]
pub struct PD1_R(crate::FieldReader<bool, bool>);
impl PD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD1` writer - Pull-down enabled"]
pub struct PD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PD1_W<'a> {
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
#[doc = "Field `PD2` reader - Pull-down enabled"]
pub struct PD2_R(crate::FieldReader<bool, bool>);
impl PD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD2` writer - Pull-down enabled"]
pub struct PD2_W<'a> {
    w: &'a mut W,
}
impl<'a> PD2_W<'a> {
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
#[doc = "Field `PD3` reader - Pull-down enabled"]
pub struct PD3_R(crate::FieldReader<bool, bool>);
impl PD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD3` writer - Pull-down enabled"]
pub struct PD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PD3_W<'a> {
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
#[doc = "Field `PD4` reader - Pull-down enabled"]
pub struct PD4_R(crate::FieldReader<bool, bool>);
impl PD4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD4` writer - Pull-down enabled"]
pub struct PD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PD4_W<'a> {
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
#[doc = "Field `PD5` reader - Pull-down enabled"]
pub struct PD5_R(crate::FieldReader<bool, bool>);
impl PD5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD5` writer - Pull-down enabled"]
pub struct PD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PD5_W<'a> {
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
#[doc = "Field `PD6` reader - Pull-down enabled"]
pub struct PD6_R(crate::FieldReader<bool, bool>);
impl PD6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD6` writer - Pull-down enabled"]
pub struct PD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PD6_W<'a> {
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
#[doc = "Field `PD7` reader - Pull-down enabled"]
pub struct PD7_R(crate::FieldReader<bool, bool>);
impl PD7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD7` writer - Pull-down enabled"]
pub struct PD7_W<'a> {
    w: &'a mut W,
}
impl<'a> PD7_W<'a> {
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
#[doc = "Field `PD8` reader - Pull-down enabled"]
pub struct PD8_R(crate::FieldReader<bool, bool>);
impl PD8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD8` writer - Pull-down enabled"]
pub struct PD8_W<'a> {
    w: &'a mut W,
}
impl<'a> PD8_W<'a> {
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
#[doc = "Field `PD9` reader - Pull-down enabled"]
pub struct PD9_R(crate::FieldReader<bool, bool>);
impl PD9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD9` writer - Pull-down enabled"]
pub struct PD9_W<'a> {
    w: &'a mut W,
}
impl<'a> PD9_W<'a> {
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
#[doc = "Field `PD10` reader - Pull-down enabled"]
pub struct PD10_R(crate::FieldReader<bool, bool>);
impl PD10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD10` writer - Pull-down enabled"]
pub struct PD10_W<'a> {
    w: &'a mut W,
}
impl<'a> PD10_W<'a> {
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
#[doc = "Field `PD11` reader - Pull-down enabled"]
pub struct PD11_R(crate::FieldReader<bool, bool>);
impl PD11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD11` writer - Pull-down enabled"]
pub struct PD11_W<'a> {
    w: &'a mut W,
}
impl<'a> PD11_W<'a> {
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
#[doc = "Field `PD12` reader - Pull-down enabled"]
pub struct PD12_R(crate::FieldReader<bool, bool>);
impl PD12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD12` writer - Pull-down enabled"]
pub struct PD12_W<'a> {
    w: &'a mut W,
}
impl<'a> PD12_W<'a> {
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
#[doc = "Field `PD13` reader - Pull-down enabled"]
pub struct PD13_R(crate::FieldReader<bool, bool>);
impl PD13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD13` writer - Pull-down enabled"]
pub struct PD13_W<'a> {
    w: &'a mut W,
}
impl<'a> PD13_W<'a> {
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
#[doc = "Field `PD14` reader - Pull-down enabled"]
pub struct PD14_R(crate::FieldReader<bool, bool>);
impl PD14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD14` writer - Pull-down enabled"]
pub struct PD14_W<'a> {
    w: &'a mut W,
}
impl<'a> PD14_W<'a> {
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
#[doc = "Field `PD15` reader - Pull-down enabled"]
pub struct PD15_R(crate::FieldReader<bool, bool>);
impl PD15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD15` writer - Pull-down enabled"]
pub struct PD15_W<'a> {
    w: &'a mut W,
}
impl<'a> PD15_W<'a> {
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
#[doc = "Field `PD16` reader - Pull-down enabled"]
pub struct PD16_R(crate::FieldReader<bool, bool>);
impl PD16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD16` writer - Pull-down enabled"]
pub struct PD16_W<'a> {
    w: &'a mut W,
}
impl<'a> PD16_W<'a> {
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
#[doc = "Field `PD17` reader - Pull-down enabled"]
pub struct PD17_R(crate::FieldReader<bool, bool>);
impl PD17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD17` writer - Pull-down enabled"]
pub struct PD17_W<'a> {
    w: &'a mut W,
}
impl<'a> PD17_W<'a> {
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
#[doc = "Field `PD18` reader - Pull-down enabled"]
pub struct PD18_R(crate::FieldReader<bool, bool>);
impl PD18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD18` writer - Pull-down enabled"]
pub struct PD18_W<'a> {
    w: &'a mut W,
}
impl<'a> PD18_W<'a> {
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
#[doc = "Field `PD19` reader - Pull-down enabled"]
pub struct PD19_R(crate::FieldReader<bool, bool>);
impl PD19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD19` writer - Pull-down enabled"]
pub struct PD19_W<'a> {
    w: &'a mut W,
}
impl<'a> PD19_W<'a> {
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
#[doc = "Field `PD20` reader - Pull-down enabled"]
pub struct PD20_R(crate::FieldReader<bool, bool>);
impl PD20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD20` writer - Pull-down enabled"]
pub struct PD20_W<'a> {
    w: &'a mut W,
}
impl<'a> PD20_W<'a> {
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
#[doc = "Field `PD21` reader - Pull-down enabled"]
pub struct PD21_R(crate::FieldReader<bool, bool>);
impl PD21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD21` writer - Pull-down enabled"]
pub struct PD21_W<'a> {
    w: &'a mut W,
}
impl<'a> PD21_W<'a> {
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
#[doc = "Field `PD22` reader - Pull-down enabled"]
pub struct PD22_R(crate::FieldReader<bool, bool>);
impl PD22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD22` writer - Pull-down enabled"]
pub struct PD22_W<'a> {
    w: &'a mut W,
}
impl<'a> PD22_W<'a> {
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
#[doc = "Field `PD23` reader - Pull-down enabled"]
pub struct PD23_R(crate::FieldReader<bool, bool>);
impl PD23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD23` writer - Pull-down enabled"]
pub struct PD23_W<'a> {
    w: &'a mut W,
}
impl<'a> PD23_W<'a> {
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
    #[doc = "Bit 0 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd16(&self) -> PD16_R {
        PD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd17(&self) -> PD17_R {
        PD17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd18(&self) -> PD18_R {
        PD18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd19(&self) -> PD19_R {
        PD19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd20(&self) -> PD20_R {
        PD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd21(&self) -> PD21_R {
        PD21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd22(&self) -> PD22_R {
        PD22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd23(&self) -> PD23_R {
        PD23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W {
        PD0_W { w: self }
    }
    #[doc = "Bit 1 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W {
        PD1_W { w: self }
    }
    #[doc = "Bit 2 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W {
        PD2_W { w: self }
    }
    #[doc = "Bit 3 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W {
        PD3_W { w: self }
    }
    #[doc = "Bit 4 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W {
        PD4_W { w: self }
    }
    #[doc = "Bit 5 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W {
        PD5_W { w: self }
    }
    #[doc = "Bit 6 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W {
        PD6_W { w: self }
    }
    #[doc = "Bit 7 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W {
        PD7_W { w: self }
    }
    #[doc = "Bit 8 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W {
        PD8_W { w: self }
    }
    #[doc = "Bit 9 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W {
        PD9_W { w: self }
    }
    #[doc = "Bit 10 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W {
        PD10_W { w: self }
    }
    #[doc = "Bit 11 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W {
        PD11_W { w: self }
    }
    #[doc = "Bit 12 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd12(&mut self) -> PD12_W {
        PD12_W { w: self }
    }
    #[doc = "Bit 13 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W {
        PD13_W { w: self }
    }
    #[doc = "Bit 14 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W {
        PD14_W { w: self }
    }
    #[doc = "Bit 15 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W {
        PD15_W { w: self }
    }
    #[doc = "Bit 16 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd16(&mut self) -> PD16_W {
        PD16_W { w: self }
    }
    #[doc = "Bit 17 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd17(&mut self) -> PD17_W {
        PD17_W { w: self }
    }
    #[doc = "Bit 18 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd18(&mut self) -> PD18_W {
        PD18_W { w: self }
    }
    #[doc = "Bit 19 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd19(&mut self) -> PD19_W {
        PD19_W { w: self }
    }
    #[doc = "Bit 20 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd20(&mut self) -> PD20_W {
        PD20_W { w: self }
    }
    #[doc = "Bit 21 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd21(&mut self) -> PD21_W {
        PD21_W { w: self }
    }
    #[doc = "Bit 22 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd22(&mut self) -> PD22_W {
        PD22_W { w: self }
    }
    #[doc = "Bit 23 - Pull-down enabled"]
    #[inline(always)]
    pub fn pd23(&mut self) -> PD23_W {
        PD23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pull-down enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd](index.html) module"]
pub struct PD_SPEC;
impl crate::RegisterSpec for PD_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [pd::R](R) reader structure"]
impl crate::Readable for PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd::W](W) writer structure"]
impl crate::Writable for PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD to value 0"]
impl crate::Resettable for PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

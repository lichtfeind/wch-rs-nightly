#[doc = "Register `PA_DRV` reader"]
pub struct R(crate::R<PA_DRV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_DRV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA_DRV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA_DRV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PA_DRV` writer"]
pub struct W(crate::W<PA_DRV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_DRV_SPEC>;
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
impl From<crate::W<PA_DRV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_DRV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO PA driving capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRV0_A {
    #[doc = "0: 0: 8mA"]
    DRV_8MA = 0,
    #[doc = "1: 1: 16mA"]
    DRV_16MA = 1,
}
impl From<DRV0_A> for bool {
    #[inline(always)]
    fn from(variant: DRV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRV0` reader - GPIO PA driving capability"]
pub struct DRV0_R(crate::FieldReader<bool, DRV0_A>);
impl DRV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRV0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRV0_A {
        match self.bits {
            false => DRV0_A::DRV_8MA,
            true => DRV0_A::DRV_16MA,
        }
    }
    #[doc = "Checks if the value of the field is `DRV_8MA`"]
    #[inline(always)]
    pub fn is_drv_8m_a(&self) -> bool {
        **self == DRV0_A::DRV_8MA
    }
    #[doc = "Checks if the value of the field is `DRV_16MA`"]
    #[inline(always)]
    pub fn is_drv_16m_a(&self) -> bool {
        **self == DRV0_A::DRV_16MA
    }
}
impl core::ops::Deref for DRV0_R {
    type Target = crate::FieldReader<bool, DRV0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRV0` writer - GPIO PA driving capability"]
pub struct DRV0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV0_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV0_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV1_A = DRV0_A;
#[doc = "Field `DRV1` reader - GPIO PA driving capability"]
pub type DRV1_R = DRV0_R;
#[doc = "Field `DRV1` writer - GPIO PA driving capability"]
pub struct DRV1_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV1_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV1_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV2_A = DRV0_A;
#[doc = "Field `DRV2` reader - GPIO PA driving capability"]
pub type DRV2_R = DRV0_R;
#[doc = "Field `DRV2` writer - GPIO PA driving capability"]
pub struct DRV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV2_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV2_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV3_A = DRV0_A;
#[doc = "Field `DRV3` reader - GPIO PA driving capability"]
pub type DRV3_R = DRV0_R;
#[doc = "Field `DRV3` writer - GPIO PA driving capability"]
pub struct DRV3_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV3_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV3_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV4_A = DRV0_A;
#[doc = "Field `DRV4` reader - GPIO PA driving capability"]
pub type DRV4_R = DRV0_R;
#[doc = "Field `DRV4` writer - GPIO PA driving capability"]
pub struct DRV4_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV4_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV4_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV5_A = DRV0_A;
#[doc = "Field `DRV5` reader - GPIO PA driving capability"]
pub type DRV5_R = DRV0_R;
#[doc = "Field `DRV5` writer - GPIO PA driving capability"]
pub struct DRV5_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV5_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV5_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV6_A = DRV0_A;
#[doc = "Field `DRV6` reader - GPIO PA driving capability"]
pub type DRV6_R = DRV0_R;
#[doc = "Field `DRV6` writer - GPIO PA driving capability"]
pub struct DRV6_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV6_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV6_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV7_A = DRV0_A;
#[doc = "Field `DRV7` reader - GPIO PA driving capability"]
pub type DRV7_R = DRV0_R;
#[doc = "Field `DRV7` writer - GPIO PA driving capability"]
pub struct DRV7_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV7_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV7_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV8_A = DRV0_A;
#[doc = "Field `DRV8` reader - GPIO PA driving capability"]
pub type DRV8_R = DRV0_R;
#[doc = "Field `DRV8` writer - GPIO PA driving capability"]
pub struct DRV8_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV8_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV8_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV9_A = DRV0_A;
#[doc = "Field `DRV9` reader - GPIO PA driving capability"]
pub type DRV9_R = DRV0_R;
#[doc = "Field `DRV9` writer - GPIO PA driving capability"]
pub struct DRV9_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV9_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV9_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV10_A = DRV0_A;
#[doc = "Field `DRV10` reader - GPIO PA driving capability"]
pub type DRV10_R = DRV0_R;
#[doc = "Field `DRV10` writer - GPIO PA driving capability"]
pub struct DRV10_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV10_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV10_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV11_A = DRV0_A;
#[doc = "Field `DRV11` reader - GPIO PA driving capability"]
pub type DRV11_R = DRV0_R;
#[doc = "Field `DRV11` writer - GPIO PA driving capability"]
pub struct DRV11_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV11_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV11_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV12_A = DRV0_A;
#[doc = "Field `DRV12` reader - GPIO PA driving capability"]
pub type DRV12_R = DRV0_R;
#[doc = "Field `DRV12` writer - GPIO PA driving capability"]
pub struct DRV12_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV12_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV12_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV13_A = DRV0_A;
#[doc = "Field `DRV13` reader - GPIO PA driving capability"]
pub type DRV13_R = DRV0_R;
#[doc = "Field `DRV13` writer - GPIO PA driving capability"]
pub struct DRV13_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV13_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV13_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV14_A = DRV0_A;
#[doc = "Field `DRV14` reader - GPIO PA driving capability"]
pub type DRV14_R = DRV0_R;
#[doc = "Field `DRV14` writer - GPIO PA driving capability"]
pub struct DRV14_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV14_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV14_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV15_A = DRV0_A;
#[doc = "Field `DRV15` reader - GPIO PA driving capability"]
pub type DRV15_R = DRV0_R;
#[doc = "Field `DRV15` writer - GPIO PA driving capability"]
pub struct DRV15_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV15_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV15_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV16_A = DRV0_A;
#[doc = "Field `DRV16` reader - GPIO PA driving capability"]
pub type DRV16_R = DRV0_R;
#[doc = "Field `DRV16` writer - GPIO PA driving capability"]
pub struct DRV16_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV16_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV16_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV17_A = DRV0_A;
#[doc = "Field `DRV17` reader - GPIO PA driving capability"]
pub type DRV17_R = DRV0_R;
#[doc = "Field `DRV17` writer - GPIO PA driving capability"]
pub struct DRV17_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV17_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV17_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV18_A = DRV0_A;
#[doc = "Field `DRV18` reader - GPIO PA driving capability"]
pub type DRV18_R = DRV0_R;
#[doc = "Field `DRV18` writer - GPIO PA driving capability"]
pub struct DRV18_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV18_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV18_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV19_A = DRV0_A;
#[doc = "Field `DRV19` reader - GPIO PA driving capability"]
pub type DRV19_R = DRV0_R;
#[doc = "Field `DRV19` writer - GPIO PA driving capability"]
pub struct DRV19_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV19_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV19_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV20_A = DRV0_A;
#[doc = "Field `DRV20` reader - GPIO PA driving capability"]
pub type DRV20_R = DRV0_R;
#[doc = "Field `DRV20` writer - GPIO PA driving capability"]
pub struct DRV20_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV20_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV20_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV21_A = DRV0_A;
#[doc = "Field `DRV21` reader - GPIO PA driving capability"]
pub type DRV21_R = DRV0_R;
#[doc = "Field `DRV21` writer - GPIO PA driving capability"]
pub struct DRV21_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV21_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV21_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV22_A = DRV0_A;
#[doc = "Field `DRV22` reader - GPIO PA driving capability"]
pub type DRV22_R = DRV0_R;
#[doc = "Field `DRV22` writer - GPIO PA driving capability"]
pub struct DRV22_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV22_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV22_A::DRV_16MA)
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
#[doc = "GPIO PA driving capability"]
pub type DRV23_A = DRV0_A;
#[doc = "Field `DRV23` reader - GPIO PA driving capability"]
pub type DRV23_R = DRV0_R;
#[doc = "Field `DRV23` writer - GPIO PA driving capability"]
pub struct DRV23_W<'a> {
    w: &'a mut W,
}
impl<'a> DRV23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRV23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "0: 8mA"]
    #[inline(always)]
    pub fn drv_8m_a(self) -> &'a mut W {
        self.variant(DRV23_A::DRV_8MA)
    }
    #[doc = "1: 16mA"]
    #[inline(always)]
    pub fn drv_16m_a(self) -> &'a mut W {
        self.variant(DRV23_A::DRV_16MA)
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
    #[doc = "Bit 0 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv0(&self) -> DRV0_R {
        DRV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv1(&self) -> DRV1_R {
        DRV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv2(&self) -> DRV2_R {
        DRV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv3(&self) -> DRV3_R {
        DRV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv4(&self) -> DRV4_R {
        DRV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv5(&self) -> DRV5_R {
        DRV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv6(&self) -> DRV6_R {
        DRV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv7(&self) -> DRV7_R {
        DRV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv8(&self) -> DRV8_R {
        DRV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv9(&self) -> DRV9_R {
        DRV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv10(&self) -> DRV10_R {
        DRV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv11(&self) -> DRV11_R {
        DRV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv12(&self) -> DRV12_R {
        DRV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv13(&self) -> DRV13_R {
        DRV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv14(&self) -> DRV14_R {
        DRV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv15(&self) -> DRV15_R {
        DRV15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv16(&self) -> DRV16_R {
        DRV16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv17(&self) -> DRV17_R {
        DRV17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv18(&self) -> DRV18_R {
        DRV18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv19(&self) -> DRV19_R {
        DRV19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv20(&self) -> DRV20_R {
        DRV20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv21(&self) -> DRV21_R {
        DRV21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv22(&self) -> DRV22_R {
        DRV22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv23(&self) -> DRV23_R {
        DRV23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv0(&mut self) -> DRV0_W {
        DRV0_W { w: self }
    }
    #[doc = "Bit 1 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv1(&mut self) -> DRV1_W {
        DRV1_W { w: self }
    }
    #[doc = "Bit 2 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv2(&mut self) -> DRV2_W {
        DRV2_W { w: self }
    }
    #[doc = "Bit 3 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv3(&mut self) -> DRV3_W {
        DRV3_W { w: self }
    }
    #[doc = "Bit 4 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv4(&mut self) -> DRV4_W {
        DRV4_W { w: self }
    }
    #[doc = "Bit 5 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv5(&mut self) -> DRV5_W {
        DRV5_W { w: self }
    }
    #[doc = "Bit 6 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv6(&mut self) -> DRV6_W {
        DRV6_W { w: self }
    }
    #[doc = "Bit 7 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv7(&mut self) -> DRV7_W {
        DRV7_W { w: self }
    }
    #[doc = "Bit 8 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv8(&mut self) -> DRV8_W {
        DRV8_W { w: self }
    }
    #[doc = "Bit 9 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv9(&mut self) -> DRV9_W {
        DRV9_W { w: self }
    }
    #[doc = "Bit 10 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv10(&mut self) -> DRV10_W {
        DRV10_W { w: self }
    }
    #[doc = "Bit 11 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv11(&mut self) -> DRV11_W {
        DRV11_W { w: self }
    }
    #[doc = "Bit 12 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv12(&mut self) -> DRV12_W {
        DRV12_W { w: self }
    }
    #[doc = "Bit 13 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv13(&mut self) -> DRV13_W {
        DRV13_W { w: self }
    }
    #[doc = "Bit 14 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv14(&mut self) -> DRV14_W {
        DRV14_W { w: self }
    }
    #[doc = "Bit 15 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv15(&mut self) -> DRV15_W {
        DRV15_W { w: self }
    }
    #[doc = "Bit 16 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv16(&mut self) -> DRV16_W {
        DRV16_W { w: self }
    }
    #[doc = "Bit 17 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv17(&mut self) -> DRV17_W {
        DRV17_W { w: self }
    }
    #[doc = "Bit 18 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv18(&mut self) -> DRV18_W {
        DRV18_W { w: self }
    }
    #[doc = "Bit 19 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv19(&mut self) -> DRV19_W {
        DRV19_W { w: self }
    }
    #[doc = "Bit 20 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv20(&mut self) -> DRV20_W {
        DRV20_W { w: self }
    }
    #[doc = "Bit 21 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv21(&mut self) -> DRV21_W {
        DRV21_W { w: self }
    }
    #[doc = "Bit 22 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv22(&mut self) -> DRV22_W {
        DRV22_W { w: self }
    }
    #[doc = "Bit 23 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn drv23(&mut self) -> DRV23_W {
        DRV23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO PA driving capability\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_drv](index.html) module"]
pub struct PA_DRV_SPEC;
impl crate::RegisterSpec for PA_DRV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_drv::R](R) reader structure"]
impl crate::Readable for PA_DRV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_drv::W](W) writer structure"]
impl crate::Writable for PA_DRV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PA_DRV to value 0"]
impl crate::Resettable for PA_DRV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

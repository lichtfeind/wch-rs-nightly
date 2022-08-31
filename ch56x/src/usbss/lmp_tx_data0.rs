#[doc = "Register `LMP_TX_DATA0` reader"]
pub struct R(crate::R<LMP_TX_DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMP_TX_DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMP_TX_DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMP_TX_DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMP_TX_DATA0` writer"]
pub struct W(crate::W<LMP_TX_DATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMP_TX_DATA0_SPEC>;
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
impl From<crate::W<LMP_TX_DATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMP_TX_DATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINK_SPEED` reader - None"]
pub struct LINK_SPEED_R(crate::FieldReader<bool, bool>);
impl LINK_SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINK_SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINK_SPEED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINK_SPEED` writer - None"]
pub struct LINK_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_SPEED_W<'a> {
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
#[doc = "Field `LMP` reader - None"]
pub struct LMP_R(crate::FieldReader<u8, u8>);
impl LMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LMP` writer - None"]
pub struct LMP_W<'a> {
    w: &'a mut W,
}
impl<'a> LMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `LMP_HP` reader - None"]
pub struct LMP_HP_R(crate::FieldReader<bool, bool>);
impl LMP_HP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LMP_HP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LMP_HP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LMP_HP` writer - None"]
pub struct LMP_HP_W<'a> {
    w: &'a mut W,
}
impl<'a> LMP_HP_W<'a> {
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
impl R {
    #[doc = "Bit 9 - None"]
    #[inline(always)]
    pub fn link_speed(&self) -> LINK_SPEED_R {
        LINK_SPEED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 5:8 - None"]
    #[inline(always)]
    pub fn lmp(&self) -> LMP_R {
        LMP_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn lmp_hp(&self) -> LMP_HP_R {
        LMP_HP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - None"]
    #[inline(always)]
    pub fn link_speed(&mut self) -> LINK_SPEED_W {
        LINK_SPEED_W { w: self }
    }
    #[doc = "Bits 5:8 - None"]
    #[inline(always)]
    pub fn lmp(&mut self) -> LMP_W {
        LMP_W { w: self }
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn lmp_hp(&mut self) -> LMP_HP_W {
        LMP_HP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link Management Packet\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmp_tx_data0](index.html) module"]
pub struct LMP_TX_DATA0_SPEC;
impl crate::RegisterSpec for LMP_TX_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmp_tx_data0::R](R) reader structure"]
impl crate::Readable for LMP_TX_DATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmp_tx_data0::W](W) writer structure"]
impl crate::Writable for LMP_TX_DATA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LMP_TX_DATA0 to value 0"]
impl crate::Resettable for LMP_TX_DATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `LMP_TX_DATA1` reader"]
pub struct R(crate::R<LMP_TX_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMP_TX_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMP_TX_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMP_TX_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMP_TX_DATA1` writer"]
pub struct W(crate::W<LMP_TX_DATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMP_TX_DATA1_SPEC>;
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
impl From<crate::W<LMP_TX_DATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMP_TX_DATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UP_STREAM` reader - None"]
pub struct UP_STREAM_R(crate::FieldReader<bool, bool>);
impl UP_STREAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UP_STREAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UP_STREAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UP_STREAM` writer - None"]
pub struct UP_STREAM_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_STREAM_W<'a> {
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
#[doc = "Field `NUM_HP_BUF` reader - None"]
pub struct NUM_HP_BUF_R(crate::FieldReader<bool, bool>);
impl NUM_HP_BUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NUM_HP_BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_HP_BUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_HP_BUF` writer - None"]
pub struct NUM_HP_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> NUM_HP_BUF_W<'a> {
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
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn up_stream(&self) -> UP_STREAM_R {
        UP_STREAM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn num_hp_buf(&self) -> NUM_HP_BUF_R {
        NUM_HP_BUF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn up_stream(&mut self) -> UP_STREAM_W {
        UP_STREAM_W { w: self }
    }
    #[doc = "Bit 2 - None"]
    #[inline(always)]
    pub fn num_hp_buf(&mut self) -> NUM_HP_BUF_W {
        NUM_HP_BUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link Management Packet\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmp_tx_data1](index.html) module"]
pub struct LMP_TX_DATA1_SPEC;
impl crate::RegisterSpec for LMP_TX_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmp_tx_data1::R](R) reader structure"]
impl crate::Readable for LMP_TX_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmp_tx_data1::W](W) writer structure"]
impl crate::Writable for LMP_TX_DATA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LMP_TX_DATA1 to value 0"]
impl crate::Resettable for LMP_TX_DATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

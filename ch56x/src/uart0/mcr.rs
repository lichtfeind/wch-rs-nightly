#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTR` reader - UART0 control DTR"]
pub struct DTR_R(crate::FieldReader<bool, bool>);
impl DTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTR` writer - UART0 control DTR"]
pub struct DTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTR_W<'a> {
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
#[doc = "Field `RTS` reader - UART0 control RTS"]
pub struct RTS_R(crate::FieldReader<bool, bool>);
impl RTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTS` writer - UART0 control RTS"]
pub struct RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_W<'a> {
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
#[doc = "Field `OUT1` reader - UART0 control OUT1"]
pub struct OUT1_R(crate::FieldReader<bool, bool>);
impl OUT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT1` writer - UART0 control OUT1"]
pub struct OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1_W<'a> {
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
#[doc = "Field `OUT2` reader - UART control OUT2"]
pub struct OUT2_R(crate::FieldReader<bool, bool>);
impl OUT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT2` writer - UART control OUT2"]
pub struct OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT2_W<'a> {
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
#[doc = "Field `LOOP` reader - UART0 enable local loop back"]
pub struct LOOP_R(crate::FieldReader<bool, bool>);
impl LOOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOP` writer - UART0 enable local loop back"]
pub struct LOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_W<'a> {
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
#[doc = "Field `AU_FLOW_EN` reader - UART0 enable autoflow control"]
pub struct AU_FLOW_EN_R(crate::FieldReader<bool, bool>);
impl AU_FLOW_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AU_FLOW_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AU_FLOW_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AU_FLOW_EN` writer - UART0 enable autoflow control"]
pub struct AU_FLOW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AU_FLOW_EN_W<'a> {
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
#[doc = "Field `TNOW` reader - UART0 enable TNOW output on DTR pin"]
pub struct TNOW_R(crate::FieldReader<bool, bool>);
impl TNOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TNOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TNOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TNOW` writer - UART0 enable TNOW output on DTR pin"]
pub struct TNOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TNOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u8 & 1) << 6);
        self.w
    }
}
#[doc = "Field `HALF` reader - UART0 enable half-duplex"]
pub struct HALF_R(crate::FieldReader<bool, bool>);
impl HALF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALF` writer - UART0 enable half-duplex"]
pub struct HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> HALF_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u8 & 1) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - UART0 control DTR"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART0 control RTS"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART0 control OUT1"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART control OUT2"]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART0 enable local loop back"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART0 enable autoflow control"]
    #[inline(always)]
    pub fn au_flow_en(&self) -> AU_FLOW_EN_R {
        AU_FLOW_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART0 enable TNOW output on DTR pin"]
    #[inline(always)]
    pub fn tnow(&self) -> TNOW_R {
        TNOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART0 enable half-duplex"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 control DTR"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DTR_W {
        DTR_W { w: self }
    }
    #[doc = "Bit 1 - UART0 control RTS"]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W {
        RTS_W { w: self }
    }
    #[doc = "Bit 2 - UART0 control OUT1"]
    #[inline(always)]
    pub fn out1(&mut self) -> OUT1_W {
        OUT1_W { w: self }
    }
    #[doc = "Bit 3 - UART control OUT2"]
    #[inline(always)]
    pub fn out2(&mut self) -> OUT2_W {
        OUT2_W { w: self }
    }
    #[doc = "Bit 4 - UART0 enable local loop back"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LOOP_W {
        LOOP_W { w: self }
    }
    #[doc = "Bit 5 - UART0 enable autoflow control"]
    #[inline(always)]
    pub fn au_flow_en(&mut self) -> AU_FLOW_EN_W {
        AU_FLOW_EN_W { w: self }
    }
    #[doc = "Bit 6 - UART0 enable TNOW output on DTR pin"]
    #[inline(always)]
    pub fn tnow(&mut self) -> TNOW_W {
        TNOW_W { w: self }
    }
    #[doc = "Bit 7 - UART0 enable half-duplex"]
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W {
        HALF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0 modem control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

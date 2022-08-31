#[doc = "Register `FIFOADDRR1` reader"]
pub struct R(crate::R<FIFOADDRR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOADDRR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOADDRR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOADDRR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOADDRR1` writer"]
pub struct W(crate::W<FIFOADDRR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOADDRR1_SPEC>;
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
impl From<crate::W<FIFOADDRR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOADDRR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFADDR1` reader - OFFADDR1"]
pub struct OFFADDR1_R(crate::FieldReader<u32, u32>);
impl OFFADDR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OFFADDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFADDR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFADDR1` writer - OFFADDR1"]
pub struct OFFADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `IRQID1` reader - IRQID1"]
pub struct IRQID1_R(crate::FieldReader<u8, u8>);
impl IRQID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IRQID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQID1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQID1` writer - IRQID1"]
pub struct IRQID1_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQID1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - OFFADDR1"]
    #[inline(always)]
    pub fn offaddr1(&self) -> OFFADDR1_R {
        OFFADDR1_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - IRQID1"]
    #[inline(always)]
    pub fn irqid1(&self) -> IRQID1_R {
        IRQID1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR1"]
    #[inline(always)]
    pub fn offaddr1(&mut self) -> OFFADDR1_W {
        OFFADDR1_W { w: self }
    }
    #[doc = "Bits 24:31 - IRQID1"]
    #[inline(always)]
    pub fn irqid1(&mut self) -> IRQID1_W {
        IRQID1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 1 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoaddrr1](index.html) module"]
pub struct FIFOADDRR1_SPEC;
impl crate::RegisterSpec for FIFOADDRR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoaddrr1::R](R) reader structure"]
impl crate::Readable for FIFOADDRR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoaddrr1::W](W) writer structure"]
impl crate::Writable for FIFOADDRR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOADDRR1 to value 0"]
impl crate::Resettable for FIFOADDRR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

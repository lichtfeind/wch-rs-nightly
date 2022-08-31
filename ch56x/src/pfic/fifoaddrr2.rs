#[doc = "Register `FIFOADDRR2` reader"]
pub struct R(crate::R<FIFOADDRR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOADDRR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOADDRR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOADDRR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOADDRR2` writer"]
pub struct W(crate::W<FIFOADDRR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOADDRR2_SPEC>;
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
impl From<crate::W<FIFOADDRR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOADDRR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFADDR2` reader - OFFADDR2"]
pub struct OFFADDR2_R(crate::FieldReader<u32, u32>);
impl OFFADDR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OFFADDR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFADDR2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFADDR2` writer - OFFADDR2"]
pub struct OFFADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `IRQID2` reader - IRQID2"]
pub struct IRQID2_R(crate::FieldReader<u8, u8>);
impl IRQID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IRQID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQID2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQID2` writer - IRQID2"]
pub struct IRQID2_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQID2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - OFFADDR2"]
    #[inline(always)]
    pub fn offaddr2(&self) -> OFFADDR2_R {
        OFFADDR2_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - IRQID2"]
    #[inline(always)]
    pub fn irqid2(&self) -> IRQID2_R {
        IRQID2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR2"]
    #[inline(always)]
    pub fn offaddr2(&mut self) -> OFFADDR2_W {
        OFFADDR2_W { w: self }
    }
    #[doc = "Bits 24:31 - IRQID2"]
    #[inline(always)]
    pub fn irqid2(&mut self) -> IRQID2_W {
        IRQID2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 2 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoaddrr2](index.html) module"]
pub struct FIFOADDRR2_SPEC;
impl crate::RegisterSpec for FIFOADDRR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoaddrr2::R](R) reader structure"]
impl crate::Readable for FIFOADDRR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoaddrr2::W](W) writer structure"]
impl crate::Writable for FIFOADDRR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOADDRR2 to value 0"]
impl crate::Resettable for FIFOADDRR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

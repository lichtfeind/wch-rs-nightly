#[doc = "Register `FIFOADDRR3` reader"]
pub struct R(crate::R<FIFOADDRR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOADDRR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOADDRR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOADDRR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOADDRR3` writer"]
pub struct W(crate::W<FIFOADDRR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOADDRR3_SPEC>;
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
impl From<crate::W<FIFOADDRR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOADDRR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFADDR3` reader - OFFADDR3"]
pub struct OFFADDR3_R(crate::FieldReader<u32, u32>);
impl OFFADDR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OFFADDR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFADDR3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFADDR3` writer - OFFADDR3"]
pub struct OFFADDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFADDR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `IRQID3` reader - IRQID3"]
pub struct IRQID3_R(crate::FieldReader<u8, u8>);
impl IRQID3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IRQID3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQID3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQID3` writer - IRQID3"]
pub struct IRQID3_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQID3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - OFFADDR3"]
    #[inline(always)]
    pub fn offaddr3(&self) -> OFFADDR3_R {
        OFFADDR3_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - IRQID3"]
    #[inline(always)]
    pub fn irqid3(&self) -> IRQID3_R {
        IRQID3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR3"]
    #[inline(always)]
    pub fn offaddr3(&mut self) -> OFFADDR3_W {
        OFFADDR3_W { w: self }
    }
    #[doc = "Bits 24:31 - IRQID3"]
    #[inline(always)]
    pub fn irqid3(&mut self) -> IRQID3_W {
        IRQID3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 3 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoaddrr3](index.html) module"]
pub struct FIFOADDRR3_SPEC;
impl crate::RegisterSpec for FIFOADDRR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoaddrr3::R](R) reader structure"]
impl crate::Readable for FIFOADDRR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoaddrr3::W](W) writer structure"]
impl crate::Writable for FIFOADDRR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOADDRR3 to value 0"]
impl crate::Resettable for FIFOADDRR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

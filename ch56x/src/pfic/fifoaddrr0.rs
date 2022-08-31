#[doc = "Register `FIFOADDRR0` reader"]
pub struct R(crate::R<FIFOADDRR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOADDRR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOADDRR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOADDRR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOADDRR0` writer"]
pub struct W(crate::W<FIFOADDRR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOADDRR0_SPEC>;
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
impl From<crate::W<FIFOADDRR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOADDRR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFADDR0` reader - OFFADDR0"]
pub struct OFFADDR0_R(crate::FieldReader<u32, u32>);
impl OFFADDR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OFFADDR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFADDR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFADDR0` writer - OFFADDR0"]
pub struct OFFADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `IRQID0` reader - IRQID0"]
pub struct IRQID0_R(crate::FieldReader<u8, u8>);
impl IRQID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IRQID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQID0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQID0` writer - IRQID0"]
pub struct IRQID0_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQID0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - OFFADDR0"]
    #[inline(always)]
    pub fn offaddr0(&self) -> OFFADDR0_R {
        OFFADDR0_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - IRQID0"]
    #[inline(always)]
    pub fn irqid0(&self) -> IRQID0_R {
        IRQID0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR0"]
    #[inline(always)]
    pub fn offaddr0(&mut self) -> OFFADDR0_W {
        OFFADDR0_W { w: self }
    }
    #[doc = "Bits 24:31 - IRQID0"]
    #[inline(always)]
    pub fn irqid0(&mut self) -> IRQID0_W {
        IRQID0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 0 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoaddrr0](index.html) module"]
pub struct FIFOADDRR0_SPEC;
impl crate::RegisterSpec for FIFOADDRR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoaddrr0::R](R) reader structure"]
impl crate::Readable for FIFOADDRR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoaddrr0::W](W) writer structure"]
impl crate::Writable for FIFOADDRR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOADDRR0 to value 0"]
impl crate::Resettable for FIFOADDRR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

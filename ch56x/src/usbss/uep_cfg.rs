#[doc = "Register `UEP_CFG` reader"]
pub struct R(crate::R<UEP_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP_CFG` writer"]
pub struct W(crate::W<UEP_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP_CFG_SPEC>;
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
impl From<crate::W<UEP_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_R_EN` reader - Endpoint rx enable 0-7"]
pub struct EP_R_EN_R(crate::FieldReader<u8, u8>);
impl EP_R_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EP_R_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_R_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_R_EN` writer - Endpoint rx enable 0-7"]
pub struct EP_R_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_R_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `EP_T_EN` reader - Endpoint tx enable 0-7"]
pub struct EP_T_EN_R(crate::FieldReader<u8, u8>);
impl EP_T_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EP_T_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_T_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_T_EN` writer - Endpoint tx enable 0-7"]
pub struct EP_T_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_T_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `EP_R_ISO` reader - ISO receive enable 1-7"]
pub struct EP_R_ISO_R(crate::FieldReader<u8, u8>);
impl EP_R_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EP_R_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_R_ISO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_R_ISO` writer - ISO receive enable 1-7"]
pub struct EP_R_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_R_ISO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 17)) | ((value as u32 & 0x7f) << 17);
        self.w
    }
}
#[doc = "Field `EP_T_ISO` reader - ISO transmit enable 1-7"]
pub struct EP_T_ISO_R(crate::FieldReader<u8, u8>);
impl EP_T_ISO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EP_T_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_T_ISO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_T_ISO` writer - ISO transmit enable 1-7"]
pub struct EP_T_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_T_ISO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Endpoint rx enable 0-7"]
    #[inline(always)]
    pub fn ep_r_en(&self) -> EP_R_EN_R {
        EP_R_EN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Endpoint tx enable 0-7"]
    #[inline(always)]
    pub fn ep_t_en(&self) -> EP_T_EN_R {
        EP_T_EN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 17:23 - ISO receive enable 1-7"]
    #[inline(always)]
    pub fn ep_r_iso(&self) -> EP_R_ISO_R {
        EP_R_ISO_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - ISO transmit enable 1-7"]
    #[inline(always)]
    pub fn ep_t_iso(&self) -> EP_T_ISO_R {
        EP_T_ISO_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint rx enable 0-7"]
    #[inline(always)]
    pub fn ep_r_en(&mut self) -> EP_R_EN_W {
        EP_R_EN_W { w: self }
    }
    #[doc = "Bits 8:15 - Endpoint tx enable 0-7"]
    #[inline(always)]
    pub fn ep_t_en(&mut self) -> EP_T_EN_W {
        EP_T_EN_W { w: self }
    }
    #[doc = "Bits 17:23 - ISO receive enable 1-7"]
    #[inline(always)]
    pub fn ep_r_iso(&mut self) -> EP_R_ISO_W {
        EP_R_ISO_W { w: self }
    }
    #[doc = "Bits 25:31 - ISO transmit enable 1-7"]
    #[inline(always)]
    pub fn ep_t_iso(&mut self) -> EP_T_ISO_W {
        EP_T_ISO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep_cfg](index.html) module"]
pub struct UEP_CFG_SPEC;
impl crate::RegisterSpec for UEP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uep_cfg::R](R) reader structure"]
impl crate::Readable for UEP_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep_cfg::W](W) writer structure"]
impl crate::Writable for UEP_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP_CFG to value 0"]
impl crate::Resettable for UEP_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

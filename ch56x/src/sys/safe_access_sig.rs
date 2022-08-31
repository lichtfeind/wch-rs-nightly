#[doc = "Register `SAFE_ACCESS_SIG` reader"]
pub struct R(crate::R<SAFE_ACCESS_SIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAFE_ACCESS_SIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAFE_ACCESS_SIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAFE_ACCESS_SIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAFE_ACCESS_SIG` writer"]
pub struct W(crate::W<SAFE_ACCESS_SIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAFE_ACCESS_SIG_SPEC>;
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
impl From<crate::W<SAFE_ACCESS_SIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAFE_ACCESS_SIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_SAFE_ACC_MODE` reader - current safe accessing mode"]
pub struct RB_SAFE_ACC_MODE_R(crate::FieldReader<u8, u8>);
impl RB_SAFE_ACC_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_SAFE_ACC_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SAFE_ACC_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SAFE_ACC_MODE` writer - current safe accessing mode"]
pub struct RB_SAFE_ACC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SAFE_ACC_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u8 & 3);
        self.w
    }
}
#[doc = "Field `RB_SAFE_ACC_TIMER` reader - safe accessing timer bit mask"]
pub struct RB_SAFE_ACC_TIMER_R(crate::FieldReader<u8, u8>);
impl RB_SAFE_ACC_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RB_SAFE_ACC_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_SAFE_ACC_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB_SAFE_ACC_TIMER` writer - safe accessing timer bit mask"]
pub struct RB_SAFE_ACC_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_SAFE_ACC_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u8 & 7) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - current safe accessing mode"]
    #[inline(always)]
    pub fn rb_safe_acc_mode(&self) -> RB_SAFE_ACC_MODE_R {
        RB_SAFE_ACC_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - safe accessing timer bit mask"]
    #[inline(always)]
    pub fn rb_safe_acc_timer(&self) -> RB_SAFE_ACC_TIMER_R {
        RB_SAFE_ACC_TIMER_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - current safe accessing mode"]
    #[inline(always)]
    pub fn rb_safe_acc_mode(&mut self) -> RB_SAFE_ACC_MODE_W {
        RB_SAFE_ACC_MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - safe accessing timer bit mask"]
    #[inline(always)]
    pub fn rb_safe_acc_timer(&mut self) -> RB_SAFE_ACC_TIMER_W {
        RB_SAFE_ACC_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "safe accessing sign register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [safe_access_sig](index.html) module"]
pub struct SAFE_ACCESS_SIG_SPEC;
impl crate::RegisterSpec for SAFE_ACCESS_SIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [safe_access_sig::R](R) reader structure"]
impl crate::Readable for SAFE_ACCESS_SIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [safe_access_sig::W](W) writer structure"]
impl crate::Writable for SAFE_ACCESS_SIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAFE_ACCESS_SIG to value 0"]
impl crate::Resettable for SAFE_ACCESS_SIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

use super::{FontFamilies, TextAlign, TextDirection, TextStyle};
use crate::interop::{AsStr, FromStrs, SetStr};
use crate::prelude::*;
use crate::{interop, scalar, FontStyle};
use skia_bindings as sb;
use std::slice;

pub type StrutStyle = Handle<sb::skia_textlayout_StrutStyle>;

impl NativeDrop for sb::skia_textlayout_StrutStyle {
    fn drop(&mut self) {
        unsafe { sb::C_StrutStyle_destruct(self) }
    }
}

impl NativeClone for sb::skia_textlayout_StrutStyle {
    fn clone(&self) -> Self {
        construct(|ss| unsafe { sb::C_StrutStyle_CopyConstruct(ss, self) })
    }
}

impl NativePartialEq for sb::skia_textlayout_StrutStyle {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_StrutStyle_equals(self, rhs) }
    }
}

impl Default for Handle<sb::skia_textlayout_StrutStyle> {
    fn default() -> Self {
        Self::new()
    }
}

impl Handle<sb::skia_textlayout_StrutStyle> {
    pub fn new() -> Self {
        StrutStyle::from_native(unsafe { sb::skia_textlayout_StrutStyle::new() })
    }

    pub fn font_families(&self) -> FontFamilies {
        unsafe {
            let mut count = 0;
            let ptr = sb::C_StrutStyle_getFontFamilies(self.native(), &mut count);
            FontFamilies(slice::from_raw_parts(ptr, count))
        }
    }

    pub fn set_font_families(&mut self, families: &[impl AsRef<str>]) -> &mut Self {
        let families: Vec<interop::String> = FromStrs::from_strs(families);
        let families = families.native();
        unsafe {
            sb::C_StrutStyle_setFontFamilies(self.native_mut(), families.as_ptr(), families.len());
        }
        self
    }

    pub fn font_style(&self) -> FontStyle {
        FontStyle::from_native(self.native().fFontStyle)
    }

    pub fn set_font_style(&mut self, font_style: FontStyle) -> &mut Self {
        self.native_mut().fFontStyle = font_style.into_native();
        self
    }

    pub fn font_size(&self) -> scalar {
        self.native().fFontSize
    }

    pub fn set_font_size(&mut self, font_size: scalar) -> &mut Self {
        self.native_mut().fFontSize = font_size;
        self
    }

    pub fn set_height(&mut self, height: scalar) -> &mut Self {
        self.native_mut().fHeight = height;
        self
    }

    pub fn height(&self) -> scalar {
        self.native().fHeight
    }

    pub fn set_leading(&mut self, leading: scalar) -> &mut Self {
        self.native_mut().fLeading = leading;
        self
    }

    pub fn leading(&self) -> scalar {
        self.native().fLeading
    }

    pub fn strut_enabled(&self) -> bool {
        self.native().fEnabled
    }

    pub fn set_strut_enabled(&mut self, enabled: bool) -> &mut Self {
        self.native_mut().fEnabled = enabled;
        self
    }

    pub fn force_strut_height(&self) -> bool {
        self.native().fForceHeight
    }

    pub fn set_force_strut_height(&mut self, force_height: bool) -> &mut Self {
        self.native_mut().fForceHeight = force_height;
        self
    }
}

pub type ParagraphStyle = Handle<sb::skia_textlayout_ParagraphStyle>;

impl NativeDrop for sb::skia_textlayout_ParagraphStyle {
    fn drop(&mut self) {
        unsafe { sb::C_ParagraphStyle_destruct(self) }
    }
}

impl NativeClone for sb::skia_textlayout_ParagraphStyle {
    fn clone(&self) -> Self {
        construct(|ps| unsafe { sb::C_ParagraphStyle_CopyConstruct(ps, self) })
    }
}

impl NativePartialEq for sb::skia_textlayout_ParagraphStyle {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { sb::C_ParagraphStyle_Equals(self, rhs) }
    }
}

impl Default for Handle<sb::skia_textlayout_ParagraphStyle> {
    fn default() -> Self {
        Self::new()
    }
}

impl Handle<sb::skia_textlayout_ParagraphStyle> {
    pub fn new() -> Self {
        Self::from_native(unsafe { sb::skia_textlayout_ParagraphStyle::new() })
    }

    pub fn strut_style(&self) -> &StrutStyle {
        StrutStyle::from_native_ref(&self.native().fStrutStyle)
    }

    pub fn set_strut_style(&mut self, strut_style: StrutStyle) -> &mut Self {
        self.native_mut().fStrutStyle.replace_with(strut_style);
        self
    }

    pub fn text_style(&self) -> &TextStyle {
        TextStyle::from_native_ref(&self.native().fDefaultTextStyle)
    }

    pub fn set_text_style(&mut self, text_style: &TextStyle) -> &mut Self {
        // TODO: implement the assignment operator in C.
        self.native_mut()
            .fDefaultTextStyle
            .replace_with(text_style.clone());
        self
    }

    pub fn text_direction(&self) -> TextDirection {
        self.native().fTextDirection
    }

    pub fn set_text_direction(&mut self, direction: TextDirection) -> &mut Self {
        self.native_mut().fTextDirection = direction;
        self
    }

    pub fn text_align(&self) -> TextAlign {
        self.native().fTextAlign
    }

    pub fn set_text_align(&mut self, align: TextAlign) -> &mut Self {
        self.native_mut().fTextAlign = align;
        self
    }

    pub fn max_lines(&self) -> Option<usize> {
        match self.native().fLinesLimit {
            std::usize::MAX => None,
            lines => Some(lines),
        }
    }

    pub fn set_max_lines(&mut self, lines: impl Into<Option<usize>>) -> &mut Self {
        self.native_mut().fLinesLimit = lines.into().unwrap_or(usize::max_value());
        self
    }

    pub fn ellipsis(&self) -> &str {
        self.native().fEllipsis.as_str()
    }

    pub fn set_ellipsis(&mut self, ellipsis: impl AsRef<str>) -> &mut Self {
        self.native_mut().fEllipsis.set_str(ellipsis);
        self
    }

    pub fn height(&self) -> scalar {
        self.native().fHeight
    }

    pub fn set_height(&mut self, height: scalar) -> &mut Self {
        self.native_mut().fHeight = height;
        self
    }

    pub fn unlimited_lines(&self) -> bool {
        self.max_lines().is_none()
    }

    pub fn ellipsized(&self) -> bool {
        !self.native().fEllipsis.as_str().is_empty()
    }

    pub fn effective_align(&self) -> TextAlign {
        unsafe { self.native().effective_align() }
    }

    pub fn hinting_is_on(&self) -> bool {
        self.native().fHintingIsOn
    }

    pub fn turn_hinting_off(&mut self) -> &mut Self {
        self.native_mut().fHintingIsOn = false;
        self
    }
}

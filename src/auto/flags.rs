// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use StaticType;
use Type;
use ffi;
use gobject_ffi;
use translate::*;
use value::FromValue;
use value::FromValueOptional;
use value::SetValue;
use value::Value;

bitflags! {
    pub struct FileTest: u32 {
        const IS_REGULAR = 1;
        const IS_SYMLINK = 2;
        const IS_DIR = 4;
        const IS_EXECUTABLE = 8;
        const EXISTS = 16;
    }
}

#[doc(hidden)]
impl ToGlib for FileTest {
    type GlibType = ffi::GFileTest;

    fn to_glib(&self) -> ffi::GFileTest {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFileTest> for FileTest {
    fn from_glib(value: ffi::GFileTest) -> FileTest {
        FileTest::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct FormatSizeFlags: u32 {
        const DEFAULT = 0;
        const LONG_FORMAT = 1;
        const IEC_UNITS = 2;
    }
}

#[doc(hidden)]
impl ToGlib for FormatSizeFlags {
    type GlibType = ffi::GFormatSizeFlags;

    fn to_glib(&self) -> ffi::GFormatSizeFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFormatSizeFlags> for FormatSizeFlags {
    fn from_glib(value: ffi::GFormatSizeFlags) -> FormatSizeFlags {
        FormatSizeFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct IOCondition: u32 {
        const IN = 1;
        const OUT = 4;
        const PRI = 2;
        const ERR = 8;
        const HUP = 16;
        const NVAL = 32;
    }
}

#[doc(hidden)]
impl ToGlib for IOCondition {
    type GlibType = ffi::GIOCondition;

    fn to_glib(&self) -> ffi::GIOCondition {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GIOCondition> for IOCondition {
    fn from_glib(value: ffi::GIOCondition) -> IOCondition {
        IOCondition::from_bits_truncate(value)
    }
}

impl StaticType for IOCondition {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_io_condition_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for IOCondition {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for IOCondition {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for IOCondition {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct KeyFileFlags: u32 {
        const NONE = 0;
        const KEEP_COMMENTS = 1;
        const KEEP_TRANSLATIONS = 2;
    }
}

#[doc(hidden)]
impl ToGlib for KeyFileFlags {
    type GlibType = ffi::GKeyFileFlags;

    fn to_glib(&self) -> ffi::GKeyFileFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GKeyFileFlags> for KeyFileFlags {
    fn from_glib(value: ffi::GKeyFileFlags) -> KeyFileFlags {
        KeyFileFlags::from_bits_truncate(value)
    }
}


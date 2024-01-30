// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StringRef(Shared<ffi::JSStringRef>);

    match fn {
        ref => |ptr| ffi::JSStringRetain(*ptr),
        unref => |ptr| ffi::JSStringRelease(*ptr),
    }
}

impl StringRef {
  #[doc(alias = "JSStringGetMaximumUTF8CStringSize")]
  pub fn maximum_utf8_cstring_size(&self) -> usize {
    unsafe { ffi::JSStringGetMaximumUTF8CStringSize(*self.to_glib_none().0) }
  }

  // #[doc(alias = "JSStringGetUTF8CString")]
  // pub fn getUTF8CString(&self, buffer: glib::GString, buffer_size: usize) -> usize {
  //    unsafe { ffi::JSStringGetUTF8CString(*self.to_glib_none().0, buffer.to_glib_f) }
  // }
}
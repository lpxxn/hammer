// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Adjustment;
use crate::Buildable;
use crate::Orientable;
use crate::ScrollType;
use crate::SensitivityType;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkRange")]
    pub struct Range(Object<ffi::GtkRange, ffi::GtkRangeClass>) @extends Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_range_get_type(),
    }
}

impl Range {
    pub const NONE: Option<&'static Range> = None;
}

pub trait RangeExt: 'static {
    #[doc(alias = "gtk_range_get_adjustment")]
    #[doc(alias = "get_adjustment")]
    fn adjustment(&self) -> Adjustment;

    #[doc(alias = "gtk_range_get_fill_level")]
    #[doc(alias = "get_fill_level")]
    fn fill_level(&self) -> f64;

    #[doc(alias = "gtk_range_get_flippable")]
    #[doc(alias = "get_flippable")]
    fn is_flippable(&self) -> bool;

    #[doc(alias = "gtk_range_get_inverted")]
    #[doc(alias = "get_inverted")]
    fn is_inverted(&self) -> bool;

    #[doc(alias = "gtk_range_get_lower_stepper_sensitivity")]
    #[doc(alias = "get_lower_stepper_sensitivity")]
    fn lower_stepper_sensitivity(&self) -> SensitivityType;

    #[cfg_attr(feature = "v3_20", deprecated = "Since 3.20")]
    #[doc(alias = "gtk_range_get_min_slider_size")]
    #[doc(alias = "get_min_slider_size")]
    fn min_slider_size(&self) -> i32;

    #[doc(alias = "gtk_range_get_range_rect")]
    #[doc(alias = "get_range_rect")]
    fn range_rect(&self) -> gdk::Rectangle;

    #[doc(alias = "gtk_range_get_restrict_to_fill_level")]
    #[doc(alias = "get_restrict_to_fill_level")]
    fn restricts_to_fill_level(&self) -> bool;

    #[doc(alias = "gtk_range_get_round_digits")]
    #[doc(alias = "get_round_digits")]
    fn round_digits(&self) -> i32;

    #[doc(alias = "gtk_range_get_show_fill_level")]
    #[doc(alias = "get_show_fill_level")]
    fn shows_fill_level(&self) -> bool;

    #[doc(alias = "gtk_range_get_slider_range")]
    #[doc(alias = "get_slider_range")]
    fn slider_range(&self) -> (i32, i32);

    #[doc(alias = "gtk_range_get_slider_size_fixed")]
    #[doc(alias = "get_slider_size_fixed")]
    fn is_slider_size_fixed(&self) -> bool;

    #[doc(alias = "gtk_range_get_upper_stepper_sensitivity")]
    #[doc(alias = "get_upper_stepper_sensitivity")]
    fn upper_stepper_sensitivity(&self) -> SensitivityType;

    #[doc(alias = "gtk_range_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> f64;

    #[doc(alias = "gtk_range_set_adjustment")]
    fn set_adjustment(&self, adjustment: &impl IsA<Adjustment>);

    #[doc(alias = "gtk_range_set_fill_level")]
    fn set_fill_level(&self, fill_level: f64);

    #[doc(alias = "gtk_range_set_flippable")]
    fn set_flippable(&self, flippable: bool);

    #[doc(alias = "gtk_range_set_increments")]
    fn set_increments(&self, step: f64, page: f64);

    #[doc(alias = "gtk_range_set_inverted")]
    fn set_inverted(&self, setting: bool);

    #[doc(alias = "gtk_range_set_lower_stepper_sensitivity")]
    fn set_lower_stepper_sensitivity(&self, sensitivity: SensitivityType);

    #[cfg_attr(feature = "v3_20", deprecated = "Since 3.20")]
    #[doc(alias = "gtk_range_set_min_slider_size")]
    fn set_min_slider_size(&self, min_size: i32);

    #[doc(alias = "gtk_range_set_range")]
    fn set_range(&self, min: f64, max: f64);

    #[doc(alias = "gtk_range_set_restrict_to_fill_level")]
    fn set_restrict_to_fill_level(&self, restrict_to_fill_level: bool);

    #[doc(alias = "gtk_range_set_round_digits")]
    fn set_round_digits(&self, round_digits: i32);

    #[doc(alias = "gtk_range_set_show_fill_level")]
    fn set_show_fill_level(&self, show_fill_level: bool);

    #[doc(alias = "gtk_range_set_slider_size_fixed")]
    fn set_slider_size_fixed(&self, size_fixed: bool);

    #[doc(alias = "gtk_range_set_upper_stepper_sensitivity")]
    fn set_upper_stepper_sensitivity(&self, sensitivity: SensitivityType);

    #[doc(alias = "gtk_range_set_value")]
    fn set_value(&self, value: f64);

    #[doc(alias = "adjust-bounds")]
    fn connect_adjust_bounds<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "change-value")]
    fn connect_change_value<F: Fn(&Self, ScrollType, f64) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "move-slider")]
    fn connect_move_slider<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_slider(&self, step: ScrollType);

    #[doc(alias = "value-changed")]
    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "adjustment")]
    fn connect_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "fill-level")]
    fn connect_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "inverted")]
    fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "lower-stepper-sensitivity")]
    fn connect_lower_stepper_sensitivity_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "restrict-to-fill-level")]
    fn connect_restrict_to_fill_level_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "round-digits")]
    fn connect_round_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-fill-level")]
    fn connect_show_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "upper-stepper-sensitivity")]
    fn connect_upper_stepper_sensitivity_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Range>> RangeExt for O {
    fn adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_range_get_adjustment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn fill_level(&self) -> f64 {
        unsafe { ffi::gtk_range_get_fill_level(self.as_ref().to_glib_none().0) }
    }

    fn is_flippable(&self) -> bool {
        unsafe { from_glib(ffi::gtk_range_get_flippable(self.as_ref().to_glib_none().0)) }
    }

    fn is_inverted(&self) -> bool {
        unsafe { from_glib(ffi::gtk_range_get_inverted(self.as_ref().to_glib_none().0)) }
    }

    fn lower_stepper_sensitivity(&self) -> SensitivityType {
        unsafe {
            from_glib(ffi::gtk_range_get_lower_stepper_sensitivity(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn min_slider_size(&self) -> i32 {
        unsafe { ffi::gtk_range_get_min_slider_size(self.as_ref().to_glib_none().0) }
    }

    fn range_rect(&self) -> gdk::Rectangle {
        unsafe {
            let mut range_rect = gdk::Rectangle::uninitialized();
            ffi::gtk_range_get_range_rect(
                self.as_ref().to_glib_none().0,
                range_rect.to_glib_none_mut().0,
            );
            range_rect
        }
    }

    fn restricts_to_fill_level(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_restrict_to_fill_level(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn round_digits(&self) -> i32 {
        unsafe { ffi::gtk_range_get_round_digits(self.as_ref().to_glib_none().0) }
    }

    fn shows_fill_level(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_show_fill_level(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn slider_range(&self) -> (i32, i32) {
        unsafe {
            let mut slider_start = mem::MaybeUninit::uninit();
            let mut slider_end = mem::MaybeUninit::uninit();
            ffi::gtk_range_get_slider_range(
                self.as_ref().to_glib_none().0,
                slider_start.as_mut_ptr(),
                slider_end.as_mut_ptr(),
            );
            let slider_start = slider_start.assume_init();
            let slider_end = slider_end.assume_init();
            (slider_start, slider_end)
        }
    }

    fn is_slider_size_fixed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_range_get_slider_size_fixed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn upper_stepper_sensitivity(&self) -> SensitivityType {
        unsafe {
            from_glib(ffi::gtk_range_get_upper_stepper_sensitivity(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn value(&self) -> f64 {
        unsafe { ffi::gtk_range_get_value(self.as_ref().to_glib_none().0) }
    }

    fn set_adjustment(&self, adjustment: &impl IsA<Adjustment>) {
        unsafe {
            ffi::gtk_range_set_adjustment(
                self.as_ref().to_glib_none().0,
                adjustment.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_fill_level(&self, fill_level: f64) {
        unsafe {
            ffi::gtk_range_set_fill_level(self.as_ref().to_glib_none().0, fill_level);
        }
    }

    fn set_flippable(&self, flippable: bool) {
        unsafe {
            ffi::gtk_range_set_flippable(self.as_ref().to_glib_none().0, flippable.into_glib());
        }
    }

    fn set_increments(&self, step: f64, page: f64) {
        unsafe {
            ffi::gtk_range_set_increments(self.as_ref().to_glib_none().0, step, page);
        }
    }

    fn set_inverted(&self, setting: bool) {
        unsafe {
            ffi::gtk_range_set_inverted(self.as_ref().to_glib_none().0, setting.into_glib());
        }
    }

    fn set_lower_stepper_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            ffi::gtk_range_set_lower_stepper_sensitivity(
                self.as_ref().to_glib_none().0,
                sensitivity.into_glib(),
            );
        }
    }

    fn set_min_slider_size(&self, min_size: i32) {
        unsafe {
            ffi::gtk_range_set_min_slider_size(self.as_ref().to_glib_none().0, min_size);
        }
    }

    fn set_range(&self, min: f64, max: f64) {
        unsafe {
            ffi::gtk_range_set_range(self.as_ref().to_glib_none().0, min, max);
        }
    }

    fn set_restrict_to_fill_level(&self, restrict_to_fill_level: bool) {
        unsafe {
            ffi::gtk_range_set_restrict_to_fill_level(
                self.as_ref().to_glib_none().0,
                restrict_to_fill_level.into_glib(),
            );
        }
    }

    fn set_round_digits(&self, round_digits: i32) {
        unsafe {
            ffi::gtk_range_set_round_digits(self.as_ref().to_glib_none().0, round_digits);
        }
    }

    fn set_show_fill_level(&self, show_fill_level: bool) {
        unsafe {
            ffi::gtk_range_set_show_fill_level(
                self.as_ref().to_glib_none().0,
                show_fill_level.into_glib(),
            );
        }
    }

    fn set_slider_size_fixed(&self, size_fixed: bool) {
        unsafe {
            ffi::gtk_range_set_slider_size_fixed(
                self.as_ref().to_glib_none().0,
                size_fixed.into_glib(),
            );
        }
    }

    fn set_upper_stepper_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            ffi::gtk_range_set_upper_stepper_sensitivity(
                self.as_ref().to_glib_none().0,
                sensitivity.into_glib(),
            );
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_range_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn connect_adjust_bounds<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn adjust_bounds_trampoline<P: IsA<Range>, F: Fn(&P, f64) + 'static>(
            this: *mut ffi::GtkRange,
            value: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Range::from_glib_borrow(this).unsafe_cast_ref(), value)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"adjust-bounds\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    adjust_bounds_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_change_value<F: Fn(&Self, ScrollType, f64) -> glib::signal::Inhibit + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn change_value_trampoline<
            P: IsA<Range>,
            F: Fn(&P, ScrollType, f64) -> glib::signal::Inhibit + 'static,
        >(
            this: *mut ffi::GtkRange,
            scroll: ffi::GtkScrollType,
            value: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                Range::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(scroll),
                value,
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"change-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    change_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_move_slider<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn move_slider_trampoline<
            P: IsA<Range>,
            F: Fn(&P, ScrollType) + 'static,
        >(
            this: *mut ffi::GtkRange,
            step: ffi::GtkScrollType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Range::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(step),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-slider\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_slider_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_slider(&self, step: ScrollType) {
        self.emit_by_name::<()>("move-slider", &[&step]);
    }

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn value_changed_trampoline<P: IsA<Range>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkRange,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"value-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    value_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adjustment_trampoline<P: IsA<Range>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkRange,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_adjustment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fill_level_trampoline<P: IsA<Range>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkRange,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::fill-level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fill_level_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_inverted_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<P: IsA<Range>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkRange,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_lower_stepper_sensitivity_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_lower_stepper_sensitivity_trampoline<
            P: IsA<Range>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRange,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::lower-stepper-sensitivity\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_lower_stepper_sensitivity_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_restrict_to_fill_level_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_restrict_to_fill_level_trampoline<
            P: IsA<Range>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRange,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::restrict-to-fill-level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_restrict_to_fill_level_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_round_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_round_digits_trampoline<P: IsA<Range>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkRange,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::round-digits\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_round_digits_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_fill_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_fill_level_trampoline<
            P: IsA<Range>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRange,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-fill-level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_fill_level_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_upper_stepper_sensitivity_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_upper_stepper_sensitivity_trampoline<
            P: IsA<Range>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkRange,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Range::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::upper-stepper-sensitivity\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_upper_stepper_sensitivity_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Range")
    }
}
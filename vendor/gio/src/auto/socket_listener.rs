// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::Socket;
use crate::SocketAddress;
use crate::SocketConnection;
use crate::SocketListenerEvent;
use crate::SocketProtocol;
use crate::SocketType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GSocketListener")]
    pub struct SocketListener(Object<ffi::GSocketListener, ffi::GSocketListenerClass>);

    match fn {
        type_ => || ffi::g_socket_listener_get_type(),
    }
}

impl SocketListener {
    pub const NONE: Option<&'static SocketListener> = None;

    #[doc(alias = "g_socket_listener_new")]
    pub fn new() -> SocketListener {
        unsafe { from_glib_full(ffi::g_socket_listener_new()) }
    }
}

impl Default for SocketListener {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SocketListenerExt: 'static {
    #[doc(alias = "g_socket_listener_accept")]
    fn accept(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(SocketConnection, Option<glib::Object>), glib::Error>;

    #[doc(alias = "g_socket_listener_accept_async")]
    fn accept_async<
        P: FnOnce(Result<(SocketConnection, Option<glib::Object>), glib::Error>) + 'static,
    >(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn accept_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(SocketConnection, Option<glib::Object>), glib::Error>,
                > + 'static,
        >,
    >;

    #[doc(alias = "g_socket_listener_accept_socket")]
    fn accept_socket(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(Socket, Option<glib::Object>), glib::Error>;

    #[doc(alias = "g_socket_listener_accept_socket_async")]
    fn accept_socket_async<
        P: FnOnce(Result<(Socket, Option<glib::Object>), glib::Error>) + 'static,
    >(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn accept_socket_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(Socket, Option<glib::Object>), glib::Error>>
                + 'static,
        >,
    >;

    #[doc(alias = "g_socket_listener_add_address")]
    fn add_address(
        &self,
        address: &impl IsA<SocketAddress>,
        type_: SocketType,
        protocol: SocketProtocol,
        source_object: Option<&impl IsA<glib::Object>>,
    ) -> Result<SocketAddress, glib::Error>;

    #[doc(alias = "g_socket_listener_add_any_inet_port")]
    fn add_any_inet_port(
        &self,
        source_object: Option<&impl IsA<glib::Object>>,
    ) -> Result<u16, glib::Error>;

    #[doc(alias = "g_socket_listener_add_inet_port")]
    fn add_inet_port(
        &self,
        port: u16,
        source_object: Option<&impl IsA<glib::Object>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_listener_add_socket")]
    fn add_socket(
        &self,
        socket: &impl IsA<Socket>,
        source_object: Option<&impl IsA<glib::Object>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_socket_listener_close")]
    fn close(&self);

    #[doc(alias = "g_socket_listener_set_backlog")]
    fn set_backlog(&self, listen_backlog: i32);

    #[doc(alias = "listen-backlog")]
    fn listen_backlog(&self) -> i32;

    #[doc(alias = "listen-backlog")]
    fn set_listen_backlog(&self, listen_backlog: i32);

    #[doc(alias = "event")]
    fn connect_event<F: Fn(&Self, SocketListenerEvent, &Socket) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "listen-backlog")]
    fn connect_listen_backlog_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SocketListener>> SocketListenerExt for O {
    fn accept(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(SocketConnection, Option<glib::Object>), glib::Error> {
        unsafe {
            let mut source_object = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_listener_accept(
                self.as_ref().to_glib_none().0,
                &mut source_object,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib_none(source_object)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn accept_async<
        P: FnOnce(Result<(SocketConnection, Option<glib::Object>), glib::Error>) + 'static,
    >(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn accept_async_trampoline<
            P: FnOnce(Result<(SocketConnection, Option<glib::Object>), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut source_object = ptr::null_mut();
            let ret = ffi::g_socket_listener_accept_finish(
                _source_object as *mut _,
                res,
                &mut source_object,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_none(source_object)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = accept_async_trampoline::<P>;
        unsafe {
            ffi::g_socket_listener_accept_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn accept_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<(SocketConnection, Option<glib::Object>), glib::Error>,
                > + 'static,
        >,
    > {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.accept_async(Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn accept_socket(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(Socket, Option<glib::Object>), glib::Error> {
        unsafe {
            let mut source_object = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_listener_accept_socket(
                self.as_ref().to_glib_none().0,
                &mut source_object,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib_none(source_object)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn accept_socket_async<
        P: FnOnce(Result<(Socket, Option<glib::Object>), glib::Error>) + 'static,
    >(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn accept_socket_async_trampoline<
            P: FnOnce(Result<(Socket, Option<glib::Object>), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut source_object = ptr::null_mut();
            let ret = ffi::g_socket_listener_accept_socket_finish(
                _source_object as *mut _,
                res,
                &mut source_object,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_none(source_object)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = accept_socket_async_trampoline::<P>;
        unsafe {
            ffi::g_socket_listener_accept_socket_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn accept_socket_future(
        &self,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(Socket, Option<glib::Object>), glib::Error>>
                + 'static,
        >,
    > {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.accept_socket_async(Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn add_address(
        &self,
        address: &impl IsA<SocketAddress>,
        type_: SocketType,
        protocol: SocketProtocol,
        source_object: Option<&impl IsA<glib::Object>>,
    ) -> Result<SocketAddress, glib::Error> {
        unsafe {
            let mut effective_address = ptr::null_mut();
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_socket_listener_add_address(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                type_.into_glib(),
                protocol.into_glib(),
                source_object.map(|p| p.as_ref()).to_glib_none().0,
                &mut effective_address,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(from_glib_full(effective_address))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_any_inet_port(
        &self,
        source_object: Option<&impl IsA<glib::Object>>,
    ) -> Result<u16, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_listener_add_any_inet_port(
                self.as_ref().to_glib_none().0,
                source_object.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_inet_port(
        &self,
        port: u16,
        source_object: Option<&impl IsA<glib::Object>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_socket_listener_add_inet_port(
                self.as_ref().to_glib_none().0,
                port,
                source_object.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_socket(
        &self,
        socket: &impl IsA<Socket>,
        source_object: Option<&impl IsA<glib::Object>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_socket_listener_add_socket(
                self.as_ref().to_glib_none().0,
                socket.as_ref().to_glib_none().0,
                source_object.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close(&self) {
        unsafe {
            ffi::g_socket_listener_close(self.as_ref().to_glib_none().0);
        }
    }

    fn set_backlog(&self, listen_backlog: i32) {
        unsafe {
            ffi::g_socket_listener_set_backlog(self.as_ref().to_glib_none().0, listen_backlog);
        }
    }

    fn listen_backlog(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "listen-backlog")
    }

    fn set_listen_backlog(&self, listen_backlog: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "listen-backlog", &listen_backlog)
    }

    fn connect_event<F: Fn(&Self, SocketListenerEvent, &Socket) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<
            P: IsA<SocketListener>,
            F: Fn(&P, SocketListenerEvent, &Socket) + 'static,
        >(
            this: *mut ffi::GSocketListener,
            event: ffi::GSocketListenerEvent,
            socket: *mut ffi::GSocket,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                SocketListener::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(event),
                &from_glib_borrow(socket),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_listen_backlog_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_listen_backlog_trampoline<
            P: IsA<SocketListener>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GSocketListener,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SocketListener::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::listen-backlog\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_listen_backlog_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SocketListener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SocketListener")
    }
}

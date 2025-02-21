// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Connection;
use crate::Object;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use crate::SettingsConnectionFlags;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use crate::SettingsUpdate2Flags;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "NMRemoteConnection")]
    pub struct RemoteConnection(Object<ffi::NMRemoteConnection, ffi::NMRemoteConnectionClass>) @extends Object, @implements Connection;

    match fn {
        type_ => || ffi::nm_remote_connection_get_type(),
    }
}

impl RemoteConnection {
    /// Send any local changes to the settings and properties of `self` to
    /// NetworkManager. If `save_to_disk` is [`true`], the updated connection will be saved to
    /// disk; if [`false`], then only the in-memory representation will be changed.
    ///
    /// # Deprecated since 1.22
    ///
    /// Use [`commit_changes_async()`][Self::commit_changes_async()] or GDBusConnection.
    /// ## `save_to_disk`
    /// whether to persist the changes to disk
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    ///
    /// # Returns
    ///
    /// [`true`] on success, [`false`] on error, in which case `error` will be set.
    #[cfg_attr(feature = "v1_22", deprecated = "Since 1.22")]
    #[doc(alias = "nm_remote_connection_commit_changes")]
    pub fn commit_changes(
        &self,
        save_to_disk: bool,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_remote_connection_commit_changes(
                self.to_glib_none().0,
                save_to_disk.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
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

    /// Asynchronously sends any local changes to the settings and properties of
    /// `self` to NetworkManager. If `save` is [`true`], the updated connection will
    /// be saved to disk; if [`false`], then only the in-memory representation will be
    /// changed.
    /// ## `save_to_disk`
    /// whether to save the changes to persistent storage
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    /// ## `callback`
    /// callback to be called when the commit operation completes
    #[doc(alias = "nm_remote_connection_commit_changes_async")]
    pub fn commit_changes_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        save_to_disk: bool,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
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
        unsafe extern "C" fn commit_changes_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_remote_connection_commit_changes_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = commit_changes_async_trampoline::<P>;
        unsafe {
            ffi::nm_remote_connection_commit_changes_async(
                self.to_glib_none().0,
                save_to_disk.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn commit_changes_future(
        &self,
        save_to_disk: bool,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.commit_changes_async(save_to_disk, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    /// Deletes the connection.
    ///
    /// # Deprecated since 1.22
    ///
    /// Use [`delete_async()`][Self::delete_async()] or GDBusConnection.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    ///
    /// # Returns
    ///
    /// [`true`] on success, [`false`] on error, in which case `error` will be set.
    #[cfg_attr(feature = "v1_22", deprecated = "Since 1.22")]
    #[doc(alias = "nm_remote_connection_delete")]
    pub fn delete(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_remote_connection_delete(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
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

    /// Asynchronously deletes the connection.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    /// ## `callback`
    /// callback to be called when the delete operation completes
    #[doc(alias = "nm_remote_connection_delete_async")]
    pub fn delete_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
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
        unsafe extern "C" fn delete_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::nm_remote_connection_delete_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = delete_async_trampoline::<P>;
        unsafe {
            ffi::nm_remote_connection_delete_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn delete_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.delete_async(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    ///
    /// # Returns
    ///
    /// file that stores the connection in case the connection is file-backed.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_remote_connection_get_filename")]
    #[doc(alias = "get_filename")]
    pub fn filename(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_remote_connection_get_filename(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the flags of the connection of type [`SettingsConnectionFlags`][crate::SettingsConnectionFlags].
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_remote_connection_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> SettingsConnectionFlags {
        unsafe { from_glib(ffi::nm_remote_connection_get_flags(self.to_glib_none().0)) }
    }

    /// Request the connection's secrets. Note that this is a blocking D-Bus call,
    /// not a simple property accessor.
    ///
    /// # Deprecated since 1.22
    ///
    /// Use [`secrets_async()`][Self::secrets_async()] or GDBusConnection.
    /// ## `setting_name`
    /// the [`Setting`][crate::Setting] object name to get secrets for
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    ///
    /// # Returns
    ///
    /// a [`glib::Variant`][struct@crate::glib::Variant] of type `NM_VARIANT_TYPE_CONNECTION` containing
    /// `self`'s secrets, or [`None`] on error.
    #[cfg_attr(feature = "v1_22", deprecated = "Since 1.22")]
    #[doc(alias = "nm_remote_connection_get_secrets")]
    #[doc(alias = "get_secrets")]
    pub fn secrets(
        &self,
        setting_name: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<glib::Variant, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_remote_connection_get_secrets(
                self.to_glib_none().0,
                setting_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    /// Asynchronously requests the connection's secrets.
    /// ## `setting_name`
    /// the [`Setting`][crate::Setting] object name to get secrets for
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    /// ## `callback`
    /// callback to be called when the secret request completes
    #[doc(alias = "nm_remote_connection_get_secrets_async")]
    #[doc(alias = "get_secrets_async")]
    pub fn secrets_async<P: FnOnce(Result<glib::Variant, glib::Error>) + 'static>(
        &self,
        setting_name: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
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
        unsafe extern "C" fn secrets_async_trampoline<
            P: FnOnce(Result<glib::Variant, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_remote_connection_get_secrets_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = secrets_async_trampoline::<P>;
        unsafe {
            ffi::nm_remote_connection_get_secrets_async(
                self.to_glib_none().0,
                setting_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn secrets_future(
        &self,
        setting_name: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Variant, glib::Error>> + 'static>>
    {
        let setting_name = String::from(setting_name);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.secrets_async(&setting_name, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    ///
    /// # Returns
    ///
    /// [`true`] if the remote connection contains changes that have not
    /// been saved to disk, [`false`] if the connection is the same as its on-disk
    /// representation.
    #[doc(alias = "nm_remote_connection_get_unsaved")]
    #[doc(alias = "get_unsaved")]
    pub fn is_unsaved(&self) -> bool {
        unsafe { from_glib(ffi::nm_remote_connection_get_unsaved(self.to_glib_none().0)) }
    }

    /// Checks if the connection is visible to the current user. If the
    /// connection is not visible then it is essentially useless; it will
    /// not contain any settings, and operations such as
    /// [`save()`][Self::save()] and [`delete()`][Self::delete()] will
    /// always fail. (`NMRemoteSettings` will not normally return
    /// non-visible connections to callers, but it is possible for a
    /// connection's visibility to change after you already have a
    /// reference to it.)
    ///
    /// # Returns
    ///
    /// [`true`] if the remote connection is visible to the current
    /// user, [`false`] if not.
    #[doc(alias = "nm_remote_connection_get_visible")]
    #[doc(alias = "get_visible")]
    pub fn is_visible(&self) -> bool {
        unsafe { from_glib(ffi::nm_remote_connection_get_visible(self.to_glib_none().0)) }
    }

    /// Saves the connection to disk if the connection has changes that have not yet
    /// been written to disk, or if the connection has never been saved.
    ///
    /// # Deprecated since 1.22
    ///
    /// Use [`save_async()`][Self::save_async()] or GDBusConnection.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    ///
    /// # Returns
    ///
    /// [`true`] on success, [`false`] on error, in which case `error` will be set.
    #[cfg_attr(feature = "v1_22", deprecated = "Since 1.22")]
    #[doc(alias = "nm_remote_connection_save")]
    pub fn save(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_remote_connection_save(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
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

    /// Saves the connection to disk if the connection has changes that have not yet
    /// been written to disk, or if the connection has never been saved.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    /// ## `callback`
    /// callback to be called when the save operation completes
    #[doc(alias = "nm_remote_connection_save_async")]
    pub fn save_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
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
        unsafe extern "C" fn save_async_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::nm_remote_connection_save_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = save_async_trampoline::<P>;
        unsafe {
            ffi::nm_remote_connection_save_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn save_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.save_async(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    /// Asynchronously calls the Update2() D-Bus method.
    /// ## `settings`
    /// optional connection to update the settings.
    /// ## `flags`
    /// update-flags
    /// ## `args`
    /// optional arguments.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    /// ## `callback`
    /// callback to be called when the commit operation completes
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_remote_connection_update2")]
    pub fn update2<P: FnOnce(Result<glib::Variant, glib::Error>) + 'static>(
        &self,
        settings: Option<&glib::Variant>,
        flags: SettingsUpdate2Flags,
        args: Option<&glib::Variant>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
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
        unsafe extern "C" fn update2_trampoline<
            P: FnOnce(Result<glib::Variant, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::nm_remote_connection_update2_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = update2_trampoline::<P>;
        unsafe {
            ffi::nm_remote_connection_update2(
                self.to_glib_none().0,
                settings.to_glib_none().0,
                flags.into_glib(),
                args.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    pub fn update2_future(
        &self,
        settings: Option<&glib::Variant>,
        flags: SettingsUpdate2Flags,
        args: Option<&glib::Variant>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Variant, glib::Error>> + 'static>>
    {
        let settings = settings.map(ToOwned::to_owned);
        let args = args.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.update2(
                settings.as_ref().map(::std::borrow::Borrow::borrow),
                flags,
                args.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "filename")]
    pub fn connect_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filename_trampoline<F: Fn(&RemoteConnection) + 'static>(
            this: *mut ffi::NMRemoteConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filename\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filename_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "flags")]
    pub fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&RemoteConnection) + 'static>(
            this: *mut ffi::NMRemoteConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "unsaved")]
    pub fn connect_unsaved_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_unsaved_trampoline<F: Fn(&RemoteConnection) + 'static>(
            this: *mut ffi::NMRemoteConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::unsaved\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_unsaved_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible")]
    pub fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<F: Fn(&RemoteConnection) + 'static>(
            this: *mut ffi::NMRemoteConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for RemoteConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RemoteConnection")
    }
}

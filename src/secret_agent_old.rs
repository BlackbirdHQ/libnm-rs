// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Connection;
use crate::SecretAgentCapabilities;
use crate::SecretAgentGetSecretsFlags;
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
    #[doc(alias = "NMSecretAgentOld")]
    pub struct SecretAgentOld(Object<ffi::NMSecretAgentOld, ffi::NMSecretAgentOldClass>);

    match fn {
        type_ => || ffi::nm_secret_agent_old_get_type(),
    }
}

impl SecretAgentOld {
    pub const NONE: Option<&'static SecretAgentOld> = None;
}

/// Trait containing all [`struct@SecretAgentOld`] methods.
///
/// # Implementors
///
/// [`SecretAgentOld`][struct@crate::SecretAgentOld]
pub trait SecretAgentOldExt: 'static {
    /// Asynchronously asks the agent to delete all saved secrets belonging to
    /// `connection`.
    /// ## `connection`
    /// a [`Connection`][crate::Connection]
    /// ## `callback`
    /// a callback, to be invoked when the operation is done
    #[doc(alias = "nm_secret_agent_old_delete_secrets")]
    fn delete_secrets<P: FnOnce(&SecretAgentOld, &Connection, &glib::Error) + 'static>(
        &self,
        connection: &impl IsA<Connection>,
        callback: P,
    );

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "nm_secret_agent_old_destroy")]
    fn destroy(&self);

    /// This has the same effect as setting [`SECRET_AGENT_OLD_AUTO_REGISTER`][crate::SECRET_AGENT_OLD_AUTO_REGISTER]
    /// property.
    ///
    /// Unlike most other functions, you may already call this function before
    /// initialization completes.
    /// ## `enable`
    /// whether to enable or disable the listener.
    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "nm_secret_agent_old_enable")]
    fn enable(&self, enable: bool);

    /// Returns a [`glib::Object`][crate::glib::Object] that stays alive as long as there are pending
    /// requests in the [`gio::DBusConnection`][crate::gio::DBusConnection]. Such requests keep the `GMainContext`
    /// alive, and thus you may want to keep iterating the context as long
    /// until a weak reference indicates that this object is gone. This is
    /// useful because even when you destroy the instance right away (and all
    /// the internally pending requests get cancelled), any pending [`DBusConnection::call()`][crate::gio::DBusConnection::call()]
    /// requests will still invoke the result on the `GMainContext`. Hence, this
    /// allows you to know how long you must iterate the context to know
    /// that all remains are cleaned up.
    ///
    /// # Returns
    ///
    /// a [`glib::Object`][crate::glib::Object] that you may register a weak pointer
    ///  to know that the `GMainContext` is still kept busy by `self`.
    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "nm_secret_agent_old_get_context_busy_watcher")]
    #[doc(alias = "get_context_busy_watcher")]
    fn context_busy_watcher(&self) -> Option<glib::Object>;

    ///
    /// # Returns
    ///
    /// the [`gio::DBusConnection`][crate::gio::DBusConnection] used by the secret agent.
    ///  You may either set this as construct property [`SECRET_AGENT_OLD_DBUS_CONNECTION`][crate::SECRET_AGENT_OLD_DBUS_CONNECTION],
    ///  or it will automatically set during initialization.
    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "nm_secret_agent_old_get_dbus_connection")]
    #[doc(alias = "get_dbus_connection")]
    fn dbus_connection(&self) -> Option<gio::DBusConnection>;

    ///
    /// # Returns
    ///
    /// the current D-Bus name owner. While this property
    ///  is set while registering, it really only makes sense when
    ///  the [`is_registered()`][Self::is_registered()] indicates that
    ///  registration is successful.
    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    #[doc(alias = "nm_secret_agent_old_get_dbus_name_owner")]
    #[doc(alias = "get_dbus_name_owner")]
    fn dbus_name_owner(&self) -> Option<glib::GString>;

    //#[cfg(any(feature = "v1_24", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    //#[doc(alias = "nm_secret_agent_old_get_main_context")]
    //#[doc(alias = "get_main_context")]
    //fn main_context(&self) -> /*Ignored*/Option<glib::MainContext>;

    /// Note that the secret agent transparently registers and re-registers
    /// as the D-Bus name owner appears. Hence, this property is not really
    /// useful. Also, to be graceful against races during registration, the
    /// instance will already accept requests while being in the process of
    /// registering.
    /// If you need to avoid races and want to wait until `self` is registered,
    /// call [`register_async()`][Self::register_async()]. If that function completes
    /// with success, you know the instance is registered.
    ///
    /// # Returns
    ///
    /// a [`true`] if the agent is registered, [`false`] if it is not.
    #[doc(alias = "nm_secret_agent_old_get_registered")]
    #[doc(alias = "get_registered")]
    fn is_registered(&self) -> bool;

    /// Asynchronously retrieves secrets belonging to `connection` for the
    /// setting `setting_name`. `flags` indicate specific behavior that the secret
    /// agent should use when performing the request, for example returning only
    /// existing secrets without user interaction, or requesting entirely new
    /// secrets from the user.
    /// ## `connection`
    /// the [`Connection`][crate::Connection] for which we're asked secrets
    /// ## `setting_name`
    /// the name of the secret setting
    /// ## `hints`
    /// hints to the agent
    /// ## `flags`
    /// flags that modify the behavior of the request
    /// ## `callback`
    /// a callback, to be invoked when the operation is done
    #[doc(alias = "nm_secret_agent_old_get_secrets")]
    #[doc(alias = "get_secrets")]
    fn secrets<P: FnOnce(&SecretAgentOld, &Connection, &glib::Variant, &glib::Error) + 'static>(
        &self,
        connection: &impl IsA<Connection>,
        setting_name: &str,
        hints: &[&str],
        flags: SecretAgentGetSecretsFlags,
        callback: P,
    );

    /// Registers the [`SecretAgentOld`][crate::SecretAgentOld] with the NetworkManager secret manager,
    /// indicating to NetworkManager that the agent is able to provide and save
    /// secrets for connections on behalf of its user.
    ///
    /// # Deprecated since 1.24
    ///
    /// Use [`enable()`][Self::enable()] or [`register_async()`][Self::register_async()].
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    ///
    /// # Returns
    ///
    /// [`true`] if registration was successful, [`false`] on error.
    ///
    /// Since 1.24, this can no longer fail unless the `cancellable` gets
    /// cancelled. Contrary to [`register_async()`][Self::register_async()], this also
    /// does not wait for the registration to succeed. You cannot synchronously
    /// (without iterating the caller's GMainContext) wait for registration.
    ///
    /// Since 1.24, registration is idempotent. It has the same effect as setting
    /// [`SECRET_AGENT_OLD_AUTO_REGISTER`][crate::SECRET_AGENT_OLD_AUTO_REGISTER] to [`true`] or [`enable()`][Self::enable()].
    #[cfg_attr(feature = "v1_24", deprecated = "Since 1.24")]
    #[doc(alias = "nm_secret_agent_old_register")]
    fn register(&self, cancellable: Option<&impl IsA<gio::Cancellable>>)
        -> Result<(), glib::Error>;

    /// Asynchronously registers the [`SecretAgentOld`][crate::SecretAgentOld] with the NetworkManager secret
    /// manager, indicating to NetworkManager that the agent is able to provide and
    /// save secrets for connections on behalf of its user.
    ///
    /// Since 1.24, registration cannot fail and is idempotent. It has
    /// the same effect as setting [`SECRET_AGENT_OLD_AUTO_REGISTER`][crate::SECRET_AGENT_OLD_AUTO_REGISTER] to [`true`]
    /// or [`enable()`][Self::enable()].
    ///
    /// Since 1.24, the asynchronous result indicates whether the instance is successfully
    /// registered. In any case, this call enables the agent and it will automatically
    /// try to register and handle secret requests. A failure of this function only indicates
    /// that currently the instance might not be ready (but since it will automatically
    /// try to recover, it might be ready in a moment afterwards). Use this function if
    /// you want to check and ensure that the agent is registered.
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    /// ## `callback`
    /// callback to call when the agent is registered
    #[doc(alias = "nm_secret_agent_old_register_async")]
    fn register_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );

    fn register_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    /// Asynchronously ensures that all secrets inside `connection` are stored to
    /// disk.
    /// ## `connection`
    /// a [`Connection`][crate::Connection]
    /// ## `callback`
    /// a callback, to be invoked when the operation is done
    #[doc(alias = "nm_secret_agent_old_save_secrets")]
    fn save_secrets<P: FnOnce(&SecretAgentOld, &Connection, &glib::Error) + 'static>(
        &self,
        connection: &impl IsA<Connection>,
        callback: P,
    );

    /// Unregisters the [`SecretAgentOld`][crate::SecretAgentOld] with the NetworkManager secret manager,
    /// indicating to NetworkManager that the agent will no longer provide or
    /// store secrets on behalf of this user.
    ///
    /// # Deprecated since 1.24
    ///
    /// Use [`enable()`][Self::enable()].
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    ///
    /// # Returns
    ///
    /// [`true`] if unregistration was successful, [`false`] on error
    ///
    /// Since 1.24, registration cannot fail and is idempotent. It has
    /// the same effect as setting [`SECRET_AGENT_OLD_AUTO_REGISTER`][crate::SECRET_AGENT_OLD_AUTO_REGISTER] to [`false`]
    /// or [`enable()`][Self::enable()].
    #[cfg_attr(feature = "v1_24", deprecated = "Since 1.24")]
    #[doc(alias = "nm_secret_agent_old_unregister")]
    fn unregister(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error>;

    /// Asynchronously unregisters the [`SecretAgentOld`][crate::SecretAgentOld] with the NetworkManager secret
    /// manager, indicating to NetworkManager that the agent will no longer provide
    /// or store secrets on behalf of this user.
    ///
    /// Since 1.24, registration cannot fail and is idempotent. It has
    /// the same effect as setting [`SECRET_AGENT_OLD_AUTO_REGISTER`][crate::SECRET_AGENT_OLD_AUTO_REGISTER] to [`false`]
    /// or [`enable()`][Self::enable()].
    ///
    /// # Deprecated since 1.24
    ///
    /// Use [`enable()`][Self::enable()].
    /// ## `cancellable`
    /// a [`gio::Cancellable`][crate::gio::Cancellable], or [`None`]
    /// ## `callback`
    /// callback to call when the agent is unregistered
    #[cfg_attr(feature = "v1_24", deprecated = "Since 1.24")]
    #[doc(alias = "nm_secret_agent_old_unregister_async")]
    fn unregister_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );

    #[cfg_attr(feature = "v1_24", deprecated = "Since 1.24")]

    fn unregister_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    /// If [`true`] (the default), the agent will always be registered when
    /// NetworkManager is running; if NetworkManager exits and restarts, the
    /// agent will re-register itself automatically.
    ///
    /// In particular, if this property is [`true`] at construct time, then the
    /// agent will register itself with NetworkManager during
    /// construction/initialization and initialization will only complete
    /// after registration is completed (either successfully or unsuccessfully).
    /// Since 1.24, a failure to register will no longer cause initialization
    /// of [`SecretAgentOld`][crate::SecretAgentOld] to fail.
    ///
    /// If the property is [`false`], the agent will not automatically register with
    /// NetworkManager, and [`enable()`][Self::enable()] or
    /// [`register_async()`][Self::register_async()] must be called to register it.
    ///
    /// Calling [`enable()`][Self::enable()] has the same effect as setting this
    /// property.
    #[doc(alias = "auto-register")]
    fn is_auto_register(&self) -> bool;

    /// If [`true`] (the default), the agent will always be registered when
    /// NetworkManager is running; if NetworkManager exits and restarts, the
    /// agent will re-register itself automatically.
    ///
    /// In particular, if this property is [`true`] at construct time, then the
    /// agent will register itself with NetworkManager during
    /// construction/initialization and initialization will only complete
    /// after registration is completed (either successfully or unsuccessfully).
    /// Since 1.24, a failure to register will no longer cause initialization
    /// of [`SecretAgentOld`][crate::SecretAgentOld] to fail.
    ///
    /// If the property is [`false`], the agent will not automatically register with
    /// NetworkManager, and [`enable()`][Self::enable()] or
    /// [`register_async()`][Self::register_async()] must be called to register it.
    ///
    /// Calling [`enable()`][Self::enable()] has the same effect as setting this
    /// property.
    #[doc(alias = "auto-register")]
    fn set_auto_register(&self, auto_register: bool);

    /// A bitfield of `NMSecretAgentCapabilities`.
    ///
    /// Changing this property is possible at any time. In case the secret
    /// agent is currently registered, this will cause a re-registration.
    fn capabilities(&self) -> SecretAgentCapabilities;

    /// A bitfield of `NMSecretAgentCapabilities`.
    ///
    /// Changing this property is possible at any time. In case the secret
    /// agent is currently registered, this will cause a re-registration.
    fn set_capabilities(&self, capabilities: SecretAgentCapabilities);

    /// Identifies this agent; only one agent in each user session may use the
    /// same identifier. Identifier formatting follows the same rules as
    /// D-Bus bus names with the exception that the ':' character is not
    /// allowed. The valid set of characters is "[A-Z][a-z][0-9]_-." and the
    /// identifier is limited in length to 255 characters with a minimum
    /// of 3 characters. An example valid identifier is 'org.gnome.nm-applet'
    /// (without quotes).
    fn identifier(&self) -> Option<glib::GString>;

    #[doc(alias = "auto-register")]
    fn connect_auto_register_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "capabilities")]
    fn connect_capabilities_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "registered")]
    fn connect_registered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SecretAgentOld>> SecretAgentOldExt for O {
    fn delete_secrets<P: FnOnce(&SecretAgentOld, &Connection, &glib::Error) + 'static>(
        &self,
        connection: &impl IsA<Connection>,
        callback: P,
    ) {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<
            P: FnOnce(&SecretAgentOld, &Connection, &glib::Error) + 'static,
        >(
            agent: *mut ffi::NMSecretAgentOld,
            connection: *mut ffi::NMConnection,
            error: *mut glib::ffi::GError,
            user_data: glib::ffi::gpointer,
        ) {
            let agent = from_glib_borrow(agent);
            let connection = from_glib_borrow(connection);
            let error = from_glib_borrow(error);
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            (*callback)(&agent, &connection, &error);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::nm_secret_agent_old_delete_secrets(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    fn destroy(&self) {
        unsafe {
            ffi::nm_secret_agent_old_destroy(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    fn enable(&self, enable: bool) {
        unsafe {
            ffi::nm_secret_agent_old_enable(self.as_ref().to_glib_none().0, enable.into_glib());
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    fn context_busy_watcher(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::nm_secret_agent_old_get_context_busy_watcher(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    fn dbus_connection(&self) -> Option<gio::DBusConnection> {
        unsafe {
            from_glib_none(ffi::nm_secret_agent_old_get_dbus_connection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    fn dbus_name_owner(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_secret_agent_old_get_dbus_name_owner(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v1_24", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_24")))]
    //fn main_context(&self) -> /*Ignored*/Option<glib::MainContext> {
    //    unsafe { TODO: call ffi:nm_secret_agent_old_get_main_context() }
    //}

    fn is_registered(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_secret_agent_old_get_registered(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn secrets<P: FnOnce(&SecretAgentOld, &Connection, &glib::Variant, &glib::Error) + 'static>(
        &self,
        connection: &impl IsA<Connection>,
        setting_name: &str,
        hints: &[&str],
        flags: SecretAgentGetSecretsFlags,
        callback: P,
    ) {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<
            P: FnOnce(&SecretAgentOld, &Connection, &glib::Variant, &glib::Error) + 'static,
        >(
            agent: *mut ffi::NMSecretAgentOld,
            connection: *mut ffi::NMConnection,
            secrets: *mut glib::ffi::GVariant,
            error: *mut glib::ffi::GError,
            user_data: glib::ffi::gpointer,
        ) {
            let agent = from_glib_borrow(agent);
            let connection = from_glib_borrow(connection);
            let secrets = from_glib_borrow(secrets);
            let error = from_glib_borrow(error);
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            (*callback)(&agent, &connection, &secrets, &error);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::nm_secret_agent_old_get_secrets(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                setting_name.to_glib_none().0,
                hints.to_glib_none().0,
                flags.into_glib(),
                callback,
                Box_::into_raw(super_callback0) as *mut _,
            );
        }
    }

    fn register(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_secret_agent_old_register(
                self.as_ref().to_glib_none().0,
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

    fn register_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
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
        unsafe extern "C" fn register_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::nm_secret_agent_old_register_finish(_source_object as *mut _, res, &mut error);
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
        let callback = register_async_trampoline::<P>;
        unsafe {
            ffi::nm_secret_agent_old_register_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn register_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.register_async(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    fn save_secrets<P: FnOnce(&SecretAgentOld, &Connection, &glib::Error) + 'static>(
        &self,
        connection: &impl IsA<Connection>,
        callback: P,
    ) {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<
            P: FnOnce(&SecretAgentOld, &Connection, &glib::Error) + 'static,
        >(
            agent: *mut ffi::NMSecretAgentOld,
            connection: *mut ffi::NMConnection,
            error: *mut glib::ffi::GError,
            user_data: glib::ffi::gpointer,
        ) {
            let agent = from_glib_borrow(agent);
            let connection = from_glib_borrow(connection);
            let error = from_glib_borrow(error);
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            (*callback)(&agent, &connection, &error);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::nm_secret_agent_old_save_secrets(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
            );
        }
    }

    fn unregister(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_secret_agent_old_unregister(
                self.as_ref().to_glib_none().0,
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

    fn unregister_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
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
        unsafe extern "C" fn unregister_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_secret_agent_old_unregister_finish(
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
        let callback = unregister_async_trampoline::<P>;
        unsafe {
            ffi::nm_secret_agent_old_unregister_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn unregister_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.unregister_async(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    fn is_auto_register(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "auto-register")
    }

    fn set_auto_register(&self, auto_register: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "auto-register", &auto_register)
    }

    fn capabilities(&self) -> SecretAgentCapabilities {
        glib::ObjectExt::property(self.as_ref(), "capabilities")
    }

    fn set_capabilities(&self, capabilities: SecretAgentCapabilities) {
        glib::ObjectExt::set_property(self.as_ref(), "capabilities", &capabilities)
    }

    fn identifier(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "identifier")
    }

    fn connect_auto_register_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_register_trampoline<
            P: IsA<SecretAgentOld>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::NMSecretAgentOld,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SecretAgentOld::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-register\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_register_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_capabilities_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_capabilities_trampoline<
            P: IsA<SecretAgentOld>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::NMSecretAgentOld,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SecretAgentOld::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::capabilities\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_capabilities_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_registered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_registered_trampoline<
            P: IsA<SecretAgentOld>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::NMSecretAgentOld,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SecretAgentOld::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::registered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_registered_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SecretAgentOld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SecretAgentOld")
    }
}

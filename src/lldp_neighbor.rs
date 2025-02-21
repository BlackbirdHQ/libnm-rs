// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use glib::translate::*;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use std::mem;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LldpNeighbor(Shared<ffi::NMLldpNeighbor>);

    match fn {
        ref => |ptr| ffi::nm_lldp_neighbor_ref(ptr),
        unref => |ptr| ffi::nm_lldp_neighbor_unref(ptr),
        type_ => || ffi::nm_lldp_neighbor_get_type(),
    }
}

impl LldpNeighbor {
    /// Creates a new [`LldpNeighbor`][crate::LldpNeighbor] object.
    ///
    /// Note that [`LldpNeighbor`][crate::LldpNeighbor] has no public API for mutating
    /// an instance. Also, libnm will not internally mutate a
    /// once exposed object. They are guaranteed to be immutable.
    /// Since 1.32, ref-counting is thread-safe.
    ///
    /// This function is not useful, as there is no public API to
    /// actually modify the (empty) instance.
    ///
    /// # Returns
    ///
    /// the new [`LldpNeighbor`][crate::LldpNeighbor] object.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_lldp_neighbor_new")]
    pub fn new() -> LldpNeighbor {
        unsafe { from_glib_full(ffi::nm_lldp_neighbor_new()) }
    }

    /// Get the type of an attribute.
    /// ## `name`
    /// the attribute name
    ///
    /// # Returns
    ///
    /// the [`glib::VariantType`][crate::glib::VariantType] of the attribute with name `name`
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_lldp_neighbor_get_attr_type")]
    #[doc(alias = "get_attr_type")]
    pub fn attr_type(&self, name: &str) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::nm_lldp_neighbor_get_attr_type(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    /// Gets the uint32 value of attribute with name `name` on `self`
    /// ## `name`
    /// the attribute name
    ///
    /// # Returns
    ///
    /// [`true`] if a uint32 attribute with name `name` was found, [`false`] otherwise
    ///
    /// ## `out_value`
    /// on return, the attribute value
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_lldp_neighbor_get_attr_uint_value")]
    #[doc(alias = "get_attr_uint_value")]
    pub fn attr_uint_value(&self, name: &str) -> Option<u32> {
        unsafe {
            let mut out_value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::nm_lldp_neighbor_get_attr_uint_value(
                self.to_glib_none().0,
                name.to_glib_none().0,
                out_value.as_mut_ptr(),
            ));
            if ret {
                Some(out_value.assume_init())
            } else {
                None
            }
        }
    }

    /// Gets the value (as a GVariant) of attribute with name `name` on `self`
    /// ## `name`
    /// the attribute name
    ///
    /// # Returns
    ///
    /// the value or [`None`] if the attribute with `name` was
    /// not found.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_lldp_neighbor_get_attr_value")]
    #[doc(alias = "get_attr_value")]
    pub fn attr_value(&self, name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::nm_lldp_neighbor_get_attr_value(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }
}

#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
impl Default for LldpNeighbor {
    fn default() -> Self {
        Self::new()
    }
}

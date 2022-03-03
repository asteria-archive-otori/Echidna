use crate::prelude::*;
use glib::subclass::prelude::*;

pub trait IsSubclassImplOf<A: glib::IsA<glib::Object>> {
    type Type: glib::IsA<A>;
}
/**
 * A trait that serves as a macro for ObjectSubclass.from_instance()
 *
 * Requires the A type parameter to be supplied in the implementation with the object subclass of the struct that's being implemented on.
 */
pub trait ToImp<A: ObjectSubclass>
where
    Self: glib::IsA<glib::Object>,
    A: IsSubclassImplOf<Self>,
{
    fn to_imp(&self) -> &A;
}

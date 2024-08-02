use gtk4::{glib::object::IsA, prelude::*, Widget};

pub fn parent_of<T>(wid: &impl IsA<Widget>) -> Option<T>
where
    T: IsA<Widget>,
{
    if let Some(parent) = wid.parent() {
        if let Some(parent) = parent.downcast_ref::<T>() {
            Some(parent.clone())
        } else {
            None
        }
    } else {
        None
    }
}

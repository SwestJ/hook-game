use std::any::type_name_of_val;

pub fn name_of_type<T>(val: &T) -> &'static str {
    type_name_of_val(val).split("::").last().unwrap()
}
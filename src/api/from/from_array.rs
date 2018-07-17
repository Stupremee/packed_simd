//! Implements `From<[T; N]>` and `Into<[T; N]>` for vector types.

macro_rules! impl_from_array {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident
     | ($non_default_array:expr, $non_default_vec:expr)) => {
        impl From<[$elem_ty; $elem_count]> for $id {
            #[inline]
            fn from(array: [$elem_ty; $elem_count]) -> Self {
                union U {
                    array: [$elem_ty; $elem_count],
                    vec: $id,
                }
                unsafe { U { array }.vec }
            }
        }

        impl From<$id> for [$elem_ty; $elem_count] {
            #[inline]
            fn from(vec: $id) -> Self {
                union U {
                    array: [$elem_ty; $elem_count],
                    vec: $id,
                }
                unsafe { U { vec }.array }
            }
        }

        // FIXME: Into::into is not inline, but due to
        // the blanket impl in `std`, which is not
        // marked `default`, we cannot override it here with
        // specialization.
        /*
        impl Into<[$elem_ty; $elem_count]> for $id {
            #[inline]
            fn into(self) -> [$elem_ty; $elem_count] {
                union U {
                    array: [$elem_ty; $elem_count],
                    vec: $id,
                }
                unsafe { U { vec: self }.array }
            }
        }

        impl Into<$id> for [$elem_ty; $elem_count] {
            #[inline]
            fn into(self) -> $id {
                union U {
                    array: [$elem_ty; $elem_count],
                    vec: $id,
                }
                unsafe { U { array: self }.vec }
            }
        }
        */

        #[cfg(test)]
        interpolate_idents! {
            mod [$id _from] {
                use super::*;
                #[test]
                fn array() {
                    let mut array: [$elem_ty; $elem_count] = Default::default();
                    let vec: $id = Default::default();

                    array[0] = $non_default_array;
                    let vec = vec.replace(0, $non_default_vec);

                    let vec_from_array = $id::from(array);
                    assert_eq!(vec_from_array, vec);
                    let array_from_vec = <[$elem_ty; $elem_count]>::from(vec);
                    assert_eq!(array_from_vec, array);

                    let array_from_into_vec: [$elem_ty; $elem_count] = vec.into();
                    assert_eq!(array_from_into_vec, array);
                    let vec_from_into_array: $id = array.into();
                    assert_eq!(vec_from_into_array, vec);
                }
            }
        }
    };
}

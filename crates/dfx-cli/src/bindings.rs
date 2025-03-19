// Generated by `wit-bindgen` 0.36.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod local {
    pub mod host {
        #[allow(dead_code, clippy::all)]
        pub mod misc {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            pub fn print(s: &str) {
                unsafe {
                    let vec0 = s;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "local:host/misc")]
                    extern "C" {
                        #[link_name = "print"]
                        fn wit_import(_: *mut u8, _: usize);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0);
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn rand() -> u8 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "local:host/misc")]
                    extern "C" {
                        #[link_name = "rand"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u8
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn time() -> u64 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "local:host/misc")]
                    extern "C" {
                        #[link_name = "time"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
        }
    }
}
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod exports {
    pub mod local {
        pub mod extension {
            #[allow(dead_code, clippy::all)]
            pub mod cli {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_spec_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::spec();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_spec<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_run_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let base3 = arg0;
                    let len3 = arg1;
                    let mut result3 = _rt::Vec::with_capacity(len3);
                    for i in 0..len3 {
                        let base = base3.add(i * 8);
                        let e3 = {
                            let l0 = *base.add(0).cast::<*mut u8>();
                            let l1 = *base.add(4).cast::<usize>();
                            let len2 = l1;
                            let bytes2 = _rt::Vec::from_raw_parts(l0.cast(), len2, len2);
                            _rt::string_lift(bytes2)
                        };
                        result3.push(e3);
                    }
                    _rt::cabi_dealloc(base3, len3 * 8, 4);
                    let result4 = T::run(result3);
                    _rt::as_i32(result4)
                }
                pub trait Guest {
                    /// spec provides a schema a for the cli
                    fn spec() -> _rt::String;
                    /// run the cli portion of the extension
                    fn run(args: _rt::Vec<_rt::String>) -> u8;
                }
                #[doc(hidden)]
                macro_rules! __export_local_extension_cli_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "local:extension/cli#spec"]
                        unsafe extern "C" fn export_spec() -> * mut u8 {
                        $($path_to_types)*:: _export_spec_cabi::<$ty > () } #[export_name
                        = "cabi_post_local:extension/cli#spec"] unsafe extern "C" fn
                        _post_return_spec(arg0 : * mut u8,) { $($path_to_types)*::
                        __post_return_spec::<$ty > (arg0) } #[export_name =
                        "local:extension/cli#run"] unsafe extern "C" fn export_run(arg0 :
                        * mut u8, arg1 : usize,) -> i32 { $($path_to_types)*::
                        _export_run_cabi::<$ty > (arg0, arg1) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_local_extension_cli_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
        }
    }
}
#[rustfmt::skip]
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub use alloc_crate::alloc;
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_extension_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::local::extension::cli::__export_local_extension_cli_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::local::extension::cli);
    };
}
#[doc(inline)]
pub(crate) use __export_extension_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.36.0:root:component:extension:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 295] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa7\x01\x01A\x02\x01\
A\x04\x01B\x06\x01@\x01\x01ss\x01\0\x04\0\x05print\x01\0\x01@\0\0}\x04\0\x04rand\
\x01\x01\x01@\0\0w\x04\0\x04time\x01\x02\x03\0\x0flocal:host/misc\x05\0\x01B\x05\
\x01@\0\0s\x04\0\x04spec\x01\0\x01ps\x01@\x01\x04args\x01\0}\x04\0\x03run\x01\x02\
\x04\0\x13local:extension/cli\x05\x01\x04\0\x18root:component/extension\x04\0\x0b\
\x0f\x01\0\x09extension\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-c\
omponent\x070.220.1\x10wit-bindgen-rust\x060.36.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

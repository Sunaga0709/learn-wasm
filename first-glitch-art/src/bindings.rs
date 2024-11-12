#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod sunaga0709 {
        #[allow(dead_code)]
        pub mod glitch_art {
            #[allow(dead_code, clippy::all)]
            pub mod png_glitchable {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
                pub enum FilterType {
                    None,
                    Sub,
                    Up,
                    Average,
                    Paeth,
                }
                impl ::core::fmt::Debug for FilterType {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            FilterType::None => {
                                f.debug_tuple("FilterType::None").finish()
                            }
                            FilterType::Sub => f.debug_tuple("FilterType::Sub").finish(),
                            FilterType::Up => f.debug_tuple("FilterType::Up").finish(),
                            FilterType::Average => {
                                f.debug_tuple("FilterType::Average").finish()
                            }
                            FilterType::Paeth => {
                                f.debug_tuple("FilterType::Paeth").finish()
                            }
                        }
                    }
                }
                impl FilterType {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> FilterType {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => FilterType::None,
                            1 => FilterType::Sub,
                            2 => FilterType::Up,
                            3 => FilterType::Average,
                            4 => FilterType::Paeth,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                #[derive(Clone)]
                pub struct ScanLine {
                    pub filter_type: FilterType,
                    pub pixel_data: _rt::Vec<u8>,
                }
                impl ::core::fmt::Debug for ScanLine {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ScanLine")
                            .field("filter-type", &self.filter_type)
                            .field("pixel-data", &self.pixel_data)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_glitch_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let result1 = T::glitch(ScanLine {
                        filter_type: FilterType::_lift(arg0 as u8),
                        pixel_data: _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                    });
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let ScanLine {
                        filter_type: filter_type3,
                        pixel_data: pixel_data3,
                    } = result1;
                    *ptr2.add(0).cast::<u8>() = (filter_type3.clone() as i32) as u8;
                    let vec4 = (pixel_data3).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr2.add(8).cast::<usize>() = len4;
                    *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_glitch<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(4).cast::<*mut u8>();
                    let l1 = *arg0.add(8).cast::<usize>();
                    let base2 = l0;
                    let len2 = l1;
                    _rt::cabi_dealloc(base2, len2 * 1, 1);
                }
                pub trait Guest {
                    fn glitch(scan_line: ScanLine) -> ScanLine;
                }
                #[doc(hidden)]
                macro_rules! __export_sunaga0709_glitch_art_png_glitchable_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "sunaga0709:glitch-art/png-glitchable#glitch"] unsafe extern "C"
                        fn export_glitch(arg0 : i32, arg1 : * mut u8, arg2 : usize,) -> *
                        mut u8 { $($path_to_types)*:: _export_glitch_cabi::<$ty > (arg0,
                        arg1, arg2) } #[export_name =
                        "cabi_post_sunaga0709:glitch-art/png-glitchable#glitch"] unsafe
                        extern "C" fn _post_return_glitch(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_glitch::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_sunaga0709_glitch_art_png_glitchable_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 12],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::vec::Vec;
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
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
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
macro_rules! __export_png_glitcher_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::sunaga0709::glitch_art::png_glitchable::__export_sunaga0709_glitch_art_png_glitchable_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::sunaga0709::glitch_art::png_glitchable);
    };
}
#[doc(inline)]
pub(crate) use __export_png_glitcher_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:sunaga0709:glitch-art:png-glitcher:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 339] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xd0\x01\x01A\x02\x01\
A\x02\x01B\x07\x01m\x05\x04none\x03sub\x02up\x07average\x05paeth\x04\0\x0bfilter\
-type\x03\0\0\x01p}\x01r\x02\x0bfilter-type\x01\x0apixel-data\x02\x04\0\x09scan-\
line\x03\0\x03\x01@\x01\x09scan-line\x04\0\x04\x04\0\x06glitch\x01\x05\x04\x01$s\
unaga0709:glitch-art/png-glitchable\x05\0\x04\x01\"sunaga0709:glitch-art/png-gli\
tcher\x04\0\x0b\x12\x01\0\x0cpng-glitcher\x03\0\0\0G\x09producers\x01\x0cprocess\
ed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
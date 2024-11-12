#[allow(dead_code)]
pub mod sunaga0709 {
    #[allow(dead_code)]
    pub mod glitch_art {
        #[allow(dead_code, clippy::all)]
        pub mod png_glitchable {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
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
                        FilterType::None => f.debug_tuple("FilterType::None").finish(),
                        FilterType::Sub => f.debug_tuple("FilterType::Sub").finish(),
                        FilterType::Up => f.debug_tuple("FilterType::Up").finish(),
                        FilterType::Average => {
                            f.debug_tuple("FilterType::Average").finish()
                        }
                        FilterType::Paeth => f.debug_tuple("FilterType::Paeth").finish(),
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
            #[allow(unused_unsafe, clippy::all)]
            pub fn glitch(scan_line: &ScanLine) -> ScanLine {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let ScanLine {
                        filter_type: filter_type0,
                        pixel_data: pixel_data0,
                    } = scan_line;
                    let vec1 = pixel_data0;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();
                    let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "sunaga0709:glitch-art/png-glitchable")]
                    extern "C" {
                        #[link_name = "glitch"]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(filter_type0.clone() as i32, ptr1.cast_mut(), len1, ptr2);
                    let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                    let l4 = *ptr2.add(4).cast::<*mut u8>();
                    let l5 = *ptr2.add(8).cast::<usize>();
                    let len6 = l5;
                    ScanLine {
                        filter_type: FilterType::_lift(l3 as u8),
                        pixel_data: _rt::Vec::from_raw_parts(l4.cast(), len6, len6),
                    }
                }
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::vec::Vec;
    extern crate alloc as alloc_crate;
}
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:sunaga0709:glitch-art:png-glitch-cli:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 343] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xd2\x01\x01A\x02\x01\
A\x02\x01B\x07\x01m\x05\x04none\x03sub\x02up\x07average\x05paeth\x04\0\x0bfilter\
-type\x03\0\0\x01p}\x01r\x02\x0bfilter-type\x01\x0apixel-data\x02\x04\0\x09scan-\
line\x03\0\x03\x01@\x01\x09scan-line\x04\0\x04\x04\0\x06glitch\x01\x05\x03\x01$s\
unaga0709:glitch-art/png-glitchable\x05\0\x04\x01$sunaga0709:glitch-art/png-glit\
ch-cli\x04\0\x0b\x14\x01\0\x0epng-glitch-cli\x03\0\0\0G\x09producers\x01\x0cproc\
essed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

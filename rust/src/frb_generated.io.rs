// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.34.

// Section: imports

use super::*;
use crate::api::simple::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_io!();

#[no_mangle]
pub extern "C" fn frbgen_testflutterbridge_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerNLU(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<NLU>>::increment_strong_count(
        ptr as _,
    );
}

#[no_mangle]
pub extern "C" fn frbgen_testflutterbridge_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerNLU(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<NLU>>::decrement_strong_count(
        ptr as _,
    );
}

#[no_mangle]
pub extern "C" fn frbgen_testflutterbridge_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnersnips_nlu_libontologyIntentParserResult(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            snips_nlu_lib::ontology::IntentParserResult,
        >,
    >::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_testflutterbridge_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnersnips_nlu_libontologyIntentParserResult(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<
        flutter_rust_bridge::for_generated::RustAutoOpaqueInner<
            snips_nlu_lib::ontology::IntentParserResult,
        >,
    >::decrement_strong_count(ptr as _);
}

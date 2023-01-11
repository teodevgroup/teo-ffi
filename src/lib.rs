use std::ffi::{c_char, c_int, c_long, c_short, c_longlong, CStr, c_ushort, c_uint, c_ulong, c_ulonglong, c_float, c_double, CString};
use std::sync::Arc;
use libc::c_uchar;
use teo::core::app::builder::AppBuilder as TeoAppBuilder;
use teo::core::app::App as TeoApp;
use teo::core::tson::Value as TeoValue;
use teo::core::object::Object as TeoObject;
use teo::core::app::environment::EnvironmentVersion;
use teo::core::pipeline::context::validity::Validity::*;
use to_mut::ToMut;
use tokio::runtime::Runtime;
use once_cell::sync::OnceCell;
use teo::core::pipeline::context::validity::Validity;

fn get_runtime() -> Result<&'static Runtime, ()> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| panic!("Tokio runtime creation error.")))
}

#[repr(C)]
pub struct AppBuilder {
    _app_builder: Arc<TeoAppBuilder>,
}

#[repr(C)]
pub struct Value {
    _value: Arc<TeoValue>
}

#[repr(C)]
pub struct Object {
    _object: Arc<TeoObject>
}

#[no_mangle]
pub extern "C" fn Value_IsNull(value: Value) -> c_int {
    if value._value.is_null() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn Value_IntoString(value: Value) -> *mut c_char {
    let str = value._value.as_str().unwrap();
    let c_string = CString::new(str).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn Value_FreeString(c_str: *mut c_char) {
    unsafe {
        let _c_string = CString::from_raw(c_str);
    }
}

#[no_mangle]
pub extern "C" fn Value_IntoBool(value: Value) -> c_int {
    if value._value.as_bool().unwrap() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn Value_IntoChar(value: Value) -> c_char {
    value._value.as_i8().unwrap()
}

#[no_mangle]
pub extern "C" fn Value_IntoShort(value: Value) -> c_short {
    value._value.as_i16().unwrap()
}

#[no_mangle]
pub extern "C" fn Value_IntoInt(value: Value) -> c_int {
    value._value.as_i32().unwrap()
}

#[no_mangle]
pub extern "C" fn Value_IntoLong(value: Value) -> c_long {
    value._value.as_i64().unwrap()
}

#[no_mangle]
pub extern "C" fn Value_IntoLongLong(value: Value) -> c_longlong {
    value._value.as_i64().unwrap()
}

#[no_mangle]
pub extern "C" fn Value_IntoUChar(value: Value) -> c_uchar {
    value._value.as_u8().unwrap()
}

#[no_mangle]
pub extern "C" fn Value_IntoUShort(value: Value) -> c_ushort {
    value._value.as_u16().unwrap()
}

#[no_mangle]
pub extern "C" fn Value_IntoUInt(value: Value) -> c_uint {
    value._value.as_u32().unwrap()
}

#[no_mangle]
pub extern "C" fn Value_IntoULong(value: Value) -> c_ulong {
    value._value.as_u64().unwrap()
}

#[no_mangle]
pub extern "C" fn Value_IntoULongLong(value: Value) -> c_ulonglong {
    value._value.as_u64().unwrap()
}

#[no_mangle]
pub extern "C" fn Value_FromNull() -> Value {
    Value { _value: Arc::new(TeoValue::Null) }
}

#[no_mangle]
pub extern "C" fn Value_FromString(string: *const c_char) -> Value {
    let c_str = unsafe {
        assert!(!string.is_null());
        CStr::from_ptr(string)
    };
    Value { _value: Arc::new(TeoValue::String(c_str.to_str().unwrap().to_string())) }
}

#[no_mangle]
pub extern "C" fn Value_BoolFromInt(int: c_int) -> Value {
    Value { _value: Arc::new(TeoValue::Bool(int == 1)) }
}

#[no_mangle]
pub extern "C" fn Value_FromChar(int: c_char) -> Value {
    Value { _value: Arc::new(TeoValue::I8(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromShort(int: c_short) -> Value {
    Value { _value: Arc::new(TeoValue::I16(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromInt(int: c_int) -> Value {
    Value { _value: Arc::new(TeoValue::I32(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromLong(int: c_long) -> Value {
    Value { _value: Arc::new(TeoValue::I64(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromLongLong(int: c_longlong) -> Value {
    Value { _value: Arc::new(TeoValue::I64(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromUChar(int: c_uchar) -> Value {
    Value { _value: Arc::new(TeoValue::U8(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromUShort(int: c_ushort) -> Value {
    Value { _value: Arc::new(TeoValue::U16(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromUInt(int: c_uint) -> Value {
    Value { _value: Arc::new(TeoValue::U32(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromULong(int: c_ulong) -> Value {
    Value { _value: Arc::new(TeoValue::U64(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromULongLong(int: c_ulonglong) -> Value {
    Value { _value: Arc::new(TeoValue::U64(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromFloat(int: c_float) -> Value {
    Value { _value: Arc::new(TeoValue::F32(int)) }
}

#[no_mangle]
pub extern "C" fn Value_FromDouble(int: c_double) -> Value {
    Value { _value: Arc::new(TeoValue::F64(int)) }
}

// #[no_mangle]
// pub extern "C" fn Value_DateFromString(string: *const c_char) -> Value {
//
// }
//
// #[no_mangle]
// pub extern "C" fn Value_DateTimeFromString(string: *const c_char) -> Value {
//
// }

#[no_mangle]
pub extern "C" fn AppBuilder_Create(lang: *const c_char, version: *const c_char) -> AppBuilder {
    let lang_c_str = unsafe {
        assert!(!lang.is_null());
        CStr::from_ptr(lang)
    };
    let lang_r_str = lang_c_str.to_str().unwrap();
    let version_c_str = unsafe {
        assert!(!version.is_null());
        CStr::from_ptr(version)
    };
    let version_r_str = version_c_str.to_str().unwrap();
    let environment_version = match lang_r_str {
        "Go" => EnvironmentVersion::Go(version_r_str.to_string()),
        "Java" => EnvironmentVersion::Java(version_r_str.to_string()),
        "C" => EnvironmentVersion::C(version_r_str.to_string()),
        _ => panic!("Only \"Go\", \"Java\" and \"C\" are considered valid lang parameter.")
    };
    AppBuilder { _app_builder: Arc::new(TeoAppBuilder::new_with_environment_version(environment_version)) }
}

#[no_mangle]
pub extern "C" fn AppBuilder_Load(app_builder: *mut AppBuilder, schema_file_name: *const c_char) {
    let file_name = if schema_file_name.is_null() {
        None
    } else {
        unsafe {
            Some(CStr::from_ptr(schema_file_name).to_str().unwrap())
        }
    };
    unsafe {
        (*app_builder)._app_builder.to_mut().load(file_name);
    }
}

#[no_mangle]
pub extern "C" fn AppBuilder_Build(app_builder: *const AppBuilder) {
    let runtime = get_runtime().unwrap();
    let teo_app_builder = unsafe {
        (*app_builder)._app_builder.clone()
    };
    let handle = runtime.spawn(async move {
        let teo_app = teo_app_builder.build().await;
        teo_app.run().await;
    });
    Box::leak(Box::new(handle));
}

#[no_mangle]
pub extern "C" fn AppBuilder_Transform(
    app_builder: *const AppBuilder,
    name: *const c_char,
    callback: extern fn(Value) -> Value
) {
    let teo_app_builder_arc = unsafe {
        (*app_builder)._app_builder.clone()
    };
    let teo_app_builder = teo_app_builder_arc.as_ref().to_mut();
    let name_c_str = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    let name_r_str = name_c_str.to_str().unwrap();
    teo_app_builder.transform(name_r_str.to_string(), move |teo_value: TeoValue| async move {
        let value = Value { _value: Arc::new(teo_value) };
        let result = callback(value);
        result._value.as_ref().clone()
    });
}

#[no_mangle]
pub extern "C" fn AppBuilder_Validate(
    app_builder: *const AppBuilder,
    name: *const c_char,
    callback: extern fn(Value) -> Value
) {
    let teo_app_builder_arc = unsafe {
        (*app_builder)._app_builder.clone()
    };
    let teo_app_builder = teo_app_builder_arc.as_ref().to_mut();
    let name_c_str = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    let name_r_str = name_c_str.to_str().unwrap();
    teo_app_builder.validate(name_r_str.to_string(), move |teo_value: TeoValue| async move {
        let value = Value { _value: Arc::new(teo_value) };
        let result = callback(value);
        match result._value.as_ref() {
            TeoValue::Null => Valid,
            TeoValue::String(s) => Validity::from(s.as_str()),
            TeoValue::Bool(b) => Validity::from(*b),
            _ => panic!("Invalid validity value."),
        }
    });
}

#[no_mangle]
pub extern "C" fn AppBuilder_Perform(
    app_builder: *const AppBuilder,
    name: *const c_char,
    callback: extern fn(Value)
) {
    let teo_app_builder_arc = unsafe {
        (*app_builder)._app_builder.clone()
    };
    let teo_app_builder = teo_app_builder_arc.as_ref().to_mut();
    let name_c_str = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    let name_r_str = name_c_str.to_str().unwrap();
    teo_app_builder.perform(name_r_str.to_string(), move |teo_value: TeoValue| async move {
        let value = Value { _value: Arc::new(teo_value) };
        callback(value)
    });
}

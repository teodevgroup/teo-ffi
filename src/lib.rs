use std::ffi::{c_char, c_int, c_long, c_short, c_longlong, CStr, c_ushort, c_uint, c_ulong, c_ulonglong, c_float, c_double};
use std::sync::Arc;
use libc::c_uchar;
use teo::core::app::builder::AppBuilder as TeoAppBuilder;
use teo::core::app::App as TeoApp;
use teo::core::tson::Value as TeoValue;
use teo::core::object::Object as TeoObject;
use teo::core::app::environment::EnvironmentVersion;
use to_mut::ToMut;
use tokio::runtime::Runtime;
use once_cell::sync::OnceCell;

fn get_runtime() -> Result<&'static Runtime, ()> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();
    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| panic!("Tokio runtime creation error.")))
}

#[repr(C)]
pub struct AppBuilder {
    _app_builder: Arc<TeoAppBuilder>,
}

#[repr(C)]
pub struct App {
    _app: Arc<TeoApp>,
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
pub extern "C" fn AppBuilder_Build(app_builder: *const AppBuilder, callback: extern fn(App) -> ()) {
    let runtime = get_runtime().unwrap();
    let teo_app_builder = unsafe {
        (*app_builder)._app_builder.clone()
    };
    runtime.spawn(async move {
        let teo_app = teo_app_builder.build().await;
        callback(App { _app: Arc::new(teo_app) });
    });
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
pub extern "C" fn App_Run(app: *const App) {
    let runtime = get_runtime().unwrap();
    let teo_app = unsafe {
        (*app)._app.clone()
    };
    let handle = runtime.spawn(async move {
        teo_app.run().await;
    });
    Box::leak(Box::new(handle));
}

use crate::manifest::TypeVariant;
use std::fmt::Display;

// Re-exports
pub use self::render::{generate_android_main_types, render_to_strings, RenderIntput};

mod functions;
mod inits;
mod properties;
mod render;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidMainStruct {
    name: String,
    is_class: bool,
    inits: Vec<AndroidMainInit>,
    methods: Vec<AndroidMainMethod>,
    static_methods: Vec<AndroidMainMethod>,
    properties: Vec<AndroidMainProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidMainInit {
    params: Vec<AndroidMainParam>,
    is_nullable: bool,
    return_call: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidMainMethod {
    name: String,
    params: Vec<AndroidMainParam>,
    is_static: bool,
    #[serde(rename = "return")]
    return_ty: AndroidMainReturn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidMainProperty {
    name: String,
    #[serde(rename = "return")]
    return_ty: AndroidMainReturn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidMainParam {
    name: String,
    ty: KotlinType,
    is_nullable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidMainReturn {
    ty: KotlinType,
    is_nullable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KotlinType(String);

impl Display for KotlinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Convert the `TypeVariant` into the appropriate Swift type.
impl From<TypeVariant> for KotlinType {
    fn from(value: TypeVariant) -> Self {
        let res = match value {
            TypeVariant::Void => "Void".to_string(),
            TypeVariant::Bool => "Bool".to_string(),
            TypeVariant::Char => "Char".to_string(),
            TypeVariant::ShortInt => "Short".to_string(),
            TypeVariant::Int => "Int".to_string(),
            TypeVariant::UnsignedInt => "UInt".to_string(),
            TypeVariant::LongInt => "Long".to_string(),
            TypeVariant::Float => "Float".to_string(),
            TypeVariant::Double => "Double".to_string(),
            TypeVariant::SizeT => "Int".to_string(),
            TypeVariant::Int8T => "Byte".to_string(),
            TypeVariant::Int16T => "Short".to_string(),
            TypeVariant::Int32T => "Int".to_string(),
            TypeVariant::Int64T => "Long".to_string(),
            TypeVariant::UInt8T => "UByte".to_string(),
            TypeVariant::UInt16T => "UShort".to_string(),
            TypeVariant::UInt32T => "UInt".to_string(),
            TypeVariant::UInt64T => "ULong".to_string(),
            TypeVariant::String => "String".to_string(),
            TypeVariant::Data => "ByteArray".to_string(),
            TypeVariant::Struct(n) | TypeVariant::Enum(n) => {
                // We strip the "TW" prefix for Swift representations of
                // structs/enums.
                n.strip_prefix("TW").map(|n| n.to_string()).unwrap_or(n)
            }
        };

        KotlinType(res)
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Any {
    #[prost(string, tag = "1")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(bytes = "bytes", tag = "2")]
    pub value: ::prost::bytes::Bytes,
}
#[cfg(feature = "prost-types")]
impl Any {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Any,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Any as ::serde::Serialize>::serialize(&Any::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Any, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(::prost_types::Any::from(<Any as ::serde::Deserialize>::deserialize(d)?))
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Any> for ::prost_types::Any {
    #[allow(deprecated)]
    fn from(other: Any) -> ::prost_types::Any {
        ::prost_types::Any {
            type_url: ::core::convert::From::from(other.type_url),
            value: ::core::convert::From::from(other.value),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Any> for Any {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Any) -> Any {
        Any {
            type_url: ::core::convert::From::from(other.type_url),
            value: ::core::convert::From::from(other.value),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceContext {
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
}
#[cfg(feature = "prost-types")]
impl SourceContext {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::SourceContext,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <SourceContext as ::serde::Serialize>::serialize(
            &SourceContext::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::SourceContext, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::SourceContext::from(
                <SourceContext as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<SourceContext> for ::prost_types::SourceContext {
    #[allow(deprecated)]
    fn from(other: SourceContext) -> ::prost_types::SourceContext {
        ::prost_types::SourceContext {
            file_name: ::core::convert::From::from(other.file_name),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::SourceContext> for SourceContext {
    #[allow(deprecated)]
    fn from(other: ::prost_types::SourceContext) -> SourceContext {
        SourceContext {
            file_name: ::core::convert::From::from(other.file_name),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Type {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    #[prost(string, repeated, tag = "3")]
    pub oneofs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    #[prost(message, optional, tag = "5")]
    pub source_context: ::core::option::Option<SourceContext>,
    #[prost(enumeration = "Syntax", tag = "6")]
    pub syntax: i32,
}
#[cfg(feature = "prost-types")]
impl Type {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Type,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Type as ::serde::Serialize>::serialize(&Type::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Type, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(::prost_types::Type::from(<Type as ::serde::Deserialize>::deserialize(d)?))
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Type> for ::prost_types::Type {
    #[allow(deprecated)]
    fn from(other: Type) -> ::prost_types::Type {
        ::prost_types::Type {
            name: ::core::convert::From::from(other.name),
            fields: other.fields.into_iter().map(::core::convert::From::from).collect(),
            oneofs: other.oneofs.into_iter().map(::core::convert::From::from).collect(),
            options: other
                .options
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            source_context: other.source_context.map(::core::convert::From::from),
            syntax: ::core::convert::From::from(other.syntax),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Type> for Type {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Type) -> Type {
        Type {
            name: ::core::convert::From::from(other.name),
            fields: other.fields.into_iter().map(::core::convert::From::from).collect(),
            oneofs: other.oneofs.into_iter().map(::core::convert::From::from).collect(),
            options: other
                .options
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            source_context: other.source_context.map(::core::convert::From::from),
            syntax: ::core::convert::From::from(other.syntax),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(enumeration = "field::Kind", tag = "1")]
    pub kind: i32,
    #[prost(enumeration = "field::Cardinality", tag = "2")]
    pub cardinality: i32,
    #[prost(int32, tag = "3")]
    pub number: i32,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub type_url: ::prost::alloc::string::String,
    #[prost(int32, tag = "7")]
    pub oneof_index: i32,
    #[prost(bool, tag = "8")]
    pub packed: bool,
    #[prost(message, repeated, tag = "9")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    #[prost(string, tag = "10")]
    pub json_name: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub default_value: ::prost::alloc::string::String,
}
#[cfg(feature = "prost-types")]
impl Field {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Field,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Field as ::serde::Serialize>::serialize(&Field::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Field, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(::prost_types::Field::from(<Field as ::serde::Deserialize>::deserialize(d)?))
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Field> for ::prost_types::Field {
    #[allow(deprecated)]
    fn from(other: Field) -> ::prost_types::Field {
        ::prost_types::Field {
            kind: ::core::convert::From::from(other.kind),
            cardinality: ::core::convert::From::from(other.cardinality),
            number: ::core::convert::From::from(other.number),
            name: ::core::convert::From::from(other.name),
            type_url: ::core::convert::From::from(other.type_url),
            oneof_index: ::core::convert::From::from(other.oneof_index),
            packed: ::core::convert::From::from(other.packed),
            options: other
                .options
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            json_name: ::core::convert::From::from(other.json_name),
            default_value: ::core::convert::From::from(other.default_value),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Field> for Field {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Field) -> Field {
        Field {
            kind: ::core::convert::From::from(other.kind),
            cardinality: ::core::convert::From::from(other.cardinality),
            number: ::core::convert::From::from(other.number),
            name: ::core::convert::From::from(other.name),
            type_url: ::core::convert::From::from(other.type_url),
            oneof_index: ::core::convert::From::from(other.oneof_index),
            packed: ::core::convert::From::from(other.packed),
            options: other
                .options
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            json_name: ::core::convert::From::from(other.json_name),
            default_value: ::core::convert::From::from(other.default_value),
        }
    }
}
/// Nested message and enum types in `Field`.
pub mod field {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Kind {
        TypeUnknown = 0,
        TypeDouble = 1,
        TypeFloat = 2,
        TypeInt64 = 3,
        TypeUint64 = 4,
        TypeInt32 = 5,
        TypeFixed64 = 6,
        TypeFixed32 = 7,
        TypeBool = 8,
        TypeString = 9,
        TypeGroup = 10,
        TypeMessage = 11,
        TypeBytes = 12,
        TypeUint32 = 13,
        TypeEnum = 14,
        TypeSfixed32 = 15,
        TypeSfixed64 = 16,
        TypeSint32 = 17,
        TypeSint64 = 18,
    }
    #[cfg(feature = "prost-types")]
    impl Kind {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::field::Kind,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <Kind as ::serde::Serialize>::serialize(&Kind::from(item.clone()), s)
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::field::Kind, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::field::Kind::from(
                    <Kind as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<Kind> for ::prost_types::field::Kind {
        #[allow(deprecated)]
        fn from(pbjson: Kind) -> ::prost_types::field::Kind {
            match pbjson {
                Kind::TypeUnknown => ::prost_types::field::Kind::TypeUnknown,
                Kind::TypeDouble => ::prost_types::field::Kind::TypeDouble,
                Kind::TypeFloat => ::prost_types::field::Kind::TypeFloat,
                Kind::TypeInt64 => ::prost_types::field::Kind::TypeInt64,
                Kind::TypeUint64 => ::prost_types::field::Kind::TypeUint64,
                Kind::TypeInt32 => ::prost_types::field::Kind::TypeInt32,
                Kind::TypeFixed64 => ::prost_types::field::Kind::TypeFixed64,
                Kind::TypeFixed32 => ::prost_types::field::Kind::TypeFixed32,
                Kind::TypeBool => ::prost_types::field::Kind::TypeBool,
                Kind::TypeString => ::prost_types::field::Kind::TypeString,
                Kind::TypeGroup => ::prost_types::field::Kind::TypeGroup,
                Kind::TypeMessage => ::prost_types::field::Kind::TypeMessage,
                Kind::TypeBytes => ::prost_types::field::Kind::TypeBytes,
                Kind::TypeUint32 => ::prost_types::field::Kind::TypeUint32,
                Kind::TypeEnum => ::prost_types::field::Kind::TypeEnum,
                Kind::TypeSfixed32 => ::prost_types::field::Kind::TypeSfixed32,
                Kind::TypeSfixed64 => ::prost_types::field::Kind::TypeSfixed64,
                Kind::TypeSint32 => ::prost_types::field::Kind::TypeSint32,
                Kind::TypeSint64 => ::prost_types::field::Kind::TypeSint64,
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::field::Kind> for Kind {
        #[allow(deprecated)]
        fn from(prost: ::prost_types::field::Kind) -> Kind {
            match prost {
                ::prost_types::field::Kind::TypeUnknown => Kind::TypeUnknown,
                ::prost_types::field::Kind::TypeDouble => Kind::TypeDouble,
                ::prost_types::field::Kind::TypeFloat => Kind::TypeFloat,
                ::prost_types::field::Kind::TypeInt64 => Kind::TypeInt64,
                ::prost_types::field::Kind::TypeUint64 => Kind::TypeUint64,
                ::prost_types::field::Kind::TypeInt32 => Kind::TypeInt32,
                ::prost_types::field::Kind::TypeFixed64 => Kind::TypeFixed64,
                ::prost_types::field::Kind::TypeFixed32 => Kind::TypeFixed32,
                ::prost_types::field::Kind::TypeBool => Kind::TypeBool,
                ::prost_types::field::Kind::TypeString => Kind::TypeString,
                ::prost_types::field::Kind::TypeGroup => Kind::TypeGroup,
                ::prost_types::field::Kind::TypeMessage => Kind::TypeMessage,
                ::prost_types::field::Kind::TypeBytes => Kind::TypeBytes,
                ::prost_types::field::Kind::TypeUint32 => Kind::TypeUint32,
                ::prost_types::field::Kind::TypeEnum => Kind::TypeEnum,
                ::prost_types::field::Kind::TypeSfixed32 => Kind::TypeSfixed32,
                ::prost_types::field::Kind::TypeSfixed64 => Kind::TypeSfixed64,
                ::prost_types::field::Kind::TypeSint32 => Kind::TypeSint32,
                ::prost_types::field::Kind::TypeSint64 => Kind::TypeSint64,
            }
        }
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::TypeUnknown => "TYPE_UNKNOWN",
                Kind::TypeDouble => "TYPE_DOUBLE",
                Kind::TypeFloat => "TYPE_FLOAT",
                Kind::TypeInt64 => "TYPE_INT64",
                Kind::TypeUint64 => "TYPE_UINT64",
                Kind::TypeInt32 => "TYPE_INT32",
                Kind::TypeFixed64 => "TYPE_FIXED64",
                Kind::TypeFixed32 => "TYPE_FIXED32",
                Kind::TypeBool => "TYPE_BOOL",
                Kind::TypeString => "TYPE_STRING",
                Kind::TypeGroup => "TYPE_GROUP",
                Kind::TypeMessage => "TYPE_MESSAGE",
                Kind::TypeBytes => "TYPE_BYTES",
                Kind::TypeUint32 => "TYPE_UINT32",
                Kind::TypeEnum => "TYPE_ENUM",
                Kind::TypeSfixed32 => "TYPE_SFIXED32",
                Kind::TypeSfixed64 => "TYPE_SFIXED64",
                Kind::TypeSint32 => "TYPE_SINT32",
                Kind::TypeSint64 => "TYPE_SINT64",
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Cardinality {
        Unknown = 0,
        Optional = 1,
        Required = 2,
        Repeated = 3,
    }
    #[cfg(feature = "prost-types")]
    impl Cardinality {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::field::Cardinality,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <Cardinality as ::serde::Serialize>::serialize(
                &Cardinality::from(item.clone()),
                s,
            )
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::field::Cardinality, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::field::Cardinality::from(
                    <Cardinality as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<Cardinality> for ::prost_types::field::Cardinality {
        #[allow(deprecated)]
        fn from(pbjson: Cardinality) -> ::prost_types::field::Cardinality {
            match pbjson {
                Cardinality::Unknown => ::prost_types::field::Cardinality::Unknown,
                Cardinality::Optional => ::prost_types::field::Cardinality::Optional,
                Cardinality::Required => ::prost_types::field::Cardinality::Required,
                Cardinality::Repeated => ::prost_types::field::Cardinality::Repeated,
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::field::Cardinality> for Cardinality {
        #[allow(deprecated)]
        fn from(prost: ::prost_types::field::Cardinality) -> Cardinality {
            match prost {
                ::prost_types::field::Cardinality::Unknown => Cardinality::Unknown,
                ::prost_types::field::Cardinality::Optional => Cardinality::Optional,
                ::prost_types::field::Cardinality::Required => Cardinality::Required,
                ::prost_types::field::Cardinality::Repeated => Cardinality::Repeated,
            }
        }
    }
    impl Cardinality {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Cardinality::Unknown => "CARDINALITY_UNKNOWN",
                Cardinality::Optional => "CARDINALITY_OPTIONAL",
                Cardinality::Required => "CARDINALITY_REQUIRED",
                Cardinality::Repeated => "CARDINALITY_REPEATED",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Enum {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub enumvalue: ::prost::alloc::vec::Vec<EnumValue>,
    #[prost(message, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    #[prost(message, optional, tag = "4")]
    pub source_context: ::core::option::Option<SourceContext>,
    #[prost(enumeration = "Syntax", tag = "5")]
    pub syntax: i32,
}
#[cfg(feature = "prost-types")]
impl Enum {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Enum,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Enum as ::serde::Serialize>::serialize(&Enum::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Enum, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(::prost_types::Enum::from(<Enum as ::serde::Deserialize>::deserialize(d)?))
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Enum> for ::prost_types::Enum {
    #[allow(deprecated)]
    fn from(other: Enum) -> ::prost_types::Enum {
        ::prost_types::Enum {
            name: ::core::convert::From::from(other.name),
            enumvalue: other
                .enumvalue
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            options: other
                .options
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            source_context: other.source_context.map(::core::convert::From::from),
            syntax: ::core::convert::From::from(other.syntax),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Enum> for Enum {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Enum) -> Enum {
        Enum {
            name: ::core::convert::From::from(other.name),
            enumvalue: other
                .enumvalue
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            options: other
                .options
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            source_context: other.source_context.map(::core::convert::From::from),
            syntax: ::core::convert::From::from(other.syntax),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValue {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub number: i32,
    #[prost(message, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
}
#[cfg(feature = "prost-types")]
impl EnumValue {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::EnumValue,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <EnumValue as ::serde::Serialize>::serialize(&EnumValue::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::EnumValue, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::EnumValue::from(
                <EnumValue as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<EnumValue> for ::prost_types::EnumValue {
    #[allow(deprecated)]
    fn from(other: EnumValue) -> ::prost_types::EnumValue {
        ::prost_types::EnumValue {
            name: ::core::convert::From::from(other.name),
            number: ::core::convert::From::from(other.number),
            options: other.options.into_iter().map(::core::convert::From::from).collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::EnumValue> for EnumValue {
    #[allow(deprecated)]
    fn from(other: ::prost_types::EnumValue) -> EnumValue {
        EnumValue {
            name: ::core::convert::From::from(other.name),
            number: ::core::convert::From::from(other.number),
            options: other.options.into_iter().map(::core::convert::From::from).collect(),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Option {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Any>,
}
#[cfg(feature = "prost-types")]
impl Option {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Option,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Option as ::serde::Serialize>::serialize(&Option::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Option, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::Option::from(
                <Option as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Option> for ::prost_types::Option {
    #[allow(deprecated)]
    fn from(other: Option) -> ::prost_types::Option {
        ::prost_types::Option {
            name: ::core::convert::From::from(other.name),
            value: other.value.map(::core::convert::From::from),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Option> for Option {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Option) -> Option {
        Option {
            name: ::core::convert::From::from(other.name),
            value: other.value.map(::core::convert::From::from),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Syntax {
    Proto2 = 0,
    Proto3 = 1,
}
impl Syntax {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Syntax::Proto2 => "SYNTAX_PROTO2",
            Syntax::Proto3 => "SYNTAX_PROTO3",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Api {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub methods: ::prost::alloc::vec::Vec<Method>,
    #[prost(message, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub source_context: ::core::option::Option<SourceContext>,
    #[prost(message, repeated, tag = "6")]
    pub mixins: ::prost::alloc::vec::Vec<Mixin>,
    #[prost(enumeration = "Syntax", tag = "7")]
    pub syntax: i32,
}
#[cfg(feature = "prost-types")]
impl Api {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Api,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Api as ::serde::Serialize>::serialize(&Api::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Api, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(::prost_types::Api::from(<Api as ::serde::Deserialize>::deserialize(d)?))
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Api> for ::prost_types::Api {
    #[allow(deprecated)]
    fn from(other: Api) -> ::prost_types::Api {
        ::prost_types::Api {
            name: ::core::convert::From::from(other.name),
            methods: other
                .methods
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            options: other
                .options
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            version: ::core::convert::From::from(other.version),
            source_context: other.source_context.map(::core::convert::From::from),
            mixins: other.mixins.into_iter().map(::core::convert::From::from).collect(),
            syntax: ::core::convert::From::from(other.syntax),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Api> for Api {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Api) -> Api {
        Api {
            name: ::core::convert::From::from(other.name),
            methods: other
                .methods
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            options: other
                .options
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            version: ::core::convert::From::from(other.version),
            source_context: other.source_context.map(::core::convert::From::from),
            mixins: other.mixins.into_iter().map(::core::convert::From::from).collect(),
            syntax: ::core::convert::From::from(other.syntax),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Method {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_type_url: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub request_streaming: bool,
    #[prost(string, tag = "4")]
    pub response_type_url: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub response_streaming: bool,
    #[prost(message, repeated, tag = "6")]
    pub options: ::prost::alloc::vec::Vec<Option>,
    #[prost(enumeration = "Syntax", tag = "7")]
    pub syntax: i32,
}
#[cfg(feature = "prost-types")]
impl Method {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Method,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Method as ::serde::Serialize>::serialize(&Method::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Method, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::Method::from(
                <Method as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Method> for ::prost_types::Method {
    #[allow(deprecated)]
    fn from(other: Method) -> ::prost_types::Method {
        ::prost_types::Method {
            name: ::core::convert::From::from(other.name),
            request_type_url: ::core::convert::From::from(other.request_type_url),
            request_streaming: ::core::convert::From::from(other.request_streaming),
            response_type_url: ::core::convert::From::from(other.response_type_url),
            response_streaming: ::core::convert::From::from(other.response_streaming),
            options: other
                .options
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            syntax: ::core::convert::From::from(other.syntax),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Method> for Method {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Method) -> Method {
        Method {
            name: ::core::convert::From::from(other.name),
            request_type_url: ::core::convert::From::from(other.request_type_url),
            request_streaming: ::core::convert::From::from(other.request_streaming),
            response_type_url: ::core::convert::From::from(other.response_type_url),
            response_streaming: ::core::convert::From::from(other.response_streaming),
            options: other
                .options
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            syntax: ::core::convert::From::from(other.syntax),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mixin {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub root: ::prost::alloc::string::String,
}
#[cfg(feature = "prost-types")]
impl Mixin {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Mixin,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Mixin as ::serde::Serialize>::serialize(&Mixin::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Mixin, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(::prost_types::Mixin::from(<Mixin as ::serde::Deserialize>::deserialize(d)?))
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Mixin> for ::prost_types::Mixin {
    #[allow(deprecated)]
    fn from(other: Mixin) -> ::prost_types::Mixin {
        ::prost_types::Mixin {
            name: ::core::convert::From::from(other.name),
            root: ::core::convert::From::from(other.root),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Mixin> for Mixin {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Mixin) -> Mixin {
        Mixin {
            name: ::core::convert::From::from(other.name),
            root: ::core::convert::From::from(other.root),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDescriptorSet {
    #[prost(message, repeated, tag = "1")]
    pub file: ::prost::alloc::vec::Vec<FileDescriptorProto>,
}
#[cfg(feature = "prost-types")]
impl FileDescriptorSet {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::FileDescriptorSet,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <FileDescriptorSet as ::serde::Serialize>::serialize(
            &FileDescriptorSet::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::FileDescriptorSet, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::FileDescriptorSet::from(
                <FileDescriptorSet as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<FileDescriptorSet> for ::prost_types::FileDescriptorSet {
    #[allow(deprecated)]
    fn from(other: FileDescriptorSet) -> ::prost_types::FileDescriptorSet {
        ::prost_types::FileDescriptorSet {
            file: other.file.into_iter().map(::core::convert::From::from).collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::FileDescriptorSet> for FileDescriptorSet {
    #[allow(deprecated)]
    fn from(other: ::prost_types::FileDescriptorSet) -> FileDescriptorSet {
        FileDescriptorSet {
            file: other.file.into_iter().map(::core::convert::From::from).collect(),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub package: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub dependency: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, repeated, packed = "false", tag = "10")]
    pub public_dependency: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "11")]
    pub weak_dependency: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "4")]
    pub message_type: ::prost::alloc::vec::Vec<DescriptorProto>,
    #[prost(message, repeated, tag = "5")]
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProto>,
    #[prost(message, repeated, tag = "6")]
    pub service: ::prost::alloc::vec::Vec<ServiceDescriptorProto>,
    #[prost(message, repeated, tag = "7")]
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, optional, tag = "8")]
    pub options: ::core::option::Option<FileOptions>,
    #[prost(message, optional, tag = "9")]
    pub source_code_info: ::core::option::Option<SourceCodeInfo>,
    #[prost(string, optional, tag = "12")]
    pub syntax: ::core::option::Option<::prost::alloc::string::String>,
}
#[cfg(feature = "prost-types")]
impl FileDescriptorProto {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::FileDescriptorProto,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <FileDescriptorProto as ::serde::Serialize>::serialize(
            &FileDescriptorProto::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::FileDescriptorProto, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::FileDescriptorProto::from(
                <FileDescriptorProto as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<FileDescriptorProto> for ::prost_types::FileDescriptorProto {
    #[allow(deprecated)]
    fn from(other: FileDescriptorProto) -> ::prost_types::FileDescriptorProto {
        ::prost_types::FileDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            package: other.package.map(::core::convert::From::from),
            dependency: other
                .dependency
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            public_dependency: other
                .public_dependency
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            weak_dependency: other
                .weak_dependency
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            message_type: other
                .message_type
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            enum_type: other
                .enum_type
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            service: other
                .service
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            extension: other
                .extension
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            options: other.options.map(::core::convert::From::from),
            source_code_info: other.source_code_info.map(::core::convert::From::from),
            syntax: other.syntax.map(::core::convert::From::from),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::FileDescriptorProto> for FileDescriptorProto {
    #[allow(deprecated)]
    fn from(other: ::prost_types::FileDescriptorProto) -> FileDescriptorProto {
        FileDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            package: other.package.map(::core::convert::From::from),
            dependency: other
                .dependency
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            public_dependency: other
                .public_dependency
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            weak_dependency: other
                .weak_dependency
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            message_type: other
                .message_type
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            enum_type: other
                .enum_type
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            service: other
                .service
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            extension: other
                .extension
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            options: other.options.map(::core::convert::From::from),
            source_code_info: other.source_code_info.map(::core::convert::From::from),
            syntax: other.syntax.map(::core::convert::From::from),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub field: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, repeated, tag = "6")]
    pub extension: ::prost::alloc::vec::Vec<FieldDescriptorProto>,
    #[prost(message, repeated, tag = "3")]
    pub nested_type: ::prost::alloc::vec::Vec<DescriptorProto>,
    #[prost(message, repeated, tag = "4")]
    pub enum_type: ::prost::alloc::vec::Vec<EnumDescriptorProto>,
    #[prost(message, repeated, tag = "5")]
    pub extension_range: ::prost::alloc::vec::Vec<descriptor_proto::ExtensionRange>,
    #[prost(message, repeated, tag = "8")]
    pub oneof_decl: ::prost::alloc::vec::Vec<OneofDescriptorProto>,
    #[prost(message, optional, tag = "7")]
    pub options: ::core::option::Option<MessageOptions>,
    #[prost(message, repeated, tag = "9")]
    pub reserved_range: ::prost::alloc::vec::Vec<descriptor_proto::ReservedRange>,
    #[prost(string, repeated, tag = "10")]
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[cfg(feature = "prost-types")]
impl DescriptorProto {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::DescriptorProto,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <DescriptorProto as ::serde::Serialize>::serialize(
            &DescriptorProto::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::DescriptorProto, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::DescriptorProto::from(
                <DescriptorProto as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<DescriptorProto> for ::prost_types::DescriptorProto {
    #[allow(deprecated)]
    fn from(other: DescriptorProto) -> ::prost_types::DescriptorProto {
        ::prost_types::DescriptorProto {
            name: other.name.map(::core::convert::From::from),
            field: other.field.into_iter().map(::core::convert::From::from).collect(),
            extension: other
                .extension
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            nested_type: other
                .nested_type
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            enum_type: other
                .enum_type
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            extension_range: other
                .extension_range
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            oneof_decl: other
                .oneof_decl
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            options: other.options.map(::core::convert::From::from),
            reserved_range: other
                .reserved_range
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            reserved_name: other
                .reserved_name
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::DescriptorProto> for DescriptorProto {
    #[allow(deprecated)]
    fn from(other: ::prost_types::DescriptorProto) -> DescriptorProto {
        DescriptorProto {
            name: other.name.map(::core::convert::From::from),
            field: other.field.into_iter().map(::core::convert::From::from).collect(),
            extension: other
                .extension
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            nested_type: other
                .nested_type
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            enum_type: other
                .enum_type
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            extension_range: other
                .extension_range
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            oneof_decl: other
                .oneof_decl
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            options: other.options.map(::core::convert::From::from),
            reserved_range: other
                .reserved_range
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            reserved_name: other
                .reserved_name
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
/// Nested message and enum types in `DescriptorProto`.
pub mod descriptor_proto {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExtensionRange {
        #[prost(int32, optional, tag = "1")]
        pub start: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub end: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "3")]
        pub options: ::core::option::Option<super::ExtensionRangeOptions>,
    }
    #[cfg(feature = "prost-types")]
    impl ExtensionRange {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::descriptor_proto::ExtensionRange,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <ExtensionRange as ::serde::Serialize>::serialize(
                &ExtensionRange::from(item.clone()),
                s,
            )
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::descriptor_proto::ExtensionRange, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::descriptor_proto::ExtensionRange::from(
                    <ExtensionRange as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<ExtensionRange>
    for ::prost_types::descriptor_proto::ExtensionRange {
        #[allow(deprecated)]
        fn from(
            other: ExtensionRange,
        ) -> ::prost_types::descriptor_proto::ExtensionRange {
            ::prost_types::descriptor_proto::ExtensionRange {
                start: other.start.map(::core::convert::From::from),
                end: other.end.map(::core::convert::From::from),
                options: other.options.map(::core::convert::From::from),
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::descriptor_proto::ExtensionRange>
    for ExtensionRange {
        #[allow(deprecated)]
        fn from(
            other: ::prost_types::descriptor_proto::ExtensionRange,
        ) -> ExtensionRange {
            ExtensionRange {
                start: other.start.map(::core::convert::From::from),
                end: other.end.map(::core::convert::From::from),
                options: other.options.map(::core::convert::From::from),
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReservedRange {
        #[prost(int32, optional, tag = "1")]
        pub start: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub end: ::core::option::Option<i32>,
    }
    #[cfg(feature = "prost-types")]
    impl ReservedRange {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::descriptor_proto::ReservedRange,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <ReservedRange as ::serde::Serialize>::serialize(
                &ReservedRange::from(item.clone()),
                s,
            )
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::descriptor_proto::ReservedRange, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::descriptor_proto::ReservedRange::from(
                    <ReservedRange as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<ReservedRange>
    for ::prost_types::descriptor_proto::ReservedRange {
        #[allow(deprecated)]
        fn from(other: ReservedRange) -> ::prost_types::descriptor_proto::ReservedRange {
            ::prost_types::descriptor_proto::ReservedRange {
                start: other.start.map(::core::convert::From::from),
                end: other.end.map(::core::convert::From::from),
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::descriptor_proto::ReservedRange>
    for ReservedRange {
        #[allow(deprecated)]
        fn from(other: ::prost_types::descriptor_proto::ReservedRange) -> ReservedRange {
            ReservedRange {
                start: other.start.map(::core::convert::From::from),
                end: other.end.map(::core::convert::From::from),
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionRangeOptions {
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[cfg(feature = "prost-types")]
impl ExtensionRangeOptions {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::ExtensionRangeOptions,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <ExtensionRangeOptions as ::serde::Serialize>::serialize(
            &ExtensionRangeOptions::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::ExtensionRangeOptions, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::ExtensionRangeOptions::from(
                <ExtensionRangeOptions as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<ExtensionRangeOptions>
for ::prost_types::ExtensionRangeOptions {
    #[allow(deprecated)]
    fn from(other: ExtensionRangeOptions) -> ::prost_types::ExtensionRangeOptions {
        ::prost_types::ExtensionRangeOptions {
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::ExtensionRangeOptions>
for ExtensionRangeOptions {
    #[allow(deprecated)]
    fn from(other: ::prost_types::ExtensionRangeOptions) -> ExtensionRangeOptions {
        ExtensionRangeOptions {
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub number: ::core::option::Option<i32>,
    #[prost(enumeration = "field_descriptor_proto::Label", optional, tag = "4")]
    pub label: ::core::option::Option<i32>,
    #[prost(enumeration = "field_descriptor_proto::Type", optional, tag = "5")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub type_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub extendee: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub default_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "9")]
    pub oneof_index: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "10")]
    pub json_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "8")]
    pub options: ::core::option::Option<FieldOptions>,
    #[prost(bool, optional, tag = "17")]
    pub proto3_optional: ::core::option::Option<bool>,
}
#[cfg(feature = "prost-types")]
impl FieldDescriptorProto {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::FieldDescriptorProto,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <FieldDescriptorProto as ::serde::Serialize>::serialize(
            &FieldDescriptorProto::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::FieldDescriptorProto, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::FieldDescriptorProto::from(
                <FieldDescriptorProto as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<FieldDescriptorProto>
for ::prost_types::FieldDescriptorProto {
    #[allow(deprecated)]
    fn from(other: FieldDescriptorProto) -> ::prost_types::FieldDescriptorProto {
        ::prost_types::FieldDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            number: other.number.map(::core::convert::From::from),
            label: other.label.map(::core::convert::From::from),
            r#type: other.r#type.map(::core::convert::From::from),
            type_name: other.type_name.map(::core::convert::From::from),
            extendee: other.extendee.map(::core::convert::From::from),
            default_value: other.default_value.map(::core::convert::From::from),
            oneof_index: other.oneof_index.map(::core::convert::From::from),
            json_name: other.json_name.map(::core::convert::From::from),
            options: other.options.map(::core::convert::From::from),
            proto3_optional: other.proto3_optional.map(::core::convert::From::from),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::FieldDescriptorProto>
for FieldDescriptorProto {
    #[allow(deprecated)]
    fn from(other: ::prost_types::FieldDescriptorProto) -> FieldDescriptorProto {
        FieldDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            number: other.number.map(::core::convert::From::from),
            label: other.label.map(::core::convert::From::from),
            r#type: other.r#type.map(::core::convert::From::from),
            type_name: other.type_name.map(::core::convert::From::from),
            extendee: other.extendee.map(::core::convert::From::from),
            default_value: other.default_value.map(::core::convert::From::from),
            oneof_index: other.oneof_index.map(::core::convert::From::from),
            json_name: other.json_name.map(::core::convert::From::from),
            options: other.options.map(::core::convert::From::from),
            proto3_optional: other.proto3_optional.map(::core::convert::From::from),
        }
    }
}
/// Nested message and enum types in `FieldDescriptorProto`.
pub mod field_descriptor_proto {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Double = 1,
        Float = 2,
        Int64 = 3,
        Uint64 = 4,
        Int32 = 5,
        Fixed64 = 6,
        Fixed32 = 7,
        Bool = 8,
        String = 9,
        Group = 10,
        Message = 11,
        Bytes = 12,
        Uint32 = 13,
        Enum = 14,
        Sfixed32 = 15,
        Sfixed64 = 16,
        Sint32 = 17,
        Sint64 = 18,
    }
    #[cfg(feature = "prost-types")]
    impl Type {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::field_descriptor_proto::Type,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <Type as ::serde::Serialize>::serialize(&Type::from(item.clone()), s)
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::field_descriptor_proto::Type, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::field_descriptor_proto::Type::from(
                    <Type as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<Type> for ::prost_types::field_descriptor_proto::Type {
        #[allow(deprecated)]
        fn from(pbjson: Type) -> ::prost_types::field_descriptor_proto::Type {
            match pbjson {
                Type::Double => ::prost_types::field_descriptor_proto::Type::Double,
                Type::Float => ::prost_types::field_descriptor_proto::Type::Float,
                Type::Int64 => ::prost_types::field_descriptor_proto::Type::Int64,
                Type::Uint64 => ::prost_types::field_descriptor_proto::Type::Uint64,
                Type::Int32 => ::prost_types::field_descriptor_proto::Type::Int32,
                Type::Fixed64 => ::prost_types::field_descriptor_proto::Type::Fixed64,
                Type::Fixed32 => ::prost_types::field_descriptor_proto::Type::Fixed32,
                Type::Bool => ::prost_types::field_descriptor_proto::Type::Bool,
                Type::String => ::prost_types::field_descriptor_proto::Type::String,
                Type::Group => ::prost_types::field_descriptor_proto::Type::Group,
                Type::Message => ::prost_types::field_descriptor_proto::Type::Message,
                Type::Bytes => ::prost_types::field_descriptor_proto::Type::Bytes,
                Type::Uint32 => ::prost_types::field_descriptor_proto::Type::Uint32,
                Type::Enum => ::prost_types::field_descriptor_proto::Type::Enum,
                Type::Sfixed32 => ::prost_types::field_descriptor_proto::Type::Sfixed32,
                Type::Sfixed64 => ::prost_types::field_descriptor_proto::Type::Sfixed64,
                Type::Sint32 => ::prost_types::field_descriptor_proto::Type::Sint32,
                Type::Sint64 => ::prost_types::field_descriptor_proto::Type::Sint64,
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::field_descriptor_proto::Type> for Type {
        #[allow(deprecated)]
        fn from(prost: ::prost_types::field_descriptor_proto::Type) -> Type {
            match prost {
                ::prost_types::field_descriptor_proto::Type::Double => Type::Double,
                ::prost_types::field_descriptor_proto::Type::Float => Type::Float,
                ::prost_types::field_descriptor_proto::Type::Int64 => Type::Int64,
                ::prost_types::field_descriptor_proto::Type::Uint64 => Type::Uint64,
                ::prost_types::field_descriptor_proto::Type::Int32 => Type::Int32,
                ::prost_types::field_descriptor_proto::Type::Fixed64 => Type::Fixed64,
                ::prost_types::field_descriptor_proto::Type::Fixed32 => Type::Fixed32,
                ::prost_types::field_descriptor_proto::Type::Bool => Type::Bool,
                ::prost_types::field_descriptor_proto::Type::String => Type::String,
                ::prost_types::field_descriptor_proto::Type::Group => Type::Group,
                ::prost_types::field_descriptor_proto::Type::Message => Type::Message,
                ::prost_types::field_descriptor_proto::Type::Bytes => Type::Bytes,
                ::prost_types::field_descriptor_proto::Type::Uint32 => Type::Uint32,
                ::prost_types::field_descriptor_proto::Type::Enum => Type::Enum,
                ::prost_types::field_descriptor_proto::Type::Sfixed32 => Type::Sfixed32,
                ::prost_types::field_descriptor_proto::Type::Sfixed64 => Type::Sfixed64,
                ::prost_types::field_descriptor_proto::Type::Sint32 => Type::Sint32,
                ::prost_types::field_descriptor_proto::Type::Sint64 => Type::Sint64,
            }
        }
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Double => "TYPE_DOUBLE",
                Type::Float => "TYPE_FLOAT",
                Type::Int64 => "TYPE_INT64",
                Type::Uint64 => "TYPE_UINT64",
                Type::Int32 => "TYPE_INT32",
                Type::Fixed64 => "TYPE_FIXED64",
                Type::Fixed32 => "TYPE_FIXED32",
                Type::Bool => "TYPE_BOOL",
                Type::String => "TYPE_STRING",
                Type::Group => "TYPE_GROUP",
                Type::Message => "TYPE_MESSAGE",
                Type::Bytes => "TYPE_BYTES",
                Type::Uint32 => "TYPE_UINT32",
                Type::Enum => "TYPE_ENUM",
                Type::Sfixed32 => "TYPE_SFIXED32",
                Type::Sfixed64 => "TYPE_SFIXED64",
                Type::Sint32 => "TYPE_SINT32",
                Type::Sint64 => "TYPE_SINT64",
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Label {
        Optional = 1,
        Required = 2,
        Repeated = 3,
    }
    #[cfg(feature = "prost-types")]
    impl Label {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::field_descriptor_proto::Label,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <Label as ::serde::Serialize>::serialize(&Label::from(item.clone()), s)
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::field_descriptor_proto::Label, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::field_descriptor_proto::Label::from(
                    <Label as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<Label> for ::prost_types::field_descriptor_proto::Label {
        #[allow(deprecated)]
        fn from(pbjson: Label) -> ::prost_types::field_descriptor_proto::Label {
            match pbjson {
                Label::Optional => ::prost_types::field_descriptor_proto::Label::Optional,
                Label::Required => ::prost_types::field_descriptor_proto::Label::Required,
                Label::Repeated => ::prost_types::field_descriptor_proto::Label::Repeated,
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::field_descriptor_proto::Label> for Label {
        #[allow(deprecated)]
        fn from(prost: ::prost_types::field_descriptor_proto::Label) -> Label {
            match prost {
                ::prost_types::field_descriptor_proto::Label::Optional => Label::Optional,
                ::prost_types::field_descriptor_proto::Label::Required => Label::Required,
                ::prost_types::field_descriptor_proto::Label::Repeated => Label::Repeated,
            }
        }
    }
    impl Label {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Label::Optional => "LABEL_OPTIONAL",
                Label::Required => "LABEL_REQUIRED",
                Label::Repeated => "LABEL_REPEATED",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneofDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<OneofOptions>,
}
#[cfg(feature = "prost-types")]
impl OneofDescriptorProto {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::OneofDescriptorProto,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <OneofDescriptorProto as ::serde::Serialize>::serialize(
            &OneofDescriptorProto::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::OneofDescriptorProto, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::OneofDescriptorProto::from(
                <OneofDescriptorProto as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<OneofDescriptorProto>
for ::prost_types::OneofDescriptorProto {
    #[allow(deprecated)]
    fn from(other: OneofDescriptorProto) -> ::prost_types::OneofDescriptorProto {
        ::prost_types::OneofDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            options: other.options.map(::core::convert::From::from),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::OneofDescriptorProto>
for OneofDescriptorProto {
    #[allow(deprecated)]
    fn from(other: ::prost_types::OneofDescriptorProto) -> OneofDescriptorProto {
        OneofDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            options: other.options.map(::core::convert::From::from),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub value: ::prost::alloc::vec::Vec<EnumValueDescriptorProto>,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<EnumOptions>,
    #[prost(message, repeated, tag = "4")]
    pub reserved_range: ::prost::alloc::vec::Vec<
        enum_descriptor_proto::EnumReservedRange,
    >,
    #[prost(string, repeated, tag = "5")]
    pub reserved_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[cfg(feature = "prost-types")]
impl EnumDescriptorProto {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::EnumDescriptorProto,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <EnumDescriptorProto as ::serde::Serialize>::serialize(
            &EnumDescriptorProto::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::EnumDescriptorProto, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::EnumDescriptorProto::from(
                <EnumDescriptorProto as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<EnumDescriptorProto> for ::prost_types::EnumDescriptorProto {
    #[allow(deprecated)]
    fn from(other: EnumDescriptorProto) -> ::prost_types::EnumDescriptorProto {
        ::prost_types::EnumDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            value: other.value.into_iter().map(::core::convert::From::from).collect(),
            options: other.options.map(::core::convert::From::from),
            reserved_range: other
                .reserved_range
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            reserved_name: other
                .reserved_name
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::EnumDescriptorProto> for EnumDescriptorProto {
    #[allow(deprecated)]
    fn from(other: ::prost_types::EnumDescriptorProto) -> EnumDescriptorProto {
        EnumDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            value: other.value.into_iter().map(::core::convert::From::from).collect(),
            options: other.options.map(::core::convert::From::from),
            reserved_range: other
                .reserved_range
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
            reserved_name: other
                .reserved_name
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
/// Nested message and enum types in `EnumDescriptorProto`.
pub mod enum_descriptor_proto {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumReservedRange {
        #[prost(int32, optional, tag = "1")]
        pub start: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub end: ::core::option::Option<i32>,
    }
    #[cfg(feature = "prost-types")]
    impl EnumReservedRange {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::enum_descriptor_proto::EnumReservedRange,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <EnumReservedRange as ::serde::Serialize>::serialize(
                &EnumReservedRange::from(item.clone()),
                s,
            )
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::enum_descriptor_proto::EnumReservedRange, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::enum_descriptor_proto::EnumReservedRange::from(
                    <EnumReservedRange as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<EnumReservedRange>
    for ::prost_types::enum_descriptor_proto::EnumReservedRange {
        #[allow(deprecated)]
        fn from(
            other: EnumReservedRange,
        ) -> ::prost_types::enum_descriptor_proto::EnumReservedRange {
            ::prost_types::enum_descriptor_proto::EnumReservedRange {
                start: other.start.map(::core::convert::From::from),
                end: other.end.map(::core::convert::From::from),
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::enum_descriptor_proto::EnumReservedRange>
    for EnumReservedRange {
        #[allow(deprecated)]
        fn from(
            other: ::prost_types::enum_descriptor_proto::EnumReservedRange,
        ) -> EnumReservedRange {
            EnumReservedRange {
                start: other.start.map(::core::convert::From::from),
                end: other.end.map(::core::convert::From::from),
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValueDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub number: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<EnumValueOptions>,
}
#[cfg(feature = "prost-types")]
impl EnumValueDescriptorProto {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::EnumValueDescriptorProto,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <EnumValueDescriptorProto as ::serde::Serialize>::serialize(
            &EnumValueDescriptorProto::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::EnumValueDescriptorProto, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::EnumValueDescriptorProto::from(
                <EnumValueDescriptorProto as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<EnumValueDescriptorProto>
for ::prost_types::EnumValueDescriptorProto {
    #[allow(deprecated)]
    fn from(other: EnumValueDescriptorProto) -> ::prost_types::EnumValueDescriptorProto {
        ::prost_types::EnumValueDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            number: other.number.map(::core::convert::From::from),
            options: other.options.map(::core::convert::From::from),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::EnumValueDescriptorProto>
for EnumValueDescriptorProto {
    #[allow(deprecated)]
    fn from(other: ::prost_types::EnumValueDescriptorProto) -> EnumValueDescriptorProto {
        EnumValueDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            number: other.number.map(::core::convert::From::from),
            options: other.options.map(::core::convert::From::from),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub method: ::prost::alloc::vec::Vec<MethodDescriptorProto>,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<ServiceOptions>,
}
#[cfg(feature = "prost-types")]
impl ServiceDescriptorProto {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::ServiceDescriptorProto,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <ServiceDescriptorProto as ::serde::Serialize>::serialize(
            &ServiceDescriptorProto::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::ServiceDescriptorProto, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::ServiceDescriptorProto::from(
                <ServiceDescriptorProto as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<ServiceDescriptorProto>
for ::prost_types::ServiceDescriptorProto {
    #[allow(deprecated)]
    fn from(other: ServiceDescriptorProto) -> ::prost_types::ServiceDescriptorProto {
        ::prost_types::ServiceDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            method: other.method.into_iter().map(::core::convert::From::from).collect(),
            options: other.options.map(::core::convert::From::from),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::ServiceDescriptorProto>
for ServiceDescriptorProto {
    #[allow(deprecated)]
    fn from(other: ::prost_types::ServiceDescriptorProto) -> ServiceDescriptorProto {
        ServiceDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            method: other.method.into_iter().map(::core::convert::From::from).collect(),
            options: other.options.map(::core::convert::From::from),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodDescriptorProto {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub input_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub output_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub options: ::core::option::Option<MethodOptions>,
    #[prost(bool, optional, tag = "5", default = "false")]
    pub client_streaming: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6", default = "false")]
    pub server_streaming: ::core::option::Option<bool>,
}
#[cfg(feature = "prost-types")]
impl MethodDescriptorProto {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::MethodDescriptorProto,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <MethodDescriptorProto as ::serde::Serialize>::serialize(
            &MethodDescriptorProto::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::MethodDescriptorProto, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::MethodDescriptorProto::from(
                <MethodDescriptorProto as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<MethodDescriptorProto>
for ::prost_types::MethodDescriptorProto {
    #[allow(deprecated)]
    fn from(other: MethodDescriptorProto) -> ::prost_types::MethodDescriptorProto {
        ::prost_types::MethodDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            input_type: other.input_type.map(::core::convert::From::from),
            output_type: other.output_type.map(::core::convert::From::from),
            options: other.options.map(::core::convert::From::from),
            client_streaming: other.client_streaming.map(::core::convert::From::from),
            server_streaming: other.server_streaming.map(::core::convert::From::from),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::MethodDescriptorProto>
for MethodDescriptorProto {
    #[allow(deprecated)]
    fn from(other: ::prost_types::MethodDescriptorProto) -> MethodDescriptorProto {
        MethodDescriptorProto {
            name: other.name.map(::core::convert::From::from),
            input_type: other.input_type.map(::core::convert::From::from),
            output_type: other.output_type.map(::core::convert::From::from),
            options: other.options.map(::core::convert::From::from),
            client_streaming: other.client_streaming.map(::core::convert::From::from),
            server_streaming: other.server_streaming.map(::core::convert::From::from),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileOptions {
    #[prost(string, optional, tag = "1")]
    pub java_package: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub java_outer_classname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "10", default = "false")]
    pub java_multiple_files: ::core::option::Option<bool>,
    #[deprecated]
    #[prost(bool, optional, tag = "20")]
    pub java_generate_equals_and_hash: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "27", default = "false")]
    pub java_string_check_utf8: ::core::option::Option<bool>,
    #[prost(
        enumeration = "file_options::OptimizeMode",
        optional,
        tag = "9",
        default = "Speed"
    )]
    pub optimize_for: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "11")]
    pub go_package: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "16", default = "false")]
    pub cc_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "17", default = "false")]
    pub java_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "18", default = "false")]
    pub py_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "42", default = "false")]
    pub php_generic_services: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "23", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "31", default = "true")]
    pub cc_enable_arenas: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "36")]
    pub objc_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "37")]
    pub csharp_namespace: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "39")]
    pub swift_prefix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "40")]
    pub php_class_prefix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "41")]
    pub php_namespace: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "44")]
    pub php_metadata_namespace: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "45")]
    pub ruby_package: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[cfg(feature = "prost-types")]
impl FileOptions {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::FileOptions,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <FileOptions as ::serde::Serialize>::serialize(
            &FileOptions::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::FileOptions, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::FileOptions::from(
                <FileOptions as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<FileOptions> for ::prost_types::FileOptions {
    #[allow(deprecated)]
    fn from(other: FileOptions) -> ::prost_types::FileOptions {
        ::prost_types::FileOptions {
            java_package: other.java_package.map(::core::convert::From::from),
            java_outer_classname: other
                .java_outer_classname
                .map(::core::convert::From::from),
            java_multiple_files: other
                .java_multiple_files
                .map(::core::convert::From::from),
            java_generate_equals_and_hash: other
                .java_generate_equals_and_hash
                .map(::core::convert::From::from),
            java_string_check_utf8: other
                .java_string_check_utf8
                .map(::core::convert::From::from),
            optimize_for: other.optimize_for.map(::core::convert::From::from),
            go_package: other.go_package.map(::core::convert::From::from),
            cc_generic_services: other
                .cc_generic_services
                .map(::core::convert::From::from),
            java_generic_services: other
                .java_generic_services
                .map(::core::convert::From::from),
            py_generic_services: other
                .py_generic_services
                .map(::core::convert::From::from),
            php_generic_services: other
                .php_generic_services
                .map(::core::convert::From::from),
            deprecated: other.deprecated.map(::core::convert::From::from),
            cc_enable_arenas: other.cc_enable_arenas.map(::core::convert::From::from),
            objc_class_prefix: other.objc_class_prefix.map(::core::convert::From::from),
            csharp_namespace: other.csharp_namespace.map(::core::convert::From::from),
            swift_prefix: other.swift_prefix.map(::core::convert::From::from),
            php_class_prefix: other.php_class_prefix.map(::core::convert::From::from),
            php_namespace: other.php_namespace.map(::core::convert::From::from),
            php_metadata_namespace: other
                .php_metadata_namespace
                .map(::core::convert::From::from),
            ruby_package: other.ruby_package.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::FileOptions> for FileOptions {
    #[allow(deprecated)]
    fn from(other: ::prost_types::FileOptions) -> FileOptions {
        FileOptions {
            java_package: other.java_package.map(::core::convert::From::from),
            java_outer_classname: other
                .java_outer_classname
                .map(::core::convert::From::from),
            java_multiple_files: other
                .java_multiple_files
                .map(::core::convert::From::from),
            java_generate_equals_and_hash: other
                .java_generate_equals_and_hash
                .map(::core::convert::From::from),
            java_string_check_utf8: other
                .java_string_check_utf8
                .map(::core::convert::From::from),
            optimize_for: other.optimize_for.map(::core::convert::From::from),
            go_package: other.go_package.map(::core::convert::From::from),
            cc_generic_services: other
                .cc_generic_services
                .map(::core::convert::From::from),
            java_generic_services: other
                .java_generic_services
                .map(::core::convert::From::from),
            py_generic_services: other
                .py_generic_services
                .map(::core::convert::From::from),
            php_generic_services: other
                .php_generic_services
                .map(::core::convert::From::from),
            deprecated: other.deprecated.map(::core::convert::From::from),
            cc_enable_arenas: other.cc_enable_arenas.map(::core::convert::From::from),
            objc_class_prefix: other.objc_class_prefix.map(::core::convert::From::from),
            csharp_namespace: other.csharp_namespace.map(::core::convert::From::from),
            swift_prefix: other.swift_prefix.map(::core::convert::From::from),
            php_class_prefix: other.php_class_prefix.map(::core::convert::From::from),
            php_namespace: other.php_namespace.map(::core::convert::From::from),
            php_metadata_namespace: other
                .php_metadata_namespace
                .map(::core::convert::From::from),
            ruby_package: other.ruby_package.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
/// Nested message and enum types in `FileOptions`.
pub mod file_options {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OptimizeMode {
        Speed = 1,
        CodeSize = 2,
        LiteRuntime = 3,
    }
    #[cfg(feature = "prost-types")]
    impl OptimizeMode {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::file_options::OptimizeMode,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <OptimizeMode as ::serde::Serialize>::serialize(
                &OptimizeMode::from(item.clone()),
                s,
            )
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::file_options::OptimizeMode, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::file_options::OptimizeMode::from(
                    <OptimizeMode as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<OptimizeMode>
    for ::prost_types::file_options::OptimizeMode {
        #[allow(deprecated)]
        fn from(pbjson: OptimizeMode) -> ::prost_types::file_options::OptimizeMode {
            match pbjson {
                OptimizeMode::Speed => ::prost_types::file_options::OptimizeMode::Speed,
                OptimizeMode::CodeSize => {
                    ::prost_types::file_options::OptimizeMode::CodeSize
                }
                OptimizeMode::LiteRuntime => {
                    ::prost_types::file_options::OptimizeMode::LiteRuntime
                }
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::file_options::OptimizeMode>
    for OptimizeMode {
        #[allow(deprecated)]
        fn from(prost: ::prost_types::file_options::OptimizeMode) -> OptimizeMode {
            match prost {
                ::prost_types::file_options::OptimizeMode::Speed => OptimizeMode::Speed,
                ::prost_types::file_options::OptimizeMode::CodeSize => {
                    OptimizeMode::CodeSize
                }
                ::prost_types::file_options::OptimizeMode::LiteRuntime => {
                    OptimizeMode::LiteRuntime
                }
            }
        }
    }
    impl OptimizeMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OptimizeMode::Speed => "SPEED",
                OptimizeMode::CodeSize => "CODE_SIZE",
                OptimizeMode::LiteRuntime => "LITE_RUNTIME",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageOptions {
    #[prost(bool, optional, tag = "1", default = "false")]
    pub message_set_wire_format: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "2", default = "false")]
    pub no_standard_descriptor_accessor: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub map_entry: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[cfg(feature = "prost-types")]
impl MessageOptions {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::MessageOptions,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <MessageOptions as ::serde::Serialize>::serialize(
            &MessageOptions::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::MessageOptions, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::MessageOptions::from(
                <MessageOptions as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<MessageOptions> for ::prost_types::MessageOptions {
    #[allow(deprecated)]
    fn from(other: MessageOptions) -> ::prost_types::MessageOptions {
        ::prost_types::MessageOptions {
            message_set_wire_format: other
                .message_set_wire_format
                .map(::core::convert::From::from),
            no_standard_descriptor_accessor: other
                .no_standard_descriptor_accessor
                .map(::core::convert::From::from),
            deprecated: other.deprecated.map(::core::convert::From::from),
            map_entry: other.map_entry.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::MessageOptions> for MessageOptions {
    #[allow(deprecated)]
    fn from(other: ::prost_types::MessageOptions) -> MessageOptions {
        MessageOptions {
            message_set_wire_format: other
                .message_set_wire_format
                .map(::core::convert::From::from),
            no_standard_descriptor_accessor: other
                .no_standard_descriptor_accessor
                .map(::core::convert::From::from),
            deprecated: other.deprecated.map(::core::convert::From::from),
            map_entry: other.map_entry.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldOptions {
    #[prost(
        enumeration = "field_options::CType",
        optional,
        tag = "1",
        default = "String"
    )]
    pub ctype: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub packed: ::core::option::Option<bool>,
    #[prost(
        enumeration = "field_options::JsType",
        optional,
        tag = "6",
        default = "JsNormal"
    )]
    pub jstype: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5", default = "false")]
    pub lazy: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10", default = "false")]
    pub weak: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[cfg(feature = "prost-types")]
impl FieldOptions {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::FieldOptions,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <FieldOptions as ::serde::Serialize>::serialize(
            &FieldOptions::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::FieldOptions, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::FieldOptions::from(
                <FieldOptions as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<FieldOptions> for ::prost_types::FieldOptions {
    #[allow(deprecated)]
    fn from(other: FieldOptions) -> ::prost_types::FieldOptions {
        ::prost_types::FieldOptions {
            ctype: other.ctype.map(::core::convert::From::from),
            packed: other.packed.map(::core::convert::From::from),
            jstype: other.jstype.map(::core::convert::From::from),
            lazy: other.lazy.map(::core::convert::From::from),
            deprecated: other.deprecated.map(::core::convert::From::from),
            weak: other.weak.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::FieldOptions> for FieldOptions {
    #[allow(deprecated)]
    fn from(other: ::prost_types::FieldOptions) -> FieldOptions {
        FieldOptions {
            ctype: other.ctype.map(::core::convert::From::from),
            packed: other.packed.map(::core::convert::From::from),
            jstype: other.jstype.map(::core::convert::From::from),
            lazy: other.lazy.map(::core::convert::From::from),
            deprecated: other.deprecated.map(::core::convert::From::from),
            weak: other.weak.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
/// Nested message and enum types in `FieldOptions`.
pub mod field_options {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CType {
        String = 0,
        Cord = 1,
        StringPiece = 2,
    }
    #[cfg(feature = "prost-types")]
    impl CType {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::field_options::CType,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <CType as ::serde::Serialize>::serialize(&CType::from(item.clone()), s)
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::field_options::CType, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::field_options::CType::from(
                    <CType as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<CType> for ::prost_types::field_options::CType {
        #[allow(deprecated)]
        fn from(pbjson: CType) -> ::prost_types::field_options::CType {
            match pbjson {
                CType::String => ::prost_types::field_options::CType::String,
                CType::Cord => ::prost_types::field_options::CType::Cord,
                CType::StringPiece => ::prost_types::field_options::CType::StringPiece,
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::field_options::CType> for CType {
        #[allow(deprecated)]
        fn from(prost: ::prost_types::field_options::CType) -> CType {
            match prost {
                ::prost_types::field_options::CType::String => CType::String,
                ::prost_types::field_options::CType::Cord => CType::Cord,
                ::prost_types::field_options::CType::StringPiece => CType::StringPiece,
            }
        }
    }
    impl CType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CType::String => "STRING",
                CType::Cord => "CORD",
                CType::StringPiece => "STRING_PIECE",
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum JsType {
        JsNormal = 0,
        JsString = 1,
        JsNumber = 2,
    }
    #[cfg(feature = "prost-types")]
    impl JsType {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::field_options::JsType,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <JsType as ::serde::Serialize>::serialize(&JsType::from(item.clone()), s)
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::field_options::JsType, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::field_options::JsType::from(
                    <JsType as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<JsType> for ::prost_types::field_options::JsType {
        #[allow(deprecated)]
        fn from(pbjson: JsType) -> ::prost_types::field_options::JsType {
            match pbjson {
                JsType::JsNormal => ::prost_types::field_options::JsType::JsNormal,
                JsType::JsString => ::prost_types::field_options::JsType::JsString,
                JsType::JsNumber => ::prost_types::field_options::JsType::JsNumber,
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::field_options::JsType> for JsType {
        #[allow(deprecated)]
        fn from(prost: ::prost_types::field_options::JsType) -> JsType {
            match prost {
                ::prost_types::field_options::JsType::JsNormal => JsType::JsNormal,
                ::prost_types::field_options::JsType::JsString => JsType::JsString,
                ::prost_types::field_options::JsType::JsNumber => JsType::JsNumber,
            }
        }
    }
    impl JsType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JsType::JsNormal => "JS_NORMAL",
                JsType::JsString => "JS_STRING",
                JsType::JsNumber => "JS_NUMBER",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneofOptions {
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[cfg(feature = "prost-types")]
impl OneofOptions {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::OneofOptions,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <OneofOptions as ::serde::Serialize>::serialize(
            &OneofOptions::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::OneofOptions, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::OneofOptions::from(
                <OneofOptions as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<OneofOptions> for ::prost_types::OneofOptions {
    #[allow(deprecated)]
    fn from(other: OneofOptions) -> ::prost_types::OneofOptions {
        ::prost_types::OneofOptions {
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::OneofOptions> for OneofOptions {
    #[allow(deprecated)]
    fn from(other: ::prost_types::OneofOptions) -> OneofOptions {
        OneofOptions {
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumOptions {
    #[prost(bool, optional, tag = "2")]
    pub allow_alias: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[cfg(feature = "prost-types")]
impl EnumOptions {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::EnumOptions,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <EnumOptions as ::serde::Serialize>::serialize(
            &EnumOptions::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::EnumOptions, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::EnumOptions::from(
                <EnumOptions as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<EnumOptions> for ::prost_types::EnumOptions {
    #[allow(deprecated)]
    fn from(other: EnumOptions) -> ::prost_types::EnumOptions {
        ::prost_types::EnumOptions {
            allow_alias: other.allow_alias.map(::core::convert::From::from),
            deprecated: other.deprecated.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::EnumOptions> for EnumOptions {
    #[allow(deprecated)]
    fn from(other: ::prost_types::EnumOptions) -> EnumOptions {
        EnumOptions {
            allow_alias: other.allow_alias.map(::core::convert::From::from),
            deprecated: other.deprecated.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValueOptions {
    #[prost(bool, optional, tag = "1", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[cfg(feature = "prost-types")]
impl EnumValueOptions {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::EnumValueOptions,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <EnumValueOptions as ::serde::Serialize>::serialize(
            &EnumValueOptions::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::EnumValueOptions, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::EnumValueOptions::from(
                <EnumValueOptions as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<EnumValueOptions> for ::prost_types::EnumValueOptions {
    #[allow(deprecated)]
    fn from(other: EnumValueOptions) -> ::prost_types::EnumValueOptions {
        ::prost_types::EnumValueOptions {
            deprecated: other.deprecated.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::EnumValueOptions> for EnumValueOptions {
    #[allow(deprecated)]
    fn from(other: ::prost_types::EnumValueOptions) -> EnumValueOptions {
        EnumValueOptions {
            deprecated: other.deprecated.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceOptions {
    #[prost(bool, optional, tag = "33", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[cfg(feature = "prost-types")]
impl ServiceOptions {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::ServiceOptions,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <ServiceOptions as ::serde::Serialize>::serialize(
            &ServiceOptions::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::ServiceOptions, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::ServiceOptions::from(
                <ServiceOptions as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<ServiceOptions> for ::prost_types::ServiceOptions {
    #[allow(deprecated)]
    fn from(other: ServiceOptions) -> ::prost_types::ServiceOptions {
        ::prost_types::ServiceOptions {
            deprecated: other.deprecated.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::ServiceOptions> for ServiceOptions {
    #[allow(deprecated)]
    fn from(other: ::prost_types::ServiceOptions) -> ServiceOptions {
        ServiceOptions {
            deprecated: other.deprecated.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodOptions {
    #[prost(bool, optional, tag = "33", default = "false")]
    pub deprecated: ::core::option::Option<bool>,
    #[prost(
        enumeration = "method_options::IdempotencyLevel",
        optional,
        tag = "34",
        default = "IdempotencyUnknown"
    )]
    pub idempotency_level: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "999")]
    pub uninterpreted_option: ::prost::alloc::vec::Vec<UninterpretedOption>,
}
#[cfg(feature = "prost-types")]
impl MethodOptions {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::MethodOptions,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <MethodOptions as ::serde::Serialize>::serialize(
            &MethodOptions::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::MethodOptions, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::MethodOptions::from(
                <MethodOptions as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<MethodOptions> for ::prost_types::MethodOptions {
    #[allow(deprecated)]
    fn from(other: MethodOptions) -> ::prost_types::MethodOptions {
        ::prost_types::MethodOptions {
            deprecated: other.deprecated.map(::core::convert::From::from),
            idempotency_level: other.idempotency_level.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::MethodOptions> for MethodOptions {
    #[allow(deprecated)]
    fn from(other: ::prost_types::MethodOptions) -> MethodOptions {
        MethodOptions {
            deprecated: other.deprecated.map(::core::convert::From::from),
            idempotency_level: other.idempotency_level.map(::core::convert::From::from),
            uninterpreted_option: other
                .uninterpreted_option
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
/// Nested message and enum types in `MethodOptions`.
pub mod method_options {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum IdempotencyLevel {
        IdempotencyUnknown = 0,
        NoSideEffects = 1,
        Idempotent = 2,
    }
    #[cfg(feature = "prost-types")]
    impl IdempotencyLevel {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::method_options::IdempotencyLevel,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <IdempotencyLevel as ::serde::Serialize>::serialize(
                &IdempotencyLevel::from(item.clone()),
                s,
            )
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::method_options::IdempotencyLevel, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::method_options::IdempotencyLevel::from(
                    <IdempotencyLevel as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<IdempotencyLevel>
    for ::prost_types::method_options::IdempotencyLevel {
        #[allow(deprecated)]
        fn from(
            pbjson: IdempotencyLevel,
        ) -> ::prost_types::method_options::IdempotencyLevel {
            match pbjson {
                IdempotencyLevel::IdempotencyUnknown => {
                    ::prost_types::method_options::IdempotencyLevel::IdempotencyUnknown
                }
                IdempotencyLevel::NoSideEffects => {
                    ::prost_types::method_options::IdempotencyLevel::NoSideEffects
                }
                IdempotencyLevel::Idempotent => {
                    ::prost_types::method_options::IdempotencyLevel::Idempotent
                }
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::method_options::IdempotencyLevel>
    for IdempotencyLevel {
        #[allow(deprecated)]
        fn from(
            prost: ::prost_types::method_options::IdempotencyLevel,
        ) -> IdempotencyLevel {
            match prost {
                ::prost_types::method_options::IdempotencyLevel::IdempotencyUnknown => {
                    IdempotencyLevel::IdempotencyUnknown
                }
                ::prost_types::method_options::IdempotencyLevel::NoSideEffects => {
                    IdempotencyLevel::NoSideEffects
                }
                ::prost_types::method_options::IdempotencyLevel::Idempotent => {
                    IdempotencyLevel::Idempotent
                }
            }
        }
    }
    impl IdempotencyLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IdempotencyLevel::IdempotencyUnknown => "IDEMPOTENCY_UNKNOWN",
                IdempotencyLevel::NoSideEffects => "NO_SIDE_EFFECTS",
                IdempotencyLevel::Idempotent => "IDEMPOTENT",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UninterpretedOption {
    #[prost(message, repeated, tag = "2")]
    pub name: ::prost::alloc::vec::Vec<uninterpreted_option::NamePart>,
    #[prost(string, optional, tag = "3")]
    pub identifier_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "4")]
    pub positive_int_value: ::core::option::Option<u64>,
    #[prost(int64, optional, tag = "5")]
    pub negative_int_value: ::core::option::Option<i64>,
    #[prost(double, optional, tag = "6")]
    pub double_value: ::core::option::Option<f64>,
    #[prost(bytes = "bytes", optional, tag = "7")]
    pub string_value: ::core::option::Option<::prost::bytes::Bytes>,
    #[prost(string, optional, tag = "8")]
    pub aggregate_value: ::core::option::Option<::prost::alloc::string::String>,
}
#[cfg(feature = "prost-types")]
impl UninterpretedOption {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::UninterpretedOption,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <UninterpretedOption as ::serde::Serialize>::serialize(
            &UninterpretedOption::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::UninterpretedOption, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::UninterpretedOption::from(
                <UninterpretedOption as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<UninterpretedOption> for ::prost_types::UninterpretedOption {
    #[allow(deprecated)]
    fn from(other: UninterpretedOption) -> ::prost_types::UninterpretedOption {
        ::prost_types::UninterpretedOption {
            name: other.name.into_iter().map(::core::convert::From::from).collect(),
            identifier_value: other.identifier_value.map(::core::convert::From::from),
            positive_int_value: other
                .positive_int_value
                .map(::core::convert::From::from),
            negative_int_value: other
                .negative_int_value
                .map(::core::convert::From::from),
            double_value: other.double_value.map(::core::convert::From::from),
            string_value: other.string_value.map(::core::convert::From::from),
            aggregate_value: other.aggregate_value.map(::core::convert::From::from),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::UninterpretedOption> for UninterpretedOption {
    #[allow(deprecated)]
    fn from(other: ::prost_types::UninterpretedOption) -> UninterpretedOption {
        UninterpretedOption {
            name: other.name.into_iter().map(::core::convert::From::from).collect(),
            identifier_value: other.identifier_value.map(::core::convert::From::from),
            positive_int_value: other
                .positive_int_value
                .map(::core::convert::From::from),
            negative_int_value: other
                .negative_int_value
                .map(::core::convert::From::from),
            double_value: other.double_value.map(::core::convert::From::from),
            string_value: other.string_value.map(::core::convert::From::from),
            aggregate_value: other.aggregate_value.map(::core::convert::From::from),
        }
    }
}
/// Nested message and enum types in `UninterpretedOption`.
pub mod uninterpreted_option {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NamePart {
        #[prost(string, required, tag = "1")]
        pub name_part: ::prost::alloc::string::String,
        #[prost(bool, required, tag = "2")]
        pub is_extension: bool,
    }
    #[cfg(feature = "prost-types")]
    impl NamePart {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::uninterpreted_option::NamePart,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <NamePart as ::serde::Serialize>::serialize(&NamePart::from(item.clone()), s)
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::uninterpreted_option::NamePart, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::uninterpreted_option::NamePart::from(
                    <NamePart as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<NamePart>
    for ::prost_types::uninterpreted_option::NamePart {
        #[allow(deprecated)]
        fn from(other: NamePart) -> ::prost_types::uninterpreted_option::NamePart {
            ::prost_types::uninterpreted_option::NamePart {
                name_part: ::core::convert::From::from(other.name_part),
                is_extension: ::core::convert::From::from(other.is_extension),
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::uninterpreted_option::NamePart>
    for NamePart {
        #[allow(deprecated)]
        fn from(other: ::prost_types::uninterpreted_option::NamePart) -> NamePart {
            NamePart {
                name_part: ::core::convert::From::from(other.name_part),
                is_extension: ::core::convert::From::from(other.is_extension),
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceCodeInfo {
    #[prost(message, repeated, tag = "1")]
    pub location: ::prost::alloc::vec::Vec<source_code_info::Location>,
}
#[cfg(feature = "prost-types")]
impl SourceCodeInfo {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::SourceCodeInfo,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <SourceCodeInfo as ::serde::Serialize>::serialize(
            &SourceCodeInfo::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::SourceCodeInfo, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::SourceCodeInfo::from(
                <SourceCodeInfo as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<SourceCodeInfo> for ::prost_types::SourceCodeInfo {
    #[allow(deprecated)]
    fn from(other: SourceCodeInfo) -> ::prost_types::SourceCodeInfo {
        ::prost_types::SourceCodeInfo {
            location: other
                .location
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::SourceCodeInfo> for SourceCodeInfo {
    #[allow(deprecated)]
    fn from(other: ::prost_types::SourceCodeInfo) -> SourceCodeInfo {
        SourceCodeInfo {
            location: other
                .location
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
/// Nested message and enum types in `SourceCodeInfo`.
pub mod source_code_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Location {
        #[prost(int32, repeated, tag = "1")]
        pub path: ::prost::alloc::vec::Vec<i32>,
        #[prost(int32, repeated, tag = "2")]
        pub span: ::prost::alloc::vec::Vec<i32>,
        #[prost(string, optional, tag = "3")]
        pub leading_comments: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "4")]
        pub trailing_comments: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "6")]
        pub leading_detached_comments: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    #[cfg(feature = "prost-types")]
    impl Location {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::source_code_info::Location,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <Location as ::serde::Serialize>::serialize(&Location::from(item.clone()), s)
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::source_code_info::Location, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::source_code_info::Location::from(
                    <Location as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<Location> for ::prost_types::source_code_info::Location {
        #[allow(deprecated)]
        fn from(other: Location) -> ::prost_types::source_code_info::Location {
            ::prost_types::source_code_info::Location {
                path: other.path.into_iter().map(::core::convert::From::from).collect(),
                span: other.span.into_iter().map(::core::convert::From::from).collect(),
                leading_comments: other
                    .leading_comments
                    .map(::core::convert::From::from),
                trailing_comments: other
                    .trailing_comments
                    .map(::core::convert::From::from),
                leading_detached_comments: other
                    .leading_detached_comments
                    .into_iter()
                    .map(::core::convert::From::from)
                    .collect(),
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::source_code_info::Location> for Location {
        #[allow(deprecated)]
        fn from(other: ::prost_types::source_code_info::Location) -> Location {
            Location {
                path: other.path.into_iter().map(::core::convert::From::from).collect(),
                span: other.span.into_iter().map(::core::convert::From::from).collect(),
                leading_comments: other
                    .leading_comments
                    .map(::core::convert::From::from),
                trailing_comments: other
                    .trailing_comments
                    .map(::core::convert::From::from),
                leading_detached_comments: other
                    .leading_detached_comments
                    .into_iter()
                    .map(::core::convert::From::from)
                    .collect(),
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratedCodeInfo {
    #[prost(message, repeated, tag = "1")]
    pub annotation: ::prost::alloc::vec::Vec<generated_code_info::Annotation>,
}
#[cfg(feature = "prost-types")]
impl GeneratedCodeInfo {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::GeneratedCodeInfo,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <GeneratedCodeInfo as ::serde::Serialize>::serialize(
            &GeneratedCodeInfo::from(item.clone()),
            s,
        )
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::GeneratedCodeInfo, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::GeneratedCodeInfo::from(
                <GeneratedCodeInfo as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<GeneratedCodeInfo> for ::prost_types::GeneratedCodeInfo {
    #[allow(deprecated)]
    fn from(other: GeneratedCodeInfo) -> ::prost_types::GeneratedCodeInfo {
        ::prost_types::GeneratedCodeInfo {
            annotation: other
                .annotation
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::GeneratedCodeInfo> for GeneratedCodeInfo {
    #[allow(deprecated)]
    fn from(other: ::prost_types::GeneratedCodeInfo) -> GeneratedCodeInfo {
        GeneratedCodeInfo {
            annotation: other
                .annotation
                .into_iter()
                .map(::core::convert::From::from)
                .collect(),
        }
    }
}
/// Nested message and enum types in `GeneratedCodeInfo`.
pub mod generated_code_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Annotation {
        #[prost(int32, repeated, tag = "1")]
        pub path: ::prost::alloc::vec::Vec<i32>,
        #[prost(string, optional, tag = "2")]
        pub source_file: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "3")]
        pub begin: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub end: ::core::option::Option<i32>,
    }
    #[cfg(feature = "prost-types")]
    impl Annotation {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::generated_code_info::Annotation,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <Annotation as ::serde::Serialize>::serialize(
                &Annotation::from(item.clone()),
                s,
            )
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::generated_code_info::Annotation, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::generated_code_info::Annotation::from(
                    <Annotation as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<Annotation>
    for ::prost_types::generated_code_info::Annotation {
        #[allow(deprecated)]
        fn from(other: Annotation) -> ::prost_types::generated_code_info::Annotation {
            ::prost_types::generated_code_info::Annotation {
                path: other.path.into_iter().map(::core::convert::From::from).collect(),
                source_file: other.source_file.map(::core::convert::From::from),
                begin: other.begin.map(::core::convert::From::from),
                end: other.end.map(::core::convert::From::from),
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::generated_code_info::Annotation>
    for Annotation {
        #[allow(deprecated)]
        fn from(other: ::prost_types::generated_code_info::Annotation) -> Annotation {
            Annotation {
                path: other.path.into_iter().map(::core::convert::From::from).collect(),
                source_file: other.source_file.map(::core::convert::From::from),
                begin: other.begin.map(::core::convert::From::from),
                end: other.end.map(::core::convert::From::from),
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Duration {
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
#[cfg(feature = "prost-types")]
impl Duration {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Duration,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Duration as ::serde::Serialize>::serialize(&Duration::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Duration, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::Duration::from(
                <Duration as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Duration> for ::prost_types::Duration {
    #[allow(deprecated)]
    fn from(other: Duration) -> ::prost_types::Duration {
        ::prost_types::Duration {
            seconds: ::core::convert::From::from(other.seconds),
            nanos: ::core::convert::From::from(other.nanos),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Duration> for Duration {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Duration) -> Duration {
        Duration {
            seconds: ::core::convert::From::from(other.seconds),
            nanos: ::core::convert::From::from(other.nanos),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldMask {
    #[prost(string, repeated, tag = "1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[cfg(feature = "prost-types")]
impl FieldMask {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::FieldMask,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <FieldMask as ::serde::Serialize>::serialize(&FieldMask::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::FieldMask, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::FieldMask::from(
                <FieldMask as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<FieldMask> for ::prost_types::FieldMask {
    #[allow(deprecated)]
    fn from(other: FieldMask) -> ::prost_types::FieldMask {
        ::prost_types::FieldMask {
            paths: other.paths.into_iter().map(::core::convert::From::from).collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::FieldMask> for FieldMask {
    #[allow(deprecated)]
    fn from(other: ::prost_types::FieldMask) -> FieldMask {
        FieldMask {
            paths: other.paths.into_iter().map(::core::convert::From::from).collect(),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct {
    #[prost(map = "string, message", tag = "1")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
#[cfg(feature = "prost-types")]
impl Struct {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Struct,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Struct as ::serde::Serialize>::serialize(&Struct::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Struct, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::Struct::from(
                <Struct as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Struct> for ::prost_types::Struct {
    #[allow(deprecated)]
    fn from(other: Struct) -> ::prost_types::Struct {
        ::prost_types::Struct {
            fields: other
                .fields
                .into_iter()
                .map(|(k, v)| (
                    ::core::convert::From::from(k),
                    ::core::convert::From::from(v),
                ))
                .collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Struct> for Struct {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Struct) -> Struct {
        Struct {
            fields: other
                .fields
                .into_iter()
                .map(|(k, v)| (
                    ::core::convert::From::from(k),
                    ::core::convert::From::from(v),
                ))
                .collect(),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Kind", tags = "1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<value::Kind>,
}
#[cfg(feature = "prost-types")]
impl Value {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Value,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Value as ::serde::Serialize>::serialize(&Value::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Value, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(::prost_types::Value::from(<Value as ::serde::Deserialize>::deserialize(d)?))
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Value> for ::prost_types::Value {
    #[allow(deprecated)]
    fn from(other: Value) -> ::prost_types::Value {
        ::prost_types::Value {
            kind: other.kind.map(::core::convert::From::from),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Value> for Value {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Value) -> Value {
        Value {
            kind: other.kind.map(::core::convert::From::from),
        }
    }
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(enumeration = "super::NullValue", tag = "1")]
        NullValue(i32),
        #[prost(double, tag = "2")]
        NumberValue(f64),
        #[prost(string, tag = "3")]
        StringValue(::prost::alloc::string::String),
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        #[prost(message, tag = "5")]
        StructValue(super::Struct),
        #[prost(message, tag = "6")]
        ListValue(super::ListValue),
    }
    #[cfg(feature = "prost-types")]
    impl Kind {
        /// Serialize the given item from `prost_types` using this crate's protobuf-json
        /// serialization rules
        pub fn serialize_prost_type<S>(
            item: &::prost_types::value::Kind,
            s: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <Kind as ::serde::Serialize>::serialize(&Kind::from(item.clone()), s)
        }
        /// Deserialize into the given type from `prost_types` using this crates's
        /// protobuf-json deserialization rules
        pub fn deserialize_to_prost_type<'de, D>(
            d: D,
        ) -> Result<::prost_types::value::Kind, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(
                ::prost_types::value::Kind::from(
                    <Kind as ::serde::Deserialize>::deserialize(d)?,
                ),
            )
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<Kind> for ::prost_types::value::Kind {
        #[allow(deprecated)]
        fn from(pbjson: Kind) -> ::prost_types::value::Kind {
            match pbjson {
                Kind::NullValue(var0) => {
                    ::prost_types::value::Kind::NullValue(
                        ::core::convert::From::from(var0),
                    )
                }
                Kind::NumberValue(var0) => {
                    ::prost_types::value::Kind::NumberValue(
                        ::core::convert::From::from(var0),
                    )
                }
                Kind::StringValue(var0) => {
                    ::prost_types::value::Kind::StringValue(
                        ::core::convert::From::from(var0),
                    )
                }
                Kind::BoolValue(var0) => {
                    ::prost_types::value::Kind::BoolValue(
                        ::core::convert::From::from(var0),
                    )
                }
                Kind::StructValue(var0) => {
                    ::prost_types::value::Kind::StructValue(
                        ::core::convert::From::from(var0),
                    )
                }
                Kind::ListValue(var0) => {
                    ::prost_types::value::Kind::ListValue(
                        ::core::convert::From::from(var0),
                    )
                }
            }
        }
    }
    #[cfg(feature = "prost-types")]
    impl ::core::convert::From<::prost_types::value::Kind> for Kind {
        #[allow(deprecated)]
        fn from(prost: ::prost_types::value::Kind) -> Kind {
            match prost {
                ::prost_types::value::Kind::NullValue(var0) => {
                    Kind::NullValue(::core::convert::From::from(var0))
                }
                ::prost_types::value::Kind::NumberValue(var0) => {
                    Kind::NumberValue(::core::convert::From::from(var0))
                }
                ::prost_types::value::Kind::StringValue(var0) => {
                    Kind::StringValue(::core::convert::From::from(var0))
                }
                ::prost_types::value::Kind::BoolValue(var0) => {
                    Kind::BoolValue(::core::convert::From::from(var0))
                }
                ::prost_types::value::Kind::StructValue(var0) => {
                    Kind::StructValue(::core::convert::From::from(var0))
                }
                ::prost_types::value::Kind::ListValue(var0) => {
                    Kind::ListValue(::core::convert::From::from(var0))
                }
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[cfg(feature = "prost-types")]
impl ListValue {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::ListValue,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <ListValue as ::serde::Serialize>::serialize(&ListValue::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::ListValue, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::ListValue::from(
                <ListValue as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<ListValue> for ::prost_types::ListValue {
    #[allow(deprecated)]
    fn from(other: ListValue) -> ::prost_types::ListValue {
        ::prost_types::ListValue {
            values: other.values.into_iter().map(::core::convert::From::from).collect(),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::ListValue> for ListValue {
    #[allow(deprecated)]
    fn from(other: ::prost_types::ListValue) -> ListValue {
        ListValue {
            values: other.values.into_iter().map(::core::convert::From::from).collect(),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NullValue {
    NullValue = 0,
}
impl NullValue {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NullValue::NullValue => "NULL_VALUE",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
#[cfg(feature = "prost-types")]
impl Timestamp {
    /// Serialize the given item from `prost_types` using this crate's protobuf-json
    /// serialization rules
    pub fn serialize_prost_type<S>(
        item: &::prost_types::Timestamp,
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <Timestamp as ::serde::Serialize>::serialize(&Timestamp::from(item.clone()), s)
    }
    /// Deserialize into the given type from `prost_types` using this crates's
    /// protobuf-json deserialization rules
    pub fn deserialize_to_prost_type<'de, D>(
        d: D,
    ) -> Result<::prost_types::Timestamp, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(
            ::prost_types::Timestamp::from(
                <Timestamp as ::serde::Deserialize>::deserialize(d)?,
            ),
        )
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<Timestamp> for ::prost_types::Timestamp {
    #[allow(deprecated)]
    fn from(other: Timestamp) -> ::prost_types::Timestamp {
        ::prost_types::Timestamp {
            seconds: ::core::convert::From::from(other.seconds),
            nanos: ::core::convert::From::from(other.nanos),
        }
    }
}
#[cfg(feature = "prost-types")]
impl ::core::convert::From<::prost_types::Timestamp> for Timestamp {
    #[allow(deprecated)]
    fn from(other: ::prost_types::Timestamp) -> Timestamp {
        Timestamp {
            seconds: ::core::convert::From::from(other.seconds),
            nanos: ::core::convert::From::from(other.nanos),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleValue {
    #[prost(double, tag = "1")]
    pub value: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatValue {
    #[prost(float, tag = "1")]
    pub value: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Value {
    #[prost(int64, tag = "1")]
    pub value: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt64Value {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Value {
    #[prost(int32, tag = "1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32Value {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolValue {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringValue {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesValue {
    #[prost(bytes = "bytes", tag = "1")]
    pub value: ::prost::bytes::Bytes,
}

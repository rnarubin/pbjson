//! Compiles Protocol Buffers and FlatBuffers schema definitions into
//! native Rust types.
//!
//! This is kept as a separate binary to generate code for manual check-in, instead of running at
//! compile time as `build.rs` -- that way downstream consumers do not require the build
//! dependencies (espeically protoc) and can import rust sources directly.

use std::env;
use std::path::PathBuf;

use clap::Parser;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Parser)]
struct Args {
    /// The path of the directory containing the protobuf sources.
    #[clap(short, long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/../protos"))]
    input_proto_dir: PathBuf,

    /// The destination directory for generated code.
    #[clap(short, long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/pb/"))]
    output_dir: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let root = args.input_proto_dir;
    let out_dir = &args.output_dir;

    let proto_files = vec![root.join("google/protobuf/types.proto")];

    std::fs::create_dir_all(out_dir)?;

    let temp_dir = tempfile::tempdir()?;
    let descriptor_path = temp_dir.path().join("proto_descriptor.bin");
    prost_build::Config::new()
        .file_descriptor_set_path(&descriptor_path)
        .compile_well_known_types()
        .disable_comments(&["."])
        .bytes(&[".google"])
        .out_dir(out_dir)
        .compile_protos(&proto_files, &[root])?;

    let descriptor_set = std::fs::read(descriptor_path)?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .exclude([
            ".google.protobuf.Duration",
            ".google.protobuf.Timestamp",
            ".google.protobuf.Value",
            ".google.protobuf.Struct",
            ".google.protobuf.ListValue",
            ".google.protobuf.NullValue",
            ".google.protobuf.BoolValue",
            ".google.protobuf.BytesValue",
            ".google.protobuf.DoubleValue",
            ".google.protobuf.FloatValue",
            ".google.protobuf.Int32Value",
            ".google.protobuf.Int64Value",
            ".google.protobuf.StringValue",
            ".google.protobuf.UInt32Value",
            ".google.protobuf.UInt64Value",
        ])
        .out_dir(out_dir)
        .build(&[".google"])?;

    let base_protos = out_dir.join("google.protobuf.rs");
    let mut generated_file = syn::parse_file(&std::fs::read_to_string(&base_protos)?)?;

    convert_prost_types::insert_prost_conversions(
        &mut generated_file,
        convert_prost_types::conversion_mapping! {

            Any                                      => ::prost_types::Any,
            Api                                      => ::prost_types::Api,
            DescriptorProto                          => ::prost_types::DescriptorProto,
            Duration                                 => ::prost_types::Duration,
            Enum                                     => ::prost_types::Enum,
            EnumDescriptorProto                      => ::prost_types::EnumDescriptorProto,
            EnumOptions                              => ::prost_types::EnumOptions,
            EnumValue                                => ::prost_types::EnumValue,
            EnumValueDescriptorProto                 => ::prost_types::EnumValueDescriptorProto,
            EnumValueOptions                         => ::prost_types::EnumValueOptions,
            ExtensionRangeOptions                    => ::prost_types::ExtensionRangeOptions,
            Field                                    => ::prost_types::Field,
            FieldDescriptorProto                     => ::prost_types::FieldDescriptorProto,
            FieldMask                                => ::prost_types::FieldMask,
            FieldOptions                             => ::prost_types::FieldOptions,
            FileDescriptorProto                      => ::prost_types::FileDescriptorProto,
            FileDescriptorSet                        => ::prost_types::FileDescriptorSet,
            FileOptions                              => ::prost_types::FileOptions,
            GeneratedCodeInfo                        => ::prost_types::GeneratedCodeInfo,
            ListValue                                => ::prost_types::ListValue,
            MessageOptions                           => ::prost_types::MessageOptions,
            Method                                   => ::prost_types::Method,
            MethodDescriptorProto                    => ::prost_types::MethodDescriptorProto,
            MethodOptions                            => ::prost_types::MethodOptions,
            Mixin                                    => ::prost_types::Mixin,
            OneofDescriptorProto                     => ::prost_types::OneofDescriptorProto,
            OneofOptions                             => ::prost_types::OneofOptions,
            Option                                   => ::prost_types::Option,
            ServiceDescriptorProto                   => ::prost_types::ServiceDescriptorProto,
            ServiceOptions                           => ::prost_types::ServiceOptions,
            SourceCodeInfo                           => ::prost_types::SourceCodeInfo,
            SourceContext                            => ::prost_types::SourceContext,
            Struct                                   => ::prost_types::Struct,
            Timestamp                                => ::prost_types::Timestamp,
            Type                                     => ::prost_types::Type,
            UninterpretedOption                      => ::prost_types::UninterpretedOption,
            Value                                    => ::prost_types::Value,
            descriptor_proto::ExtensionRange         => ::prost_types::descriptor_proto::ExtensionRange,
            descriptor_proto::ReservedRange          => ::prost_types::descriptor_proto::ReservedRange,
            enum_descriptor_proto::EnumReservedRange => ::prost_types::enum_descriptor_proto::EnumReservedRange,
            field::Cardinality                       => ::prost_types::field::Cardinality,
            field::Kind                              => ::prost_types::field::Kind,
            field_descriptor_proto::Label            => ::prost_types::field_descriptor_proto::Label,
            field_descriptor_proto::Type             => ::prost_types::field_descriptor_proto::Type,
            field_options::CType                     => ::prost_types::field_options::CType,
            field_options::JsType                    => ::prost_types::field_options::JsType,
            file_options::OptimizeMode               => ::prost_types::file_options::OptimizeMode,
            generated_code_info::Annotation          => ::prost_types::generated_code_info::Annotation,
            method_options::IdempotencyLevel         => ::prost_types::method_options::IdempotencyLevel,
            source_code_info::Location               => ::prost_types::source_code_info::Location,
            uninterpreted_option::NamePart           => ::prost_types::uninterpreted_option::NamePart,
            value::Kind                              => ::prost_types::value::Kind,

        },
    )?;
    std::fs::write(base_protos, prettyplease::unparse(&generated_file))?;

    Ok(())
}

mod convert_prost_types {
    use super::Result;
    use proc_macro2::TokenStream;
    use quote::quote;
    use quote::ToTokens;
    use std::collections::HashMap;

    macro_rules! conversion_mapping {
        ($($pbjson_type:path => $prost_type:path),*$(,)?) => {
            {
                let mut pbjson_to_prost_mapping = std::collections::HashMap::new();
                $(
                pbjson_to_prost_mapping.insert(
                    ::syn::parse2(::quote::quote! { $pbjson_type })?,
                    ::syn::parse2(::quote::quote! { $prost_type })?,
                );
                )*
                crate::convert_prost_types::ConversionTargets {
                    pbjson_to_prost_mapping
                }
            }
        }
    }
    pub(super) use conversion_mapping;

    pub(super) struct ConversionTargets {
        pub(super) pbjson_to_prost_mapping: HashMap<syn::Path, syn::Path>,
    }

    /// Given a parsed source file, insert code necessary for [de]serializing and converting
    /// to/from prost-types, for the given target types
    pub(super) fn insert_prost_conversions(
        file: &mut syn::File,
        targets: ConversionTargets,
    ) -> Result<()> {
        // while traversing the source code, determine the visited type's path by building a path
        // along the traversal. Items in this path will be pushed/popped as modules are entered and
        // exited
        let mut root_path = syn::Path {
            leading_colon: None,
            segments: syn::punctuated::Punctuated::new(),
        };

        visit_item_list(&mut file.items, &mut root_path, &targets)?;

        Ok(())
    }

    /// Visit each item in the given list of syntax items, and insert new conversion code where
    /// necessary.
    fn visit_item_list(
        items: &mut Vec<syn::Item>,
        path: &mut syn::Path,
        targets: &ConversionTargets,
    ) -> Result<()> {
        // this iterates the list with manual indexing in order to insert items at internal
        // indices. The alternative of using e.g. `iter_mut` would require the new items to be
        // placed at the end of the file/module, instead of next to the relevant source definitions
        let mut i = 0;
        while i < items.len() {
            let item = &mut items[i];
            for additional_item in visit_item(item, path, targets)? {
                i += 1;
                items.insert(i, additional_item);
            }
            i += 1;
        }

        Ok(())
    }

    /// Visit the given item;
    ///
    /// If it's a struct or enum, generate the conversion code and return it for the caller to
    /// insert adjacent to the item.
    ///
    /// If it's a module, recursively visit the items of that module
    fn visit_item(
        item: &mut syn::Item,
        path: &mut syn::Path,
        targets: &ConversionTargets,
    ) -> Result<Vec<syn::Item>> {
        use syn::Item::*;
        Ok(match item {
            Struct(item_struct) => {
                // push the struct name onto the end of the path, and use this path to check
                // whether this type is a target for generating conversion code
                path.segments.push(item_struct.ident.clone().into());
                let additional_items = generate_code_for_struct(item_struct, path, targets)?;
                path.segments.pop();

                additional_items
            }
            Enum(item_enum) => {
                path.segments.push(item_enum.ident.clone().into());
                let additional_items = generate_code_for_enum(item_enum, path, targets)?;
                path.segments.pop();
                additional_items
            }
            Mod(syn::ItemMod {
                content: Some((_, items)),
                ident,
                ..
            }) => {
                // push the module name onto the accumulated path, then descend recursively

                path.segments.push(ident.clone().into());
                visit_item_list(items, path, targets)?;
                path.segments.pop();

                // because the module will add items internaly, no items need to be returned to
                // higher layers
                vec![]
            }
            _ => vec![],
        })
    }

    fn generate_code_for_struct(
        item: &syn::ItemStruct,
        pbjson_path: &syn::Path,
        targets: &ConversionTargets,
    ) -> Result<Vec<syn::Item>> {
        let prost_type = match targets.pbjson_to_prost_mapping.get(pbjson_path) {
            Some(prost_type) => prost_type,
            None => return Ok(vec![]),
        };
        let pbjson_type = &item.ident;

        let fields = match &item.fields {
            syn::Fields::Named(fields) => &fields.named,
            // prost currently only generates structs with named fields, and not e.g. tuple structs
            other @ _ => {
                return Err(format!("Unimplemented conversion for field type {other:?}").into())
            }
        };

        // construct field assignments `x: other.x` for the `From` impls programmatically. There
        // are different conversions depending on whether the field's type is an Option/Vec/Map
        let field_assignments = fields
            .iter()
            .map(|field| {
                let field_ident = &field.ident;
                // if the field's type is some wrapper, use the appropriate conversion
                if let syn::Type::Path(syn::TypePath { path, .. }) = &field.ty {
                    let type_segments = path.segments.iter().map(|segment| &segment.ident);

                    if type_segments.clone().eq(&["core", "option", "Option"]) {
                        return quote! {
                            #field_ident: other
                                .#field_ident
                                .map(::core::convert::From::from)
                        };
                    } else if type_segments.clone().eq(&["prost", "alloc", "vec", "Vec"]) {
                        return quote! {
                            #field_ident: other
                                .#field_ident
                                .into_iter()
                                .map(::core::convert::From::from)
                                .collect()
                        };
                    } else if type_segments.clone().eq(&["std", "collections", "HashMap"])
                        || type_segments.eq(&["std", "collections", "BTreeMap"])
                    {
                        return quote! {
                            #field_ident: other
                                .#field_ident
                                .into_iter()
                                .map(|(k, v)| (
                                    ::core::convert::From::from(k),
                                    ::core::convert::From::from(v)
                                ))
                                .collect()
                        };
                    }
                }
                // other cases fall through to this general conversion
                quote! {
                    #field_ident: ::core::convert::From::from(other.#field_ident)
                }
            })
            .map(|stream| Ok(syn::parse2(stream)?))
            .collect::<Result<Vec<syn::FieldValue>>>()?;

        let from_impls = parse_items(quote! {
            #[cfg(feature = "prost-types")]
            impl ::core::convert::From<#pbjson_type> for #prost_type {
                #[allow(deprecated)]
                fn from(other: #pbjson_type) -> #prost_type {
                    #prost_type {
                        #(#field_assignments,)*
                    }
                }
            }

            #[cfg(feature = "prost-types")]
            impl ::core::convert::From<#prost_type> for #pbjson_type {
                #[allow(deprecated)]
                fn from(other: #prost_type) -> #pbjson_type {
                    #pbjson_type {
                        #(#field_assignments,)*
                    }
                }
            }
        })?;

        Ok(generate_code_shared(pbjson_type, prost_type)?
            .into_iter()
            .chain(from_impls)
            .collect())
    }

    fn generate_code_for_enum(
        item: &syn::ItemEnum,
        pbjson_path: &syn::Path,
        targets: &ConversionTargets,
    ) -> Result<Vec<syn::Item>> {
        let prost_type = match targets.pbjson_to_prost_mapping.get(pbjson_path) {
            Some(prost_type) => prost_type,
            None => return Ok(vec![]),
        };
        let pbjson_type = &item.ident;

        // construct the match arms for the `From` impls programmatically. These get a little tricky
        // if the enum variants have fields, as is the case in prost's generation of `oneof` cases
        let (from_pbjson_arms, from_prost_arms) = &item
            .variants
            .iter()
            .map(|variant| {
                let variant_ident = &variant.ident;
                // Constructing the match arms depends on whether the variant has args or not
                match &variant.fields {
                    // unit variants have no arguments -> simple mapping
                    syn::Fields::Unit => {
                        let make_arm = |from: &dyn ToTokens, to: &dyn ToTokens| {
                            quote! { #from::#variant_ident => #to::#variant_ident }
                        };
                        (
                            make_arm(pbjson_type, prost_type),
                            make_arm(prost_type, pbjson_type),
                        )
                    }

                    // unnamed fields need synthetic variable names for binding
                    syn::Fields::Unnamed(fields) => {
                        let bind_var = (0..)
                            .map(|i| quote::format_ident!("var{i}"))
                            .take(fields.unnamed.len())
                            .collect::<Vec<_>>();
                        let make_arm = |from: &dyn ToTokens, to: &dyn ToTokens| {
                            quote! {
                                #from::#variant_ident(#(#bind_var),*) =>
                                    #to::#variant_ident(#(::core::convert::From::from(#bind_var)),*)
                            }
                        };
                        (
                            make_arm(pbjson_type, prost_type),
                            make_arm(prost_type, pbjson_type),
                        )
                    }

                    // prost doesn't generate named fields at this time
                    syn::Fields::Named(_) => {
                        panic!("Unexpected named-field enum in generated code")
                    }
                }
            })
            .map(|(from_pbjson, from_prost)| {
                Ok((syn::parse2(from_pbjson)?, syn::parse2(from_prost)?))
            })
            .collect::<Result<Vec<(syn::Arm, syn::Arm)>>>()?
            .into_iter()
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let from_impls = parse_items(quote! {
            #[cfg(feature = "prost-types")]
            impl ::core::convert::From<#pbjson_type> for #prost_type {
                #[allow(deprecated)]
                fn from(pbjson: #pbjson_type) -> #prost_type {
                    match pbjson {
                        #(#from_pbjson_arms),*
                    }
                }
            }

            #[cfg(feature = "prost-types")]
            impl ::core::convert::From<#prost_type> for #pbjson_type {
                #[allow(deprecated)]
                fn from(prost: #prost_type) -> #pbjson_type {
                    match prost {
                        #(#from_prost_arms),*
                    }
                }
            }
        })?;

        Ok(generate_code_shared(pbjson_type, prost_type)?
            .into_iter()
            .chain(from_impls)
            .collect())
    }

    /// Generate code that is the same between structs/enums
    fn generate_code_shared(
        pbjson_type: &syn::Ident,
        prost_type: &syn::Path,
    ) -> Result<Vec<syn::Item>> {
        parse_items(quote! {
            #[cfg(feature = "prost-types")]
            impl #pbjson_type {
                /// Serialize the given item from `prost_types` using this crate's protobuf-json
                /// serialization rules
                pub fn serialize_prost_type<S>(item: &#prost_type, s: S) -> Result<S::Ok, S::Error>
                    where S: ::serde::Serializer
                {
                    <#pbjson_type as ::serde::Serialize>::serialize(&#pbjson_type::from(item.clone()), s)
                }

                /// Deserialize into the given type from `prost_types` using this crates's
                /// protobuf-json deserialization rules
                pub fn deserialize_to_prost_type<'de, D>(d: D) -> Result<#prost_type, D::Error>
                    where D: ::serde::Deserializer<'de>
                {
                    Ok(#prost_type::from(<#pbjson_type as ::serde::Deserialize>::deserialize(d)?))
                }
            }
        })
    }

    fn parse_items(stream: TokenStream) -> Result<Vec<syn::Item>> {
        syn::parse::Parser::parse2(syn::Block::parse_within, stream)?
            .into_iter()
            .map(|statement| match statement {
                syn::Stmt::Item(item) => Ok(item),
                _ => Err("unexpected non-item in generated source".into()),
            })
            .collect()
    }
}

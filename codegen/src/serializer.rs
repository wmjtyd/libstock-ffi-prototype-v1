use combine::parser::char::{digit, letter, spaces, string};
use combine::{choice, many1, sep_by1, token, EasyParser, Parser};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SerializeType {
    Field(usize),
    Struct,
}

pub fn inner_serializer_function(item: TokenStream) -> TokenStream {
    let item = item.to_string();
    let Parameters { typ, left, right } = parse_parameters(&item);

    // We have asserted in parse_parameters.
    let ffi_type = Ident::new(&left, Span::call_site());
    let rust_type = Ident::new(&right, Span::call_site());

    let snake_case_ffi_type = ffi_type.to_string().to_case(Case::Snake);

    let func_ident = format_ident!("serialize_{snake_case_ffi_type}");
    let doc_str = format!("Serialize `{ffi_type}` to the specified buffer.");

    let inner_body = match typ {
        SerializeType::Field(len) => {
            quote! { crate::serializer::inner_serialize_field::<#len, #rust_type, _, _>(input, buf, written_len) }
        }
        SerializeType::Struct => {
            quote! { crate::serializer::inner_serialize_struct::<#rust_type, _, _>(input, buf, written_len) }
        }
    };

    quote! {
        #[ffi_export]
        #[doc = #doc_str]
        pub fn #func_ident<'a>(
            input: &#ffi_type,
            buf: &'a mut ::safer_ffi::prelude::c_slice::Mut<'a, u8>,
            written_len: &mut usize,
        ) -> crate::errors::LibstockErrors {
            crate::utils::result_to_status_code(
                #inner_body
            )
        }
    }
}

pub fn inner_deserializer_function(item: TokenStream) -> TokenStream {
    let item = item.to_string();
    let Parameters { typ, left, right } = parse_parameters(&item);

    // We have asserted in parse_parameters.
    let rust_type = Ident::new(&left, Span::call_site());
    let ffi_type = Ident::new(&right, Span::call_site());

    let snake_case_ffi_type = ffi_type.to_string().to_case(Case::Snake);

    let func_ident = format_ident!("deserialize_{snake_case_ffi_type}");
    let doc_str = format!("Deserialize `{ffi_type}` from the specified buffer.");

    match typ {
        SerializeType::Field(len) => {
            quote! {
                #[ffi_export]
                #[doc = #doc_str]
                pub fn #func_ident<'a>(
                    input: &'a c_slice::Ref<'a, u8>,
                    output: &mut #ffi_type
                ) -> crate::errors::LibstockErrors {
                    crate::utils::result_to_status_code(
                        crate::serializer::inner_deserialize_field::<#len, #rust_type, _, _>(input, output)
                    )
                }
            }
        }
        SerializeType::Struct => {
            quote! {
                #[ffi_export]
                #[doc = #doc_str]
                pub fn #func_ident<'a>(
                    input: &'a c_slice::Ref<'a, u8>,
                    output: &mut #ffi_type
                ) -> crate::errors::LibstockErrors {
                    crate::utils::result_to_status_code(
                        crate::serializer::inner_deserialize_struct::<#rust_type, _, _>(input, output)
                    )
                }
            }
        }
    }
}

struct Parameters {
    pub typ: SerializeType,
    pub left: String,
    pub right: String,
}

fn parse_parameters(input: &str) -> Parameters {
    //                      ~~~~ Ignore
    // TYP, [PriceDataField -> RPriceDataField] -> ident_parser
    //        ~~~~left~~~~    ~~~~right~~~~~~
    let field_parser = string("Field")
        .skip(spaces().with(token('<')))
        .with(
            spaces().with(many1(digit()))
                .map(|s: String| s.parse::<usize>().expect("field is not a valid number")),
        )
        .skip(spaces().with(token('>')))
        .map(SerializeType::Field);
    
    let struct_parser = string("Struct").map(|_| SerializeType::Struct);

    let (typ, remaining) = choice((field_parser, struct_parser))
        .skip(spaces().with(token(',')))
        .easy_parse(input)
        .unwrap();

    let mut ident_parser = sep_by1::<Vec<String>, _, _, _>(
        spaces().with(many1(choice((letter(), digit())))),
        spaces().skip(string("->")),
    );

    let (result, remaining) = ident_parser
        .easy_parse(remaining)
        .expect("failed to parse the parameters");

    assert_eq!(result.len(), 2);
    assert_eq!(remaining, "");

    Parameters {
        typ,
        left: result[0].clone(),
        right: result[1].clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_parameters;

    #[test]
    fn test_parse_parameter() {
        const ALL_OF_THEM_ARE_THE_SAME_FIELD: &[&str] = &[
            "Field<10>, PriceDataField -> RPriceDataField",
            "Field<10>, PriceDataField ->RPriceDataField",
            "Field<10>, PriceDataField-> RPriceDataField",
            "Field<10>, PriceDataField->RPriceDataField",
            "Field <10> , PriceDataField -> RPriceDataField",
            "Field <10> , PriceDataField ->RPriceDataField",
            "Field < 10>, PriceDataField-> RPriceDataField",
            "Field < 10 >, PriceDataField->RPriceDataField",
            "Field < 10 > , PriceDataField->RPriceDataField",
        ];

        const ALL_OF_THEM_ARE_THE_SAME_STRUCT: &[&str] = &[
            "Struct, BboStructure -> RBboStructure",
            "Struct, BboStructure ->RBboStructure",
            "Struct, BboStructure-> RBboStructure",
            "Struct, BboStructure->RBboStructure",
        ];

        for testcase in ALL_OF_THEM_ARE_THE_SAME_FIELD {
            let result = parse_parameters(testcase);
            assert_eq!(result.typ, super::SerializeType::Field(10));
            assert_eq!(result.left, "PriceDataField");
            assert_eq!(result.right, "RPriceDataField");
        }

        for testcase in ALL_OF_THEM_ARE_THE_SAME_STRUCT {
            let result = parse_parameters(testcase);
            assert_eq!(result.typ, super::SerializeType::Struct);
            assert_eq!(result.left, "BboStructure");
            assert_eq!(result.right, "RBboStructure");
        }
    }

    mod failcases {
        macro_rules! build_failcase {
            ($case_name:ident, $case: expr) => {
                #[test]
                #[should_panic]
                fn $case_name() {
                    super::parse_parameters($case);
                }
            };
        }

        build_failcase!(fc_1, "Field, PriceDataField ->");
        build_failcase!(fc_2, "Field, PriceDataField");
        build_failcase!(fc_3, "Field, PriceDataField-> RPriceDataField ->");
        build_failcase!(fc_4, "Field, PriceDataField->RPriceDataField->->->");
        build_failcase!(fc_5, "WTF, PriceDataField-> RPriceDataField");
        build_failcase!(fc_6, "ABC, PriceDataField->RPriceDataField");
        build_failcase!(fc_7, "WTF, PriceDataField->RPriceDataField");
        build_failcase!(fc_8, "ABC, PriceDataField->RPriceDataField");
        build_failcase!(fc_9, "Field, PriceDataField->RPriceDataField");
        build_failcase!(fc_10, "Field<10>, PriceDataField ->");
        build_failcase!(fc_11, "Field <10> , PriceDataField");
        build_failcase!(fc_12, "Field< 10 >, PriceDataField-> RPriceDataField ->");
        build_failcase!(fc_13, "Field<10>, PriceDataField->RPriceDataField->->->");
    }
}

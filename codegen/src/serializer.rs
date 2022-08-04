use combine::parser::char::{digit, letter, spaces, string};
use combine::{choice, many1, sep_by1, EasyParser, Parser};
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

pub fn inner_serializer_function(item: TokenStream) -> TokenStream {
    let item = item.to_string();
    let parameters = parse_parameters(&item);

    // We have asserted in parse_parameters.
    let ffi_type = Ident::new(&parameters[0], Span::call_site());
    let rust_type = Ident::new(&parameters[1], Span::call_site());

    let snake_case_ffi_type = ffi_type.to_string().to_case(Case::Snake);

    let func_ident = format_ident!("serialize_{snake_case_ffi_type}");
    let doc_str = format!("Serialize `{ffi_type}` to the specified buffer.");

    quote! {
        #[ffi_export]
        #[doc = #doc_str]
        pub fn #func_ident<'a>(
            input: &#ffi_type,
            buf: &'a mut ::safer_ffi::prelude::c_slice::Mut<'a, u8>,
            written_len: &mut usize,
        ) -> crate::errors::LibstockErrors {
            crate::utils::result_to_status_code(
                crate::serializer::inner_serialize_field::<#rust_type, _>(input, buf, written_len)
            )
        }
    }
}

pub fn inner_deserializer_function(item: TokenStream) -> TokenStream {
    let item = item.to_string();
    let parameters = parse_parameters(&item);

    // We have asserted in parse_parameters.
    let rust_type = Ident::new(&parameters[0], Span::call_site());
    let ffi_type = Ident::new(&parameters[1], Span::call_site());

    let snake_case_ffi_type = ffi_type.to_string().to_case(Case::Snake);

    let func_ident = format_ident!("deserialize_{snake_case_ffi_type}");
    let doc_str = format!("Deserialize `{ffi_type}` from the specified buffer.");

    quote! {
        #[ffi_export]
        #[doc = #doc_str]
        pub fn #func_ident<'a>(
            input: &'a c_slice::Ref<'a, u8>,
            output: &mut #ffi_type
        ) -> crate::errors::LibstockErrors {
            crate::utils::result_to_status_code(
                crate::serializer::inner_deserialize_field::<#rust_type, _>(input, output)
            )
        }
    }
}

fn parse_parameters(input: &str) -> Vec<String> {
    //               ~~~~ Ignore
    // PriceDataField -> RPriceDataField
    // ~~~~~[0]~~~~~~    ~~~~~[1]~~~~~~~
    let mut ident_parser = sep_by1::<Vec<String>, _, _, _>(
        spaces().with(many1(choice((letter(), digit())))),
        spaces().skip(string("->")),
    );

    let (result, remaining) = ident_parser
        .easy_parse(input)
        .expect("failed to parse the parameters");

    assert_eq!(result.len(), 2);
    assert_eq!(remaining, "");

    result
}

#[cfg(test)]
mod tests {
    use super::parse_parameters;

    #[test]
    fn test_parse_parameter() {
        const ALL_OF_THEM_ARE_THE_SAME: &[&str] = &[
            "PriceDataField -> RPriceDataField",
            "PriceDataField ->RPriceDataField",
            "PriceDataField-> RPriceDataField",
            "PriceDataField->RPriceDataField",
        ];

        for testcase in ALL_OF_THEM_ARE_THE_SAME {
            let result = parse_parameters(testcase);
            assert_eq!(result[0], "PriceDataField");
            assert_eq!(result[1], "RPriceDataField");
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

        build_failcase!(fc_1, "PriceDataField ->");
        build_failcase!(fc_2, "PriceDataField");
        build_failcase!(fc_3, "PriceDataField-> RPriceDataField ->");
        build_failcase!(fc_4, "PriceDataField->RPriceDataField->->->");
    }
}

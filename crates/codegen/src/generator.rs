use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn interface(input: TokenStream2) -> Result<TokenStream2, syn::Error> {
    let mut mod_func: Vec<TokenStream2> = vec![];

    let input = syn::parse2::<syn::ItemTrait>(input)?;
    let mod_name = input.ident;

    for item in input.items {
        if let syn::TraitItem::Method(method) = item {
            let func_name_str = method.sig.ident.to_string();
            let func_name = method.sig.ident;

            let mut args: Vec<TokenStream2> = vec![];
            let mut call_args: Vec<TokenStream2> = vec![];
            for arg in method.sig.inputs.iter() {
                if let syn::FnArg::Typed(a) = arg {
                    if let syn::Pat::Ident(pat_ident) = &*a.pat {
                        let arg_name = &pat_ident.ident;

                        if let syn::Type::Path(type_path) = &*a.ty {
                            let path_seg = &type_path.path.segments[0];
                            let name = path_seg.ident.to_string();
                            match name.as_str() {
                                "Integer" => {
                                    args.push(quote!(
                                        #arg_name: i64
                                    ));

                                    call_args.push(quote!(
                                        call_arg_int(#arg_name);
                                    ));
                                }
                                "Boolean" => {
                                    args.push(quote!(
                                        #arg_name: bool
                                    ));

                                    call_args.push(quote!(
                                        call_arg_bool(#arg_name);
                                    ));
                                }
                                "Binary" => {
                                    args.push(quote!(
                                        #arg_name: &[u8]
                                    ));

                                    call_args.push(quote!(
                                        call_arg_binary(#arg_name);
                                    ));
                                }
                                "String" => {
                                    args.push(quote!(
                                        #arg_name: &str
                                    ));

                                    call_args.push(quote!(
                                        call_arg_string(#arg_name);
                                    ));
                                }
                                "Payment" => {
                                    args.push(quote!(
                                        #arg_name: (&[u8], i64)
                                    ));

                                    call_args.push(quote!(
                                        call_payment(#arg_name.0, #arg_name.1);
                                    ));
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }

            mod_func.push(quote!(
                pub fn #func_name(contract_id: &[u8], #( #args ),*) -> i32 {
                    unsafe {
                        #( #call_args )*
                        call_contract(contract_id, #func_name_str)
                    }
                }
            ));
        }
    }

    Ok(quote!(
        mod #mod_name {
            use we_contract_sdk::*;

            #( #mod_func )*
        }
    ))
}

pub fn action(input: TokenStream2) -> Result<TokenStream2, syn::Error> {
    let input = syn::parse2::<syn::ItemFn>(input)?;

    let action_name = &input.sig.ident;
    let action_args = &input.sig.inputs;
    let action_block = &input.block;

    Ok(quote!(
        #[no_mangle]
        pub extern "C" fn #action_name (#action_args) -> i32 {
            unsafe {
                #action_block
            }

            0
        }
    ))
}
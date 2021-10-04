use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;
use syn::Block;
use syn::Ident;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn run_in_tx(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    run_in_tx_inner(attr.into(), item.into()).into()
}

fn run_in_tx_inner(conn: TokenStream, original_func: TokenStream) -> TokenStream {
    let original_func: ItemFn = parse2(original_func).expect("parse2 original_func to ItemFn");
    let mut _original_func = original_func.clone();
    let _original_func_name =
        Ident::new(&format!("_{}", _original_func.sig.ident), Span::call_site());
    _original_func.sig.ident = _original_func_name.clone();
    let mut call_original_func_in_tx = original_func.clone();
    call_original_func_in_tx.sig.inputs.clear();
    // #[cfg(features = "diesel")]
    let call_original_func_in_tx_block = quote! {
        {
            let conn = #conn;
            conn.transaction(|| {
                #_original_func_name(&conn)
            })
        }
    };
    // #[cfg(features = "sqlx")]
    // let call_original_func_in_tx_block = quote! {
    //     {
    //         let conn = #conn;
    //         conn.transaction(|tx| {
    //             #_original_func_name(tx)
    //         })
    //     }
    // };
    println!("{}", call_original_func_in_tx_block.to_string());
    let call_original_func_in_tx_block = call_original_func_in_tx_block.into();
    let call_original_func_in_tx_block: Block =
        parse2(call_original_func_in_tx_block).expect("parse2 Block");
    call_original_func_in_tx.block = Box::new(call_original_func_in_tx_block);
    quote! {
        #_original_func
        #call_original_func_in_tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_a_test() {
        let actual = run_in_tx_inner(
            quote! {
                get_conn()
            },
            quote! {
                fn usecase(conn: &Conn) -> i32 {
                    conn.execute();
                    1
                }
            },
        );
        assert_eq!(
            actual.to_string(),
            quote! {
                fn _usecase(conn: &Conn) -> i32 {
                    conn.execute();
                    1
                }
                fn usecase() -> i32 {
                    let conn = get_conn();

                    conn.transaction(| | {
                        _usecase(&conn)
                    })
                }
            }
            .to_string()
        );
    }
}

use darling::{FromMeta};
use syn::{parse_macro_input,ItemFn,AttributeArgs};
use proc_macro::{TokenStream};
use quote::quote;

mod attributes;
use attributes::MainAttr;

#[proc_macro_attribute]
pub fn expand(attr_tks: proc_macro::TokenStream,item_tks: proc_macro::TokenStream)->proc_macro::TokenStream{
    let item_ast = parse_macro_input!(item_tks as ItemFn);
    let attr_ast = parse_macro_input!(attr_tks as AttributeArgs);
    let attr:MainAttr = match FromMeta::from_list(&attr_ast){
        Ok(attr)=>attr,
        Err(errs)=>return TokenStream::from(errs.write_errors()),
    };
    println!("tzz_attr {:?}",attr);
    let app_main = &item_ast.sig.ident;
    let tk = quote!{
        #item_ast
        use pf_age_macro_rule::debug;
        fn main(){
            println!("game main");
            #app_main();
            debug!();
        }
    };
    return tk.into()
}


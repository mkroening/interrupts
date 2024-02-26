extern crate proc_macro;

use proc_macro2::Ident;
use syn::{parse_macro_input, DeriveInput, Data, Type, TypePath, parse_quote};
use quote::quote;

#[proc_macro_derive(InterruptSend)]
pub fn interrupt_send(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let tys = match &input.data {
        Data::Struct(data_struct) => data_struct.fields.iter().map(|field| &field.ty).collect::<Vec<_>>(),
        Data::Enum(data_enum) => todo!(),
        Data::Union(data_union) => todo!(),
    };


    let trait_ = parse_quote!(::interrupts::InterruptSend);
    let output = generate(&trait_, &input.ident, &tys);

    proc_macro::TokenStream::from(output)
}

fn generate(trait_: &TypePath, target: &Ident, inner_tys: &[&Type]) -> proc_macro2::TokenStream {
    quote! {
        const _: () = {
            fn assert_interrupt_send<T: #trait_>() {}

            fn assert() {
                #(assert_interrupt_send::<#inner_tys>();)*
            }
        };

        unsafe impl ::interrupts::InterruptSend for #target {}
    }
}

#[proc_macro_derive(InterruptSync)]
pub fn interrupt_sync(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    todo!()
}
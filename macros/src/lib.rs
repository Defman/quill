use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn, ItemStruct, Fields};

#[proc_macro_attribute]
pub fn init(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    let quill_init = quill_init(&item.sig.ident);

    let item = quote! {
        #item

        #quill_init
    };

    TokenStream::from(item)
}

fn quill_init(ident: &Ident) -> proc_macro2::TokenStream {
    quote! {
        #[no_mangle]
        pub extern "C" fn __quill_init_() {
            let plugin = #ident();
            // let container = ::quill::internal::PluginContainer::from(plugin);
            // Box::into_raw(Box::new(container))
        }
    }
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    let fields = match item.fields {
        Fields::Named(_) => {
            let fields = item.fields.iter().map(|field| {
                let field_ident = field.ident.as_ref().map(|ident| ident.to_string());
                let field_ty = &field.ty;
                quote! { (#field_ident, &<#field_ty as ::quill::layout::TypeLayout>::LAYOUT) }
            });
            quote! { quill::layout::Fields::Struct(&[#(#fields),*]) }
        },
        Fields::Unnamed(_) => {
            let fields = item.fields.iter().map(|field| {
                let field_ty = &field.ty;
                quote! {
                    &<#field_ty as ::quill::layout::TypeLayout>::LAYOUT
                }
            });
            quote! {
                ::quill::layout::Fields::Tuple(&[#(#fields),*])
            }
        },
        Fields::Unit => quote! { ::quill::layout::Fields::Unit },
    };

    let ident = &item.ident;
    
    let item = quote! {
        #[repr(C)]
        #item

        impl ::quill::layout::TypeLayout for #ident {
            const LAYOUT: ::quill::layout::Layout = ::quill::layout::Layout {
                name: ::std::any::type_name::<Self>(),
                version: 0,
                fields: &#fields,
            };
        }
    };

    TokenStream::from(item)
}

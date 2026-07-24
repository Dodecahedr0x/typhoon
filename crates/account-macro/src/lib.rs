use {
    keys::PrimaryKeys,
    quote::{quote, ToTokens},
    syn::{parse_macro_input, spanned::Spanned, Error, Ident, Item},
    typhoon_discriminator::DiscriminatorBuilder,
};

mod keys;

/// Reads the serialization strategy from `#[account_state(<strategy>)]`, where
/// `<strategy>` is `bytemuck` (default, zero-copy `Pod`), `wincode`, or `borsh`.
///
/// A helper attribute is used rather than sniffing sibling derives: the compiler
/// strips the `#[derive(...)]` list before invoking each derive macro, so a
/// derive can never see which *other* derives are present.
fn strategy_attr(attrs: &[syn::Attribute]) -> Option<syn::Result<Ident>> {
    attrs
        .iter()
        .find(|a| a.path().is_ident("account_state"))
        .map(|a| a.parse_args::<Ident>())
}

#[proc_macro_derive(AccountState, attributes(key, no_space, account_state))]
pub fn derive_account(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as Item);
    let (attrs, name, generics, fields) = match item {
        Item::Struct(ref item_struct) => (
            &item_struct.attrs,
            &item_struct.ident,
            &item_struct.generics,
            &item_struct.fields,
        ),
        _ => {
            return Error::new(item.span(), "Invalid account type")
                .into_compile_error()
                .into()
        }
    };

    let space_token = if attrs.iter().any(|a| a.path().is_ident("no_space")) {
        None
    } else {
        Some(quote! {
            impl #name {
                pub const SPACE: usize = <#name as Discriminator>::DISCRIMINATOR.len() + core::mem::size_of::<#name>();
            }
        })
    };
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let keys = match PrimaryKeys::try_from(fields) {
        Ok(fields) => fields,
        Err(err) => return err.to_compile_error().into(),
    };
    let seeded_trait = keys.split_for_impl(name);
    let discriminator = DiscriminatorBuilder::new(&name.to_string()).build();
    let account_strategy = match strategy_attr(attrs) {
        Some(Ok(ident)) => match ident.to_string().as_str() {
            "wincode" => quote!(
                WincodeStrategy<
                    {
                        matches!(
                            <Self as wincode::SchemaRead<'static, wincode::config::DefaultConfig>>::TYPE_META,
                            wincode::TypeMeta::Static { zero_copy: true, .. }
                        )
                    },
                >
            ),
            "borsh" => quote!(BorshStrategy),
            "bytemuck" => quote!(BytemuckStrategy),
            _ => {
                return Error::new(
                    ident.span(),
                    "Unknown account strategy (expected `bytemuck`, `wincode`, or `borsh`).",
                )
                .into_compile_error()
                .into()
            }
        },
        Some(Err(err)) => return err.into_compile_error().into(),
        None => quote!(BytemuckStrategy),
    };

    quote! {
        impl CheckOwner for #name #ty_generics #where_clause {
            #[inline(always)]
            fn owned_by(owner: &Address) -> bool {
                address_eq(owner, &crate::ID)
            }
        }

        impl Discriminator for #name #ty_generics #where_clause {
            const DISCRIMINATOR: &'static [u8] = &[#(#discriminator),*];
        }

        impl DataStrategy for #name #ty_generics #where_clause {
            type Strategy = #account_strategy;
        }

        #space_token

        #seeded_trait
    }
    .into_token_stream()
    .into()
}

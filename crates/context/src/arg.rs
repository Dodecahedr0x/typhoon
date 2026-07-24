use {
    crate::HandlerContext,
    solana_account_view::AccountView,
    solana_address::Address,
    typhoon_errors::Error,
    typhoon_traits::{Accessor, BytemuckStrategy},
};

pub type ArgData<'a, T, S> = <S as Accessor<'a, T>>::Data;

pub struct Arg<'a, T, S = BytemuckStrategy>(pub ArgData<'a, T, S>)
where
    S: Accessor<'a, T>;

impl<'b, 'c, T, S> HandlerContext<'_, 'b, 'c> for Arg<'c, T, S>
where
    S: Accessor<'c, T>,
{
    #[inline(always)]
    fn from_entrypoint(
        _program_id: &Address,
        accounts: &'b mut [AccountView],
        instruction_data: &mut &'c [u8],
    ) -> Result<(Self, &'b mut [AccountView]), Error> {
        Ok((Self(S::read(instruction_data)?), accounts))
    }
}

/// A raw instruction-data handler argument: a `&[u8]` parameter consumes the
/// remaining, undecoded instruction bytes.
///
/// `Arg<T>` decodes a fixed-size `T` with `bytemuck`; a variable-length tail
/// (an arbitrary payload, a serialized body, ...) has no fixed `T`, so it's
/// taken verbatim as a byte slice. Place it after the fixed context/args, the
/// way `AccountIter` takes the remaining accounts.
impl<'b, 'c> HandlerContext<'_, 'b, 'c> for &'c [u8] {
    #[inline(always)]
    fn from_entrypoint(
        _program_id: &Address,
        accounts: &'b mut [AccountView],
        instruction_data: &mut &'c [u8],
    ) -> Result<(Self, &'b mut [AccountView]), Error> {
        Ok((core::mem::take(instruction_data), accounts))
    }
}

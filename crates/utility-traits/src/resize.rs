use {
    core::ops::DerefMut,
    pinocchio::{sysvars::rent::Rent, AccountView},
    typhoon_accounts::{Account, Mut},
    typhoon_errors::Error,
    typhoon_traits::Discriminator,
    typhoon_utility::resize_account,
};

/// Imperative account resizing for program-owned data accounts.
///
/// Backs the `realloc` constraint family, and can also be called directly from a
/// handler for dynamic growth:
/// `registry.realloc(new_len, &mut payer, &rent)?`. Growing tops the account
/// back up to rent-exemption from `payer`; shrinking refunds the surplus to
/// `payer`. Newly-grown bytes are always zero-initialized by the runtime.
pub trait ReallocAccount: DerefMut<Target = AccountView> {
    #[inline(always)]
    fn realloc<P>(&mut self, new_len: usize, payer: &mut P, rent: &Rent) -> Result<(), Error>
    where
        P: DerefMut<Target = AccountView>,
    {
        let account: &mut AccountView = self;
        let payer: &mut AccountView = payer;
        resize_account(account, new_len, payer, rent).map_err(Into::into)
    }
}

impl<T> ReallocAccount for Mut<'_, Account<'_, T>> where T: Discriminator {}

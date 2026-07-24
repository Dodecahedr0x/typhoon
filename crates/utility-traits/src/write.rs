use {
    core::ops::DerefMut,
    pinocchio::{sysvars::rent::Rent, AccountView},
    typhoon_accounts::WritableAccountData,
    typhoon_errors::Error,
    typhoon_traits::{DataStrategy, Discriminator, Write},
    typhoon_utility::resize_account,
};

/// Serialize a value into a variable-length (`wincode`/`borsh`) account,
/// resizing the account to fit and settling rent against `payer`.
///
/// This is the write half of first-class variable-length account state: the
/// account's serialization strategy (selected by `#[derive(AccountState)]` from
/// a `SchemaRead`/`BorshDeserialize` derive) computes the encoded size, the
/// account is grown or shrunk to `discriminator + size`, and the value is
/// written in right after the discriminator.
///
/// ```ignore
/// note.write_data(&Note { author, body }, &mut payer, &rent)?;
/// ```
pub trait WriteAccountData: WritableAccountData + DerefMut<Target = AccountView> {
    #[inline(always)]
    fn write_data<P>(&mut self, value: &Self::Data, payer: &mut P, rent: &Rent) -> Result<(), Error>
    where
        P: DerefMut<Target = AccountView>,
        <Self::Data as DataStrategy>::Strategy: Write<Self::Data>,
    {
        let disc = <Self::Data as Discriminator>::DISCRIMINATOR.len();
        let size =
            <<Self::Data as DataStrategy>::Strategy as Write<Self::Data>>::size(value)?;

        {
            let account: &mut AccountView = self;
            let payer: &mut AccountView = payer;
            resize_account(account, disc + size, payer, rent)?;
        }

        let mut data = self.as_mut().try_borrow_mut()?;
        <<Self::Data as DataStrategy>::Strategy as Write<Self::Data>>::write_into(
            &mut data[disc..],
            value,
        )?;

        Ok(())
    }
}

impl<T> WriteAccountData for T where T: WritableAccountData + DerefMut<Target = AccountView> {}

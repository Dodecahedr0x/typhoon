use {
    pinocchio::{sysvars::rent::Rent, AccountView, ProgramResult, Resize},
    pinocchio_system::instructions::Transfer,
};

/// Resize an already-initialized, program-owned `account` to `new_len` bytes and
/// settle rent against `payer`.
///
/// This is the imperative core behind the `realloc` constraint family. Growing
/// tops the account back up to the rent-exempt minimum for its new size by
/// transferring from `payer` (which must be a system-owned signer). Shrinking
/// refunds the now-surplus lamports back to `payer`. The runtime zero-extends
/// any newly-grown bytes automatically.
///
/// A single instruction may only grow an account by
/// [`solana_account_view::MAX_PERMITTED_DATA_INCREASE`] bytes; larger growth
/// must be spread across multiple instructions.
#[inline(always)]
pub fn resize_account(
    account: &mut AccountView,
    new_len: usize,
    payer: &mut AccountView,
    rent: &Rent,
) -> ProgramResult {
    resize_account_signed(account, new_len, payer, rent, &[])
}

/// Like [`resize_account`] but for a PDA `payer` whose transfer must be signed
/// with `signers`.
#[inline(always)]
pub fn resize_account_signed(
    account: &mut AccountView,
    new_len: usize,
    payer: &mut AccountView,
    rent: &Rent,
    signers: &[pinocchio::cpi::Signer],
) -> ProgramResult {
    let new_minimum = rent.try_minimum_balance(new_len)?;
    let current = account.lamports();

    if new_minimum > current {
        Transfer {
            from: payer,
            to: account,
            lamports: new_minimum - current,
        }
        .invoke_signed(signers)?;
    } else if current > new_minimum {
        let surplus = current - new_minimum;
        account.set_lamports(new_minimum);
        payer.set_lamports(payer.lamports().wrapping_add(surplus));
    }

    account.resize(new_len)
}

use core::mem::MaybeUninit;

use pinocchio::{
    account_info::AccountInfo,
    cpi::{slice_invoke_signed, MAX_CPI_ACCOUNTS},
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

/// Memo instruction.
///
/// ### Accounts:
///   0. `..+N` `[SIGNER]` N signing accounts
pub struct Memo<'a> {
    /// Signing accounts
    pub signers: &'a [&'a AccountInfo],
    /// Memo
    pub memo: &'a str,
}

impl Memo<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers_seeds: &[Signer]) -> ProgramResult {
        const UNINIT_META: MaybeUninit<AccountMeta> = MaybeUninit::<AccountMeta>::uninit();

        // We don't know num_accounts at compile time, so we use MAX_CPI_ACCOUNTS
        let mut account_metas = [UNINIT_META; MAX_CPI_ACCOUNTS];

        let num_accounts = self.signers.len();

        for i in 0..num_accounts {
            unsafe {
                // SAFETY: num_accounts is less than MAX_CPI_ACCOUNTS
                account_metas
                    .get_unchecked_mut(i)
                    .write(AccountMeta::readonly_signer(self.signers[i].key()));
            }
        }

        // SAFETY: len(account_metas) <= MAX_CPI_ACCOUNTS
        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: unsafe {
                core::slice::from_raw_parts(account_metas.as_ptr() as _, num_accounts)
            },
            data: self.memo.as_bytes(),
        };

        slice_invoke_signed(&instruction, self.signers, signers_seeds)
    }
}

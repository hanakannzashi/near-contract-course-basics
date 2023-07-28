mod cross;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PublicKey, Promise, log, PromiseError, PromiseOrValue, PanicOnDefault};
use near_sdk::json_types::U128;
use crate::cross::linkdrop_contract;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    linkdrop_contract_id: AccountId
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(linkdrop_contract_id: AccountId) -> Self {
        Self { linkdrop_contract_id }
    }

    #[payable]
    pub fn create_account_by_linkdrop(
        &mut self,
        new_account_id: AccountId,
        new_public_key: PublicKey
    ) -> PromiseOrValue<()> {
        let amount = env::attached_deposit();

        linkdrop_contract::ext(self.linkdrop_contract_id.clone())
            .with_attached_deposit(amount)
            .create_account(new_account_id, new_public_key)
            .then(
                Self::ext(env::current_account_id())
                    .resolve_create_account(env::predecessor_account_id(), U128(amount))
            ).into()
    }

    pub fn resolve_create_account(
        &mut self,
        account_owner_id: AccountId,
        amount: U128,
        #[callback_result] is_success: Result<bool, PromiseError>
    ) {
        if is_success.unwrap_or(false) {
            log!("Account is successfully created.");
        } else {
            log!("Fail to create account, refund the money.");
            Promise::new(account_owner_id).transfer(amount.0);
        }
    }
}

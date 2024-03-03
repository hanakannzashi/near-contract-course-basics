use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::LookupMap;
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    descriptions: LookupMap<AccountId, String>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Descriptions,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            descriptions: LookupMap::new(StorageKey::Descriptions),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn set_account_description(&mut self, account_id: AccountId, description: String) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Only contract owner can call this method."
        );
        self.descriptions.insert(account_id, description);
    }

    pub fn get_account_description(&self, account_id: AccountId) -> Option<&String> {
        self.descriptions.get(&account_id)
    }
}

#[cfg(test)] // 标注测试模块
mod test {
    use crate::Contract;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    fn alice() -> AccountId {
        "alice.near".parse().unwrap()
    }

    fn bob() -> AccountId {
        "bob.near".parse().unwrap()
    }

    #[test] // 标注测试任务
    #[should_panic(expected = "Only contract owner can call this method.")] // 错误信息
    fn test_setter_without_permission() {
        let mut contract = Contract::init(alice());
        contract.set_account_description(bob(), "Nice Bob".to_string());
    }

    #[test]
    fn test_setter_getter() {
        let mut contract = Contract::init(alice());

        let context = VMContextBuilder::new()
            .predecessor_account_id(alice())
            .build();
        testing_env!(context);

        contract.set_account_description(bob(), "Nice Bob".to_string());
        let description = contract.get_account_description(bob()).unwrap();
        assert_eq!(description, "Nice Bob");
    }
}

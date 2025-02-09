#![cfg_attr(not(feature = "std"), no_std)]
#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;
    use ink::env::DefaultEnvironment;

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller: AccountId = ink::env::caller::<DefaultEnvironment>(); 
            let mut balances = Mapping::new();
            balances.insert(caller, &initial_supply); // ✅ Corrected: added `&`
            Self {
                total_supply: initial_supply,
                balances,
            }
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let caller: AccountId = ink::env::caller::<DefaultEnvironment>(); 
            let caller_balance = self.balance_of(caller);

            if let Some(new_caller_balance) = caller_balance.checked_sub(value) {
                self.balances.insert(caller, &new_caller_balance); 
            } else {
                return false; // Not enough balance
            }

            let receiver_balance = self.balance_of(to);

            if let Some(new_receiver_balance) = receiver_balance.checked_add(value) {
                self.balances.insert(to, &new_receiver_balance); 
            } else {
                return false; 
            }

            true
        }
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }
    }
}

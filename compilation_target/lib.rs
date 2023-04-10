#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    // imports from openbrush
    use openbrush::contracts::psp22::*;
    use openbrush::traits::Storage;
    use openbrush::contracts::ownable::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    // Section contains default implementation without any modifications
    impl PSP22 for Contract {}
    impl Ownable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn a(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
            _instance._mint_to(_instance.env().caller(), initial_supply).expect("Should mint"); 
            _instance._init_with_owner(_instance.env().caller());
            _instance
        }
    }
}
#[cfg(test)]
mod compiler_tests{
    use super::super::*;

    extern crate dotenv;
    use dotenv::dotenv;
    use std::fs::remove_file;

    #[test]
    #[ignore]
    fn works() {
        // Load env variables
        dotenv().ok();

        // Init compiler
        let queue = CompilationQueue::new();
        let compilation_queue = Arc::new(queue);
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let compiler = Compiler::init(compilation_queue, shutdown_flag.clone());

        let wizard_message = WizardMessage{
            address: "ABC".to_string(),
            code: LIB_RS_CODE.to_string(),
            features: vec!["psp22".to_string(), "ownable".to_string()],
        };

        // Create lib.rs file
        compiler.create_contract_files(&wizard_message)
            .expect("Could not create lib.rs file");

        // Compile contract
        let result = compiler.compile_contract();

        // Check if the contract was compiled successfully
        assert!(result.is_ok());

        // Delete test compilation
        compiler.delete_compilation_files();

        // Delete test file
        let file_path = compiler.dir_path.join("lib.rs");
        remove_file(&file_path).expect("Error deleting file");
    }

    const LIB_RS_CODE: &str = "#![cfg_attr(not(feature = \"std\"), no_std)]
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
            pub fn new(initial_supply: Balance) -> Self {
                let mut _instance = Self::default();
                _instance._mint_to(_instance.env().caller(), initial_supply).expect(\"Should mint\"); 
                _instance._init_with_owner(_instance.env().caller());
                _instance
            }
        }
    }";
}
#[cfg(test)]
mod contract_utils_tests {
    use super::super::*;

    mod create_cargo_toml_file {
        use super::*;

        #[test]
        fn works() {
            // Create test dir
            let dir_path = Path::new("create_cargo_toml_file_0");
            create_dir(dir_path).expect("Could not create dir");

            // Test features
            let features = vec!["psp22".to_string()];

            // Create file
            let result = create_cargo_toml_file(&features, dir_path);

            // Check if the file was created successfully
            assert!(result.is_ok());

            // Open and read file content
            let mut toml_file =
                File::open(dir_path.join("Cargo.toml")).expect("Could not open file");

            let mut toml = String::new();
            toml_file
                .read_to_string(&mut toml)
                .expect("Could not read file");

            let features_list = parse_features(&features).expect("Could not check features");

            // Check file content
            assert_eq!(toml, CARGO_TOML.replace("features_list", &features_list));

            // Delete test dir
            delete_files(dir_path);
        }

        #[test]
        fn dir_does_not_exist() {
            // Test dir path
            let dir_path = Path::new("create_cargo_toml_file_1");

            // Test features
            let features = vec!["psp22".to_string()];

            // Create file
            let result = create_cargo_toml_file(&features, dir_path);

            // Check that the file was not created because the dir does not exist
            assert!(result.is_err());
        }
    }

    mod create_lib_rs_file {
        use super::*;

        #[test]
        fn works() {
            // Create test dir
            let dir_path = Path::new("create_lib_rs_file_0");
            create_dir(dir_path).expect("Could not create dir");

            // Test code
            let code = "fn main() { println!(\"Hello, world!\"); }".to_string();

            // Create file
            let result = create_lib_rs_file(&code, dir_path);

            // Check if the file was created successfully
            assert!(result.is_ok());

            // Open and read file content
            let mut lib_rs_file = File::open(dir_path.join("lib.rs")).expect("Could not open file");

            let mut lib_rs = String::new();
            lib_rs_file
                .read_to_string(&mut lib_rs)
                .expect("Could not read file");

            // Check file content
            assert_eq!(lib_rs, code);

            // Delete test dir
            delete_files(dir_path);
        }

        #[test]
        fn dir_does_not_exist() {
            // Test dir path
            let dir_path = Path::new("create_lib_rs_file_1");

            // Test code
            let code = "fn main() { println!(\"Hello, world!\"); }".to_string();

            // Create file
            let result = create_lib_rs_file(&code, dir_path);

            // Check that the file was not created because the dir does not exist
            assert!(result.is_err());
        }
    }

    mod create_files {
        use super::*;

        #[test]
        fn works() {
            // Create WizardMessage
            let wizard_message = WizardMessage {
                address: "5DCqNV2n4hifzJDNKbsYn8UyMDWsP5aHvnU2mS4zuc6sUYkm".to_string(),
                features: vec!["psp22".to_string()],
                code: "fn main() { println!(\"Hello, world!\"); }".to_string(),
            };

            // Create files
            let result = create_files(&wizard_message);

            // Check if the files were created successfully
            assert!(result.is_ok());

            // Delete test dir
            delete_files(&result.unwrap());
        }

        #[test]
        fn dir_already_exists() {
            // Create test dir
            let dir_path = Path::new("/tmp/5Dsykc2KUHcziwcTgZkHxyDDTotBJbGNh3BakfZ5PdDGMzfm");
            create_dir(dir_path).expect("Could not create dir");

            // Create WizardMessage
            let wizard_message = WizardMessage {
                address: "5Dsykc2KUHcziwcTgZkHxyDDTotBJbGNh3BakfZ5PdDGMzfm".to_string(),
                features: vec!["psp22".to_string()],
                code: "fn main() { println!(\"Hello, world!\"); }".to_string(),
            };

            // Create files
            let result = create_files(&wizard_message);

            // Check that the files were not created because the dir already exists
            assert!(result.is_err());

            // Delete test dir
            delete_files(&dir_path);
        }
    }

    mod compile_contract {
        use super::*;
        extern crate dotenv;
        use dotenv::dotenv;

        #[test]
        #[ignore]
        fn works() {
            // Load env variables
            dotenv().ok();

            // Get cargo path
            let cargo = env::var("CARGO").expect("CARGO env variable not set");

            // Create test dir
            let dir_path = Path::new("compile_contract_0");
            create_dir(dir_path).expect("Could not create dir");

            // Create lib.rs file
            create_lib_rs_file(&LIB_RS_CODE.to_string(), dir_path)
                .expect("Could not create lib.rs file");

            // Create Cargo.toml file
            let features = vec!["psp22".to_string(), "ownable".to_string()];
            create_cargo_toml_file(&features, dir_path).expect("Could not create Cargo.toml file");

            // Compile contract
            let result = compile_contract(&cargo, dir_path);

            // Check if the contract was compiled successfully
            assert!(result.is_ok());

            // Delete test dir
            delete_files(dir_path);
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
}

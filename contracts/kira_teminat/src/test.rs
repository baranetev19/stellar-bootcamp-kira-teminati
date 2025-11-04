 use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_token_sdk::testutils as token_testutils;
use soroban_token_sdk::token::TokenClient;
 
#[test]
#[ignore]
 fn happy_path_release() {
     let env = Env::default();
     env.mock_all_auths();
 
     let tenant = Address::generate(&env);
     let landlord = Address::generate(&env);
    let token_admin = Address::generate(&env);
    // Register a token contract for testing (Stellar Asset style)
    let token_id = token_testutils::register_stellar_asset_contract(&env, &token_admin);
    let token_client = TokenClient::new(&env, &token_id);
 
     // Mint to tenant
    token_testutils::mint(&env, &token_id, &token_admin, &tenant, &1_000i128);
 
     // Deploy escrow
     let contract_id = env.register_contract(None, KiraTeminat);
     let client = KiraTeminatClient::new(&env, &contract_id);
 
     client.init(&tenant, &landlord, &token_id, &500).unwrap();
     client.fund().unwrap();
     assert!(client.is_funded_view());
 
     // Release to landlord
     client.release(&landlord).unwrap();
     assert!(!client.is_funded_view());
 
    // Landlord should have +500
    let landlord_bal = token_client.balance(&landlord);
     assert_eq!(landlord_bal, 500);
 }
 
#[test]
#[ignore]
 fn refund_flow() {
     let env = Env::default();
     env.mock_all_auths();
 
     let tenant = Address::generate(&env);
     let landlord = Address::generate(&env);
    let admin = Address::generate(&env);
    let token_id = token_testutils::register_stellar_asset_contract(&env, &admin);
    let token_client = TokenClient::new(&env, &token_id);
    token_testutils::mint(&env, &token_id, &admin, &tenant, &1_000i128);
 
     let contract_id = env.register_contract(None, KiraTeminat);
     let client = KiraTeminatClient::new(&env, &contract_id);
     client.init(&tenant, &landlord, &token_id, &250).unwrap();
     client.fund().unwrap();
     client.refund().unwrap();
     assert!(!client.is_funded_view());
 
     // Tenant should have at least 750 now (1000 - 250 + 250 = 1000)
     let tenant_bal = token_client.balance(&tenant);
     assert_eq!(tenant_bal, 1_000);
 }


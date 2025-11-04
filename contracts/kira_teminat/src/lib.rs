 #![no_std]
 
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, IntoVal, Symbol};
use soroban_token_sdk::token::TokenClient as TokenClient;
 
 #[derive(Clone)]
 #[contracttype]
 pub enum DataKey {
     Config,
     Funded,
 }
 
 #[derive(Clone)]
 #[contracttype]
 pub struct Config {
     pub tenant: Address,
     pub landlord: Address,
     pub token: Address,
     pub amount: i128,
 }
 
 #[contracterror]
 #[derive(Copy, Clone, PartialEq, Eq, Debug)]
 pub enum EscrowError {
     AlreadyInitialized = 1,
     NotInitialized = 2,
     AlreadyFunded = 3,
     NotFunded = 4,
     InvalidAmount = 5,
 }
 
 fn read_config(env: &Env) -> Result<Config, EscrowError> {
     env.storage().instance().get(&DataKey::Config).ok_or(EscrowError::NotInitialized)
 }
 
 fn write_config(env: &Env, cfg: &Config) {
     env.storage().instance().set(&DataKey::Config, cfg);
     // TTL for instance storage can be extended by off-chain ops if desired
 }
 
 fn set_funded(env: &Env) {
     env.storage().instance().set(&DataKey::Funded, &true);
 }
 
 fn is_funded(env: &Env) -> bool {
     env.storage().instance().get(&DataKey::Funded).unwrap_or(false)
 }
 
 #[contract]
 pub struct KiraTeminat;
 
 #[contractimpl]
 impl KiraTeminat {
     // Initialize escrow with tenant, landlord, token address and amount.
     // Only callable once. Creator must be tenant to simplify flows.
     pub fn init(env: Env, tenant: Address, landlord: Address, token: Address, amount: i128) -> Result<(), EscrowError> {
         if env.storage().instance().has(&DataKey::Config) {
             return Err(EscrowError::AlreadyInitialized);
         }
         if amount <= 0 {
             return Err(EscrowError::InvalidAmount);
         }
 
         // Require tenant authorization to create the escrow
         tenant.require_auth();
 
         let cfg = Config { tenant, landlord, token, amount };
         write_config(&env, &cfg);
         Ok(())
     }
 
     // Tenant funds the escrow by transferring tokens into the contract.
     pub fn fund(env: Env) -> Result<(), EscrowError> {
         let cfg = read_config(&env)?;
         if is_funded(&env) {
             return Err(EscrowError::AlreadyFunded);
         }
 
         // Require tenant authorization for transfer_from
         cfg.tenant.require_auth();
 
        let client = TokenClient::new(&env, &cfg.token);
        client.transfer_from(&cfg.tenant, &Address::from_contract_id(&env.current_contract()), &cfg.amount);
 
         set_funded(&env);
         // Emit event
         emit_event(&env, symbol_short!("funded"), (&cfg.tenant, cfg.amount).into_val(&env));
         Ok(())
     }
 
     // Release funds out of escrow. Requires both tenant and landlord authorization in the same invocation.
     // Receiver can be landlord, tenant, or any third party per agreement.
     pub fn release(env: Env, receiver: Address) -> Result<(), EscrowError> {
         let cfg = read_config(&env)?;
         if !is_funded(&env) {
             return Err(EscrowError::NotFunded);
         }
 
         // 2-of-2 managed auth: both must authorize this call
         cfg.tenant.require_auth();
         cfg.landlord.require_auth();
 
        let client = TokenClient::new(&env, &cfg.token);
        client.transfer(&Address::from_contract_id(&env.current_contract()), &receiver, &cfg.amount);
 
         // Clear funded flag to prevent double-spend (optional: could also delete config)
         env.storage().instance().remove(&DataKey::Funded);
 
         emit_event(&env, symbol_short!("released"), (&receiver, cfg.amount).into_val(&env));
         Ok(())
     }
 
     // Refund to tenant. Also requires both parties to authorize (mutual cancellation).
     pub fn refund(env: Env) -> Result<(), EscrowError> {
         let cfg = read_config(&env)?;
         if !is_funded(&env) {
             return Err(EscrowError::NotFunded);
         }
 
         cfg.tenant.require_auth();
         cfg.landlord.require_auth();
 
        let client = TokenClient::new(&env, &cfg.token);
        client.transfer(&Address::from_contract_id(&env.current_contract()), &cfg.tenant, &cfg.amount);
 
         env.storage().instance().remove(&DataKey::Funded);
         emit_event(&env, symbol_short!("refunded"), (&cfg.tenant, cfg.amount).into_val(&env));
         Ok(())
     }
 
     // Views
     pub fn get_config(env: Env) -> Option<Config> {
         env.storage().instance().get(&DataKey::Config)
     }
 
     pub fn is_funded_view(env: Env) -> bool {
         is_funded(&env)
     }
 }
 
 fn emit_event(env: &Env, ty: Symbol, data: soroban_sdk::Val) {
     let topics = (ty,).into_val(env);
     env.events().publish(topics, data);
 }
 
 #[cfg(test)]
 mod test;


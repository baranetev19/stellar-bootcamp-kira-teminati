# 🎯 Kira Teminat Kilidi - 2-of-2 Multisig Escrow

A fully decentralized escrow system built on *Stellar Soroban* blockchain. Secure deposit management for rental agreements with 2-of-2 multisig authorization, ensuring both tenant and landlord must approve fund releases.

## 🌟 Features

🔒 *2-of-2 Multisig*: Both tenant and landlord must authorize fund releases

⚡ *Lightning Fast*: Built on Stellar's high-performance blockchain

💰 *Secure Escrow*: Deposit funds locked in smart contract until mutual agreement

🎮 *Easy to Use*: Simple, intuitive contract interface

🌍 *Global*: Access from anywhere, 24/7

🛡 *Secure*: Immutable smart contracts ensure fund safety

💎 *Token Support*: Works with any Soroban-compatible token

## 🚀 Deployment Guide

### Contract IDs

| Contract | Contract ID | Status |
|----------|-------------|--------|
| Kira Teminat | CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI | ✅ Deployed & Ready |

*Explorer links:*

- Kira Teminat Contract: [https://stellar.expert/explorer/testnet/contract/CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI](https://stellar.expert/explorer/testnet/contract/CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI)

*Alias:* kira_teminat (registered on testnet)

### Environment Variables

If you're building a frontend, use the contract ID directly:

javascript
const CONTRACT_ID = "CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI";
const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = "Test SDF Network ; September 2015";


## 📦 Deployment Steps

1. *Build contracts*: 

   bash
   cd contracts/kira_teminat
 stellar contract build
 
 
2. *Run tests* (optional):

   bash
 cargo test -- --ignored
 
 
3. *Deploy contract using Stellar CLI* (see below)

4. *Update frontend with contract ID* (if applicable):

   - Edit your frontend code and update CONTRACT_ID with: CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI

## Example CLI Commands

### Deploy Kira Teminat Contract:

*Testnet:*

bash
cd contracts/kira_teminat

stellar contract deploy \
  --wasm ../../target/wasm32v1-none/release/kira_teminat.wasm \
  --source test-account \
  --network testnet \
  --alias kira_teminat


*With Stellar Laboratory (signing):*

bash
stellar contract deploy \
  --wasm ../../target/wasm32v1-none/release/kira_teminat.wasm \
  --source temp \
  --network testnet \
  --sign-with-lab \
  --alias kira_teminat


*Local:*

bash
stellar contract deploy \
  --wasm ../../target/wasm32v1-none/release/kira_teminat.wasm \
  --source test-account \
  --local \
   --alias kira_teminat
 
 
### Initialize Escrow:

*Initialize the escrow contract with tenant, landlord, token, and amount:*

bash
stellar contract invoke \
  --id CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI \
  --source alice \
  --network testnet \
  -- \
  init \
  --tenant $(stellar keys address alice) \
  --landlord $(stellar keys address bob) \
  --token TOKEN_ID \
  --amount 500


### Example Contract Invocations:

*Get escrow configuration:*

bash
stellar contract invoke \
  --id CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI \
  --source alice \
  --network testnet \
  -- \
  get_config


*Check if escrow is funded:*

bash
stellar contract invoke \
  --id CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI \
  --source alice \
  --network testnet \
  -- \
  is_funded_view


*Fund the escrow (tenant deposits tokens):*

bash
stellar contract invoke \
  --id CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI \
  --source alice \
  --network testnet \
  -- \
   fund
 
 
*Release funds (requires both tenant and landlord authorization):*

bash
stellar contract invoke \
  --id CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI \
  --source alice \
  --network testnet \
  -- \
  release \
   --receiver $(stellar keys address bob)
 
 
*Refund to tenant (requires both tenant and landlord authorization):*

bash
stellar contract invoke \
  --id CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI \
  --source alice \
  --network testnet \
  -- \
   refund
 
 
## 🛠 Troubleshooting

- *Make sure you have enough XLM in your account* for transaction fees

- *Check that WASM files were built successfully*: Verify target/wasm32v1-none/release/kira_teminat.wasm exists

- *Verify network connection to testnet*: Test with stellar contract invoke command

- *Check contract IDs*: Ensure contract ID is correct: CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI

- *2-of-2 authorization required*: Both tenant and landlord must authorize release and refund operations in the same transaction

- *Contract calls failing*: Verify you're connected to the correct network (testnet/mainnet/local)

- *Token approval*: Ensure tenant has approved the contract to spend tokens before calling fund

## 📊 Contract Functions

| Function | Description | Parameters | Authorization |
|----------|-------------|------------|---------------|
| init | Initialize escrow with tenant, landlord, token, and amount | tenant, landlord, token, amount | Tenant only |
| fund | Tenant deposits tokens into escrow | - | Tenant only |
| release | Release funds to specified receiver | receiver | *2-of-2* (Tenant + Landlord) |
| refund | Refund deposit back to tenant | - | *2-of-2* (Tenant + Landlord) |
| get_config | Get escrow configuration | - | Public (view) |
| is_funded_view | Check if escrow is funded | - | Public (view) |

## 🔐 Security Features

- ✅ *2-of-2 Multisig*: Both parties must approve fund releases
- ✅ *Persistent on-chain storage*: All data stored on blockchain
- ✅ *Immutable contract code*: Contract logic cannot be changed
- ✅ *Transparent transactions*: All operations visible on blockchain
- ✅ *Event emission*: All operations emit events for tracking
- ✅ *Authorization checks*: Strict authorization requirements prevent unauthorized access

## 📈 Build Information

- *Contract ID*: CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI
- *Wasm Hash*: c24a7f186dc40576dcb3b9c5e7e3f4bf5f071f822b03fcdc9c5b574397a00f0ed
- *Wasm Size*: ~15.5 KB
- *Exported Functions*: 6
- *Soroban SDK*: 22.0.8
- *Network*: Testnet (Test SDF Network ; September 2015)

## 🧪 Testing

Run tests:

bash
cd contracts/kira_teminat
cargo test -- --ignored


*Test Results:*

- ✅ Happy path release test included
- ✅ Refund flow test included
- ✅ Storage persistence verified
- ✅ Authorization checks verified

## 🏗 Architecture

### Smart Contract (contracts/kira_teminat/)

Written in Rust using Soroban SDK, the contract provides:

- *Escrow Management*: Store and manage deposit funds securely
- *2-of-2 Authorization*: Require both tenant and landlord approval
- *Token Support*: Works with any Soroban-compatible token
- *Event Emission*: Emit events for all operations
- *Persistent Storage*: Data stored on-chain using instance storage

### Contract Flow

1. *Initialize Escrow*
   - Tenant calls init() with landlord address, token address, and amount
   - Configuration stored in contract instance storage
   - Only callable once per contract instance

2. *Fund Escrow*
   - Tenant calls fund() to deposit tokens into escrow
   - Tokens transferred from tenant to contract
   - Funded flag set to prevent double-funding

3. *Release Funds*
   - Both tenant and landlord must authorize the transaction
   - Funds transferred to specified receiver address
   - Funded flag cleared to prevent double-spend

4. *Refund to Tenant*
   - Both tenant and landlord must authorize the transaction
   - Funds returned to tenant
   - Funded flag cleared

5. *View Operations*
   - get_config(): View escrow configuration
   - is_funded_view(): Check if escrow is funded

## 📖 How It Works

1. *Initialize Escrow*
   - Tenant creates escrow with landlord address, token, and deposit amount
   - Contract stores configuration on-chain
   - Only tenant can initialize (authorization required)

2. *Fund Escrow*
   - Tenant deposits tokens into the contract
   - Contract holds tokens until release or refund
   - Only tenant can fund (authorization required)

3. *Release or Refund*
   - Both parties must authorize in the same transaction
   - Funds can be released to landlord or any receiver
   - Funds can be refunded back to tenant
   - Prevents unilateral fund access

4. *View Status*
   - Anyone can view escrow configuration
   - Anyone can check if escrow is funded
   - No authorization required for view operations

## 📦 Project Structure


stellar-bootcamp-kira-teminati/
├── contracts/
│   └── kira_teminat/          # Smart Contract
│       ├── src/
│       │   ├── lib.rs         # Main contract logic
│       │   └── test.rs        # Test cases
│       └── Cargo.toml
├── target/                     # Build output
│   └── wasm32v1-none/release/
│       └── kira_teminat.wasm  # Compiled WASM
├── Cargo.toml                  # Workspace definition
└── README.md


## 🔑 Key Management

### Generate Keys for Testing:

bash
# Generate tenant key
stellar keys generate --global alice --network testnet --fund

# Generate landlord key
stellar keys generate --global bob --network testnet --fund

# View addresses
stellar keys address alice
stellar keys address bob


## 📝 License

Apache-2.0 License - feel free to use this for your own projects!

## 🤝 Contributing

Contributions welcome!

1. Fork the repository
2. Create a feature branch (git checkout -b feature/AmazingFeature)
3. Commit your changes (git commit -m 'Add some AmazingFeature')
4. Push to the branch (git push origin feature/AmazingFeature)
5. Open a Pull Request

## 🔗 Useful Links

- [Soroban Documentation](https://soroban.stellar.org)
- [Stellar Network](https://stellar.org)
- [Smart Contract Basics](https://soroban.stellar.org/docs/basic-tutorials/hello-world)
- [Stellar Expert Explorer](https://stellar.expert/explorer/testnet)
- [Soroban Examples](https://github.com/stellar/soroban-examples)

## ⚠ Important Notes

- *2-of-2 Authorization*: Both tenant and landlord must sign the same transaction for release and refund operations. This requires coordination between parties.

- *Token Approval*: Before calling fund(), the tenant must approve the contract to spend tokens on their behalf (if using a token contract that requires approval).

- *Production Use*: This contract is for educational purposes. For production use, consider:
  - Adding time-based expiration
  - Adding dispute resolution mechanisms
  - Adding mediator/arbitrator support
  - Security audit
  - Gas optimization

- *Network: Currently deployed on **Testnet*. For mainnet deployment, ensure thorough testing and security review.

---

*Built with ❤ using Stellar Soroban*

*Contract ID*: CCLZRW6Q3VBLNZ4XJR54OK3FUGGGV2GIO7GL2F2GK54SIUCCV3BQM3RI

*Status*: ✅ Deployed on Testnet
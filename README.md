# ⚙️ AMM on Solana (Anchor Framework)

A fully on-chain **Automated Market Maker (AMM)** built in **Rust** using **Anchor**.  
Implements a **constant product (x × y = k)** liquidity model.

---

## Overview
- Initialize a pool for two SPL tokens  
- Add liquidity (deposit A + B → mint LP)  
- Swap tokens (A↔B via k-invariant)  
- Remove liquidity (burn LP → withdraw share)  

Uses PDAs for vault authority, LP mint, and pool security.

---

## Smart Contract Logic

### 1️⃣ Initialize Pool
Creates `Pool` account and PDAs for:
- Pool authority → controls vaults  
- LP mint → issues liquidity tokens  
- Vault ATAs for Token A/B  

---

### 2️⃣ Add Liquidity
- Transfers user tokens → vaults  
- First LP → `sqrt(A×B)` liquidity  
- Later LPs maintain ratio via:  
  `lp = total_liq × (amount_a / vault_a)`  
- Mints LP tokens + updates pool total.

---

### 3️⃣ Swap Tokens
- Constant product invariant `x*y=k`
- Swap output: `Δy = y - (k / (x + Δx))`
- Uses signer seeds for PDA authority.

---

### 4️⃣ Remove Liquidity
- Burns LP tokens  
- Returns proportional A + B amounts  
- Updates `total_liquidity`

---

## Testing
All features tested in `tests/amm.ts`  
Run:
```bash
yarn install
anchor test
```
Example output:
```
✔ Initialized!
✔ Added Liquidity!
✔ Swap Successful!
✔ Removed Liquidity!
4 passing (4s)
```

---

## PDAs Used

| Account | Seeds | Purpose |
|----------|--------|----------|
| Pool | `["pool", mintA, mintB]` | Pool instance |
| Authority | `["authority", poolKey]` | Vault signer |
| LP Mint | `["mint", poolKey]` | LP token mint |
| Vaults | Associated Token Accounts | Token storage |

---

## Token Flow
```
User Wallet  →  Pool Vaults (A,B)
     ↓                ↑
   Mint LP        Burn LP
     ↓                ↑
 User LP ATA   ←   Withdraw Assets
```

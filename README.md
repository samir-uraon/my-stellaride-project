# 🚀 Affiliate Platform (Soroban Beginner Project)

## 📌 Project Description

This is a **basic beginner-level Affiliate Platform smart contract** built using **Soroban on the Stellar blockchain**.

It demonstrates how to:

* Register users as affiliates
* Store referral codes
* Track who referred whom

This project is designed for learning and understanding **smart contract basics on Stellar**.

---

## 🖼️ Project Screenshot

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/03e4fb86-9c75-4246-afd2-09554e1a1366" />


---

## 🌐 Deployed Smart Contract

🔗 https://stellar.expert/explorer/testnet/contract/CDIKTBH3WCRLIFY3EHZ7NR6B6JO5YFLQ5GQGPOVKN7OQQTJZMF6RBKDW

📄 **Contract Address:**

```bash
CDIKTBH3WCRLIFY3EHZ7NR6B6JO5YFLQ5GQGPOVKN7OQQTJZMF6RBKDW
```

---

## ⚙️ What It Does

* Register users as affiliates
* Store referral codes
* Record referrals between users
* Retrieve affiliate and referral data

---

## ✨ Features

* ✅ Simple and beginner-friendly
* 🔐 Basic authentication using `require_auth()`
* 📦 On-chain storage
* ⚡ Fast and low-cost transactions
* 🧱 Built with Soroban

---

## 🛠️ Tech Stack

* Rust (Soroban SDK)
* Stellar Testnet
* WASM Smart Contracts

---

## 📂 Functions

### Register Affiliate

```rust
register(env, user, code)
```

### Get Affiliate Code

```rust
get_code(env, user)
```

### Record Referral

```rust
record_referral(env, referrer, referee)
```

### Get Referrer

```rust
get_referrer(env, user)
```

---

## 🚀 Future Scope

* Add reward system 💰
* Add token integration 🪙
* Build frontend dashboard 🌐

---

## 🧑‍💻 Author

Built for learning Soroban 🚀

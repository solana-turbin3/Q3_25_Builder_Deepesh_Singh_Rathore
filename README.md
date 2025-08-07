# Q3 2025 Builder Program - Deepesh Singh Rathore

This repository contains my work for the Turbin3 Q3 2025 Builder Program, showcasing various Solana blockchain development projects and exercises.

## 🚀 Projects Overview

### Core Blockchain Development

#### **Airdrop System** (`/airdrop`)
- Token airdrop implementation with enrollment and key generation
- Features: User enrollment, secure key management, token distribution
- **Tech Stack:** TypeScript, Solana Web3.js

#### **Automated Market Maker (AMM)** (`/amm`)
- Decentralized exchange protocol implementation
- Features: Liquidity pools, token swapping, price discovery
- **Tech Stack:** Rust, Anchor Framework

#### **Anchor Marketplace** (`/anchor-marketplace`)
- NFT marketplace built with Anchor framework
- Features: Listing, buying, selling NFTs
- **Tech Stack:** Rust, Anchor, TypeScript

#### **Escrow System** (`/escrow`)
- Secure escrow service for peer-to-peer transactions
- Features: Multi-party agreements, automated settlements
- **Tech Stack:** Rust, Anchor Framework

#### **Vault Program** (`/vault`)
- Secure token storage and management system
- Features: Multi-signature wallets, time-locked transactions
- **Tech Stack:** Rust, Anchor Framework

### Capstone Project

#### **Trustplay Gaming Platform** (`/Trustplay_Capstone_Project`)
- Blockchain-based gaming platform with room joining functionality
- Features: Game room management, player matching, secure transactions
- **Tech Stack:** Rust, Anchor, TypeScript

### Learning Modules

#### **Solana Starter** (`/solana-starter`)
- Foundational Solana development exercises
- **Rust Components** (`/rs`):
  - Cluster interactions
  - Program development basics
  - Prerequisites and setup
- **TypeScript Components** (`/ts`):
  - NFT minting and metadata
  - SPL token operations
  - Vault management
  - Transaction handling

#### **Rust Fundamentals** (`/rust`)
- Core Rust programming concepts and implementations
- Foundation for Solana program development

#### **Convert Utilities** (`/convert`)
- Utility tools for data conversion between different formats
- **Tech Stack:** TypeScript, CommonJS

## 🛠️ Technologies Used

- **Languages:** Rust, TypeScript, JavaScript
- **Frameworks:** Anchor, Solana Web3.js
- **Blockchain:** Solana
- **Tools:** Cargo, npm, Anchor CLI

## 📁 Repository Structure

```
├── airdrop/                 # Token airdrop system
├── amm/                     # Automated Market Maker
├── anchor-marketplace/      # NFT marketplace
├── escrow/                  # Escrow service
├── vault/                   # Vault program
├── Trustplay_Capstone_Project/ # Gaming platform capstone
├── solana-starter/          # Learning exercises
├── rust/                    # Rust fundamentals
└── convert/                 # Utility tools
```

## 🚦 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/deepesh-sr/Q3_25_Builder_Deepesh_Singh_Rathore.git
cd Q3_25_Builder_Deepesh_Singh_Rathore
```

2. Set up Solana environment:
```bash
solana config set --url localhost
solana-keygen new
```

3. For each project directory, install dependencies:
```bash
# For TypeScript projects
npm install

# For Rust/Anchor projects
anchor build
```

### Running Projects

Each project directory contains its own README with specific instructions. Generally:

- **Anchor Projects**: Use `anchor build`, `anchor test`, `anchor deploy`
- **TypeScript Projects**: Use `npm run build`, `npm test`, `npm start`
- **Rust Projects**: Use `cargo build`, `cargo test`, `cargo run`

## 📚 Learning Path

1. **Foundation**: Start with `/rust` and `/solana-starter`
2. **Core Concepts**: Explore `/airdrop` and `/vault`
3. **Advanced Features**: Study `/amm` and `/escrow`
4. **Marketplace Development**: Dive into `/anchor-marketplace`
5. **Capstone Application**: Review `/Trustplay_Capstone_Project`

## 🎯 Key Learning Outcomes

- Solana program development with Rust and Anchor
- Smart contract design patterns and security best practices
- DeFi protocol implementation (AMM, Escrow)
- NFT marketplace development
- Full-stack blockchain application development
- Token economics and distribution mechanisms

## 📖 Documentation

- Each project contains detailed documentation in its respective directory
- Code is thoroughly commented for educational purposes
- Test files demonstrate usage patterns and expected behaviors

## 🤝 Contributing

This repository represents coursework for the Turbin3 Builder Program. While primarily for educational purposes, feedback and suggestions are welcome through issues and discussions.

## 📄 License

This project is part of the Turbin3 Builder Program curriculum. Please respect the educational nature of this work.

---

**Author:** Deepesh Singh Rathore  
**Program:** Turbin3 Q3 2025 Builder Program  
**Focus:** Solana Blockchain Development

For questions or collaboration opportunities, please reach out through GitHub issues or discussions.

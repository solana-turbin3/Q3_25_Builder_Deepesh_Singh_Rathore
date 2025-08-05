# JoinRoom - Solana Learning Project

A simple Solana program built with Anchor framework for learning blockchain development concepts.

## Overview

This project demonstrates basic Solana program development patterns including:
- Creating and initializing program-derived accounts (PDAs)
- Room creation and participant management
- Cross-program invocations and account validation

## Features

- **Initialize Room**: Create a new room with a custom name
- **Join Room**: Allow participants to join existing rooms
- **State Management**: Track room data and participant information using PDAs

## Quick Start

```bash
# Install dependencies
npm install

# Build the program
anchor build

# Run tests
anchor test

# Deploy (localnet)
anchor deploy
```

## Learning Objectives

This project covers fundamental Solana concepts:
- Program Derived Addresses (PDAs)
- Account initialization and rent exemption
- Instruction handlers and context validation
- TypeScript client integration with Anchor

## Structure

- `programs/joinroom/` - Rust program source code
- `tests/` - TypeScript tests using Anchor framework
- `target/` - Compiled program artifacts

---

*This is a learning project for exploring Solana blockchain development.*
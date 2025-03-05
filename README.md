# FRAME Pallet Challenge - Polkadot SDK Tutorial

This project provides a hands-on introduction to Substrate's FRAME development patterns by implementing a simplified version of common blockchain pallets.

## Challenge Overview

After learning about Substrate, FRAME, Runtime, and Pallets in Polkadot SDK, this coding challenge helps you understand how these components work together by implementing a simplified version of the actual architecture.

### Key Concepts:

- **FRAME Pallets**: Libraries for building Substrate runtime, including core frame components and functional pallets
- **Runtime**: A collection of FRAME pallets that define blockchain logic

### Project Structure

This challenge simulates three core components:
- `system.rs`: Foundation module similar to `frame_system`
- `staking.rs`: Token staking module similar to `pallet_staking`
- `governance.rs`: On-chain proposal system similar to `pallet_collective` or `pallet_democracy`

### Runtime Configuration

The project defines a simplified runtime that configures concrete types for the generic pallets:

```rust
pub struct Runtime;

impl SystemConfig for Runtime {
    type AccountId = u64;
}

impl StakingConfig for Runtime {
    type Balance = u64;
}

impl GovernanceConfig for Runtime {
    // No additional types needed
}
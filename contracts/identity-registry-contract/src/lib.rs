#![no_std]

mod storage;
mod types;
mod events;
mod error;
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct IdentityRegistryContract;

#[contractimpl]
impl IdentityRegistryContract {
    // Functions should be added here
}
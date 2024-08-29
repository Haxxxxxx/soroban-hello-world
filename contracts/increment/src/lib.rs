#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter and returns the value.
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0); // If no value set, assume 0.
        
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Save the count.
        env
            .storage()
            .instance()
            .set(&COUNTER, &count);

        // Extend the contract's lifetime.
        env.storage().instance().extend_ttl(50, 100);

        // Return the count to the caller.
        count
    }

    /// Decrement decreases an internal counter and returns the value.
    pub fn decrement(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0); // If no value set, assume 0.

        log!(&env, "count: {}", count);

        // Decrement the count if it's greater than 0.
        if count > 0 {
            count -= 1;
        }

        // Save the count.
        env
            .storage()
            .instance()
            .set(&COUNTER, &count);

        // Extend the contract's lifetime.
        env.storage().instance().extend_ttl(50, 100);

        // Return the count to the caller.
        count
    }

    /// Get the current value of the counter.
    pub fn get_current_value(env: Env) -> u32 {
        // Get the current count.
        let count: u32 = env
            .storage()
            .instance()
            .get(&COUNTER)
            .unwrap_or(0); // If no value set, assume 0.

        log!(&env, "current count: {}", count);

        // Return the current count to the caller.
        count
    }

    /// Reset the counter to zero.
    pub fn reset(env: Env) {
        // Reset the count to 0.
        let count: u32 = 0;

        log!(&env, "resetting count to: {}", count);

        // Save the reset count.
        env
            .storage()
            .instance()
            .set(&COUNTER, &count);

        // Extend the contract's lifetime.
        env.storage().instance().extend_ttl(50, 100);
    }
}

mod test;

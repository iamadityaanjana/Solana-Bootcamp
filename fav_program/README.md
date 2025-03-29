# Favorite Program on Solana

This repository contains a Solana program written in Rust using the Anchor framework. The program allows users to set and store their favorite attributes, including their favorite number, color, and hobbies.

## Features

- Store a user's favorite number, color, and hobbies on the Solana blockchain.
- Use Anchor's account management and PDA (Program Derived Address) features for secure and efficient data storage.
- Automatically initialize accounts if they do not exist.

## Program Overview

### Program ID
The program ID is automatically generated when deployed on the Solana Playground:
```
9mtgSz5kAGxgCi4iJw8XGr5xtRAUjMdbea7LPuhLr36p
```

### Main Function: `set_fav`

The `set_fav` function allows users to set their favorite attributes. It logs the user's public key, favorite number, color, and hobbies, and updates the `Favorites` account.

#### Arguments:
- `context`: A `Context<SetFav>` object providing access to the accounts involved in the transaction.
- `number`: A `u64` representing the user's favorite number.
- `color`: A `String` representing the user's favorite color.
- `hobbies`: A `Vec<String>` containing the user's favorite hobbies.

#### Example:
```rust
let context = ...; // Context<SetFav> object
let number = 42;
let color = String::from("blue");
let hobbies = vec![String::from("reading"), String::from("coding")];

fav::set_fav(context, number, color, hobbies)?;
```

## Account Structures

### `Favorites` Account
A struct representing a user's favorite attributes:
- `number`: A 64-bit unsigned integer.
- `color`: A string with a maximum length of 50 characters.
- `hobbies`: A vector of strings, each with a length between 5 and 50 characters.

### `SetFav` Context
Defines the accounts required for the `set_fav` instruction:
- `user`: A mutable signer account representing the user.
- `fav`: An account of type `Favorites`, initialized if needed.
- `system_program`: A reference to the Solana System Program.

## How to Use

1. Deploy the program on the Solana Playground or your preferred Solana environment.
2. Use the `set_fav` function to set your favorite attributes.
3. Query the `Favorites` account to retrieve stored data.

## Dependencies

- [Anchor](https://github.com/coral-xyz/anchor): A framework for Solana smart contract development.
- Rust programming language.



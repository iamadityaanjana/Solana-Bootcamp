//import anchor to the program
use anchor_lang::prelude::*;

// program id for smart contract - automatically added when we run the program on solana playground
declare_id!("9mtgSz5kAGxgCi4iJw8XGr5xtRAUjMdbea7LPuhLr36p");

//anchor to specify what type of account it is
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; // save 8 byte + whatever we are saving


#[program]
/// Sets the favorite attributes for a user, including their favorite number, color, and hobbies.
///
/// # Arguments
///
/// * `context` - A `Context<SetFav>` object that provides access to the accounts involved in the transaction.
/// * `number` - A `u64` representing the user's favorite number.
/// * `color` - A `String` representing the user's favorite color.
/// * `hobbies` - A `Vec<String>` containing the user's favorite hobbies.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the operation is successful, otherwise returns an error.
///
/// # Behavior
///
/// This function logs the user's public key, favorite number, favorite color, and hobbies.
/// It then updates the `fav` account with the provided favorite attributes.
///
/// # Example
///
/// ```rust
/// let context = ...; // Context<SetFav> object
/// let number = 42;
/// let color = String::from("blue");
/// let hobbies = vec![String::from("reading"), String::from("coding")];
///
/// fav::set_fav(context, number, color, hobbies)?;
/// ```
pub mod fav{
    use super::*;

    pub fn set_fav(context : Context<SetFav>,
     number:u64 ,
      color:String ,
       hobbies : Vec<String>)->Result<()>{
        //add function 
        let user_public_key= context.accounts.user.key();

        msg!(
            "User {user_public_key}'s favorite number is {number}, favorite color is: {color}",
        );

        msg!(
            "User's hobbies are: {:?}",
            hobbies
        ); 

        context.accounts.fav.set_inner(Favorites{
            number,
            color,
            hobbies

        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
/// A struct representing a user's favorites.
///
/// # Fields
/// 
/// * `number` - A 64-bit unsigned integer representing a favorite number.
/// 
/// * `color` - A string representing a favorite color. The maximum length of this string is 50 characters.
/// 
/// * `hobbies` - A vector of strings representing favorite hobbies. Each string in the vector must have a length
///   between 5 and 50 characters.
pub struct Favorites{
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5,50)]
    pub hobbies: Vec<String>

}

#[derive(Accounts)]
/// The `SetFav` struct represents the context for the `set_fav` instruction in the Solana program.
/// It defines the accounts required for the operation and their constraints.
///
/// # Fields
///
/// * `user` - A mutable signer account representing the user who is interacting with the program.
///   This account will also be used to pay for the initialization of the `fav` account if needed.
///
/// * `fav` - An account of type `Favorites` that stores the user's favorite data. 
///   This account will be initialized if it does not already exist. The account is derived using
///   a program-derived address (PDA) with the seed `[b"favorites", user.key().as_ref()]`.
///   The `space` allocated for this account includes the Anchor discriminator size and the
///   initial space required by the `Favorites` struct. A bump seed is also used for the PDA.
///
/// * `system_program` - A reference to the Solana System Program, which is required for account
///   creation and other system-level operations.
pub struct SetFav<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites",user.key().as_ref()],
        bump
    )]
    pub fav:Account<'info , Favorites>,

    pub system_program:Program<'info,System>
}
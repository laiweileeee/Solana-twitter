use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program; // for system_program::ID

declare_id!("XG2JhaxMJTgtMSDRip9cLuhZgne2i6XPxETRDSYnjXZ");

//login for instructions
#[program]
pub mod solana_twitter {
    use super::*;
    // What is ctx of an instruction?? It contains the accounts necessary for the instruction to run successfully.
    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> ProgramResult {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap(); // Clock can only be accessed if System Program is provided as an account.

        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into())
        }

        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into())
        }

        tweet.author = *author.key;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;

        Ok(())
    }

    pub fn update_tweet(ctx: Context<UpdateTweet>, topic: String, content: String) -> ProgramResult {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;

        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into())
        }

        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into())
        }

        tweet.topic = topic;
        tweet.content = content;

        Ok(())
    }

    pub fn delete_tweet(_ctx: Context<DeleteTweet>) -> ProgramResult {
        Ok(())
    }

}

#[error]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}

/* Defines the context for instructions, which is also an account */
/* 
    The line below is a derive attribute provided by Anchor that allows 
    the framework to generate a lot of code and macros for our 
    struct context. Without it, these few lines of code would be 
    a lot more complex.
*/ 
#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer = author, space = Tweet::LEN)] //This is an account constraint by anchor - Rust attribute by anchor for security and access control, and to initialise an account for us at the right size
    pub tweet: Account<'info, Tweet>, //This is a low-level Solana structure that can represent any account. When using AccountInfo, the account's data will be an unparsed array of bytes.
    #[account(mut)] // this is an account constraint by anchor - make author mutable since we need the author is paying for the rent-exempt money
    pub author: Signer<'info>, //This is the same as the AccountInfo type except we're also saying this account should sign the instruction.
    #[account(address = system_program::ID)] // system_program::ID is a constant defined in Solana's codebase. By default, it's not included in Anchor's prelude::* import so we need to add the following line afterwards — at the very top of our lib.rs file.
    pub system_program: AccountInfo<'info>, //This is an account type provided by Anchor. It wraps the AccountInfo in another struct that parses the data according to an account struct provided as a generic type. In the example above, Account<'info, Tweet> means this is an account of type Tweet and the data should be parsed accordingly.
}

#[derive(Accounts)]
pub struct UpdateTweet<'info> {
    #[account(mut, has_one = author)] //has_one - Anchor will build an appropriate macro for us that will reject the instruction if the signed author does not match the tweet’s author.
    pub tweet: Account<'info, Tweet>,
    pub author: Signer<'info>,
    /* !Note: No need system_program account here as we no need to access Clock component in the system program */
}

#[derive(Accounts)]
pub struct DeleteTweet<'info> {
    #[account(mut, has_one = author, close = author)]
    pub tweet: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

// 1. Define the structure of the Tweet account.
/*
    The line below is a custom Rust attribute provided 
    by the Anchor framework. It removes a huge amount 
    of boilerplate for us when it comes to defining accounts — 
    such as parsing the account to and from an array of bytes.
*/
#[account] 
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

// 2. Add some useful constants for sizing propeties.
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.

// 3. Add a constant on the Tweet account that provides its total size.
impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content.
}
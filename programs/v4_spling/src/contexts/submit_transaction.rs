use crate::*;
use std::fmt::Write;
use solana_program::program::invoke;

// 60 seconds = 1 minute
const TIME_UNIT_IN_SECONDS: i64 = 60;
// 60 transactions max
const TX_MAX_PER_TIME_UNIT: u16 = 60;
// amount a user gets taxed when he spams 
const TAX_AMOUNT: u64 = 10000000;
// grow the tax amount per severity level
const TAX_EXPO: u64 = 2;

#[derive(Accounts)]
pub struct SubmitTransaction<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(init_if_needed, payer = sender, 
        space = 8 + 40, // mem::size_of::<SenderLog>()
        seeds = [b"sender".as_ref(), 
        sender.key.as_ref()], 
        bump)]
    pub sender_log: Account<'info, SenderLog>,
    #[account(mut, seeds = [b"payer"], bump = payer.bump)]
    pub payer: Account<'info, Payer>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> SubmitTransaction<'_> {
    pub fn process(&mut self, operation: u8, schema: u8, storage: Pubkey, hash: Option<String>, pointer: Option<String>, unique: Option<String>) -> Result<()> {
        let Self {sender, sender_log, payer,..} = self;

        // get the current time
        let clock: Clock = Clock::get().unwrap();
        // cast as i64 which matches the TS data type in our PDA
        let time_now: i64 = clock.unix_timestamp as i64;

        // define the cost of executing this program, in lamports
        let mut action_cost: u64 = 5000;

        
        // check if PDA is active, if so we append existing values
        if sender_log.active == true {
            // check if sender has performed more than 60 transactions in the last 60 seconds
            if sender_log.counter >= TX_MAX_PER_TIME_UNIT && (time_now - sender_log.ts) <= TIME_UNIT_IN_SECONDS {
                // user is most likely spamming: log to status field and throw error
                sender_log.status += 1;

                // check if user spammed more than 10 times 
                if sender_log.status >= 11 {

                    // log the status level which is an indicator of severity of spam
                    msg!("Status elevated: {}", &sender_log.status);

                    // 10 times caught spamming, this cowboy needs to be stopped
                    return Err(ErrorCode::SlowDown.into())

                    // user has not exceeded limit (10x spamming), hit user with a spam tax
                } else {

                    // calculate the spam tax for the user (depending on status / severity of spam)
                    let tax: u64 = sender_log.status as u64 * TAX_AMOUNT * TAX_EXPO;

                    // collect tax from sender and send to the sender_log PDA
                    invoke(
                        &system_instruction::transfer(
                        &sender.to_account_info().key,
                        &sender_log.to_account_info().key, 
                        tax,             
                        ),
                        &[
                            sender.to_account_info().clone(),
                            sender_log.to_account_info().clone(),
                        ],
                    )?;

                    // log the status level which is an indicator of severity of spam
                    msg!("Status elevated: {}", &sender_log.status);
                }

            } else {
                // call is not spam / not more than 60 calls in 60 seconds

                // check if the counter is higher than 100, if so - reset to 1
                if sender_log.counter >= TX_MAX_PER_TIME_UNIT {
                    // reset to 1
                    sender_log.counter = 1;
                } else {
                    // add 1 to counter
                    sender_log.counter += 1;
                }

                // ensure status is 0 (no elevation)
                sender_log.status = 0;
                // set TS to current time
                sender_log.ts = time_now;
            }

        } else {
            // PDA does not exist, we initialize the fields
            sender_log.active = true;
            sender_log.status = 0;
            sender_log.ts = time_now;
            sender_log.counter = 1;

            // rent-exemption of sender_log PDA 
            action_cost += 1219960;
        }

        // check for optional values, if not provided treat them as JSON null's
        let hash_string = hash.unwrap_or_else(|| String::from("null"));
        let pointer_string = pointer.unwrap_or_else(|| String::from("null"));
        let unique = unique.unwrap_or_else(|| String::from("null"));

        // create new variable to hold the message we will add to the transaction log
        let mut message = String::new();

        // write variables into message 
        write!(&mut message, "Spling: {{\"operation\": {}, \"schema\": {}, \"storage\": \"{}\", \"hash\": \"{}\", \"pointer\": \"{}\", \"unique\": \"{}\"}}", operation, schema, storage, hash_string, pointer_string, unique).expect("Error writing to string");
         
        // log the message, broadcasting it for everyone to see
        msg!("{}", &message);


        // offset the cost of executing this action
        // transfer SOL tokens
        **payer.to_account_info().try_borrow_mut_lamports()? -= action_cost;
        **sender.try_borrow_mut_lamports()? += action_cost;


        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Slow down fren")]
    SlowDown,
}


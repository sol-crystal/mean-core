// Program API, (de)serializing instruction data

use std::{ mem::size_of, convert::TryInto };

use solana_program::{
    msg,
    pubkey::Pubkey,
    instruction::{ AccountMeta, Instruction }
};

use crate::{
    check_program_account,
    error::StreamError
};

pub enum StreamInstruction {

    /// Initialize a new stream contract
    ///
    /// 0. `[signer]` The treasurer account (The creator of the money stream).
    /// 1. `[writable]` The treasurer token account.
    /// 2. `[writable]` The beneficiary token account.
    /// 3. `[]` The treasury account (The stream contract treasury account).
    /// 4. `[]` The treasury token account.
    /// 5. `[writable]` The stream account (The stream contract account).
    /// 6. `[]` The associated token mint account    
    /// 7.  [] The Money Streaming Program account.
    /// 8. `[]` The SPL Token Program account.
    /// 9. `[]` System Account.
    CreateStream {
        beneficiary_address: Pubkey,
        stream_name: String,        
        funding_amount: f64, // OPTIONAL
        rate_amount: f64,
        rate_interval_in_seconds: u64,
        start_utc: u64,
        rate_cliff_in_seconds: u64,
        cliff_vest_amount: f64, // OPTIONAL
        cliff_vest_percent: f64, // OPTIONAL
        auto_pause_in_seconds: u64
    },

    /// Adds a specific amount of funds to a stream
    ///
    /// 0. `[signer]` The contributor account
    /// 1. `[writable]` The contributor token account
    /// 2. `[writable]` The contributor treasury token account (the account of the token issued by the treasury and owned by the contributor)
    /// 3. `[]` The beneficiary mint account
    /// 4. `[]` The treasury account (Money stream treasury account).
    /// 5. `[writable]` The treasury token account.    
    /// 6. `[]` The treasury mint account (the mint of the treasury pool token)
    /// 7. `[writable]` The stream account (The stream contract account).
    /// 8.  [writable] The Money Streaming Protocol account.
    /// 9.  [writable] The Money Streaming Protocol operating token account.
    /// 10.  [] The Money Streaming Program account.
    /// 11. `[]` The SPL Token Program account.
    AddFunds {
        contribution_amount: f64,
        resume: bool
    },

    /// Recovers a specific amount of funds from a previously funded stream
    ///
    /// 0. `[signer]` The contributor account
    /// 1. `[writable]` The contributor token account
    /// 2. `[writable]` The contributor treasury token account (the account of the token issued by the treasury and owned by the contributor)
    /// 3. `[]` The contributor mint account
    /// 4. `[]` The treasury account (Money streaming treasury account).
    /// 5. `[writable]` The treasury token account.    
    /// 6. `[]` The treasury mint account (the mint of the treasury pool token)
    /// 7. `[writable]` The stream account (The stream contract account).
    /// 8.  [writable] The Money Streaming Protocol account.
    /// 9.  [writable] The Money Streaming Protocol operating token account.
    /// 10.  [] The Money Streaming Program account.
    /// 11. `[]` The SPL Token Program account.
    RecoverFunds {
        recover_amount: f64
    },

    /// 0. `[signer]` The beneficiary account
    /// 1. `[writable]` The beneficiary token account (the recipient of the money)
    /// 2. `[]` The beneficiary token mint account
    /// 3. `[]` The treasury account
    /// 4. `[writable]` The treasury token account
    /// 5. `[writable]` The stream account (Money streaming state account).
    /// 6. `[writable]` The Money Streaming Protocol operating token account.
    /// 7. `[]` The Money Streaming Program account.
    /// 8. `[]` The SPL Token Program account.
    /// 9. `[]` System Program account.
    Withdraw { 
        withdrawal_amount: f64
    },

    /// 0. `[signer]` The initializer of the transaction (msp => `auto pause`, treasurer or beneficiary)
    /// 1. `[writable]` The stream account (Money stream state account).
    /// 2. `[writable]` The Money Streaming Protocol operating account.
    /// 3. `[]` System Program account.
    PauseStream,

    /// 0. `[signer]` The initializer of the transaction (msp => `auto resume`, treasurer or beneficiary)
    /// 1. `[writable]` The stream account (Money stream state account).
    /// 2. `[writable]` The Money Streaming Protocol operating token account.
    /// 3. `[]` System Program account.
    ResumeStream,

    /// 0. `[signer]` The initializer of the transaction (treasurer or beneficiary)
    /// 1. `[writable]` The stream terms account (Update proposal account).
    /// 2. `[]` The counterparty's account (if the initializer is the treasurer then it would be the beneficiary or vice versa)
    /// 3. `[writable]` The stream account
    /// 4.  [writable] The Money Streaming Protocol operating token account.
    /// 5. `[]` System Program account.
    ProposeUpdate {
        proposed_by: Pubkey,
        stream_name: String,
        treasurer_address: Pubkey,
        beneficiary_address: Pubkey,
        associated_token_address: Pubkey, // OPTIONAL
        rate_amount: f64,
        rate_interval_in_seconds: u64,
        rate_cliff_in_seconds: u64,
        cliff_vest_amount: f64, // OPTIONAL
        cliff_vest_percent: f64, // OPTIONAL
        auto_pause_in_seconds: u64
    },

    /// 0. `[signer]` The initializer of the transaction (treasurer or beneficiary)
    /// 1. `[writable]` The stream terms account (Update proposal account).
    /// 2. `[]` The counterparty's account (if the initializer is the treasurer then it would be the beneficiary or vice versa)
    /// 3. `[writable]` The stream account
    /// 4.  [writable] The Money Streaming Protocol operating token account.
    /// 5. `[]` System Program account.
    AnswerUpdate {
        approve: bool
    },

    /// 0. `[signer]` The initializer account (treasurer/beneficiary)
    /// 1. `[]` The counterparty account (treasurer/beneficiary)
    /// 2. `[writable]` The stream account (The stream contract account).
    /// 3. `[writable]` The beneficiary token account.
    /// 4. `[]` The beneficiary token mint account
    /// 5. `[writable]` The treasury token account
    /// 6. `[writable]` The treasury token owner (The Money Streaming Program)
    /// 7. `[writable]` The Money Streaming Protocol ccount.
    /// 8. `[writable]` The Money Streaming Protocol operating token account.
    /// 9. `[]` System Program account.
    /// 10. `[]` The SPL Token Program account.
    CloseStream,

    /// 0. `[signer]` The treasurer account (the creator of the treasury)
    /// 1. `[writable]` The treasury account
    /// 2. `[writable]` The treasury pool mint account
    /// 3. `[writable]` The Money Streaming Protocol ccount.
    /// 4. `[writable]` The Money Streaming Protocol operating token account.
    /// 5. `[]` The SPL Token Program account.
    /// 6. `[]` System Program account.
    /// 7. `[]` SysvarRent account.
    CreateTreasury {
        nounce: u8
    },

    /// Transfers a specific amount of tokens between 2 accounts
    ///
    /// 0. `[signer]` The source account
    /// 1. `[writable]` The source token account
    /// 2. `[writable]` The destination token account.
    /// 3. `[]` The associated token mint account
    /// 4.  [writable] The Money Streaming Protocol operating token account.
    /// 5. `[]` The SPL Token Program account.
    Transfer {
        amount: f64
    }
}

impl StreamInstruction {

    pub fn unpack(instruction_data: &[u8]) -> Result<Self, StreamError> {

        let (&tag, result) = instruction_data
            .split_first()
            .ok_or(StreamError::InvalidStreamInstruction.into())?;
                
        Ok(match tag {

            0 => Self::unpack_create_stream(result)?,
            1 => Self::unpack_add_funds(result)?,
            2 => Self::unpack_recover_funds(result)?,
            3 => Self::unpack_withdraw(result)?,
            4 => Ok(Self::PauseStream)?,
            5 => Ok(Self::ResumeStream)?,
            6 => Self::unpack_propose_update(result)?,
            7 => Self::unpack_answer_update(result)?,
            8 => Ok(Self::CloseStream)?,
            9 => Self::unpack_create_treasury(result)?,
            10 => Self::unpack_transfer(result)?,

            _ => return Err(StreamError::InvalidStreamInstruction.into()),
        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());

        match self {

            Self::CreateStream {
                beneficiary_address,
                stream_name,
                funding_amount,
                rate_amount,
                rate_interval_in_seconds,
                start_utc,
                rate_cliff_in_seconds,
                cliff_vest_amount,
                cliff_vest_percent,
                auto_pause_in_seconds

            } => {

                buf.push(0);

                buf.extend_from_slice(beneficiary_address.as_ref());
                buf.extend_from_slice(stream_name.as_ref());
                buf.extend_from_slice(&funding_amount.to_le_bytes());
                buf.extend_from_slice(&rate_amount.to_le_bytes());
                buf.extend_from_slice(&rate_interval_in_seconds.to_le_bytes());
                buf.extend_from_slice(&start_utc.to_le_bytes());
                buf.extend_from_slice(&rate_cliff_in_seconds.to_le_bytes());
                buf.extend_from_slice(&cliff_vest_amount.to_le_bytes());
                buf.extend_from_slice(&cliff_vest_percent.to_le_bytes());
                buf.extend_from_slice(&auto_pause_in_seconds.to_le_bytes());               
            },

            &Self::AddFunds { 
                contribution_amount,
                resume

            } => {
                buf.push(1);

                buf.extend_from_slice(&contribution_amount.to_le_bytes());
                
                let resume = match resume {
                    false => [0],
                    true => [1]
                };

                buf.push(resume[0] as u8);
            },

            &Self::RecoverFunds { recover_amount } => {
                buf.push(2);
                buf.extend_from_slice(&recover_amount.to_le_bytes());
            },

            &Self::Withdraw { withdrawal_amount } => {
                buf.push(3);
                buf.extend_from_slice(&withdrawal_amount.to_le_bytes());
            },

            &Self::PauseStream => buf.push(4),

            &Self::ResumeStream => buf.push(5),

            Self::ProposeUpdate {
                proposed_by,
                stream_name,
                treasurer_address,
                beneficiary_address,
                associated_token_address,
                rate_amount,
                rate_interval_in_seconds,
                rate_cliff_in_seconds,
                cliff_vest_amount,
                cliff_vest_percent,
                auto_pause_in_seconds

            } => {
                buf.push(6);

                buf.extend_from_slice(proposed_by.as_ref());
                buf.extend_from_slice(stream_name.as_ref());
                buf.extend_from_slice(treasurer_address.as_ref());
                buf.extend_from_slice(beneficiary_address.as_ref());
                buf.extend_from_slice(associated_token_address.as_ref());
                buf.extend_from_slice(&rate_amount.to_le_bytes());
                buf.extend_from_slice(&rate_interval_in_seconds.to_le_bytes());
                buf.extend_from_slice(&rate_cliff_in_seconds.to_le_bytes());
                buf.extend_from_slice(&cliff_vest_amount.to_le_bytes());
                buf.extend_from_slice(&cliff_vest_percent.to_le_bytes());
                buf.extend_from_slice(&auto_pause_in_seconds.to_le_bytes());                
            },

            &Self::AnswerUpdate { approve } => { 
                buf.push(7);

                let approve = match approve {
                    false => [0],
                    true => [1]
                };

                buf.push(approve[0] as u8);
            },

            &Self::CloseStream => buf.push(8),
            
            &Self::CreateTreasury { nounce } => {
                buf.push(9);
                buf.extend_from_slice(&nounce.to_le_bytes());
            },

            &Self::Transfer { amount } => {
                buf.push(10);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
        };

        buf
    }

    fn unpack_create_stream(input: &[u8]) -> Result<Self, StreamError> {

        let (beneficiary_address, result) = Self::unpack_pubkey(input)?;
        let (stream_name, result) = Self::unpack_string(result)?;

        let (funding_amount, result) = result.split_at(8);
        let funding_amount = Self::unpack_f64(funding_amount)?;

        let (rate_amount, result) = result.split_at(8);
        let rate_amount = Self::unpack_f64(rate_amount)?;

        let (rate_interval_in_seconds, result) = result.split_at(8);
        let rate_interval_in_seconds = Self::unpack_u64(rate_interval_in_seconds)?;

        let (start_utc, result) = result.split_at(8);
        let start_utc = Self::unpack_u64(start_utc)?;

        let (rate_cliff_in_seconds, result) = result.split_at(8);
        let rate_cliff_in_seconds = Self::unpack_u64(rate_cliff_in_seconds)?;

        let (cliff_vest_amount, result) = result.split_at(8);
        let cliff_vest_amount = Self::unpack_f64(cliff_vest_amount)?;

        let (cliff_vest_percent, result) = result.split_at(8);
        let cliff_vest_percent = Self::unpack_f64(cliff_vest_percent)?;

        let (auto_pause_in_seconds, _result) = result.split_at(8);
        let auto_pause_in_seconds = Self::unpack_u64(auto_pause_in_seconds)?;

        Ok(Self::CreateStream {
            beneficiary_address,
            stream_name,
            funding_amount,
            rate_amount,
            rate_interval_in_seconds,
            start_utc,
            rate_cliff_in_seconds,
            cliff_vest_amount,
            cliff_vest_percent,
            auto_pause_in_seconds
        })
    }

    fn unpack_add_funds(input: &[u8]) -> Result<Self, StreamError> {
        let (contribution_amount, result) = input.split_at(8);
        let contribution_amount = Self::unpack_f64(contribution_amount)?;

        let (resume, _result) = result.split_at(1);
        let resume = match resume {
            [0] => false,
            [1] => true,
            _ => false
        };

        Ok(Self::AddFunds { 
            contribution_amount,
            resume
        })
    }

    fn unpack_recover_funds(input: &[u8]) -> Result<Self, StreamError> {
        let (recover_amount, result) = input.split_at(8);
        let recover_amount = Self::unpack_f64(recover_amount)?;

        Ok(Self::RecoverFunds { recover_amount })
    }

    fn unpack_withdraw(input: &[u8]) -> Result<Self, StreamError> {
        let (withdrawal_amount, _result) = input.split_at(8);
        let withdrawal_amount = Self::unpack_f64(withdrawal_amount)?;

        Ok(Self::Withdraw { withdrawal_amount })
    }

    fn unpack_propose_update(input: &[u8]) -> Result<Self, StreamError> {
        let (proposed_by, result) = Self::unpack_pubkey(input)?;
        let (stream_name, result) = Self::unpack_string(result)?;
        let (treasurer_address, result) = Self::unpack_pubkey(result)?;
        let (beneficiary_address, result) = Self::unpack_pubkey(result)?;
        let (associated_token_address, result) = Self::unpack_pubkey(result)?;

        let (rate_amount, result) = result.split_at(8);
        let rate_amount = Self::unpack_f64(rate_amount)?;

        let (rate_interval_in_seconds, result) = result.split_at(8);
        let rate_interval_in_seconds = Self::unpack_u64(rate_interval_in_seconds)?;

        let (rate_cliff_in_seconds, result) = result.split_at(8);
        let rate_cliff_in_seconds = Self::unpack_u64(rate_cliff_in_seconds)?;

        let (cliff_vest_amount, result) = result.split_at(8);
        let cliff_vest_amount = Self::unpack_f64(cliff_vest_amount)?;

        let (cliff_vest_percent, result) = result.split_at(8);
        let cliff_vest_percent = Self::unpack_f64(cliff_vest_percent)?;

        let (auto_pause_in_seconds, _result) = result.split_at(8);
        let auto_pause_in_seconds = Self::unpack_u64(auto_pause_in_seconds)?;        

        Ok(Self::ProposeUpdate {
            proposed_by,
            stream_name,
            treasurer_address,
            beneficiary_address,
            associated_token_address,
            rate_amount,
            rate_interval_in_seconds,
            rate_cliff_in_seconds,
            cliff_vest_amount,
            cliff_vest_percent,
            auto_pause_in_seconds
        })
    }

    fn unpack_answer_update(input: &[u8]) -> Result<Self, StreamError> {
        let (approve, _result) = input.split_at(1);
        let approve = match approve {
            [0] => false,
            [1] => true,
            _ => false
        };

        Ok(Self::AnswerUpdate { approve })
    }

    fn unpack_create_treasury(input: &[u8]) -> Result<Self, StreamError> {

        let (&nounce, _result) = input
            .split_first()
            .ok_or(StreamError::InvalidStreamInstruction.into())?;

        Ok(Self::CreateTreasury { nounce })
    }

    fn unpack_transfer(input: &[u8]) -> Result<Self, StreamError> {

        let (amount, result) = input.split_at(8);
        let amount = Self::unpack_f64(amount)?;

        Ok(Self::Transfer { amount })
    }

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), StreamError> {
        if input.len() >= 32 {
            let (key, rest) = input.split_at(32);
            let pk = Pubkey::new(key);

            Ok((pk, rest))
        } else {
            Err(StreamError::InvalidArgument.into())
        }
    }

    fn unpack_string(input: &[u8]) -> Result<(String, &[u8]), StreamError> {
        if input.len() >= 32 {
            let (bytes, rest) = input.split_at(32);
            Ok((String::from_utf8_lossy(bytes).to_string(), rest))
        } else {
            Err(StreamError::InvalidArgument.into())
        }
    }

    fn unpack_u64(input: &[u8]) -> Result<u64, StreamError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(StreamError::InvalidStreamInstruction)?;

        Ok(amount)
    }

    fn unpack_f64(input: &[u8]) -> Result<f64, StreamError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(f64::from_le_bytes)
            .ok_or(StreamError::InvalidStreamInstruction)?;

        Ok(amount)
    }
 }

 pub fn create_stream(
    program_id: &Pubkey,
    treasurer_address: Pubkey,
    treasurer_token_address: Pubkey,
    beneficiary_token_address: Pubkey,
    treasury_address: Pubkey,
    treasury_token_address: Pubkey,
    stream_address: Pubkey,
    mint_address: Pubkey,
    msp_ops_address: Pubkey,
    beneficiary_address: Pubkey,
    stream_name: String,
    funding_amount: f64,
    rate_amount: f64,
    rate_interval_in_seconds: u64,
    start_utc: u64,
    rate_cliff_in_seconds: u64,
    cliff_vest_amount: f64,
    cliff_vest_percent: f64,
    auto_pause_in_seconds: u64

 ) -> Result<Instruction, StreamError> {

    check_program_account(program_id);

    let data = StreamInstruction::CreateStream {
        beneficiary_address,
        stream_name,
        funding_amount,
        rate_amount,
        rate_interval_in_seconds,
        start_utc,
        rate_cliff_in_seconds,
        cliff_vest_amount,
        cliff_vest_percent,
        auto_pause_in_seconds

    }.pack();

    let accounts = vec![
        AccountMeta::new_readonly(treasurer_address, true),
        AccountMeta::new(treasurer_token_address, false),
        AccountMeta::new(beneficiary_token_address, false),
        AccountMeta::new_readonly(treasury_address, false),
        AccountMeta::new(treasury_token_address, false),
        AccountMeta::new(stream_address, false),
        AccountMeta::new(mint_address, false),
        AccountMeta::new(msp_ops_address, false),
        AccountMeta::new_readonly(*program_id, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false)
    ];

    Ok(Instruction { 
        program_id: *program_id, 
        accounts, 
        data 
    })
 }

 pub fn add_funds(
    program_id: &Pubkey,
    stream_address: &Pubkey,
    treasury_address: &Pubkey,
    contribution_token_address: Pubkey,
    contribution_amount: f64,
    resume: bool

 ) -> Result<Instruction, StreamError> {

    check_program_account(program_id);

    let data = StreamInstruction::AddFunds { 
        contribution_amount,
        resume

    }.pack();

    let accounts = vec![
        AccountMeta::new(contribution_token_address, true),
        AccountMeta::new(*stream_address, false),
        AccountMeta::new_readonly(*treasury_address, false)
    ];

    Ok(Instruction { 
        program_id: *program_id, 
        accounts, 
        data 
    })
 }

 pub fn withdraw(
    program_id: &Pubkey,
    beneficiary_account_address: Pubkey,
    stream_account_address: Pubkey,
    treasury_account_address: Pubkey,
    withdrawal_amount: f64,

 ) -> Result<Instruction, StreamError> {

    check_program_account(program_id);

    let data = StreamInstruction::Withdraw { withdrawal_amount }.pack();
    let accounts = vec![
        AccountMeta::new_readonly(beneficiary_account_address, false),
        AccountMeta::new(stream_account_address, false),
        AccountMeta::new_readonly(treasury_account_address, false)
    ];

    Ok(Instruction { 
        program_id: *program_id, 
        accounts, 
        data 
    })
 }

 pub fn close_stream(
    initializer_account_key: &Pubkey,
    stream_account_key: &Pubkey,
    counterparty_account_key: &Pubkey,
    treasury_account_key: &Pubkey,
    program_id: &Pubkey,

 ) -> Result<Instruction, StreamError> {

    check_program_account(program_id);

    let data = StreamInstruction::CloseStream.pack();
    let accounts = vec![
        AccountMeta::new(*initializer_account_key, true),
        AccountMeta::new(*stream_account_key, false),
        AccountMeta::new_readonly(*counterparty_account_key, false),
        AccountMeta::new_readonly(*treasury_account_key, false)
    ];

    Ok(Instruction { program_id: *program_id, accounts, data })
 }

 pub fn transfer(
     source_address: Pubkey,
     source_token_address: Pubkey,
     destination_token_address: Pubkey,
     mint_address: Pubkey,
    //  msp_ops_address: Pubkey,
     program_id: &Pubkey,
     amount: f64

 ) -> Result<Instruction, StreamError> {

    check_program_account(program_id);

    let data = StreamInstruction::Transfer { amount }.pack();
    let accounts = vec![
        AccountMeta::new_readonly(source_address, true),
        AccountMeta::new(source_token_address, false),
        AccountMeta::new(destination_token_address, false),
        AccountMeta::new(mint_address, false),
        // AccountMeta::new(msp_ops_address, false),
        // AccountMeta::new_readonly(*program_id, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        // AccountMeta::new_readonly(solana_program::system_program::id(), false)
    ];

    Ok(Instruction { 
        program_id: *program_id, 
        accounts, 
        data 
    })
 }
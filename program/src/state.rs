// Program objects, (de)serializing state

use std::string::String;

use solana_program::{
    pubkey::Pubkey,
    program_error::ProgramError,    
    program_pack::{ IsInitialized, Pack, Sealed }
};

use arrayref::{
    array_mut_ref,
    mut_array_refs,
    array_ref, 
    array_refs, 
};

use crate::error::StreamError;

pub const LAMPORTS_PER_SOL: u64 = 1000000000;

#[derive(Clone, Debug)]
pub struct StreamTerms {
    pub initialized: bool,
    pub proposed_by: Pubkey,
    pub stream_name: String,
    pub treasurer_address: Pubkey,
    pub beneficiary_address: Pubkey,
    pub stream_associated_token: Pubkey,
    pub treasury_address: Pubkey,
    pub rate_amount: f64,
    pub rate_interval_in_seconds: u64,
    pub start_utc: u64,
    pub rate_cliff_in_seconds: u64
}

impl Sealed for StreamTerms {}

impl IsInitialized for StreamTerms {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for StreamTerms {
    fn default() -> Self {
        Self {
            initialized: false,
            proposed_by: Pubkey::default(),
            stream_name: String::default(),
            treasurer_address: Pubkey::default(),
            beneficiary_address: Pubkey::default(),
            stream_associated_token: Pubkey::default(),
            treasury_address: Pubkey::default(),                 
            rate_amount: 0.0,
            rate_interval_in_seconds: 0,
            start_utc: 0,
            rate_cliff_in_seconds: 0    
        }
    }
}

impl Pack for StreamTerms {
    const LEN: usize = 225;

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, StreamTerms::LEN];
        let (
            initialized_output,
            proposed_by_output,
            stream_name_output,
            treasurer_address_output,
            beneficiary_address_output,
            stream_associated_token_output,
            treasury_address_output,
            rate_amount_output,
            rate_interval_in_seconds_output,
            start_utc_output,
            rate_cliff_in_seconds_output    
            
        ) = mut_array_refs![output, 1, 32, 32, 32, 32, 32, 32, 8, 8, 8, 8];

        let StreamTerms {
            initialized,
            proposed_by,
            stream_name,
            treasurer_address,
            beneficiary_address,
            stream_associated_token,
            treasury_address,
            rate_amount,
            rate_interval_in_seconds,
            start_utc,
            rate_cliff_in_seconds

        } = self;

        initialized_output[0] = *initialized as u8;
        proposed_by_output.copy_from_slice(proposed_by.as_ref());
        stream_name_output.copy_from_slice(stream_name.as_ref());
        treasurer_address_output.copy_from_slice(treasurer_address.as_ref());
        beneficiary_address_output.copy_from_slice(beneficiary_address.as_ref());
        stream_associated_token_output.copy_from_slice(stream_associated_token.as_ref());
        treasury_address_output.copy_from_slice(treasury_address.as_ref());
        *rate_amount_output = rate_amount.to_le_bytes();
        *rate_interval_in_seconds_output = rate_interval_in_seconds.to_le_bytes();
        *start_utc_output = start_utc.to_le_bytes();
        *rate_cliff_in_seconds_output = rate_cliff_in_seconds.to_le_bytes();
    }
    
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, StreamTerms::LEN];
        let (
            initialized,
            proposed_by,
            stream_name,
            treasurer_address,
            beneficiary_address,
            stream_associated_token,
            treasury_address,
            rate_amount,
            rate_interval_in_seconds,
            start_utc,
            rate_cliff_in_seconds,
            
        ) = array_refs![input, 1, 32, 32, 32, 32, 32, 32, 8, 8, 8, 8];

        let initialized = match initialized {
            [0] => false,
            [1] => true,
            _ => return Err(StreamError::InvalidStreamData.into()),
        };

        Ok(StreamTerms {
            initialized, 
            proposed_by: Pubkey::new_from_array(*proposed_by),
            stream_name: String::from_utf8_lossy(stream_name).to_string(),
            treasurer_address: Pubkey::new_from_array(*treasurer_address),
            beneficiary_address: Pubkey::new_from_array(*beneficiary_address),
            stream_associated_token: Pubkey::new_from_array(*stream_associated_token),
            treasury_address: Pubkey::new_from_array(*treasury_address),          
            rate_amount: f64::from_le_bytes(*rate_amount),
            rate_interval_in_seconds: u64::from_le_bytes(*rate_interval_in_seconds),
            start_utc: u64::from_le_bytes(*start_utc),
            rate_cliff_in_seconds: u64::from_le_bytes(*rate_cliff_in_seconds)
        })
    }
}

#[derive(Clone, Debug)]
pub struct Stream {
    pub initialized: bool,
    pub stream_name: String,
    pub treasurer_address: Pubkey,
    pub rate_amount: f64,
    pub rate_interval_in_seconds: u64,
    pub start_utc: u64,
    pub rate_cliff_in_seconds: u64,
    pub cliff_vest_amount: f64,
    pub cliff_vest_percent: f64,
    pub beneficiary_address: Pubkey,
    pub stream_associated_token: Pubkey,
    pub treasury_address: Pubkey,
    pub treasury_estimated_depletion_utc: u64,
    pub total_deposits: f64,
    pub total_withdrawals: f64,
    pub escrow_vested_amount_snap: f64,
    pub escrow_vested_amount_snap_block_height: u64,
    pub auto_pause_in_seconds: u64,
    pub is_streaming: bool
}

impl Sealed for Stream {}

impl IsInitialized for Stream {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for Stream {
    fn default() -> Self {
        Self {
            initialized: false,
            stream_name: String::default(),
            treasurer_address: Pubkey::default(),                   
            rate_amount: 0.0,
            rate_interval_in_seconds: 0,
            start_utc: 0,
            rate_cliff_in_seconds: 0,
            cliff_vest_amount: 0.0,
            cliff_vest_percent: 0.0,
            beneficiary_address: Pubkey::default(),
            stream_associated_token: Pubkey::default(),
            treasury_address: Pubkey::default(), 
            treasury_estimated_depletion_utc: 0,
            total_deposits: 0.0,
            total_withdrawals: 0.0,
            escrow_vested_amount_snap: 0.0,
            escrow_vested_amount_snap_block_height: 0,
            auto_pause_in_seconds: 0,
            is_streaming: true
        }
    }
}

impl Pack for Stream {
    const LEN: usize = 258;

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, Stream::LEN];
        let (
            initialized_output,
            stream_name_output,
            treasurer_address_output,
            rate_amount_output,
            rate_interval_in_seconds_output,
            start_utc_output,
            rate_cliff_in_seconds_output,
            cliff_vest_amount_output,
            cliff_vest_percent_output,
            beneficiary_address_output,
            stream_associated_token_output,
            treasury_address_output,
            treasury_estimated_depletion_utc_output,
            total_deposits_output,
            total_withdrawals_output,
            escrow_vested_amount_snap_output,
            escrow_vested_amount_snap_block_height_output,
            auto_pause_in_seconds_output,
            is_streaming_output,
            
        ) = mut_array_refs![output, 1, 32, 32, 8, 8, 8, 8, 8, 8, 32, 32, 32, 8, 8, 8, 8, 8, 8, 1];

        let Stream {
            initialized,
            stream_name,
            treasurer_address,
            rate_amount,
            rate_interval_in_seconds,
            start_utc,
            rate_cliff_in_seconds,
            cliff_vest_amount,
            cliff_vest_percent,
            beneficiary_address,
            stream_associated_token,
            treasury_address,
            treasury_estimated_depletion_utc,
            total_deposits,
            total_withdrawals,
            escrow_vested_amount_snap,
            escrow_vested_amount_snap_block_height,
            auto_pause_in_seconds,
            is_streaming

        } = self;

        initialized_output[0] = *initialized as u8;
        stream_name_output.copy_from_slice(stream_name.as_ref());
        treasurer_address_output.copy_from_slice(treasurer_address.as_ref());
        *rate_amount_output = rate_amount.to_le_bytes();
        *rate_interval_in_seconds_output = rate_interval_in_seconds.to_le_bytes();
        *start_utc_output = start_utc.to_le_bytes();
        *rate_cliff_in_seconds_output = rate_cliff_in_seconds.to_le_bytes();
        *cliff_vest_amount_output = cliff_vest_amount.to_le_bytes();
        *cliff_vest_percent_output = cliff_vest_percent.to_le_bytes();
        beneficiary_address_output.copy_from_slice(beneficiary_address.as_ref());
        stream_associated_token_output.copy_from_slice(stream_associated_token.as_ref());
        treasury_address_output.copy_from_slice(treasury_address.as_ref());
        *treasury_estimated_depletion_utc_output = treasury_estimated_depletion_utc.to_le_bytes();
        *total_deposits_output = total_deposits.to_le_bytes();
        *total_withdrawals_output = total_withdrawals.to_le_bytes();
        *escrow_vested_amount_snap_output = escrow_vested_amount_snap.to_le_bytes();
        *escrow_vested_amount_snap_block_height_output = escrow_vested_amount_snap_block_height.to_le_bytes();
        *auto_pause_in_seconds_output = auto_pause_in_seconds.to_le_bytes();
        is_streaming_output[0] = *is_streaming as u8;
    }
    
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, Stream::LEN];
        let (
            initialized,
            stream_name,
            treasurer_address,
            rate_amount,
            rate_interval_in_seconds,
            start_utc,
            rate_cliff_in_seconds,
            cliff_vest_amount,
            cliff_vest_percent,
            beneficiary_address,
            stream_associated_token,
            treasury_address,
            treasury_estimated_depletion_utc,
            total_deposits,
            total_withdrawals,
            escrow_vested_amount_snap,
            escrow_vested_amount_snap_block_height,
            auto_pause_in_seconds,
            is_streaming
            
        ) = array_refs![input, 1, 32, 32, 8, 8, 8, 8, 8, 8, 32, 32, 32, 8, 8, 8, 8, 8, 8, 1];

        let initialized = match initialized {
            [0] => false,
            [1] => true,
            _ => return Err(StreamError::InvalidStreamData.into()),
        };

        let is_streaming = match is_streaming {
            [0] => false,
            [1] => true,
            _ => return Err(StreamError::InvalidStreamData.into()),
        };

        Ok(Stream {
            initialized, 
            stream_name: String::from_utf8_lossy(stream_name).to_string(),
            treasurer_address: Pubkey::new_from_array(*treasurer_address),                   
            rate_amount: f64::from_le_bytes(*rate_amount),
            rate_interval_in_seconds: u64::from_le_bytes(*rate_interval_in_seconds),
            start_utc: u64::from_le_bytes(*start_utc),
            rate_cliff_in_seconds: u64::from_le_bytes(*rate_cliff_in_seconds),
            cliff_vest_amount: f64::from_le_bytes(*cliff_vest_amount),
            cliff_vest_percent: f64::from_le_bytes(*cliff_vest_percent),
            beneficiary_address: Pubkey::new_from_array(*beneficiary_address),
            stream_associated_token: Pubkey::new_from_array(*stream_associated_token),
            treasury_address: Pubkey::new_from_array(*treasury_address), 
            treasury_estimated_depletion_utc: u64::from_le_bytes(*treasury_estimated_depletion_utc),
            total_deposits: f64::from_le_bytes(*total_deposits),
            total_withdrawals: f64::from_le_bytes(*total_withdrawals),
            escrow_vested_amount_snap: f64::from_le_bytes(*escrow_vested_amount_snap),
            escrow_vested_amount_snap_block_height: u64::from_le_bytes(*escrow_vested_amount_snap_block_height),
            auto_pause_in_seconds: u64::from_le_bytes(*auto_pause_in_seconds),
            is_streaming
        })
    }
}
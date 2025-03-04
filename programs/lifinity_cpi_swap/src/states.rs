use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct LifinityPool {
	pub initializer_key: Pubkey,                        // 32 bytes
	pub initializer_deposit_token_account: Pubkey,      // 32 bytes
	pub initializer_receive_token_account: Pubkey,      // 32 bytes
	pub initializer_amount: u64,                        // 8 bytes
	pub taker_amount: u64,                              // 8 bytes
	pub is_initialized: bool,                           // 1 byte
	pub bump_seed: u8,                                  // 1 byte
	pub freeze_trade: u8,                               // 1 byte
	pub freeze_deposit: u8,                             // 1 byte
	pub freeze_withdraw: u8,                            // 1 byte
	pub base_decimals: u8,                              // 1 byte
	pub token_program_id: Pubkey,                       // 32 bytes
	pub token_a_account: Pubkey,                        // 32 bytes
	pub token_b_account: Pubkey,                        // 32 bytes
	pub pool_mint: Pubkey,                              // 32 bytes
	pub token_a_mint: Pubkey,                           // 32 bytes
	pub token_b_mint: Pubkey,                           // 32 bytes
	pub fee_account: Pubkey,                            // 32 bytes
	pub oracle_main_account: Pubkey,                    // 32 bytes
	pub oracle_sub_account: Pubkey,                     // 32 bytes
	pub oracle_pc_account: Pubkey,                      // 32 bytes
	pub fees: AmmFees,                                  // 64 bytes
	pub curve: AmmCurve,                                // 9 bytes
	pub config: AmmConfig,                              // 232 bytes
	pub amm_p_temp1: Pubkey,                            // 32 bytes
	pub amm_p_temp2: Pubkey,                            // 32 bytes
	pub amm_p_temp3: Pubkey,                            // 32 bytes
	pub amm_p_temp4: Pubkey,                            // 32 bytes
	pub amm_p_temp5: Pubkey,                            // 32 bytes
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Default)]
pub struct AmmFees {
	pub trade_fee_numerator: u64,
	pub trade_fee_denominator: u64,
	pub owner_trade_fee_numerator: u64,
	pub owner_trade_fee_denominator: u64,
	pub owner_withdraw_fee_numerator: u64,
	pub owner_withdraw_fee_denominator: u64,
	pub host_fee_numerator: u64,
	pub host_fee_denominator: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Default)]
pub struct AmmCurve {
	pub curve_type: u8,
	pub curve_parameters: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Default)]
pub struct AmmConfig {
	pub last_price: u64,
	pub last_balanced_price: u64,
	pub config_denominator: u64,
	pub volume_x: u64,
	pub volume_y: u64,
	pub volume_x_in_y: u64,
	pub deposit_cap: u64,
	pub regression_target: u64,
	pub oracle_type: u64,
	pub oracle_status: u64,
	pub oracle_main_slot_limit: u64,
	pub oracle_sub_confidence_limit: u64,
	pub oracle_sub_slot_limit: u64,
	pub oracle_pc_confidence_limit: u64,
	pub oracle_pc_slot_limit: u64,
	pub std_spread: u64,
	pub std_spread_buffer: u64,
	pub spread_coefficient: u64,
	pub price_buffer_coin: i64,
	pub price_buffer_pc: i64,
	pub rebalance_ratio: u64,
	pub fee_trade: u64,
	pub fee_platform: u64,
	pub oracle_main_slot_buffer: u64,
	pub config_temp4: u64,
	pub config_temp5: u64,
	pub config_temp6: u64,
	pub config_temp7: u64,
	pub config_temp8: u64,
}
use anchor_lang::Accounts;

#[derive(Accounts)]
pub struct Swap {
	pub amount_in: u64,
	pub min_amount_out: u64
}
use crate::constant::{AI_ALIENS_PDA_SEED, NFT_MINTED_PDA_SEED};
use crate::helper::{get_creator_pubkey, get_metadata_program_id, get_token_metadata_init_space};
use crate::state::{AiAliensPda, NftMintedPda};

use anchor_lang::{prelude::*, solana_program::rent::Rent};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};
use holder_metadata::state::AnchorField;

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(mut, address = get_creator_pubkey()?)]
    pub creator: Signer<'info>,
    #[account(
        init_if_needed,
        space = AiAliensPda::LEN,
        payer = creator,
        seeds = [AI_ALIENS_PDA_SEED.as_bytes()],
        bump)
    ]
    pub ai_aliens_pda: Account<'info, AiAliensPda>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Account checked in CPI
    #[account(
        init,
        signer,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = 0,
        mint::authority = ai_aliens_pda,
        mint::freeze_authority = ai_aliens_pda,
        // extensions::metadata_pointer::authority = ai_aliens_pda,
        // extensions::metadata_pointer::metadata_address = metadata,
        // extensions::group_member_pointer::authority = ai_aliens_pda,
        // extensions::transfer_hook::authority = ai_aliens_pda,
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    /// CHECK: Account checked in CPI
    #[account(
        init,
        payer = payer,
        space = get_token_metadata_init_space(1)?,
        owner = metadata_program.key(),
    )]
    pub metadata: UncheckedAccount<'info>,
    #[account(mut, seeds = [AI_ALIENS_PDA_SEED.as_bytes()], bump)]
    pub ai_aliens_pda: Account<'info, AiAliensPda>,
    #[account(
        init,
        payer = payer,
        space = NftMintedPda::LEN,
        seeds = [NFT_MINTED_PDA_SEED.as_bytes(), &index.to_le_bytes()],
        bump,
    )]
    pub nft_minted_pda: Account<'info, NftMintedPda>,
    /// CHECK: Account checked in CPI
    #[account(mut)]
    pub field_pda: UncheckedAccount<'info>,
    /// CHECK: Account checked in constraints
    #[account(executable, address = get_metadata_program_id()?)]
    pub metadata_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: We're just giving them tokens
    pub dest: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = dest,
    )]
    pub dest_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(seeds = [AI_ALIENS_PDA_SEED.as_bytes()], bump)]
    pub ai_aliens_pda: Account<'info, AiAliensPda>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(field: AnchorField, val: String)]
pub struct UpdateField<'info> {
    #[account(mut, address = get_creator_pubkey()?)]
    pub creator: Signer<'info>,
    /// CHECK: Account checked in CPI
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    #[account(seeds = [AI_ALIENS_PDA_SEED.as_bytes()], bump)]
    pub ai_aliens_pda: Account<'info, AiAliensPda>,
    /// CHECK: Account checked in constraints
    #[account(executable, address = get_metadata_program_id()?)]
    pub metadata_program: UncheckedAccount<'info>,
}

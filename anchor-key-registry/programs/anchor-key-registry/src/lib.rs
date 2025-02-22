use anchor_lang::prelude::*;


declare_id!("5nJoiCvyCArgjGf8BQvGS3U5XriYU1S7zyN4fYxyUieN");

#[program]
pub mod key_registry {
    use super::*;

    /// Initializes the registry account.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.keys = Vec::new();
        registry.authority = *ctx.accounts.user.key;
        Ok(())
    }

    /// Registers a new key with an owner.
    pub fn register_key(ctx: Context<RegisterKey>, key: String, owner: Pubkey) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        // Check if the key already exists
        if registry.keys.iter().any(|entry| entry.key == key) {
            return Err(ErrorCode::KeyAlreadyExists.into());
        }
        let entry = KeyEntry { key, owner };
        registry.keys.push(entry);
        Ok(())
    }

    /// Updates the owner of an existing key.
    pub fn update_key_owner(
        ctx: Context<UpdateKeyOwner>,
        key: String,
        new_owner: Pubkey,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        // Ensure the user is the authority
        require!(
            registry.authority == *ctx.accounts.user.key,
            ErrorCode::Unauthorized
        );
        if let Some(entry) = registry.keys.iter_mut().find(|e| e.key == key) {
            entry.owner = new_owner;
            Ok(())
        } else {
            Err(ErrorCode::KeyNotFound.into())
        }
    }

    /// Retrieves the owner of a specified key.
    pub fn get_key_owner(ctx: Context<GetKeyOwner>, key: String) -> Result<()> {
        let registry = &ctx.accounts.registry;
        if let Some(entry) = registry.keys.iter().find(|e| e.key == key) {
            msg!("Owner of key {}: {}", key, entry.owner);
            Ok(())
        } else {
            Err(ErrorCode::KeyNotFound.into())
        }
    }
}

/// Account structure for initialization.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64 + 1024)]
    // Space for discriminator, authority, and keys
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Account structure for registering a key.
#[derive(Accounts)]
pub struct RegisterKey<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    pub user: Signer<'info>,
}

/// Account structure for updating a key's owner.
#[derive(Accounts)]
pub struct UpdateKeyOwner<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
}

/// Account structure for querying a key's owner.
#[derive(Accounts)]
pub struct GetKeyOwner<'info> {
    pub registry: Account<'info, Registry>,
}

/// The registry account storing keys and owners.
#[account]
pub struct Registry {
    pub authority: Pubkey,   // The authority who initialized the registry
    pub keys: Vec<KeyEntry>, // List of key-owner pairs
}

/// A single key-owner pair.
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct KeyEntry {
    pub key: String,   // The key (e.g., a unique identifier)
    pub owner: Pubkey, // The public key of the owner
}

/// Custom error codes.
#[error_code]
pub enum ErrorCode {
    #[msg("Key already exists in the registry.")]
    KeyAlreadyExists,
    #[msg("Key not found in the registry.")]
    KeyNotFound,
    #[msg("Unauthorized action.")]
    Unauthorized,
}

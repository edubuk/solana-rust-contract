use anchor_lang::prelude::*;
use solana_program::clock::UnixTimestamp;

declare_id!("Ez2LGsCeMXJFvvA6oaLqGDoGBHdSy2rFrdyQhD4jxJqK");

#[program]
pub mod edubuk_eseal_solana {
    use super::*;

    pub fn add_certificate_record(
        ctx: Context<AddCertificateRecord>,
        certificate_issued_to: String,
        certificate_issued_by: String,
        certificate_type: String,
        certificate_filehash: String,
        witness: Pubkey,
        timestamp: UnixTimestamp,
    ) -> Result<()> {
        msg!("Movie review account created");
        msg!("Certificate issued to: {}", certificate_issued_to);
        msg!("Certificate issued by: {}", certificate_issued_by);
        msg!("Certificate type: {}", certificate_type);
        msg!("Certificate filehash: {}", certificate_filehash);
        msg!("Witness: {}", witness);
        msg!("Timestamp: {}", timestamp);

        let certificate_record = &mut ctx.accounts.certificate_record;
        certificate_record.witness = ctx.accounts.initializer.key();
        certificate_record.certificate_issued_to = certificate_issued_to;
        certificate_record.certificate_issued_by = certificate_issued_by;
        certificate_record.certificate_type = certificate_type;
        certificate_record.certificate_filehash = certificate_filehash;
        certificate_record.timestamp = timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(certificate_issued_to: String, certificate_issued_by: String, certificate_type: String, certificate_filehash: String, timestamp: UnixTimestamp)]
pub struct AddCertificateRecord<'info> {
    #[account(
        init,
        seeds=[certificate_issued_to.as_bytes(), certificate_issued_by.as_bytes(), certificate_type.as_bytes(), certificate_filehash.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 1 + 4 + certificate_issued_to.len() + 4 + certificate_issued_by.len() + 4 + certificate_type.len() + 4 + certificate_filehash.len() + 4 + 64
    )]
    pub certificate_record: Account<'info, CertificateAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CertificateAccountState {
    pub certificate_issued_to: String,
    pub certificate_issued_by: String,
    pub certificate_type: String,
    pub certificate_filehash: String,
    pub witness: Pubkey,
    pub timestamp: UnixTimestamp,
}

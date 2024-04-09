use anchor_lang::prelude::*;
use solana_program::clock::UnixTimestamp;
declare_id!("Ez2LGsCeMXJFvvA6oaLqGDoGBHdSy2rFrdyQhD4jxJqK");
/// A program for managing certificate records on the Solana blockchain.
#[program]
pub mod edubuk_eseal_solana {
use super::*;
/// Adds a certificate record to the blockchain.
/// This operation is restricted to authorized users, verified by
signature.
pub fn add_certificate_record(
ctx: Context<AddCertificateRecord>,
certificate_issued_to: String,
certificate_issued_by: String,
certificate_type: String,
certificate_filehash: String,
timestamp: UnixTimestamp,
) -> Result<()> {
let certificate_record = &mut ctx.accounts.certificate_record;
// Ensuring data consistency and avoiding overwriting existing

records.
if certificate_record.initialized {
return Err(error!(ErrorCode::RecordAlreadyInitialized));
}
certificate_record.certificate_issued_to = certificate_issued_to;
certificate_record.certificate_issued_by = certificate_issued_by;
certificate_record.certificate_type = certificate_type;
certificate_record.certificate_filehash = certificate_filehash;
certificate_record.witness = ctx.accounts.initializer.key();
certificate_record.timestamp = timestamp;
certificate_record.initialized = true;
Ok(())
}
}
/// Accounts required for adding a certificate record.
#[derive(Accounts)]
pub struct AddCertificateRecord<'info> {
#[account(
init,
payer = initializer,
space = CertificateAccountState::calculate_space(

&certificate_issued_to, &certificate_issued_by,

&certificate_type, &certificate_filehash
),
seeds = [
b"certificate",
initializer.key().as_ref(),
certificate_filehash.as_bytes()
],
bump
)]
/// The certificate record account to be created or updated.
pub certificate_record: Account<'info, CertificateAccountState>,
/// The account initiating the transaction; must be a signer.
#[account(mut)]
pub initializer: Signer<'info>,
/// The system program account; used for creating the certificate
record account.
pub system_program: Program<'info, System>,
}
/// Represents the state of a certificate record on the blockchain.
#[account]
pub struct CertificateAccountState {
pub certificate_issued_to: String,
pub certificate_issued_by: String,
pub certificate_type: String,
pub certificate_filehash: String,
pub witness: Pubkey,
pub timestamp: UnixTimestamp,
/// Indicates if the record has been initialized to prevent overwriting.
pub initialized: bool,
}
impl CertificateAccountState {
/// Calculates the required space for the certificate account
dynamically based on the input.

fn calculate_space(certificate_issued_to: &String,
certificate_issued_by: &String, certificate_type: &String,
certificate_filehash: &String) -> usize {
8 + // Discriminator
4 + certificate_issued_to.len() + // certificate_issued_to
4 + certificate_issued_by.len() + // certificate_issued_by
4 + certificate_type.len() + // certificate_type
4 + certificate_filehash.len() + // certificate_filehash
32 + // witness Pubkey
8 + // timestamp UnixTimestamp
1 // initialized flag
}
}
/// Custom error codes for the program.
#[error_code]
pub enum ErrorCode {
/// The certificate record is already initialized, preventing
overwrites.
RecordAlreadyInitialized,
}

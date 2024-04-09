**Edubuk's Rust Smart Contract on Solana Blockchain: Detail**
The scope of this Solana smart contract, developed using the Anchor framework, is to provide a secure and efficient
means for managing digital certificate records on the blockchain. It is designed to store critical information about
certificates, including details about the issuer, recipient, certificate type, and a unique file hash, along
with a timestamp and the transaction initiator's public key. The contract ensures that only authorized
parties can add new records, employing a combination of signature verification and account initialization
checks to prevent unauthorized access and data overwrites. This contract caters to applications requiring
immutable, verifiable records of certification, such as educational credentials, professional licenses, or authenticity
certificates for digital or physical assets, thereby facilitating trust and transparency in digital transactions and
certificate management on the Solana blockchain.
Let's break down its key components and functionality:
Program Declaration

**● declare_id!("Ez2LGsCeMXJFvvA6oaLqGDoGBHdSy2rFrdyQhD4jxJqK");:** This line sets the program
ID for the smart contract. The ID is a unique identifier for the contract on the Solana network.
Main Program Module

**● #[program] pub mod edubuk_eseal_solana:** This attribute and declaration define the main module of
the smart contract. The module name, edubuk_eseal_solana, suggests that the contract is for managing
electronic or digital seals for an entity named "Edubuk."
Functionality

**● add_certificate_record**: This function adds a certificate record to the blockchain. It's designed to be
called by authorized users, as verified by their signatures. The function takes several parameters, including the
issuer, recipient, certificate type, and file hash, along with a timestamp. The function
checks if the record is already initialized to prevent overwriting existing records. If not, it initializes the
record with the provided data.
Accounts Structure

● **#[derive(Accounts)] pub struct AddCertificateRecord**: This structure defines the accounts
required for the add_certificate_record function. It uses the #[account] attribute to specify the
requirements for each involved account.

● **The certificate_record account is marked to be initialized within this function call**. It uses a combination
of seeds, including a static string "certificate", the initializer's public key, and the certificate file hash, to
generate a unique address for this account.

● The initializer account, which pays for the transaction and must sign it, indicating authorization.

● The system_program is a reference to the Solana System Program, used for creating and managing
accounts on the blockchain.

**Certificate Account State**
● #[account] pub struct CertificateAccountState: This structure represents the state of a
certificate record stored on the blockchain. It includes fields for the certificate's issuer, recipient, type, and file
hash, as well as the transaction's witness (the public key of the initializer) and timestamp. An initialized flag
indicates whether the record has been set up.

**Implementation Details**
● impl CertificateAccountState: This implementation block for CertificateAccountState
includes a function to dynamically calculate the required space for storing a certificate account. This
calculation takes into account the sizes of all string fields and fixed-size fields.
Error Handling

● **#[error_code] pub enum ErrorCode**: This enumeration defines custom error codes for the program.
The only defined error, RecordAlreadyInitialized, is used to prevent overwriting an already
initialized certificate record.
Security and Best Practices

● **Authorization Checks**: The contract ensures that only authorized signers (initializers) can add certificate
records. This is a fundamental security measure to prevent unauthorized access and modifications.

**● Data Consistency**: By checking the initialized flag before setting a certificate record, the contract prevents
accidental overwrites, ensuring data consistency and integrity.

● **Efficient Resource Use**: The dynamic calculation of account space needed for storing certificate data helps
optimize blockchain storage, ensuring that only the necessary amount of space is allocated.


We have created a decentralized Application to Digitally Record & e-Seal Educational and Work-Experience Certificates on the Solana Blockchain 
making it significantly cheaper and faster for Universities and Employers to conduct background verification process of Students and Employees

There are 2 components of the dApp : **eSealing and Verification** 

**eSealing Tab**: is used to register the file on the Blockchain after signing the transaction using web3 wallet. 
While registering any certificate on the e-Sealer section, we enter 3 metadata field : Certificate Issued to (Certificate Beneficiary), Certificate Issued by (Issuing Authority) and Certificate Type (about the Certificate).
We then upload the digital certificate copy on the blockchain from your local computer/desktop.
Then we click on "Register File Hash" button, a cryptographic wallet opens up, we sign the transaction on chain and pay the gas fees. 
This registers a unique cryptographic hash of the file and generates a transaction-hash and block-hash on the chain.
This also records time stamp (when the certifcate was recorded on the chain, in UTC time and date format: YYYY-MM-DD) and records the Unique wallet address which registered the certificate and signed the transaction on the chain.

Since, it is a B2B solution, we will issue: 1 Unique Wallet Address for 1 Education Institute, 1 Wallet address for 1 Employer and 1 Unique Wallet Address for each of the Study Abroad Consultant.

Presently we have created the first part of the dApp (recording any certificate individually on the blockchain and getting it verified) on the Blockchain

Here, during the e-Sealing of the Certificate: we generate Unique File Hash, TimeStamp (when the certificate was recorded) of each Certificate, and record 6 fields on the chain: 

a.) Who the certificate was issued to (Certificate Beneficiary) 

b.) Who is it issued by (Certifying Authority) 

c.) What is the certificate about (Details of the Certificate)

d.) Unique file hash (cryptographic hash) of the certificate 

e.) Time stamp of the certificate (when the certificate was recorded on the chain, in UTC time) 

f.) Who recorded the certificate on the chain (witness’: Certifying Authority’s wallet address)

**Verification Tab**: is used to upload a digital certificate and click on "Verify Certificate" tab on the dapp.
It then displays the 6 fields as retrieved from the Blockchain and verified with the previously generated information during e-Sealing (that can be stored off chain in a database).

Once all 6 fields are completely checked and matching is 100%, the dApp shows this message: "Certificate Verified with green color and right tick mark"
On the contrary, if there is any change in the certificate, i.e. it has been tampered with, its hash will change, 
Hence the information will Not be 100% verified or in the case, if any certificate was Never recorded on the chain using the dApp, 
then the dApp shows this message: "Error! Certificate Not Verified" with red color and cross mark.






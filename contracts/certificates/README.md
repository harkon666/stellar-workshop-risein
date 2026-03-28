# Stellar Academic Certificates DApp

**Academic Certificates DApp** - Blockchain-Based Academic Credential Registry on Stellar

## Project Description

Academic Certificates DApp is a decentralized smart contract solution built on the Stellar blockchain using Soroban SDK. It provides a secure, immutable platform for registering and verifying academic certificates and credentials. The contract enables educational institutions to issue certificate hashes on-chain, allowing anyone to verify the authenticity and existence of academic credentials.

This system leverages the transparency and security of the Stellar network to solve real-world problems: fake degrees, credential verification delays, and the inability to prove when a certificate was issued.

## Project Vision

Our vision is to modernize academic credentialing by:

- **Eliminating Fake Degrees** - Each certificate hash is permanently stored on-chain, making it impossible to forge credentials
- **Instant Verification** - Employers can verify certificates in seconds without contacting universities
- **Proving Existence** - Timestamps prove exactly when a certificate was issued
- **Decentralizing Trust** - No single authority controls the records; the blockchain is the source of truth
- **Global Accessibility** - Anyone with internet access can verify a certificate from anywhere in the world

We envision a future where academic credentials are as verifiable as cryptocurrency transactions - instant, transparent, and tamper-proof.

## Key Features

### 1. **Certificate Registration**

- Register certificates with issuer, subject, document hash, and metadata
- Automated ID generation for unique identification
- Ledger timestamp captures exact registration time
- Persistent storage on the Stellar blockchain

### 2. **Certificate Verification**

- Verify certificate authenticity by ID
- Compare document hash to detect tampering
- View all certificate details including issuer and timestamp

### 3. **Query Capabilities**

- Fetch all registered certificates
- Filter by issuer address (e.g., find all certificates from "MIT")
- Filter by subject name (e.g., find all certificates for "John Doe")

### 4. **Security & Immutability**

- All records permanently stored on-chain
- Cannot be altered or deleted by third parties
- Document hash ensures content integrity
- Issuer address provides accountability

### 5. **Stellar Network Integration**

- Leverages Stellar's high speed and low cost
- Built using Soroban Smart Contract SDK
- Scalable architecture for growing certificate collections
- Interoperable with other Stellar-based services

## Contract Details

- **Network**: Stellar Testnet
- **Contract ID**: `CDDIBMNHDHEUBBM3D5ZKOIE5Z4MVHXTSS3WXWTYQNBXWUDUFFD47BKXU`
- **WASM Size**: 4965 bytes
- **Exported Functions**: 5

## Contract Functions

### `register_certificate`

Register a new academic certificate on the blockchain.

```rust
pub fn register_certificate(
    env: Env,
    issuer: Address,       // Institution issuing the certificate
    subject: String,       // Recipient name
    document_hash: String, // SHA-256 hash of original document
    metadata: String,      // Additional info (degree, year, etc.)
) -> String
```

**Returns**: `"Certificate berhasil didaftarkan"` on success

**Example**:
```bash
stellar contract invoke \
  --id CDDIBMNHDHEUBBM3D5ZKOIE5Z4MVHXTSS3WXWTYQNBXWUDUFFD47BKXU \
  --source alice \
  --network testnet \
  -- register_certificate \
  --issuer GDKG... \
  --subject "John Doe" \
  --document_hash "sha256:abc123..." \
  --metadata "Bachelor of Science, Computer Science, 2024"
```

### `verify_certificate`

Verify a certificate by its ID.

```rust
pub fn verify_certificate(env: Env, id: u64) -> Certificate
```

**Returns**: `Certificate` struct with all details, or empty certificate with id=0 if not found

**Example**:
```bash
stellar contract invoke \
  --id CDDIBMNHDHEUBBM3D5ZKOIE5Z4MVHXTSS3WXWTYQNBXWUDUFFD47BKXU \
  --source alice \
  --network testnet \
  -- verify_certificate \
  --id 1
```

### `get_all_certificates`

Retrieve all registered certificates.

```rust
pub fn get_all_certificates(env: Env) -> Vec<Certificate>
```

**Returns**: `Vec<Certificate>` containing all certificates

**Example**:
```bash
stellar contract invoke \
  --id CDDIBMNHDHEUBBM3D5ZKOIE5Z4MVHXTSS3WXWTYQNBXWUDUFFD47BKXU \
  --source alice \
  --network testnet \
  -- get_all_certificates
```

### `get_by_issuer`

Get all certificates issued by a specific institution.

```rust
pub fn get_by_issuer(env: Env, issuer: Address) -> Vec<Certificate>
```

**Example**:
```bash
stellar contract invoke \
  --id CDDIBMNHDHEUBBM3D5ZKOIE5Z4MVHXTSS3WXWTYQNBXWUDUFFD47BKXU \
  --source alice \
  --network testnet \
  -- get_by_issuer \
  --issuer GDKG...
```

### `get_by_subject`

Get all certificates for a specific recipient.

```rust
pub fn get_by_subject(env: Env, subject: String) -> Vec<Certificate>
```

**Example**:
```bash
stellar contract invoke \
  --id CDDIBMNHDHEUBBM3D5ZKOIE5Z4MVHXTSS3WXWTYQNBXWUDUFFD47BKXU \
  --source alice \
  --network testnet \
  -- get_by_subject \
  --subject "John Doe"
```

## Data Structure

### Certificate

```rust
pub struct Certificate {
    id: u64,              // Unique identifier
    issuer: Address,      // Institution address
    subject: String,       // Recipient name
    document_hash: String,// SHA-256 hash
    timestamp: u64,       // Ledger sequence number
    metadata: String,     // Additional info
}
```

## Use Cases

### 1. **University Degree Verification**

**Scenario**: Employer receives a resume claiming "MIT Degree, 2024"

**Without Blockchain**: Email MIT registrar, wait 2-4 weeks, pay verification fee

**With This Contract**:
1. MIT registers: `register_certificate(issuer=MIT, subject="John Doe", hash="abc123", metadata="BS Computer Science, 2024")`
2. Employer verifies: `verify_certificate(id=X)`
3. Instant result with timestamp proving when it was issued

### 2. **Online Course Certificates**

**Platforms**: Coursera, edX, Udemy can issue blockchain certificates

**Benefits**:
- No need for manual verification
- Tamper-proof credentials
- Globally accessible
- Permanent record

### 3. **Professional Licensing**

**Industries**: Medical boards, bar associations, engineering councils

**Use**:
- Issue certificate hashes when professionals pass exams
- Employers verify licenses on-chain
- Regulatory bodies can audit issued credentials

### 4. **Document Timestamping**

**Legal Documents**: Prove a document existed at a specific time

**Use**:
- Hash the document locally
- Register hash on blockchain
- Timestamp proves existence before any disputes

## Business Model

### For Educational Institutions

**Value Proposition**:
- Modernize credentialing system
- Reduce verification workload
- Enhance institution reputation
- Zero infrastructure cost (uses Stellar network)

**Revenue**: None needed - this is a public good / cost reduction

### For Verification Requesters (Employers)

**Value Proposition**:
- Instant verification (seconds vs weeks)
- Low cost (Stellar fees ~0.0001 XLM)
- Reliable and tamper-proof
- 24/7 availability

### For the Ecosystem

**Network Effect**:
- More institutions = more valuable verification
- Students can carry credentials across platforms
- Global standard for academic verification

## Technical Requirements

- **SDK**: Soroban SDK 25
- **Language**: Rust
- **Target**: wasm32v1-none (WebAssembly)
- **Blockchain**: Stellar Network
- **CLI**: Stellar CLI

## Getting Started

### Prerequisites

1. Install Stellar CLI:
   ```bash
   curl -fsSL https://github.com/stellar/stellar-cli/releases/latest/download/install.sh | sh
   ```

2. Generate a key pair:
   ```bash
   stellar keys generate my-account
   ```

3. Fund account on testnet:
   ```bash
   stellar friendbot --source my-account
   ```

### Build Contract

```bash
stellar contract build
```

### Deploy Contract

```bash
stellar contract deploy \
  --source my-account \
  --network testnet
```

### Interact with Contract

**Register a certificate:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-account \
  --network testnet \
  -- register_certificate \
  --issuer <ISSUER_ADDRESS> \
  --subject "Student Name" \
  --document_hash "sha256:abc123..." \
  --metadata "Degree, Major, Year"
```

**Verify a certificate:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-account \
  --network testnet \
  -- verify_certificate \
  --id 1
```

### Run Tests

```bash
cargo test --manifest-path contracts/certificates/Cargo.toml
```

## Future Scope

### Short-Term Enhancements

1. **Batch Registration** - Register multiple certificates in one transaction
2. **Certificate Revocation** - Allow issuers to revoke certificates
3. **Expiration Dates** - Add validity period for certificates
4. **IPFS Integration** - Store actual documents on IPFS, hash on-chain

### Medium-Term Development

5. **Multi-Issuers** - Support multiple institutions with different permissions
6. **Access Control** - Only authorized issuers can register
7. **Events** - Emit events for certificate registration/revocation
8. **Upgrade Path** - Allow contract upgrades while preserving data

### Long-Term Vision

9. **ZK Proofs** - Zero-knowledge proofs for privacy-preserving verification
10. **Cross-Chain** - Bridge certificates to other blockchains
11. **NFT Integration** - Represent certificates as NFTs
12. **DAO Governance** - Community-managed certificate authority
13. **Mobile App** - User-friendly mobile verification app
14. **QR Codes** - Scan certificate QR code to verify instantly

### Enterprise Features

15. **API Service** - REST API for verification services
16. **Batch Verification** - Verify multiple certificates at once
17. **Analytics Dashboard** - View verification statistics
18. **Integration Plugins** - Plugins for HR systems (SAP, Workday)

## Project Structure

```
contracts/certificates/
├── Cargo.toml           # Rust package configuration
├── Makefile             # Build automation
├── README.md            # This file
└── src/
    ├── lib.rs           # Main contract implementation
    └── test.rs          # Unit tests
```

## Security Considerations

1. **Document Hash** - Only hash is stored; original document must be kept by issuer
2. **Issuer Trust** - Verification assumes issuer is legitimate
3. **No Deletion** - Certificates cannot be deleted (by design)
4. **Address Verification** - Ensure issuer address is correct

## Limitations

1. **Document Not Stored** - Only hash is on-chain; need external document storage
2. **No Revocation** - Currently certificates cannot be revoked
3. **Single Language** - Metadata is plain text; no structured data
4. **No Access Control** - Anyone can register certificates

## Glossary

- **Soroban**: Stellar's smart contract platform
- **WASM**: WebAssembly - binary instruction format for contracts
- **Document Hash**: SHA-256 hash of the original document
- **Ledger Sequence**: Stellar's block number equivalent
- **Issuer**: Educational institution issuing the certificate
- **Subject**: Person receiving the certificate

## Resources

- [Stellar Developers Documentation](https://developers.stellar.org)
- [Soroban Smart Contract Documentation](https://developers.stellar.org/docs/build/smart-contracts/overview)
- [Stellar CLI Reference](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli)
- [Soroban SDK Docs](https://docs.rs/soroban-sdk/latest/soroban_sdk/)
- [Stellar Testnet Explorer](https://stellar.expert/explorer/testnet/)

---

**Stellar Academic Certificates DApp** - Trustless Academic Credential Verification on the Blockchain

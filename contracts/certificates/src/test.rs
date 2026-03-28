#![cfg(test)]
use crate::{CertificatesContract, CertificatesContractClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, String};

#[test]
fn test_register_and_verify_certificate() {
    let env = Env::default();
    let contract_id = env.register(CertificatesContract, ());
    let client = CertificatesContractClient::new(&env, &contract_id);

    let issuer = Address::generate(&env);
    let subject = String::from_str(&env, "John Doe");
    let doc_hash = String::from_str(&env, "abc123def456");
    let metadata = String::from_str(&env, "Computer Science, 2024");

    // Register certificate
    let result = client.register_certificate(&issuer, &subject, &doc_hash, &metadata);

    assert_eq!(
        result,
        String::from_str(&env, "Certificate berhasil didaftarkan")
    );

    // Verify certificate
    let cert = client.verify_certificate(&1);
    assert_eq!(cert.id, 1);
    assert_eq!(cert.subject, subject);
    assert_eq!(cert.document_hash, doc_hash);
}

#[test]
fn test_get_all_certificates() {
    let env = Env::default();
    let contract_id = env.register(CertificatesContract, ());
    let client = CertificatesContractClient::new(&env, &contract_id);

    let issuer1 = Address::generate(&env);
    let issuer2 = Address::generate(&env);

    // Register multiple certificates
    client.register_certificate(
        &issuer1,
        &String::from_str(&env, "Alice"),
        &String::from_str(&env, "hash1"),
        &String::from_str(&env, "Physics"),
    );

    client.register_certificate(
        &issuer2,
        &String::from_str(&env, "Bob"),
        &String::from_str(&env, "hash2"),
        &String::from_str(&env, "Chemistry"),
    );

    let all = client.get_all_certificates();
    assert_eq!(all.len(), 2);
}

#[test]
fn test_get_by_issuer() {
    let env = Env::default();
    let contract_id = env.register(CertificatesContract, ());
    let client = CertificatesContractClient::new(&env, &contract_id);

    let issuer = Address::generate(&env);

    client.register_certificate(
        &issuer,
        &String::from_str(&env, "Student A"),
        &String::from_str(&env, "hash1"),
        &String::from_str(&env, "Math"),
    );

    client.register_certificate(
        &issuer,
        &String::from_str(&env, "Student B"),
        &String::from_str(&env, "hash2"),
        &String::from_str(&env, "Science"),
    );

    let from_issuer = client.get_by_issuer(&issuer);
    assert_eq!(from_issuer.len(), 2);
}

#[test]
fn test_get_by_subject() {
    let env = Env::default();
    let contract_id = env.register(CertificatesContract, ());
    let client = CertificatesContractClient::new(&env, &contract_id);

    client.register_certificate(
        &Address::generate(&env),
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "hash1"),
        &String::from_str(&env, "2024"),
    );

    client.register_certificate(
        &Address::generate(&env),
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "hash2"),
        &String::from_str(&env, "2025"),
    );

    let from_subject = client.get_by_subject(&String::from_str(&env, "John Doe"));
    assert_eq!(from_subject.len(), 2);
}

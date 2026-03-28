#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};

// Struktur data Certificate
#[contracttype]
#[derive(Clone, Debug)]
pub struct Certificate {
    id: u64,
    issuer: Address,       // Alamat institution yang mengeluarkan
    subject: String,       // Nama penerima certificate
    document_hash: String, // SHA-256 hash dari dokumen
    timestamp: u64,        // Waktu registrasi (ledger seq)
    metadata: String,      // Info tambahan (jurusan, tahun, dll)
}

// Storage keys
const CERT_DATA: Symbol = symbol_short!("CERT_DATA");
const CERT_COUNTER: Symbol = symbol_short!("CERT_CNT");

#[contract]
pub struct CertificatesContract;

#[contractimpl]
impl CertificatesContract {
    // Fungsi untuk mendaftarkan certificate baru
    pub fn register_certificate(
        env: Env,
        issuer: Address,
        subject: String,
        document_hash: String,
        metadata: String,
    ) -> String {
        // 1. Ambil semua certificates dari storage
        let mut certs: Vec<Certificate> = env
            .storage()
            .instance()
            .get(&CERT_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Ambil counter untuk ID baru
        let mut counter: u64 = env.storage().instance().get(&CERT_COUNTER).unwrap_or(0);

        counter += 1;

        // 3. Buat certificate baru
        let cert = Certificate {
            id: counter,
            issuer: issuer,
            subject: subject,
            document_hash: document_hash,
            timestamp: env.ledger().sequence() as u64,
            metadata: metadata,
        };

        // 4. Tambahkan ke list
        certs.push_back(cert);

        // 5. Simpan kembali ke storage
        env.storage().instance().set(&CERT_DATA, &certs);
        env.storage().instance().set(&CERT_COUNTER, &counter);

        return String::from_str(&env, "Certificate berhasil didaftarkan");
    }

    // Fungsi untuk verifikasi certificate berdasarkan ID
    pub fn verify_certificate(env: Env, id: u64) -> Certificate {
        let certs: Vec<Certificate> = env
            .storage()
            .instance()
            .get(&CERT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..certs.len() {
            let cert = certs.get(i).unwrap();
            if cert.id == id {
                return cert;
            }
        }

        // Return empty certificate if not found (id 0 indicates not found)
        return Certificate {
            id: 0,
            issuer: Address::from_string(&String::from_str(
                &env,
                "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF",
            )),
            subject: String::from_str(&env, ""),
            document_hash: String::from_str(&env, ""),
            timestamp: 0,
            metadata: String::from_str(&env, ""),
        };
    }

    // Fungsi untuk mendapatkan semua certificates
    pub fn get_all_certificates(env: Env) -> Vec<Certificate> {
        return env
            .storage()
            .instance()
            .get(&CERT_DATA)
            .unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk mendapatkan certificates berdasarkan issuer
    pub fn get_by_issuer(env: Env, issuer: Address) -> Vec<Certificate> {
        let all_certs: Vec<Certificate> = env
            .storage()
            .instance()
            .get(&CERT_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result: Vec<Certificate> = Vec::new(&env);

        for i in 0..all_certs.len() {
            let cert = all_certs.get(i).unwrap();
            if cert.issuer == issuer {
                result.push_back(cert);
            }
        }

        return result;
    }

    // Fungsi untuk mendapatkan certificates berdasarkan subject
    pub fn get_by_subject(env: Env, subject: String) -> Vec<Certificate> {
        let all_certs: Vec<Certificate> = env
            .storage()
            .instance()
            .get(&CERT_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result: Vec<Certificate> = Vec::new(&env);

        for i in 0..all_certs.len() {
            let cert = all_certs.get(i).unwrap();
            if cert.subject == subject {
                result.push_back(cert);
            }
        }

        return result;
    }
}

mod test;

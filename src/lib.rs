use serde::{Serialize, Deserialize};
use sha3::{Digest, Sha3_512};
use uuid::Uuid;

/// Estrutura de metadados ESG conforme o ERC-8040.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ESGMetadata {
    pub standard: String,
    pub category: String,
    pub geo: String,
    pub carbon_value: f64,
    pub cycle: String,
    pub digest: String,
    pub physical_id: String,
    pub attestation: Attestation,
    pub status: String,
    pub evidence: String,
}

/// Estrutura de atestação do Autonomous Trust Framework for Artificial Intelligence (ATF-AI).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attestation {
    pub atf_digest: String,
    pub signer: String,
}

/// Função utilitária para gerar digest SHA3-512 a partir de uma string
pub fn generate_digest(input: &str) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Cria um novo ESGMetadata com base nos parâmetros recebidos
pub fn create_metadata(category: &str, geo: &str, carbon_value: f64, cycle: &str) -> ESGMetadata {
    // gera um digest único
    let digest = generate_digest(&format!(
        "{}:{}:{}:{}",
        category, geo, carbon_value, cycle
    ));

    // gera um UUID pro physical_id
    let physical_id = format!("seal:{}", Uuid::new_v4());

    // clona o digest pra atf_digest antes de mover
    let attestation = Attestation {
        atf_digest: digest.clone(),
        signer: "did:atf:ai:validator".to_string(),
    };

    ESGMetadata {
        standard: "ERC-8040/1.0".to_string(),
        category: category.to_string(),
        geo: geo.to_string(),
        carbon_value,
        cycle: cycle.to_string(),
        digest,
        physical_id,
        attestation,
        status: "issued".to_string(),
        evidence: "cid:QmExample".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_generation() {
        let meta = create_metadata("carbon", "BR-RS", 12.5, "2025-Q3");
        assert_eq!(meta.standard, "ERC-8040/1.0");
        assert_eq!(meta.category, "carbon");
        assert_eq!(meta.geo, "BR-RS");
        assert_eq!(meta.status, "issued");
        assert!(!meta.digest.is_empty());
        assert!(!meta.attestation.atf_digest.is_empty());
    }
}

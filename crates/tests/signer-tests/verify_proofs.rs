use anyhow::Result;
use nuts::{Error, nut01::PublicKey, nut02::KeysetId};
use signer::{DeclareKeysetRequest, DeclareKeysetResponse};
use signer_tests::init_signer_client;
use std::str::FromStr;

#[tokio::test]
async fn test_verify_proofs_valid() -> Result<()> {
    let client = init_signer_client().await?;
    let request = VerifyProofsRequest {
        proofs: vec![/* populate with valid proofs */],
    };

    let response = client.verify_proofs(Request::new(request)).await?;
    assert!(response.into_inner().is_valid);

    Ok(())
}

#[tokio::test]
async fn test_verify_proofs_invalid_keyset_id() -> Result<()> {
    let client = init_signer_client().await?;
    let request = VerifyProofsRequest {
        proofs: vec![/* populate with invalid keyset_id */],
    };

    let response = client.verify_proofs(Request::new(request)).await;
    assert!(response.is_err());
    assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);

    Ok(())
}

#[tokio::test]
async fn test_verify_proofs_invalid_signature() -> Result<()> {
    let client = init_signer_client().await?;
    let request = VerifyProofsRequest {
        proofs: vec![/* populate with invalid signature */],
    };

    let response = client.verify_proofs(Request::new(request)).await;
    assert!(response.is_err());
    assert_eq!(response.unwrap_err().code(), tonic::Code::InvalidArgument);

    Ok(())
}

#[tokio::test]
async fn test_verify_proofs_keyset_not_found() -> Result<()> {
    let client = init_signer_client().await?;
    let request = VerifyProofsRequest {
        proofs: vec![/* populate with non-existent keyset_id */],
    };

    let response = client.verify_proofs(Request::new(request)).await;
    assert!(response.is_err());
    assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);

    Ok(())
}

#[tokio::test]
async fn test_verify_proofs_amount_not_found() -> Result<()> {
    let client = init_signer_client().await?;
    let request = VerifyProofsRequest {
        proofs: vec![/* populate with non-existent amount */],
    };

    let response = client.verify_proofs(Request::new(request)).await;
    assert!(response.is_err());
    assert_eq!(response.unwrap_err().code(), tonic::Code::NotFound);

    Ok(())
}

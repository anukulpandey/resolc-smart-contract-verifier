use crate::{
    compiler::DetailedVersion, verify, Error, EvmCompilersPool, OnChainContract, SolcCompiler,
    SolcInput, VerificationResult,
};

type Content = SolcInput;

pub struct VerificationRequest {
    pub contract: OnChainContract,
    pub compiler_version: DetailedVersion,
    pub content: Content,
}

pub async fn verify(
    compilers: &EvmCompilersPool<SolcCompiler>,
    request: VerificationRequest,
) -> Result<VerificationResult, Error> {
    let to_verify = vec![request.contract];

    let results = verify::compile_and_verify(
        to_verify,
        compilers,
        &request.compiler_version,
        request.content,
    )
    .await?;

    let result = results
        .into_iter()
        .next()
        .expect("we sent exactly one contract to verify");

    Ok(result)
}

#[derive(Clone, Debug)]
pub struct BatchVerificationRequest {
    pub contracts: Vec<OnChainContract>,
    pub compiler_version: DetailedVersion,
    pub content: Content,
}

pub async fn batch_verify(
    compilers: &EvmCompilersPool<SolcCompiler>,
    request: BatchVerificationRequest,
) -> Result<Vec<VerificationResult>, Error> {
    println!(
        "[batch_verify] Starting batch verification for {} contracts, compiler_version: {:?}",
        request.contracts.len(),
        request.compiler_version
    );

    let to_verify = request.contracts;

    let results = verify::compile_and_verify(
        to_verify,
        compilers,
        &request.compiler_version,
        request.content,
    )
    .await?;

    println!("[batch_verify] Verification results: {:?}", results);

    Ok(results)
}

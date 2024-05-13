use goplus_rs::{GpError, Session};

#[tokio::test]
async fn endpoint_tests() -> Result<(), GpError> {
    let session = Session::new();
    let _ = session.supported_chains().await?;
    Ok(())
}



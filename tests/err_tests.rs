use goplus_rs::Session;

#[tokio::test]
async fn incorrect_endpoint_err() -> Result<(), std::io::Error> {
    let session = Session::new();
    
    let res = session.supported_chains().await;
    // assert!(res.is_ok());
    if let Err(e) =  &res {
        println!("Error: {:#?}", e);
    }
    // let res=res.unwrap();
    // tracing::error!("Error: {:#?}", res);
    Ok(())
}



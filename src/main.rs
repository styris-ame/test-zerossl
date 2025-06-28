use std::{env, fs};
use std::io::Write;
use std::path::Path;
use zerossl::{generate_rsa_2048_priv_key, Client, CreateCertificateReq, Resp, VerifyCertificateReq};
use zerossl::certs::csr::Csr;
use zerossl::client::validation::ValidationType;

#[tokio::main]
async fn main() {
    let test_domain = "zerossl.styris.net".to_string();
    let is_ip = false;

    let api_key = env::var("API_KEY").unwrap();
    let client = Client::new(api_key);

    println!("Breh1");
    let pkey = generate_rsa_2048_priv_key().unwrap();

    let mut domains: Vec<String> = Vec::new();
    domains.push(test_domain.clone());
    println!("Breh2");
    let bytes = pkey.private_key_to_pem_pkcs8().unwrap();

    println!("L RATIO");

    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let mut file = std::fs::File::create("./server.key").unwrap();
    file.write_all(&bytes).unwrap();

    println!("FINISH FILE WRITTEN");

    let mut csr = Csr::new(test_domain.clone());
    let csr = csr.with_alt_names(domains.clone(), is_ip)
        .with_country("AU".to_string())
        .with_org_name("Lit".to_string())
        .with_org_unit("Node Devs".to_string());

    let cert_req = CreateCertificateReq::from_csr(&pkey, &csr)
        .expect("failed to make cert req");

    client.purge_certificates(test_domain.clone(), true, false).await
        .expect("failed to purge certs");

    let cert_res = client.create_certificate(&cert_req).await
        .expect("failed to get cert");
    println!("OUT: {:#?}", cert_res);
    let (validation_id, file_validation_content) = cert_res.certificate().file_validation(&test_domain).unwrap();
    println!("FV: {:#?}, {:#?}", validation_id, file_validation_content);

    let validation_dir = "./dist/.well-known/pki-validation";
    if !Path::new(validation_dir).is_dir() {
        println!("Data directory '{validation_dir}' not found");
        fs::create_dir_all(validation_dir).unwrap();
        println!("Data directory '{validation_dir}' created");
    }
    let validation_file = format!("{validation_dir}/{}.txt", validation_id);
    println!("Validation file: {validation_file}");
    fs::write(validation_file, file_validation_content.join("\n")).unwrap();
    
    let verify_result = client.verify_certificate(validation_id, &VerifyCertificateReq::new(ValidationType::HttpCsrHash, None)).await.unwrap();

    if let Some(err) = verify_result.err_msg_string() {
        println!("Err: {err}");
        return;
    }
    
    let mut download_result = client.download_certificate(validation_id).await.unwrap();
    if let Some(err) = download_result.err_msg_string() {
        println!("ErrD: {err}");
        return;
    }
    
    let cert = download_result.take_certificate_crt().unwrap();
    fs::write("server.crt", cert).unwrap();

    let cert = download_result.take_ca_bundle_crt().unwrap();
    fs::write("ca-bundle.crt", cert).unwrap();
    
    println!("Complete W!")
}

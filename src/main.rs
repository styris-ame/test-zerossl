use std::env;
use std::io::Write;
use zerossl::{generate_rsa_2048_priv_key, Client};

fn main() {
    let test_domain = "dev.getlit.sh".to_string();
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
}

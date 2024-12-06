use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use anyhow::{anyhow, Context};
use base64::{prelude::BASE64_STANDARD, Engine};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn include_encrypted_string(_item: TokenStream) -> TokenStream {
    let input: syn::LitStr = syn::parse(_item).unwrap();
    let filename = input.value();
    let ciphertext = std::fs::read_to_string(filename).expect("Could not open file");
    let nonce = std::fs::read_to_string("./inputs/nonce.txt").expect("Could not read nonce");
    let key = std::env::var("ENC_KEY").expect("Encryption key not set in environment");
    let plaintext = decrypt(&key, &nonce, &ciphertext).expect("Could not decrypt file contents");
    let _ = encrypt_input_files();
    quote! {
        #plaintext
    }
    .into()
}

fn encrypt(key: &str, nonce: &str, text: &str) -> anyhow::Result<String> {
    let key = BASE64_STANDARD.decode(key)?;
    let key: [u8; 32] = key
        .try_into()
        .map_err(|_| anyhow!("Key must be 32 bytes long."))?;

    let nonce = BASE64_STANDARD.decode(nonce)?;
    let nonce: [u8; 12] = nonce
        .try_into()
        .map_err(|_| anyhow!("Nonce must be 12 bytes long."))?;

    let cipher = Aes256Gcm::new(&key.into());
    let ciphertext = cipher
        .encrypt(&nonce.into(), text.as_ref())
        .map_err(|_| anyhow!("Could not encrypt plaintext."))?;
    let ciphertext = BASE64_STANDARD.encode(ciphertext);

    Ok(ciphertext)
}

fn decrypt(key: &str, nonce: &str, ciphertext: &str) -> anyhow::Result<String> {
    let key = BASE64_STANDARD.decode(key)?;
    let key: [u8; 32] = key
        .try_into()
        .map_err(|_| anyhow!("Key must be 32 bytes long."))?;

    let nonce = BASE64_STANDARD.decode(nonce)?;
    let nonce: [u8; 12] = nonce
        .try_into()
        .map_err(|_| anyhow!("Nonce must be 12 bytes long."))?;

    let ciphertext = BASE64_STANDARD.decode(ciphertext)?;

    let cipher = Aes256Gcm::new(&key.into());
    let plaintext = cipher
        .decrypt(&nonce.into(), ciphertext.as_ref())
        .map_err(|_| anyhow!("Could not decrypt ciphertext."))?;
    let plaintext = String::from_utf8(plaintext)?;

    Ok(plaintext)
}

fn encrypt_input_files() -> anyhow::Result<()> {
    let input_dir = std::fs::read_dir("./inputs")?;
    let files = input_dir
        .flatten()
        .filter(|entry| entry.metadata().map(|m| m.is_file()).unwrap_or(false))
        .map(|entry| entry.path())
        .filter(|p| p.extension().map(|x| x == "plain").unwrap_or(false));

    let nonce = std::fs::read_to_string("./inputs/nonce.txt")?;
    let key = std::env::var("ENC_KEY")?;
    for file in files {
        let plaintext = std::fs::read_to_string(&file)?;
        let parent = file.parent().context("Couldn't find file parent")?;
        let new_path = parent.join(format!(
            "{}.enc",
            &file
                .file_stem()
                .context("Couldn't get file stem")?
                .to_string_lossy()
        ));
        let ciphertext = encrypt(&key, &nonce, &plaintext)?;
        std::fs::write(new_path, ciphertext)?;
        std::fs::rename(
            &file,
            parent.join(format!(
                "{}.plain.encrypted",
                file.file_stem()
                    .context("Couldn't get file stem")?
                    .to_string_lossy()
            )),
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use aes_gcm::{aead::OsRng, AeadCore, Aes256Gcm, KeyInit};
    use base64::{prelude::BASE64_STANDARD, Engine};

    #[test]
    fn generate_nonce() {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let nonce = BASE64_STANDARD.encode(nonce);
        println!("Nonce: {nonce}");
    }

    #[test]
    fn generate_key() {
        let key = Aes256Gcm::generate_key(OsRng);
        let key = BASE64_STANDARD.encode(key);
        println!("Key: {key}");
    }

    #[test]
    fn encrypt_input_files() {
        super::encrypt_input_files().unwrap();
    }
}

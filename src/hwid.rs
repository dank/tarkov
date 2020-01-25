use rand::Rng;

fn random_md5<R: Rng + ?Sized>(rng: &mut R) -> String {
    format!("{:x}", md5::compute(&rng.gen::<i32>().to_le_bytes()))
}

/// Generate a random EFT compatible HWID.
pub fn generate_hwid() -> String {
    let mut rng = rand::thread_rng();

    let short_md5 = {
        let mut hash = random_md5(&mut rng);
        hash.truncate(hash.len() - 8);

        hash
    };

    format!(
        "#1-{}:{}:{}-{}-{}-{}-{}-{}",
        random_md5(&mut rng),
        random_md5(&mut rng),
        random_md5(&mut rng),
        random_md5(&mut rng),
        random_md5(&mut rng),
        random_md5(&mut rng),
        random_md5(&mut rng),
        short_md5
    )
}

#[test]
fn test_generate_hwid() {
    assert_eq!(generate_hwid().len(), 258)
}

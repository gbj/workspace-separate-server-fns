use leptos::{server, ServerFnError};

#[server]
pub async fn random_number() -> Result<u32, ServerFnError> {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    Ok(rng.gen_range(1..=100))
}

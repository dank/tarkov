use crate::{Tarkov, LAUNCHER_ENDPOINT, LAUNCHER_VERSION};
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct LoginRequest<'a>{
    email: &'a str,
    password: &'a str,
}

impl Tarkov {
    pub async fn login(&self, email: &str, password: &str) {
        let url = format!("{}/launcher/login?launcherVersion={}&branch=live", LAUNCHER_ENDPOINT, LAUNCHER_VERSION);
        let req = LoginRequest {
            email,
            password: "a",
        };
        let mut res = self.client.post(url).send_json(&req).await.unwrap();
        let body = res.body().await.unwrap();

        println!("{:?}", body);
//        match res.status() {
//            StatusCode::OK => Ok(serde_json::from_slice::<T>(&body)?),
//            _ => {}
//        };
    }
}

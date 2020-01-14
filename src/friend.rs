use crate::{Tarkov, Result, PROD_ENDPOINT, UNITY_VERSION, GAME_VERSION};
use flate2::read::ZlibDecoder;
use std::io::Read;

impl Tarkov {
    pub async fn get_friends(&self) -> Result<()> {
        let url = format!("{}/client/friend/list", PROD_ENDPOINT);
        let mut res = self.client
            .post(url)
            .header("User-Agent", format!("UnityPlayer/{} (UnityWebRequest/1.0, libcurl/7.52.0-DEV)", UNITY_VERSION))
            .header("App-Version", format!("EFT Client {}", GAME_VERSION))
            .header("X-Unity-Version", UNITY_VERSION)
            .header("Cookie", format!("PHPSESSID={}", self.session))
            .send_json(&{})
            .await?;

        let body = res.body().await?;
        let mut decode = ZlibDecoder::new(&body[..]);
        let mut body = String::new();
        decode.read_to_string(&mut body)?;

        println!("{:?}", body);

        Ok(())
    }
}

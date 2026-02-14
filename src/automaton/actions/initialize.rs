use log::debug;
use crate::automaton::client::HttpClient;

pub async fn run_initialize(http_client:HttpClient) -> Result<String, String> {

    let headers = HttpClient::create_headers("application/x-www-form-urlencoded", None);

    match http_client.get("/", headers).await {
        Ok(response) => {
            let status = response.status();
            debug!("Response status: {}", status);
            if (status!=200){
                return Err(format!("http status code: {}", status));
            }

            match http_client.get_cookie("/").await {
                Ok(cookie) => {
                    if let Some(cookie_value) = cookie {
                        debug!("Cookie obtained: {}", cookie_value);
                        Ok(cookie_value)
                    }else{
                        Err("Failed to get cookie value".to_string())
                    }
                }
                Err(e) => {
                    Err(format!("Failed to get cookie /: {}", e))
                }
            }
        }
        Err(e) => {
            Err(format!("Failed to initialize: {}", e))
        }
    }
}

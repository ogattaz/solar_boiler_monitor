use std::collections::HashMap;
use log::debug;
use crate::automaton::client::HttpClient;

pub async fn run_logoff(http_client:HttpClient) -> Result<bool, String> {


    let mut params = HashMap::new();
    params.insert("connexion", "deconnecter");

    let body = HttpClient::create_form_data(params);
    let headers = HttpClient::create_headers("application/x-www-form-urlencoded", None);

    match http_client.post("/admin/", headers, &body).await {
        Ok(response) => {

            let status = response.status();
            debug!("Response status: {}", status);
            if (status!=200){
                return Err(format!("http status code: {}", status));
            }
            Ok(true)
        }
        Err(e) => {
            return Err(format!("Failed to logoff: {}", e));
        }
    }
}

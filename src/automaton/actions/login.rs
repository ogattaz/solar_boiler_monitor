use std::collections::HashMap;
use log::info;
use crate::automaton::client::HttpClient;

pub async fn run_login(http_client:HttpClient,user_id:String,user_pass:String) -> Result<bool, String> {


    let mut params = HashMap::new();
    params.insert("id", user_id.as_str());
    params.insert("pass", user_pass.as_str());
    params.insert("ihm", "admin");
    params.insert("connexion", "Se connecter");

    let body = HttpClient::create_form_data(params);
    let headers = HttpClient::create_headers("application/x-www-form-urlencoded", None);

    match http_client.post("/admin/", headers, &body).await {
        Ok(response) => {
            info!("Response status: {}", response.status());
            Ok(true)
        }
        Err(e) => {
            return Err(format!("Failed to login: {}", e));
        }
    }
}

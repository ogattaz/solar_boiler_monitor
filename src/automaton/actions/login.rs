use std::collections::HashMap;
use log::{debug, info};
use crate::automaton::client::HttpClient;

pub async fn run_login(http_client:HttpClient,user_id:String,user_pass:String,cookie_value:String) -> Result<bool, String> {


    let mut params = HashMap::new();
    params.insert("id", user_id.as_str());
    params.insert("pass", user_pass.as_str());
    params.insert("ihm", "admin");
    params.insert("connexion", "Se connecter");

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
            return Err(format!("Failed to login: {}", e));
        }
    }
}

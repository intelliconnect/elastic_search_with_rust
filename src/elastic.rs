use actix_web::{web, HttpResponse};
use elasticsearch::{
    auth::Credentials, http::transport::Transport, DeleteParts, Elasticsearch, IndexParts,
    SearchParts, UpdateParts,
};
use serde_json::{json, Value};
use std::env::var;

pub fn get_client() -> Result<Elasticsearch, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let cloud_id = var("CLOUD_ID")?;
    let user = var("ELASTIC_USER")?;
    let pass = var("ELASTIC_PASS")?;
    let credentials = Credentials::Basic(user, pass);
    let transport = Transport::cloud(cloud_id.as_str(), credentials)?;
    Ok(Elasticsearch::new(transport))
}

pub async fn search(search_term: web::Json<Value>) -> HttpResponse {
    //we can return a custom error with internal server error response
    let client = get_client().unwrap();
    if let (Some(key), Some(value)) = (search_term.get("key"), search_term.get("value")) {
        let query = json!(
            {"query": {
                "match": {
                    key.to_string().trim_matches('"'): value.to_string().trim_matches('"')
                }
            }}
        );
        match client
            .search(SearchParts::Index(&["kibana_sample_data_ecommerce"]))
            .body(query)
            .send()
            .await
        {
            Ok(res) => HttpResponse::Ok().json(res.json::<Value>().await.unwrap()),
            Err(err) => HttpResponse::InternalServerError().json(json!({"err": err.to_string()})),
        }
    } else {
        HttpResponse::BadRequest().json(json!({"err": "bad request"}))
    }
}

pub async fn search_complex(search_term: web::Json<Value>) -> HttpResponse {
    let client = get_client().unwrap();
    if let (Some(must), Some(must_not)) = (search_term.get("include"), search_term.get("exclude")) {
        let query = json!({
            "query": {
                "bool": {
                    "must": [
                        {
                            "match": {
                                must["key"].to_string().trim_matches('"'): must["value"].to_string().trim_matches('"')
                            }
                        }
                    ],
                    "must_not": [
                        {
                            "match": {
                                must_not["key"].to_string().trim_matches('"'): must_not["value"].to_string().trim_matches('"')
                            }
                        }
                    ]
                }
            }
        });
        match client
            .search(SearchParts::Index(&["kibana_sample_data_ecommerce"]))
            .body(query)
            .send()
            .await
        {
            Ok(res) => HttpResponse::Ok().json(res.json::<Value>().await.unwrap()),
            Err(err) => HttpResponse::InternalServerError().json(json!({"err": err.to_string()})),
        }
    } else {
        HttpResponse::BadRequest().json(json!({"err": "bad request"}))
    }
}

pub async fn create_index(data: web::Json<Value>) -> HttpResponse {
    let client = get_client().unwrap();
    if let Some(user) = data.into_inner().get("user") {
        match client
            .index(IndexParts::Index("kibana_sample_data_ecommerce"))
            .body(json!({"user": user.to_string().trim_matches('"')}))
            .send()
            .await
        {
            Ok(res) => HttpResponse::Ok().json(res.json::<Value>().await.unwrap()),
            Err(err) => HttpResponse::InternalServerError().json(json!({"err": err.to_string()})),
        }
    } else {
        HttpResponse::BadRequest().json(json!({"err": "bad request"}))
    }
}

pub async fn update(data: web::Json<Value>) -> HttpResponse {
    let client = get_client().unwrap();
    let data = data.into_inner();
    if let Some(user_id) = data.get("id") {
        match client
            .update(UpdateParts::IndexId(
                "kibana_sample_data_ecommerce",
                user_id.to_string().trim_matches('"'),
            ))
            //source: https://www.elastic.co/guide/en/elasticsearch/reference/7.14/docs-update.html
            .body(json!({
                "script": {
                    "source": "ctx._source.user = params.user",
                    "lang": "painless",
                    "params": {
                        "user": data["change"].to_string().trim_matches('"')
                    }
                }
            }))
            .send()
            .await
        {
            Ok(res) => HttpResponse::Ok().json(res.json::<Value>().await.unwrap()),
            Err(err) => HttpResponse::InternalServerError().json(json!({"err": err.to_string()})),
        }
    } else {
        HttpResponse::BadRequest().json(json!({"err": "bad request"}))
    }
}

pub async fn delete_index(data: web::Json<Value>) -> HttpResponse {
    let client = get_client().unwrap();
    let data = data.into_inner();
    if let Some(id) = data.get("id") {
        match client
            .delete(DeleteParts::IndexId(
                "kibana_sample_data_ecommerce",
                id.to_string().trim_matches('"'),
            ))
            .send()
            .await
        {
            Ok(res) => HttpResponse::Ok().json(res.json::<Value>().await.unwrap()),
            Err(err) => HttpResponse::InternalServerError().json(json!({"err": err.to_string()})),
        }
    } else {
        HttpResponse::BadRequest().json(json!({"err": "bad request"}))
    }
}

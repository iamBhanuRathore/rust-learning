use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] // Derive Serialize and Clone for the struct
pub struct User {
    name: String,
    age: u16,
}

// The handler function must conform to Axum's requirements
pub async fn get_all_users() -> Json<Vec<User>> {
    let user_list: Vec<User> = vec![
        User {
            name: String::from("Bhanu"),
            age: 25,
        },
        User {
            name: String::from("Rahul"),
            age: 30,
        },
    ];

    // Return the user list as JSON directly
    Json(user_list)
}

#[derive(Serialize, Deserialize)] // Derive Serialize and Clone for the struct
pub struct Message {
    message: String,
}
#[derive(Serialize, Deserialize)] // Derive Serialize and Clone for the struct
pub struct MessageBody {
    message: String,
    messsage_from_server: String,
}

pub async fn return_same(Json(body): Json<Message>) -> Json<MessageBody> {
    let response: MessageBody = MessageBody {
        message: body.message,
        messsage_from_server: String::from("Server"),
    };
    Json(response)
}

#[derive(Serialize, Deserialize)] // Derive Serialize and Clone for the struct
pub struct Product {
    id: u16,
    name: String,
}

pub async fn product_controller(Path(id): Path<u16>) -> Json<Product> {
    let response: Product = Product {
        id,
        name: String::from("Product"),
    };
    Json(response)
}

#[derive(Deserialize, Serialize)] // Derive Serialize and Clone for the struct
pub struct BodyAndParam {
    name: String,
}

pub async fn body_and_param(Path(id): Path<u16>, Json(body): Json<BodyAndParam>) -> Json<Product> {
    Json(Product {
        id,
        name: body.name,
    })
}

#[derive(Serialize, Deserialize)] // Derive Serialize and Clone for the struct
pub struct QueryPath {
    message: String,
    age: u8,
}
#[derive(Serialize, Deserialize)] // Derive Serialize and Clone for the struct
pub struct BodyAndQueryAndParam {
    message: String,
    age: u8,
    name: String,
    id: u32,
}
pub async fn body_and_query_and_param(
    Path(id): Path<u32>,
    Query(query): Query<QueryPath>,
    Json(body): Json<BodyAndParam>,
) -> Json<BodyAndQueryAndParam> {
    Json(BodyAndQueryAndParam {
        id,
        name: body.name,
        age: query.age,
        message: query.message,
    })
}
#[derive(Serialize, Deserialize)] // Derive Serialize and Clone for the struct
pub struct BodyAndQueryAndParamAndHeader {
    message: String,
    age: u8,
    name: String,
    id: u32,
    headers: String,
}
pub async fn body_and_query_and_param_and_header(
    headers: HeaderMap,
    Path(id): Path<u32>,
    Query(query): Query<QueryPath>,
    Json(body): Json<BodyAndParam>,
) -> Json<BodyAndQueryAndParamAndHeader> {
    let user_agent = headers.get("user-agent");
    match user_agent {
        Some(user_agent) => {
            let response: BodyAndQueryAndParamAndHeader = BodyAndQueryAndParamAndHeader {
                message: query.message,
                age: query.age,
                name: body.name,
                id,
                headers: user_agent.to_owned().to_str().unwrap().to_string(),
            };
            Json(response)
        }
        None => {
            let response: BodyAndQueryAndParamAndHeader = BodyAndQueryAndParamAndHeader {
                message: query.message,
                age: query.age,
                name: body.name,
                id,
                headers: String::from("No user agent"),
            };
            Json(response)
        }
    }
}

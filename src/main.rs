use actix_web::{App,HttpServer};
use mongodb::{bson::{doc, Document},options::Credential,Client, Collection,};
use actix_web_httpauth::extractors::basic::BasicAuth;

use actix_web::{get, post,web::{self, Json},HttpRequest, HttpResponse, Responder, http::StatusCode,};
use serde::{Deserialize, Serialize};
use futures::StreamExt;





const EMAIL: &str = "Parth123@gmail.com";
const CO_PASSWORD: &str = "Parth@123";



#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyDataUpdate {
    Company_name: String,
    Company_id: u32,
    Company_address: String,
    Company_city:String,
}

// get request body and add to EmployeeDataInsert struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyDataInsert {
    Email: String,
    Compass: String,
    Company_name: String,
    Company_id: u32,
    Company_address: String,
    Company_city:String,
}
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(datagreet)
            .service(getData)
            .service(getSpecificCompanyData)
            .service(deletedata)
            .service(updatedata)
            .service(insertdata)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await

}

#[get("/")]
pub async fn datagreet() -> impl Responder {
    HttpResponse::build(StatusCode::OK).content_type("text/html; charset=utf-8").body(include_str!("../index.html"))
}

#[post("/data/{email}")]
pub async fn getSpecificCompanyData(
    companyemail: web::Path<String>,
    credential: BasicAuth,
) -> impl Responder {
    if credential.user_id().eq(EMAIL) && credential.password().unwrap().eq(CO_PASSWORD) {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("can'n connect with server");

        let col: Collection<Document> = client.database("companydatabase").collection("ComMONAPI");

        match col
            .find(
                doc! {
                "Email":
                companyemail.to_string()
                },
                None,
            )
            .await
        {
            Ok(mut docm) => {
                let data = &mut docm.next().await.unwrap().unwrap();
                let mydata = serde_json::to_string_pretty(&data).unwrap();
                HttpResponse::Ok().body(mydata)
            }
            Err(e) => HttpResponse::InternalServerError().body("Email not found  this data"),
        }
    } else {
        HttpResponse::InternalServerError().body("data authenticate is faild")
    }
}

#[post("/data")]
pub async fn getData(credential: BasicAuth) -> impl Responder {
    if credential.user_id().eq(EMAIL) && credential.password().unwrap().eq(CO_PASSWORD) {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .unwrap();

        let col: Collection<Document> = client.database("companydatabase").collection("ComMONAPI");
        let mut result = col.find(doc! {}, None).await.expect("faild");
        let mut data: Vec<Document> = vec![];

        while let Some(Ok(i)) = result.next().await {
            data.push(i);
        }
        let serial_data = serde_json::to_string_pretty(&data).unwrap();

        HttpResponse::Ok().body(serial_data)
    } else {
        HttpResponse::InternalServerError().body("data authenticate is faild")
    }
}

#[post("/insertdata")]
pub async fn insertdata(credential: BasicAuth, data: Json<CompanyDataInsert>) -> impl Responder {
    if credential.user_id().eq(EMAIL) && credential.password().unwrap().eq(CO_PASSWORD) {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("can'n connect with server");

        let col: Collection<Document> = client.database("companydatabase").collection("ComMONAPI");

        let userdata = doc! {"Email":data.Email.to_string(),"Company Password":data.Compass.to_string(),"Company_name":&data.Company_name,"Company_id":&data.Company_id,"Company_address":&data.Company_address,"Company_city":&data.Company_city,};
        match col.insert_one(userdata, None).await {
            Ok(v) => HttpResponse::Ok().body("new company add in register"),
            Err(e) => HttpResponse::InternalServerError().body("Email not found  this data"),
        }
    } else {
        HttpResponse::InternalServerError().body("data authenticate is faild")
    }
}

#[post("/updatedata/{email}")]
pub async fn updatedata(
    companyemail: web::Path<String>,
    credential: BasicAuth,
    data: Json<CompanyDataUpdate>,
) -> impl Responder {
    if credential.user_id().eq(EMAIL) && credential.password().unwrap().eq(CO_PASSWORD) {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("Faild to connect with server");

        let col: Collection<Document> = client.database("companydatabase").collection("ComMONAPI");

        let update = doc! {"$set":{"Company_name":&data.Company_name,"Company_id":&data.Company_id,"Company_address":&data.Company_address,"Company_city":&data.Company_city,}};
        match col
            .update_one(
                doc! {
                "Email":
                companyemail.to_string()},
                update,
                None,
            )
            .await
        {
            Ok(v) => HttpResponse::Ok().body(" Company data updated"),
            Err(e) => HttpResponse::InternalServerError().body("Email not found  this data"),
        }
    } else {
        HttpResponse::InternalServerError().body("data authenticate is faild")
    }
}


#[post("/deletedata/{email}")]
pub async fn deletedata(companyemail: web::Path<String>, credential: BasicAuth) -> impl Responder {
    if credential.user_id().eq(EMAIL) && credential.password().unwrap().eq(CO_PASSWORD) {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("Faild to connect with server");

        let col: Collection<Document> = client.database("companydatabase").collection("ComMONAPI");

        match col
            .delete_one(doc! {"Username":companyemail.to_string()}, None)
            .await
        {
            Ok(v) => HttpResponse::Ok().body("User successfully deleted"),
            Err(e) => HttpResponse::InternalServerError().body("No user found for this username"),
        }
    } else {
        HttpResponse::InternalServerError().body("Faild to authenticate request")
    }
}
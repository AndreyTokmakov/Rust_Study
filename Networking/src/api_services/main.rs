

mod example_one
{
    use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
    use serde::{Deserialize, Serialize};
    use utoipa::{OpenApi, ToSchema};
    use utoipa_swagger_ui::SwaggerUi;

    #[derive(Serialize, Deserialize, ToSchema)]
    struct User {
        id: u32,
        name: String,
    }

    #[derive(Deserialize, ToSchema)]
    struct CreateUserRequest {
        name: String,
    }

    #[utoipa::path(
        get,
        path = "/users/{id}",
        params(("id" = u32, Path, description = "User ID")),
        responses((status = 200, description = "User found", body = User), (status = 404, description = "User not found"))
    )]
    #[get("/users/{id}")]
    async fn get_user(path: web::Path<u32>) -> impl Responder {
        let id = path.into_inner();

        if id == 1 {
            HttpResponse::Ok().json(User { id, name: "John".to_string(), })
        } else {
            HttpResponse::NotFound().finish()
        }
    }

    #[utoipa::path(
        post,
        path = "/users",
        request_body = CreateUserRequest,
        responses((status = 201, description = "User created", body = User))
    )]
    #[post("/users")]
    async fn create_user(body: web::Json<CreateUserRequest>) -> impl Responder {
        HttpResponse::Created().json(User {
            id: 1,
            name: body.name.clone(),
        })
    }

    #[derive(OpenApi)]
    #[openapi(paths(get_user, create_user),
              components(schemas(User, CreateUserRequest)),
              tags((name = "users", description = "User management endpoints"))
    )]
    struct ApiDoc;

    #[actix_web::main]
    pub async fn run_service() -> std::io::Result<()>
    {
        HttpServer::new(|| { App::new()
                .service(get_user)
                .service(create_user)
                .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()), )
        }).bind(("127.0.0.1", 8888))?.run().await

        // http://localhost:8888/swagger-ui/
    }
}

pub fn test_all()
{
    let _ = example_one::run_service();
}
use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use actix_web_lab::respond::Html;
use async_graphql::{
  http::{playground_source, GraphQLPlaygroundConfig},
  EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod star_wars;
use self::star_wars::{QueryRoot, StarWars, StarWarsSchema};

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<StarWarsSchema>, req: GraphQLRequest) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

/// GraphQL playground UI
#[get("/playground")]
async fn graphql_playground() -> impl Responder {
  Html(playground_source(
    GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
  ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
    .data(StarWars::new())
    .finish();

  log::info!("starting HTTP server on port 8080");
  log::info!("GraphiQL playground: http://localhost:8080/playground");

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(schema.clone()))
      .service(graphql)
      .service(graphql_playground)
      .wrap(Cors::permissive())
      .wrap(Logger::default())
  })
  .workers(2)
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}

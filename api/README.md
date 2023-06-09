# Espetinho API: Actix Web API with Diesel and PostgreSQL

This is the repo for the API to handle the espetinho for Alcance Social

Technologies used:

- Rust
- Actix Web
- Diesel ORM
- PostgreSQL
- Docker
- Fly.io

## Adding new services

for `GET` and `DELETE` methods:

Go to the `services` directory and add a new file containing a public async function with the respective http method macro, following the pattern for other routes. If it requires arguments in the url, add `path: Path<T>` in the functions's arguments.

In the `messages` directory create a new file containing a public struct with the `Message` macro and whatever else that may need. Also add a `#[rtype(result = "QueryResult<T>")]` with the `QueryResult`'s Payload Model (checkout `payload_models.rs` to add a new one) or plain Database Model (from `db_models.rs`). If that route takes arguments, add the arguments needed as public inside the struct.

Then, in the directory `actors` create a new file and implement a new `Handler<[MESSAGE]>` for the `DbActor`. In the `handle()` function, only use `msg` if the query uses arguments. Personalize the Diesel query with whatever it is you wanna do.
Finally, add the service to the app in `main.rs` with `.service([SERVICE])`. And the new route should be functional.

If you're making a `POST` or `PATCH` query, the service instead takes a `body: Json<T>` argument which contains the body to be sent to the server, the remaining `message` and `Handler` is the same, only differing in what it does to the database.

## ENV

for the `.env` file just input the database password, the databse url and the database port. 
absolutely DO NOT remove the `.env` from `.gitignore`

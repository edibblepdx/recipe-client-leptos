## To build and run the server
```
cd recipe-server && \
mkdir db && \
sqlx database create && \
sqlx migrate run
```
You can create and run a container afterwards (requires database files).
First populate the database with
```
cd recipe-server && \
cargo run --init-from assets/static/receipes_from_around_the_world.csv
```
Then create and run the container
```
cd recipe-server
podman build -t recipe-server .
podman run -dt --name recipe-server -p 3000:3000 recipe-server:latest
```
See `recipe-server` for more details.

## To run the client
Simply run `trunk serve --open` from the root of the project.

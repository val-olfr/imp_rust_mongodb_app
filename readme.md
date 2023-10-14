## Intro

This is repository of example Rust application for operation with MongoDB and Rust MongoDB driver.

## Prerequisition:
Start local MongoDB with auth. It is possible to use next docker compose file:
```
version: '3'
services:
  mongo:
    image: 'mongo:7.0.2'
    container_name: mongo_noauth
    ports:
      - '27017:27017'
    volumes:
      - './data_noauth:/data/db'
```

Build the application with `cargo`. Run application for testing with `cargo run`.

## Example of commands:

Get all books:
```
curl http://localhost:8080/book
```

Create new book:
```
curl -X POST http://localhost:8080/book \
    -H "content-type: application/json" \
    -d '{"name": "Book01", "author": "Author01", "num_pages": 101, "tags": ["positive"]}' 
###
curl -X POST http://localhost:8080/book \
    -H "content-type: application/json" \
    -d '{"name": "Book02", "author": "Author02", "num_pages": 250, "tags": ["good", "enough"]}' 
```

Change the book:
```
curl -X PUT http://localhost:8080/book/652a21e6a73cfc294f2b6d4b \
    -H "content-type: application/json" \
    -d '{"name": "Book01_upd", "author": "Author01_upd", "num_pages": 101, "tags": ["changed"]}' 
```

Delete the book:
```
curl -X DELETE http://localhost:8080/book/652a21e6a73cfc294f2b6d4b
```


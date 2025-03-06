# Requests

## Pre-requisites
- Install [httpie](https://httpie.org/)
- Run the server: `cargo run`


* `[GET] /products` : Get list of products
```bash
http :8000/products
```
* `[POST] /products/add` : Add a product
```bash
http POST :8000/products/add name="Product 1"
```

* `[GET] /products/{id}` : Get a product by id
```bash
http :8000/products/1
```


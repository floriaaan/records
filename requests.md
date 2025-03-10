# Requests

## Pre-requisites
- Install [httpie](https://httpie.org/)
- Run the server: `cargo run`

* `[GET] /` : Get the welcome message
```bash
http :8000
```

* `[GET] /users` : Get the list of users
```bash
http :8000/users
```

* `[POST] /users/` : Add a user
```bash
http POST :8000/users/ email="florian@mail.com" password="password"
```

* `[GET] /records` : Get list of records
```bash
http :8000/records
```
* `[POST] /records/` : Add a record
```bash
http POST :8000/records/ title="Discovery" artist="Daft Punk" release_date="2001-03-12" cover_url="https://upload.wikimedia.org/wikipedia/en/a/a7/Daft_Punk_-_Discovery.jpg" discogs_url="https://www.discogs.com/fr/Daft-Punk-Discovery/master/10367" spotify_url="https://open.spotify.com/album/2noRn2Aes5aoNVsU6iWThc"
```

* `[GET] /records/{id}` : Get a record by id
```bash
http :8000/records/1
```


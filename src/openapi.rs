use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use serde_json::{json, Value};

/// Returns the OpenAPI JSON specification
#[get("/openapi.json")]
pub async fn openapi_json() -> Json<Value> {
    let openapi_spec = json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Records API",
            "description": "API for managing your record collection",
            "version": "1.0.0"
        },
        "components": {
            "securitySchemes": {
                "BearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "JWT",
                    "description": "JWT authentication token"
                }
            },
            "schemas": {
                "User": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "integer", "format": "int64" },
                        "email": { "type": "string", "format": "email" },
                        "username": { "type": "string" },
                        "created_at": { "type": "string", "format": "date-time" },
                        "updated_at": { "type": "string", "format": "date-time" }
                    }
                },
                "Jwt": {
                    "type": "object",
                    "properties": {
                        "token": { "type": "string" },
                        "expires_at": { "type": "string", "format": "date-time" }
                    }
                },
                "UserLoginInput": {
                    "type": "object",
                    "required": ["email", "password"],
                    "properties": {
                        "email": { "type": "string", "format": "email" },
                        "password": { "type": "string" }
                    }
                },
                "UserRegisterInput": {
                    "type": "object",
                    "required": ["email", "password", "username", "password_confirmation"],
                    "properties": {
                        "email": { "type": "string", "format": "email" },
                        "username": { "type": "string" },
                        "password": { "type": "string" },
                        "password_confirmation": { "type": "string" }
                    }
                },
                "UserUpdateInput": {
                    "type": "object",
                    "required": ["email", "username"],
                    "properties": {
                        "email": { "type": "string", "format": "email" },
                        "username": { "type": "string" }
                    }
                },
                "Record": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string", "format": "uuid" },
                        "title": { "type": "string" },
                        "artist": { "type": "string" },
                        "release_date": { "type": "string", "format": "date" },
                        "cover_url": { "type": "string", "format": "uri" },
                        "discogs_url": { "type": "string", "format": "uri" },
                        "spotify_url": { "type": "string", "format": "uri" },
                        "owned": { "type": "boolean" },
                        "wanted": { "type": "boolean" },
                        "created_at": { "type": "string", "format": "date-time" },
                        "updated_at": { "type": "string", "format": "date-time" },
                        "tags": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "id": { "type": "string", "format": "uuid" },
                                    "name": { "type": "string" }
                                }
                            }
                        }
                    }
                },
                "RecordInput": {
                    "type": "object",
                    "required": ["title", "artist"],
                    "properties": {
                        "title": { "type": "string" },
                        "artist": { "type": "string" },
                        "release_date": { "type": "string", "format": "date" },
                        "cover_url": { "type": "string", "format": "uri" },
                        "discogs_url": { "type": "string", "format": "uri" },
                        "spotify_url": { "type": "string", "format": "uri" },
                        "owned": { "type": "boolean" },
                        "wanted": { "type": "boolean" },
                        "tags": {
                            "type": "array",
                            "items": { "type": "string" }
                        }
                    }
                },
                "CollectionToken": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string", "format": "uuid" },
                        "token": { "type": "string" },
                        "user_id": { "type": "string", "format": "uuid" },
                        "created_at": { "type": "string", "format": "date-time" }
                    }
                }
            }
        },
        "paths": {
            "/auth/login": {
                "post": {
                    "summary": "Log in to the application",
                    "description": "Authenticates a user and returns a JWT token",
                    "tags": ["Authentication"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/UserLoginInput"
                                },
                                "example": {
                                    "email": "user@example.com",
                                    "password": "password"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Login successful",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/Jwt"
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Invalid credentials"
                        }
                    }
                }
            },
            "/auth/register": {
                "post": {
                    "summary": "Register a new user",
                    "description": "Creates a new user account and returns a JWT token",
                    "tags": ["Authentication"],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/UserRegisterInput"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Registration successful",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/Jwt"
                                    }
                                }
                            }
                        },
                        "400": {
                            "description": "Invalid input"
                        }
                    }
                }
            },
            "/auth/me": {
                "get": {
                    "summary": "Get current user",
                    "description": "Returns information about the authenticated user",
                    "tags": ["Authentication"],
                    "security": [{ "BearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "User information",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "sub": { "type": "string" },
                                            "exp": { "type": "integer" }
                                        }
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Unauthorized"
                        }
                    }
                }
            },
            "/users": {
                "get": {
                    "summary": "Get all users",
                    "description": "Returns a list of all users",
                    "tags": ["Users"],
                    "responses": {
                        "200": {
                            "description": "List of users",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/components/schemas/User"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "put": {
                    "summary": "Update user",
                    "description": "Updates the authenticated user's information",
                    "tags": ["Users"],
                    "security": [{ "BearerAuth": [] }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/UserUpdateInput"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Updated user",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/User"
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Unauthorized"
                        }
                    }
                },
                "delete": {
                    "summary": "Delete user",
                    "description": "Deletes the authenticated user's account",
                    "tags": ["Users"],
                    "security": [{ "BearerAuth": [] }],
                    "responses": {
                        "204": {
                            "description": "User deleted"
                        },
                        "401": {
                            "description": "Unauthorized"
                        }
                    }
                }
            },
            "/records": {
                "get": {
                    "summary": "Get records",
                    "description": "Returns a list of records for the authenticated user",
                    "tags": ["Records"],
                    "security": [{ "BearerAuth": [] }],
                    "parameters": [
                        {
                            "name": "owned",
                            "in": "query",
                            "description": "Filter by owned records",
                            "required": false,
                            "schema": {
                                "type": "boolean"
                            }
                        },
                        {
                            "name": "wanted",
                            "in": "query",
                            "description": "Filter by wanted records",
                            "required": false,
                            "schema": {
                                "type": "boolean"
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "List of records",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/components/schemas/Record"
                                        }
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Unauthorized"
                        }
                    }
                },
                "post": {
                    "summary": "Add records",
                    "description": "Adds one or more records to the authenticated user's collection",
                    "tags": ["Records"],
                    "security": [{ "BearerAuth": [] }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "array",
                                    "items": {
                                        "$ref": "#/components/schemas/RecordInput"
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Added records",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/components/schemas/Record"
                                        }
                                    }
                                }
                            }
                        },
                        "400": {
                            "description": "Invalid input"
                        },
                        "401": {
                            "description": "Unauthorized"
                        }
                    }
                }
            },
            "/records/search": {
                "get": {
                    "summary": "Search records",
                    "description": "Searches for records matching a query",
                    "tags": ["Records"],
                    "security": [{ "BearerAuth": [] }],
                    "parameters": [
                        {
                            "name": "query",
                            "in": "query",
                            "description": "Search query",
                            "required": true,
                            "schema": {
                                "type": "string"
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Search results",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/components/schemas/Record"
                                        }
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Unauthorized"
                        }
                    }
                }
            },
            "/records/collection/tokens": {
                "post": {
                    "summary": "Create collection token",
                    "description": "Creates a new token for sharing a user's collection",
                    "tags": ["Collections"],
                    "security": [{ "BearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Collection token",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/CollectionToken"
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Unauthorized"
                        }
                    }
                },
                "get": {
                    "summary": "List collection tokens",
                    "description": "Lists all collection tokens for the authenticated user",
                    "tags": ["Collections"],
                    "security": [{ "BearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Collection tokens",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/components/schemas/CollectionToken"
                                        }
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Unauthorized"
                        }
                    }
                }
            },
            "/records/collection/tokens/{token}": {
                "delete": {
                    "summary": "Delete collection token",
                    "description": "Deletes a collection token",
                    "tags": ["Collections"],
                    "security": [{ "BearerAuth": [] }],
                    "parameters": [
                        {
                            "name": "token",
                            "in": "path",
                            "required": true,
                            "schema": {
                                "type": "string"
                            }
                        }
                    ],
                    "responses": {
                        "204": {
                            "description": "Collection token deleted"
                        },
                        "401": {
                            "description": "Unauthorized"
                        },
                        "404": {
                            "description": "Token not found"
                        }
                    }
                }
            },
            "/records/collection/{token}": {
                "get": {
                    "summary": "Get collection by token",
                    "description": "Gets a user's collection using a shared token",
                    "tags": ["Collections"],
                    "parameters": [
                        {
                            "name": "token",
                            "in": "path",
                            "required": true,
                            "schema": {
                                "type": "string"
                            }
                        },
                        {
                            "name": "owned",
                            "in": "query",
                            "description": "Filter by owned records",
                            "required": false,
                            "schema": {
                                "type": "boolean"
                            }
                        },
                        {
                            "name": "wanted",
                            "in": "query",
                            "description": "Filter by wanted records",
                            "required": false,
                            "schema": {
                                "type": "boolean"
                            }
                        },
                        {
                            "name": "format",
                            "in": "query",
                            "description": "Response format (html or json)",
                            "required": false,
                            "schema": {
                                "type": "string",
                                "enum": ["html", "json"]
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Collection",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "$ref": "#/components/schemas/Record"
                                        }
                                    }
                                },
                                "text/html": {
                                    "schema": {
                                        "type": "string"
                                    }
                                }
                            }
                        },
                        "404": {
                            "description": "Token not found"
                        }
                    }
                }
            }
        }
    });

    Json(openapi_spec)
}

/// Returns the Swagger UI HTML page
#[get("/")]
pub async fn swagger_ui() -> RawHtml<String> {
    // Generate a simple HTML page that includes the Swagger UI library and points to our OpenAPI JSON
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Records API Documentation</title>
        <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.10.3/swagger-ui.css" />
        <style>
            html, body { margin: 0; padding: 0; height: 100%; width: 100%; }
            #swagger-ui { height: 100%; }
        </style>
    </head>
    <body>
        <div id="swagger-ui"></div>
        <script src="https://unpkg.com/swagger-ui-dist@5.10.3/swagger-ui-bundle.js"></script>
        <script>
            window.onload = function() {
                const ui = SwaggerUIBundle({
                    url: "/docs/openapi.json",
                    dom_id: '#swagger-ui',
                    deepLinking: true,
                    presets: [
                        SwaggerUIBundle.presets.apis,
                        SwaggerUIBundle.SwaggerUIStandalonePreset
                    ],
                    layout: "BaseLayout",
                    defaultModelsExpandDepth: -1,
                    docExpansion: "list",
                });
                window.ui = ui;
            };
        </script>
    </body>
    </html>
    "#;

    RawHtml(html.to_string())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![openapi_json, swagger_ui]
}

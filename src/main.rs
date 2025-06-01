use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
};

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    tareas: Vec<Tarea>,
}

#[derive(Template)]
#[template(path = "index.html", block = "tarea_frag")]
struct TareaFrag {
    tareas: Vec<Tarea>,
}

struct Tarea {
    description: String,
}

#[axum::debug_handler]
async fn index() -> impl IntoResponse {
    let template = Index {
        tareas: vec![
            Tarea {
                description: "Comprar cosas de limpieza".to_string(),
            },
            Tarea {
                description: "Otra tarea artificial".to_string(),
            },
        ],
    };

    Html(template.render().unwrap().into_response())
}

#[axum::debug_handler]
async fn nueva() -> impl IntoResponse {
    let tareas = vec![
        Tarea {
            description: "Comprar cosas de limpieza".to_string(),
        },
        Tarea {
            description: "Otra tarea artificial".to_string(),
        },
        Tarea {
            description: "Tarea de prueba".to_string(),
        },
    ];

    Html(TareaFrag { tareas }.render().unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new()
        .route("/", get(index))
        .route("/nueva", post(nueva))
    ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

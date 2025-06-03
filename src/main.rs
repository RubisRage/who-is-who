#![allow(unused_braces)]

use axum::{
    response::Html,
    routing::{get, post},
    Form,
};
use maud::html;
use maud::Markup;
use serde::Deserialize;

#[derive(Debug)]
struct Tarea {
    id: &'static str,
    description: String,
}

fn render_page(tareas: &[Tarea]) -> Markup {
    html! {
            html lang="en" {
                head {
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    script src="https://unpkg.com/htmx.org@2.0.4" integrity="sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+" crossorigin="anonymous" {}
                    script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
                    title { "Who is who" }
                }
                body {
                    h1 { "Ejemplo de uso" }
                    main {
                        section {
                            h3 { "Lista de tareas" }
                            ul id="lista" {
                                @for tarea in tareas {
                                    (render_tarea(tarea))
                                }
                            }
                        }
                        form {
                            h2 { "Crear una tarea" }
                            div {
                                label for="description" { "Descripción:" }
                                input #"description" name="description" type="text";
                            }
                            button type="submit" hx-post="/nueva" hx-target="#lista" hx-swap="beforeend" {
                                "Añadir"
                            }
                        }
                    }
                }
            }
        }
}

fn render_tarea(tarea: &Tarea) -> Markup {
    html! {
        li id={ (tarea.id) } {
            p { (tarea.description) }
        }
    }
}

#[axum::debug_handler]
async fn index() -> Html<String> {
    let tareas = vec![
        Tarea {
            id: "1",
            description: "Comprar leche".to_string(),
        },
        Tarea {
            id: "2",
            description: "Wazaaa".to_string(),
        },
        Tarea {
            id: "3",
            description: "Terminar de comer".to_string(),
        },
    ];

    Html(render_page(&tareas).into_string())
}

#[derive(Deserialize)]
struct NuevaReq {
    description: String,
}

#[axum::debug_handler]
async fn nueva(Form(req): Form<NuevaReq>) -> Html<String> {
    let tarea = Tarea {
        id: "estoy habría que generarlo y eso",
        description: req.description,
    };

    Html(render_tarea(&tarea).into_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new()
        .route("/", get(index))
        .route("/nueva", post(nueva));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

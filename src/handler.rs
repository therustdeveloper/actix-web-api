use crate::{
    model::NoteModel,
    schema::{FilterOptions, CreateNoteSchema},
    AppState,
};
use actix_web::{get, web, HttpResponse, Responder, post};
use serde_json::json;

#[get("/notes")]
pub async fn note_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    ).fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all note items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let notes = query_result.unwrap();

    let json_response = json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/notes")]
async fn create_note_handler(
    body: web::Json<CreateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title, content, category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
        .fetch_one(&data.db)
        .await;
    
    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success", "data": serde_json::json!({
                "note": note,
            })});

            HttpResponse::Ok().json(note_response)
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint") {
                return HttpResponse::BadRequest()
                    .json(json!({"status": "fail", "message": "Note with that title already exists"}));
            }

            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("{:?}", e)}))
        }
    }
}

#[get("/notes/{id}")]
async fn get_note_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let note_id = path.into_inner();
    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes WHERE id = $1",
        note_id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"state": "success", "data": json!({
                "note": note
            })});

            HttpResponse::Ok().json(note_response)
        }
        Err(_) => {
            let message = format!("Note with ID: {} not found", note_id);
            HttpResponse::NotFound()
                .json(json!({"status": "fail", "message": message}))
        }
    }
}

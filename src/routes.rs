use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    Form,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::content::{self, Area};

// ── Estado ───────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct AppState {
    /// Reservado para futuras integraciones (mail, base de datos, etc.).
    #[allow(dead_code)]
    pub consultas: Arc<Vec<String>>,
}

// ── Plantillas ───────────────────────────────────────────────────────────────

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexPage {
    pub site: &'static content::Site,
    pub title: &'static str,
    pub description: &'static str,
    pub active: &'static str,
    pub areas: &'static [Area],
    pub steps: &'static [content::Step],
    pub values: &'static [content::Value],
}

#[derive(Template)]
#[template(path = "areas.html")]
pub struct AreasPage {
    pub site: &'static content::Site,
    pub title: &'static str,
    pub description: &'static str,
    pub active: &'static str,
    pub areas: &'static [Area],
}

#[derive(Template)]
#[template(path = "estudio.html")]
pub struct EstudioPage {
    pub site: &'static content::Site,
    pub title: &'static str,
    pub description: &'static str,
    pub active: &'static str,
    pub values: &'static [content::Value],
    pub team: &'static [content::Member],
    pub faq: &'static [content::Faq],
}

#[derive(Template)]
#[template(path = "contacto.html")]
pub struct ContactoPage {
    pub site: &'static content::Site,
    pub title: &'static str,
    pub description: &'static str,
    pub active: &'static str,
    pub areas: &'static [Area],
    pub sent: bool,
    pub errors: Vec<String>,
    pub form: ContactForm,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundPage {
    pub site: &'static content::Site,
    pub title: &'static str,
    pub description: &'static str,
    pub active: &'static str,
}

// ── Formulario ───────────────────────────────────────────────────────────────

#[derive(Debug, Default, Deserialize)]
pub struct ContactForm {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub area: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub consent: Option<String>,
    /// Honeypot anti-spam: debe quedar vacío.
    #[serde(default)]
    pub website: String,
}

impl ContactForm {
    pub fn consent_checked(&self) -> bool {
        self.consent.as_deref() == Some("si")
    }

    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.name.trim().chars().count() < 2 {
            errors.push("Ingresá tu nombre y apellido.".into());
        }
        let email = self.email.trim();
        if !email.contains('@') || !email.split('@').nth(1).is_some_and(|d| d.contains('.')) {
            errors.push("Ingresá un correo electrónico válido.".into());
        }
        if self.message.trim().chars().count() < 10 {
            errors.push("Contanos un poco más sobre tu caso (mínimo 10 caracteres).".into());
        }
        if self.consent.as_deref() != Some("si") {
            errors.push("Necesitamos tu autorización para responder la consulta.".into());
        }

        errors
    }
}

#[derive(Debug, Deserialize)]
pub struct ContactoQuery {
    #[serde(default)]
    pub enviado: Option<String>,
    #[serde(default)]
    pub area: Option<String>,
}

// ── Handlers ─────────────────────────────────────────────────────────────────

fn render<T: Template>(page: T) -> Response {
    match page.render() {
        Ok(body) => {
            let mut response = Html(body).into_response();
            // Evita que el navegador muestre una versión vieja del HTML.
            response.headers_mut().insert(
                axum::http::header::CACHE_CONTROL,
                axum::http::HeaderValue::from_static("no-store, must-revalidate"),
            );
            response
        }
        Err(err) => {
            eprintln!("error al renderizar plantilla: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No pudimos generar la página.",
            )
                .into_response()
        }
    }
}

pub async fn index() -> Response {
    render(IndexPage {
        site: &content::SITE,
        title: "Abogados en Córdoba Capital",
        description: "Estudio jurídico en Córdoba Capital. Asesoramiento en derecho civil, laboral, familia, sucesiones, comercial y administrativo, con atención directa y honorarios claros.",
        active: "inicio",
        areas: content::AREAS,
        steps: content::STEPS,
        values: content::VALUES,
    })
}

pub async fn areas() -> Response {
    render(AreasPage {
        site: &content::SITE,
        title: "Áreas de práctica",
        description: "Civil, laboral, familia, sucesiones, comercial y administrativo. Conocé cómo trabaja el Estudio Robert en cada materia.",
        active: "areas",
        areas: content::AREAS,
    })
}

pub async fn estudio() -> Response {
    render(EstudioPage {
        site: &content::SITE,
        title: "El estudio y el equipo",
        description: "Un estudio jurídico pequeño y riguroso en Córdoba Capital. Conocé nuestro método de trabajo, valores y equipo.",
        active: "estudio",
        values: content::VALUES,
        team: content::TEAM,
        faq: content::FAQ,
    })
}

pub async fn contacto_get(Query(query): Query<ContactoQuery>) -> Response {
    let area = query
        .area
        .filter(|slug| content::AREAS.iter().any(|a| a.slug == slug))
        .unwrap_or_default();

    render(ContactoPage {
        site: &content::SITE,
        title: "Contacto",
        description: "Escribinos o coordiná una consulta con el Estudio Robert en Córdoba Capital. Respuesta en el día hábil.",
        active: "contacto",
        areas: content::AREAS,
        sent: query.enviado.as_deref() == Some("1"),
        errors: Vec::new(),
        form: ContactForm { area, ..Default::default() },
    })
}

pub async fn contacto_post(
    State(_state): State<AppState>,
    Form(form): Form<ContactForm>,
) -> Response {
    // Honeypot: si viene completo, simulamos éxito sin procesar.
    if !form.website.trim().is_empty() {
        return Redirect::to("/contacto?enviado=1").into_response();
    }

    let errors = form.validate();
    if !errors.is_empty() {
        return render(ContactoPage {
            site: &content::SITE,
            title: "Contacto",
            description: "Escribinos o coordiná una consulta con el Estudio Robert en Córdoba Capital.",
            active: "contacto",
            areas: content::AREAS,
            sent: false,
            errors,
            form,
        });
    }

    // Punto de integración: acá iría el envío de mail o el guardado en base.
    // No registramos datos personales en logs.
    println!(
        "nueva consulta recibida | area={} | mensaje={} caracteres",
        if form.area.is_empty() { "sin especificar" } else { &form.area },
        form.message.trim().chars().count()
    );

    Redirect::to("/contacto?enviado=1").into_response()
}

pub async fn not_found() -> Response {
    let mut response = render(NotFoundPage {
        site: &content::SITE,
        title: "Página no encontrada",
        description: "La página que buscás no existe.",
        active: "",
    });
    *response.status_mut() = StatusCode::NOT_FOUND;
    response
}

pub async fn health() -> &'static str {
    "ok"
}

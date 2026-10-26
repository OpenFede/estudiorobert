# Estudio Robert — sitio web

Sitio institucional para un estudio jurídico de Córdoba Capital, hecho en Rust con `axum`, `tokio` y plantillas `askama`. Estilos con Tailwind CSS vía CDN (sin Node.js).

## Correr

```bash
cargo run
# http://localhost:3000
```

El puerto se puede cambiar con `PORT=8080 cargo run`. Ejecutá desde la raíz del proyecto para que `/static` se resuelva correctamente.

Producción:

```bash
cargo build --release
PORT=3000 ./target/release/estudio-robert
```

## Estructura

```
Cargo.toml          dependencias mínimas
src/main.rs         router y arranque del servidor
src/routes.rs       handlers, plantillas y validación del formulario
src/content.rs      TODO el contenido editable (textos, áreas, equipo, FAQ, contacto)
templates/base.html layout: header, footer, WhatsApp, estilos globales
templates/*.html    inicio, áreas, estudio, contacto y 404
static/favicon.svg  isotipo
static/app.js      revelado al scroll, barra de progreso y header
```

## Páginas

| Ruta | Contenido |
|---|---|
| `/` | Hero, áreas destacadas, método de trabajo, valores y CTA |
| `/areas` | Seis áreas con detalle, índice con anclas y CTA por área |
| `/estudio` | Enfoque, valores, equipo y preguntas frecuentes |
| `/contacto` | Formulario validado, datos de contacto y checklist previo |
| `/salud` | Healthcheck (`ok`) |

## Editar contenido

Casi todo el texto vive en `src/content.rs`: navegación, áreas, pasos, valores, equipo, FAQ y datos de contacto.

**Importante:** los datos de contacto y el equipo son *placeholders*. Antes de publicar, reemplazá en `src/content.rs`:

- dirección, teléfono, WhatsApp, email y horario;
- nombres, cargos y matrículas del equipo;
- URL de Google Maps;
- año del footer (`YEAR`).

## Formulario

`POST /contacto` valida nombre, email, mensaje y consentimiento (Ley 25.326), incluye un honeypot anti-spam y usa el patrón PRG (`303` a `/contacto?enviado=1`). No persiste datos personales: el punto de integración para enviar un mail o guardar en base está marcado en `src/routes.rs` (`contacto_post`).

## Animaciones

El sitio usa animaciones CSS suaves y revelado al hacer scroll con `IntersectionObserver` (`static/app.js`). Todo se desactiva con `prefers-reduced-motion` y hay fallback sin JavaScript.

## Tailwind

Se usa el CDN oficial para desarrollo y despliegues simples. Para producción con CSS optimizado, generá una hoja con el CLI standalone:

```bash
tailwindcss -i ./static/input.css -o ./static/app.css --minify
```

y reemplazá el `<script src="https://cdn.tailwindcss.com">` por `<link rel="stylesheet" href="/static/app.css">` en `templates/base.html`.

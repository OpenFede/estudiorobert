//! Contenido editable del sitio. Cambiá textos, datos de contacto y equipo acá.

pub const SITE_NAME: &str = "Estudio Robert";
pub const YEAR: &str = "2026";

// ── Datos de contacto (PLACEHOLDER: reemplazar por los reales) ──────────────
pub const ADDRESS: &str = "Av. Hipólito Yrigoyen 240, Piso 4";
pub const CITY: &str = "Córdoba Capital, Argentina";
pub const PHONE_DISPLAY: &str = "+54 351 555 0140";
pub const PHONE_TEL: &str = "+543515550140";
pub const WHATSAPP: &str = "5493515550140";
pub const EMAIL: &str = "consultas@estudiorobert.com.ar";
pub const HOURS: &str = "Lunes a viernes, 9:00 a 18:00";
pub const MAPS_URL: &str =
    "https://www.google.com/maps/search/?api=1&query=Av.+Hip%C3%B3lito+Yrigoyen+240,+C%C3%B3rdoba";

pub struct NavItem {
    pub label: &'static str,
    pub href: &'static str,
    pub key: &'static str,
}

pub const NAV: &[NavItem] = &[
    NavItem { label: "Inicio", href: "/", key: "inicio" },
    NavItem { label: "Áreas", href: "/areas", key: "areas" },
    NavItem { label: "Estudio", href: "/estudio", key: "estudio" },
    NavItem { label: "Contacto", href: "/contacto", key: "contacto" },
];

/// Datos comunes que consume el layout base.
pub struct Site {
    pub nav: &'static [NavItem],
    pub address: &'static str,
    pub city: &'static str,
    pub phone_display: &'static str,
    pub phone_tel: &'static str,
    pub whatsapp: &'static str,
    pub email: &'static str,
    pub hours: &'static str,
    pub maps: &'static str,
    pub year: &'static str,
}

pub const SITE: Site = Site {
    nav: NAV,
    address: ADDRESS,
    city: CITY,
    phone_display: PHONE_DISPLAY,
    phone_tel: PHONE_TEL,
    whatsapp: WHATSAPP,
    email: EMAIL,
    hours: HOURS,
    maps: MAPS_URL,
    year: YEAR,
};

pub struct Area {
    pub slug: &'static str,
    pub title: &'static str,
    pub short: &'static str,
    pub description: &'static str,
    pub items: &'static [&'static str],
    /// Trazo SVG 24x24 (stroke, sin fill).
    pub icon: &'static str,
}

pub const AREAS: &[Area] = &[
    Area {
        slug: "civil",
        title: "Civil y contractual",
        short: "Contratos, daños y conflictos patrimoniales resueltos con criterio técnico y mirada práctica.",
        description:
            "Redactamos y revisamos contratos, asistimos en negociaciones y litigamos cuando la vía judicial es la mejor alternativa. Trabajamos para que cada decisión tenga respaldo documental y una estrategia clara.",
        items: &[
            "Contratos civiles y comerciales",
            "Responsabilidad civil y daños",
            "Propiedad horizontal y consorcios",
            "Defensa del consumidor",
            "Mediaciones y acuerdos",
        ],
        icon: "M9 12h6m-6 4h6M9 8h6M6 3h9l3 3v15H6z",
    },
    Area {
        slug: "laboral",
        title: "Laboral",
        short: "Defensa de trabajadores y asesoramiento a empleadores con foco en acuerdos sostenibles.",
        description:
            "Analizamos el caso, estimamos el riesgo real y proponemos un camino: acuerdo, mediación o juicio. También acompañamos a empresas en el diseño de políticas laborales y en la prevención de conflictos.",
        items: &[
            "Despidos y indemnizaciones",
            "Accidentes de trabajo y ART",
            "Diferencias salariales",
            "Negociación y mediación",
            "Asesoramiento a empleadores",
        ],
        icon: "M4 7h16M9 7V5h6v2m-9 0v13h12V7M10 11h4m-4 4h4",
    },
    Area {
        slug: "familia",
        title: "Familia",
        short: "Decisiones difíciles tratadas con reserva, empatía y una estrategia que cuida los vínculos.",
        description:
            "Acompañamos divorcios, acuerdos de convivencia y conflictos de cuidado personal. Priorizamos las soluciones consensuadas y, cuando no es posible, litigamos con firmeza y discreción.",
        items: &[
            "Divorcios y separaciones",
            "Cuidado personal y régimen comunicacional",
            "Cuotas alimentarias",
            "Uniones convivenciales",
            "Violencia familiar y medidas urgentes",
        ],
        icon: "M12 20s-7-4.35-7-9a4 4 0 017-2.65A4 4 0 0119 11c0 4.65-7 9-7 9z",
    },
    Area {
        slug: "sucesiones",
        title: "Sucesiones",
        short: "Ordenamos el patrimonio familiar con tiempos claros y documentación impecable.",
        description:
            "Iniciamos y llevamos adelante la sucesión de principio a fin, incluida la partición y la inscripción de bienes. También asesoramos en planificación sucesoria y testamentos.",
        items: &[
            "Declaratoria de herederos",
            "Partición e inscripción de bienes",
            "Testamentos",
            "Planificación patrimonial",
            "Sucesiones con conflicto entre herederos",
        ],
        icon: "M12 3v18M5 7h14M7 7v5a5 5 0 0010 0V7M5 21h14",
    },
    Area {
        slug: "comercial",
        title: "Comercial y societario",
        short: "Estructura, contratos y cobros para que la empresa pueda ocuparse de crecer.",
        description:
            "Constituimos sociedades, redactamos contratos y diseñamos mecanismos de cobro. Cuando la deuda se vuelve un problema estructural, evaluamos concursos y acuerdos preventivos.",
        items: &[
            "Constitución de sociedades",
            "Contratos comerciales",
            "Recupero de créditos",
            "Concursos y acuerdos preventivos",
            "Gobierno societario y actas",
        ],
        icon: "M4 20h16M6 20V9l6-4 6 4v11M10 20v-4h4v4",
    },
    Area {
        slug: "administrativo",
        title: "Administrativo y amparos",
        short: "Defensa frente al Estado: habilitaciones, multas, licitaciones y derechos vulnerados.",
        description:
            "Intervenimos en procedimientos administrativos y en amparos por salud, obra social y derechos constitucionales. Buscamos respuestas rápidas cuando el tiempo es determinante.",
        items: &[
            "Amparos de salud y obra social",
            "Habilitaciones comerciales",
            "Multas y clausuras",
            "Contrataciones y licitaciones",
            "Recursos administrativos",
        ],
        icon: "M12 3l8 4v5c0 5-3.5 8-8 9-4.5-1-8-4-8-9V7z",
    },
];

pub struct Step {
    pub n: &'static str,
    pub title: &'static str,
    pub text: &'static str,
}

pub const STEPS: &[Step] = &[
    Step {
        n: "01",
        title: "Consulta inicial",
        text: "Escuchamos el caso, revisamos la documentación y evaluamos la viabilidad real antes de proponer cualquier acción.",
    },
    Step {
        n: "02",
        title: "Estrategia y propuesta",
        text: "Recibís un plan con alternativas, plazos estimados y honorarios por escrito. Sin sorpresas.",
    },
    Step {
        n: "03",
        title: "Ejecución y seguimiento",
        text: "Llevamos el expediente adelante y te informamos cada avance relevante en lenguaje claro.",
    },
    Step {
        n: "04",
        title: "Cierre y resultados",
        text: "Documentamos el resultado, explicamos efectos y dejamos el archivo ordenado para el futuro.",
    },
];

pub struct Value {
    pub title: &'static str,
    pub text: &'static str,
}

pub const VALUES: &[Value] = &[
    Value {
        title: "Claridad",
        text: "Explicamos qué se puede, qué no y cuánto cuesta. La incertidumbre no debería ser parte del proceso.",
    },
    Value {
        title: "Confidencialidad",
        text: "Cada legajo se trata con reserva estricta, protocolos de acceso y un único canal de contacto.",
    },
    Value {
        title: "Rigor técnico",
        text: "Cada presentación se revisa dos veces. La prolijidad documental también es una forma de defender.",
    },
    Value {
        title: "Compromiso",
        text: "Respondemos consultas en el día y mantenemos un interlocutor fijo durante todo el caso.",
    },
];

pub struct Member {
    pub initials: &'static str,
    pub name: &'static str,
    pub role: &'static str,
    pub focus: &'static str,
    pub bio: &'static str,
}

/// PLACEHOLDER: nombres, cargos y matrículas de ejemplo. Reemplazar por el equipo real.
pub const TEAM: &[Member] = &[
    Member {
        initials: "MR",
        name: "Dr. Martín Robert",
        role: "Socio director",
        focus: "Civil · Comercial · Litigios",
        bio: "Dirige el estudio y coordina las causas de mayor complejidad patrimonial. Especializado en contratos y responsabilidad civil.",
    },
    Member {
        initials: "LF",
        name: "Dra. Lucía Ferrero",
        role: "Socia",
        focus: "Laboral · Familia",
        bio: "Acompaña procesos laborales y familiares con enfoque en mediación y acuerdos sostenibles.",
    },
    Member {
        initials: "IP",
        name: "Dr. Ignacio Ponce",
        role: "Asociado",
        focus: "Sucesiones · Administrativo",
        bio: "Lleva sucesiones y trámites administrativos, con seguimiento detallado de plazos y registros.",
    },
];

pub struct Faq {
    pub q: &'static str,
    pub a: &'static str,
}

pub const FAQ: &[Faq] = &[
    Faq {
        q: "¿La primera consulta tiene costo?",
        a: "La primera consulta de evaluación tiene un arancel fijo que se informa antes de coordinar. Si el caso se acepta, ese importe se descuenta de los honorarios pactados.",
    },
    Faq {
        q: "¿Cómo se determinan los honorarios?",
        a: "Dependen del tipo de caso, la complejidad y la vía elegida. Siempre entregamos una propuesta por escrito con el alcance y la forma de pago antes de comenzar.",
    },
    Faq {
        q: "¿Atienden de forma remota?",
        a: "Sí. Coordinamos videollamadas y recibimos documentación digital. La firma de instrumentos se realiza en el estudio o donde el caso lo requiera.",
    },
    Faq {
        q: "¿Cuánto demora un juicio?",
        a: "Los tiempos dependen del fuero y de la complejidad. En la propuesta inicial incluimos un rango realista y te avisamos si el expediente se demora por causas ajenas al estudio.",
    },
    Faq {
        q: "¿Qué documentación debo llevar a la primera reunión?",
        a: "Todo lo que tengas por escrito: contratos, correos, mensajes, recibos, notificaciones y datos de las partes. Con eso podemos evaluar el caso con precisión.",
    },
];

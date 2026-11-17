#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Language {
    En,
    PtBr
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::PtBr => "pt-BR"
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "pt-BR" => Self::PtBr,
            _ => Self::En
        }
    }
}

#[derive(Clone, Copy)]
pub enum Text {
    NavbarTitle,
    NavbarSubtitle,
    Home,
    Methodology,
    Results,
    Implementation,
    About,
    HomeTitle,
    HomeDescription,
    MethodologyDescription,
    ResultsDescription,
    ImplementationDescription,
    AboutDescription,
    ThemeToggle
}

pub fn translate(lang: Language, key: Text) -> &'static str {
    use Text::*;

    match (lang, key) {
        (Language::En, NavbarTitle) => "GPU Path Tracing",
        (Language::PtBr, NavbarTitle) => "GPU Path Tracing",

        (Language::En, NavbarSubtitle) => "Graduation Project",
        (Language::PtBr, NavbarSubtitle) => "Trabalho de Conclusão de Curso",

        (Language::En, Home) => "Home",
        (Language::PtBr, Home) => "Início",

        (Language::En, Methodology) => "Methodology",
        (Language::PtBr, Methodology) => "Metodologia",

        (Language::En, Results) => "Results",
        (Language::PtBr, Results) => "Resultados",

        (Language::En, Implementation) => "Implementation",
        (Language::PtBr, Implementation) => "Implementação",

        (Language::En, About) => "About",
        (Language::PtBr, About) => "Sobre",

        (Language::En, HomeTitle) => "GPU Path Tracing",
        (Language::PtBr, HomeTitle) => "Path Tracing em GPU",

        (Language::En, HomeDescription) =>
            "Path Tracing: Ray Tracing Performance Analysis on Dedicated Hardware.",
        (Language::PtBr, HomeDescription) =>
            "Path Tracing: Análise de Desempenho em Ray Tracing Acelerado por Hardware Dedicado.",

        (Language::En, MethodologyDescription) =>
            "Comparison of rendering pipelines and optimizations.",
        (Language::PtBr, MethodologyDescription) =>
            "Comparação de pipelines de renderização e otimizações.",

        (Language::En, ResultsDescription) =>
            "Performance benchmarks and comparisons.",
        (Language::PtBr, ResultsDescription) =>
            "Testes de desempenho e comparações.",

        (Language::En, ImplementationDescription) =>
            "Technical details of the Vulkan path tracer.",
        (Language::PtBr, ImplementationDescription) =>
            "Detalhes técnicos do path tracer em Vulkan.",

        (Language::En, AboutDescription) =>
            "Computer Engineering graduation project.",
        (Language::PtBr, AboutDescription) =>
            "Trabalho de conclusão de curso em Engenharia de Computação.",

        (Language::En, ThemeToggle) => "Toggle theme",
        (Language::PtBr, ThemeToggle) => "Alternar tema",
    }
}

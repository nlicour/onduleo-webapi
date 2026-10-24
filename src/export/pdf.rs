use std::path::PathBuf;

use chrono::Datelike;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, World};
use typst_pdf::PdfOptions;

use crate::models::{Categorie, Discipline, Exercice, Niveau};

// ============ Helpers texte (enum → string minuscule pour le template) ============

fn discipline_str(d: &Discipline) -> &'static str {
    match d {
        Discipline::Crawl => "crawl",
        Discipline::Dos => "dos",
        Discipline::Brasse => "brasse",
        Discipline::Papillon => "papillon",
        Discipline::QuatreNages => "quatreNages",
    }
}

fn categorie_str(c: &Categorie) -> &'static str {
    match c {
        Categorie::Technique => "technique",
        Categorie::Endurance => "endurance",
        Categorie::Vitesse => "vitesse",
        Categorie::Recuperation => "recuperation",
        Categorie::Echaufement => "echaufement",
    }
}

fn niveau_str(n: &Niveau) -> &'static str {
    match n {
        Niveau::Debutant => "debutant",
        Niveau::Intermediaire => "intermediaire",
        Niveau::Avance => "avance",
        Niveau::Expert => "expert",
    }
}

fn equipement_list(ex: &Exercice) -> Vec<&'static str> {
    let mut items = Vec::new();
    if ex.bipalme.unwrap_or(false) {
        items.push("Bipalme");
    }
    if ex.monopalme.unwrap_or(false) {
        items.push("Monopalme");
    }
    if ex.planche.unwrap_or(false) {
        items.push("Planche");
    }
    if ex.tuba.unwrap_or(false) {
        items.push("Tuba");
    }
    if ex.chronometre.unwrap_or(false) {
        items.push("Chronomètre");
    }
    items
}

// ============ Échappement string Typst ============

fn escape_typst(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "")
}

// ============ Génération du code Typst (données + template) ============

fn exercice_to_typst(ex: &Exercice) -> String {
    let equip = equipement_list(ex);
    let equip_str = if equip.is_empty() {
        String::new()
    } else {
        equip
            .iter()
            .map(|e| format!("\"{}\"", escape_typst(e)))
            .collect::<Vec<_>>()
            .join(", ")
    };

    format!(
        r#"  (
    code: "{}",
    nom: "{}",
    discipline: "{}",
    categorie: "{}",
    niveau: "{}",
    objectif: "{}",
    description: "{}",
    consigneCle: "{}",
    indicateurSucces: "{}",
    erreurTypiqueCorrigee: "{}",
    equipement: ({})
  )"#,
        escape_typst(&ex.code),
        escape_typst(&ex.nom),
        discipline_str(&ex.discipline),
        categorie_str(&ex.categorie),
        niveau_str(&ex.niveau),
        escape_typst(&ex.objectif),
        escape_typst(&ex.description),
        escape_typst(&ex.consigne_cle),
        escape_typst(&ex.indicateur_succes),
        escape_typst(&ex.erreur_typique_corrigee),
        equip_str,
    )
}

fn build_typst_source(exercices: &[Exercice], titre: &str) -> String {
    let template = include_str!("../../templates/fiche.typ");
    let date = chrono::Local::now().format("%d/%m/%Y").to_string();

    let mut data = String::new();
    data.push_str(&format!("#let titre = \"{}\"\n", escape_typst(titre)));
    data.push_str(&format!("#let date = \"{}\"\n", date));
    data.push_str("#let exercices = (\n");
    for ex in exercices {
        data.push_str(&exercice_to_typst(ex));
        data.push_str(",\n");
    }
    data.push_str(")\n\n");

    format!("{}{}", data, template)
}

// ============ Chargement des polices système ============

const FONT_PATHS: &[&str] = &[
    // macOS
    "/System/Library/Fonts/Supplemental/Arial.ttf",
    "/System/Library/Fonts/Helvetica.ttc",
    "/Library/Fonts/Arial.ttf",
    // Linux
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    "/usr/share/fonts/TTF/DejaVuSans.ttf",
    "/usr/share/fonts/truetype/freefont/FreeSans.ttf",
    // Windows
    "C:\\Windows\\Fonts\\arial.ttf",
];

fn load_system_fonts() -> Vec<Font> {
    let mut fonts = Vec::new();
    for path in FONT_PATHS {
        if let Ok(bytes) = std::fs::read(path) {
            for index in 0..2u32 {
                if let Some(font) = Font::new(Bytes::new(bytes.clone()), index) {
                    fonts.push(font);
                    break;
                }
            }
        }
    }
    fonts
}

// ============ World Typst ============

struct TypstWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    main: FileId,
    source: Source,
    fonts: Vec<Font>,
}

impl TypstWorld {
    fn new(source_code: String) -> Self {
        let fonts = load_system_fonts();
        let library = LazyHash::new(Library::default());
        let book = LazyHash::new(FontBook::from_fonts(&fonts));
        let main = FileId::new(None, VirtualPath::new("main.typ"));
        let source = Source::new(main, source_code);

        Self {
            library,
            book,
            main,
            source,
            fonts,
        }
    }
}

impl World for TypstWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(PathBuf::from(
                id.vpath().as_rootless_path(),
            )))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(PathBuf::from(
            id.vpath().as_rootless_path(),
        )))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        let now = chrono::Local::now();
        Datetime::from_ymd(now.year(), now.month() as u8, now.day() as u8)
    }
}

// ============ Génération PDF ============

pub fn generate_pdf(exercices: &[Exercice], titre: &str) -> Result<Vec<u8>, String> {
    let source_code = build_typst_source(exercices, titre);
    let world = TypstWorld::new(source_code);

    // Compiler le template Typst en document
    let warned = typst::compile(&world);
    let document = warned.output.map_err(|errors| {
        let msgs: Vec<String> = errors.iter().map(|e| format!("{:?}", e)).collect();
        format!("Erreur compilation Typst: {}", msgs.join("; "))
    })?;

    // Exporter en PDF
    let pdf_bytes = typst_pdf::pdf(&document, &PdfOptions::default()).map_err(|errors| {
        let msgs: Vec<String> = errors.iter().map(|e| format!("{:?}", e)).collect();
        format!("Erreur génération PDF: {}", msgs.join("; "))
    })?;

    Ok(pdf_bytes)
}

// ============ Tests ============

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seed::mock_exercices;

    #[test]
    fn test_generate_pdf_all_exercices() {
        let exercices = mock_exercices();
        eprintln!("\n[test] {} exercices dans le seed", exercices.len());

        let result = generate_pdf(&exercices, "Fiche exercices — Tous (test)");
        assert!(
            result.is_ok(),
            "Génération PDF échouée: {:?}",
            result.err()
        );

        let bytes = result.unwrap();
        eprintln!("[test] PDF généré: {} bytes", bytes.len());

        assert!(bytes.len() > 1000, "PDF trop petit: {} bytes", bytes.len());
        assert_eq!(&bytes[0..4], b"%PDF", "Magic bytes PDF invalides");

        let _ = std::fs::write("./tmp/test-typst-all.pdf", &bytes);
        eprintln!("[test] PDF sauvegardé: ./tmp/test-typst-all.pdf");
    }

    #[test]
    fn test_generate_pdf_single_exercice() {
        let exercices = mock_exercices();
        let single = &exercices[0..1];
        eprintln!("\n[test] 1 exercice: {}", single[0].code);

        let result = generate_pdf(single, "Fiche exercice individuel (test)");
        assert!(result.is_ok(), "Génération PDF échouée: {:?}", result.err());

        let bytes = result.unwrap();
        eprintln!("[test] PDF généré: {} bytes", bytes.len());
        assert_eq!(&bytes[0..4], b"%PDF", "Magic bytes PDF invalides");

        let _ = std::fs::write("/tmp/test-typst-one.pdf", &bytes);
    }

    #[test]
    fn test_generate_pdf_empty() {
        let result = generate_pdf(&[], "Document vide (test)");
        assert!(result.is_ok(), "Génération PDF échouée: {:?}", result.err());

        let bytes = result.unwrap();
        eprintln!("\n[test] PDF vide: {} bytes", bytes.len());
        assert_eq!(&bytes[0..4], b"%PDF", "Magic bytes PDF invalides");

        let _ = std::fs::write("/tmp/test-typst-empty.pdf", &bytes);
    }

    #[test]
    fn test_escape_typst_special_chars() {
        assert_eq!(escape_typst("hello"), "hello");
        assert_eq!(escape_typst("il dit \"bonjour\""), "il dit \\\"bonjour\\\"");
        assert_eq!(escape_typst("back\\slash"), "back\\\\slash");
        assert_eq!(escape_typst("ligne1\nligne2"), "ligne1\\nligne2");
    }

    #[test]
    fn test_exercice_to_typst_format() {
        let exercices = mock_exercices();
        let code = exercice_to_typst(&exercices[0]);
        assert!(code.contains("code:"));
        assert!(code.contains("nom:"));
        assert!(code.contains("discipline:"));
        assert!(code.contains("niveau:"));
        assert!(code.contains("equipement:"));
    }
}

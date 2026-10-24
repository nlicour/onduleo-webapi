use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Discipline {
    Crawl,
    Dos,
    Brasse,
    Papillon,
    QuatreNages,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Categorie {
    Technique,
    Endurance,
    Vitesse,
    Recuperation,
    Echaufement,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Niveau {
    Debutant,
    Intermediaire,
    Avance,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exercice {
    pub code: String,
    pub nom: String,
    pub discipline: Discipline,
    pub categorie: Categorie,
    pub niveau: Niveau,
    pub objectif: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bipalme: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monopalme: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planche: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuba: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chronometre: Option<bool>,
    pub consigne_cle: String,
    pub indicateur_succes: String,
    pub erreur_typique_corrigee: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ressources: Option<Vec<String>>,
}

/// Payload for creating a new Exercice (same fields, used for POST)
pub type ExerciceCreate = Exercice;

/// Payload for updating an Exercice — all fields optional
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExerciceUpdate {
    pub nom: Option<String>,
    pub discipline: Option<Discipline>,
    pub categorie: Option<Categorie>,
    pub niveau: Option<Niveau>,
    pub objectif: Option<String>,
    pub description: Option<String>,
    pub bipalme: Option<bool>,
    pub monopalme: Option<bool>,
    pub planche: Option<bool>,
    pub tuba: Option<bool>,
    pub chronometre: Option<bool>,
    pub consigne_cle: Option<String>,
    pub indicateur_succes: Option<String>,
    pub erreur_typique_corrigee: Option<String>,
    pub ressources: Option<Vec<String>>,
}

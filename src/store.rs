use std::collections::HashMap;
use std::sync::RwLock;

use crate::models::{Exercice, ExerciceUpdate};

#[derive(Default)]
pub struct ExerciceStore {
    data: RwLock<HashMap<String, Exercice>>,
}

impl ExerciceStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list(&self) -> Vec<Exercice> {
        self.data.read().unwrap().values().cloned().collect()
    }

    pub fn get(&self, code: &str) -> Option<Exercice> {
        self.data.read().unwrap().get(code).cloned()
    }

    pub fn create(&self, exercice: Exercice) -> Result<Exercice, String> {
        let mut data = self.data.write().unwrap();
        if data.contains_key(&exercice.code) {
            return Err(format!("Exercice with code '{}' already exists", exercice.code));
        }
        data.insert(exercice.code.clone(), exercice.clone());
        Ok(exercice)
    }

    pub fn update(&self, code: &str, update: ExerciceUpdate) -> Option<Exercice> {
        let mut data = self.data.write().unwrap();
        let exercice = data.get_mut(code)?;

        if let Some(v) = update.nom { exercice.nom = v; }
        if let Some(v) = update.discipline { exercice.discipline = v; }
        if let Some(v) = update.categorie { exercice.categorie = v; }
        if let Some(v) = update.niveau { exercice.niveau = v; }
        if let Some(v) = update.objectif { exercice.objectif = v; }
        if let Some(v) = update.description { exercice.description = v; }
        if update.bipalme.is_some() { exercice.bipalme = update.bipalme; }
        if update.monopalme.is_some() { exercice.monopalme = update.monopalme; }
        if update.planche.is_some() { exercice.planche = update.planche; }
        if update.tuba.is_some() { exercice.tuba = update.tuba; }
        if update.chronometre.is_some() { exercice.chronometre = update.chronometre; }
        if let Some(v) = update.consigne_cle { exercice.consigne_cle = v; }
        if let Some(v) = update.indicateur_succes { exercice.indicateur_succes = v; }
        if let Some(v) = update.erreur_typique_corrigee { exercice.erreur_typique_corrigee = v; }
        if update.ressources.is_some() { exercice.ressources = update.ressources; }

        Some(exercice.clone())
    }

    pub fn delete(&self, code: &str) -> Option<Exercice> {
        self.data.write().unwrap().remove(code)
    }

    /// Insert mock/seed data. Existing codes are skipped.
    pub fn seed(&self, exercices: Vec<Exercice>) -> usize {
        let mut data = self.data.write().unwrap();
        let mut inserted = 0;
        for exercice in exercices {
            if !data.contains_key(&exercice.code) {
                data.insert(exercice.code.clone(), exercice);
                inserted += 1;
            }
        }
        inserted
    }
}

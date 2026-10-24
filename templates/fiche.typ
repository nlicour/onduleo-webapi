// ============================================================
//  Template Typst — Fiche d'exercices Onduleo
//  Modifiable sans recompiler l'application.
//
//  Variables attendues (injectées par le Rust) :
//    #let titre     = "..."
//    #let date      = "..."
//    #let exercices = ( (code: "...", nom: "...", ...), ... )
// ============================================================

#set page(
  paper: "a4",
  margin: (x: 20mm, top: 15mm, bottom: 15mm),
)

#set text(
  font: ("Arial", "DejaVu Sans", "Libertinus Serif"),
  size: 10pt,
  fill: rgb("#1a1a2e"),
  lang: "fr",
)

// ============ Helpers ============

#let couleur-niveau(niveau) = {
  if niveau == "debutant"      { rgb("#d8f3dc") }
  else if niveau == "intermediaire" { rgb("#fff3b0") }
  else if niveau == "avance"    { rgb("#ffd6a5") }
  else                          { rgb("#ffadad") }
}

#let couleur-niveau-texte(niveau) = {
  if niveau == "debutant"      { rgb("#2d6a4f") }
  else if niveau == "intermediaire" { rgb("#9c6500") }
  else if niveau == "avance"    { rgb("#bc4b09") }
  else                          { rgb("#a4133c") }
}

#let badge(texte, bg, fg) = {
  box(
    fill: bg,
    inset: (x: 6pt, y: 3pt),
    radius: 7pt,
    text(size: 7pt, fill: fg, weight: "bold")[#texte],
  )
}

#let section-titre(label) = {
  v(6pt)
  text(size: 7.5pt, weight: "bold", fill: rgb("#0077b6"), tracking: 0.5pt)[#upper(label)]
  v(2pt)
}

// ============ En-tête ============

#text(size: 20pt, weight: "bold", fill: rgb("#0077b6"))[#titre]
#v(-2pt)
#line(length: 100%, stroke: 2.5pt + rgb("#00b4d8"))
#v(4pt)
#text(size: 9pt, fill: rgb("#666666"))[#date — #exercices.len() exercice(s)]
#v(14pt)

// ============ Cartes exercices ============

#for ex in exercices [
  #block(
    width: 100%,
    stroke: 0.5pt + rgb("#dddddd"),
    inset: 10pt,
    radius: 3pt,
    fill: white,
    above: 10pt,
    below: 10pt,
  )[
    // --- Ligne titre + badges ---
    #grid(
      columns: (1fr, auto),
      column-gutter: 8pt,
      align: (horizon, right + horizon),
    )[
      #text(size: 12pt, weight: "bold")[#ex.nom] \
      #text(size: 8pt, fill: rgb("#888888"))[#ex.code]
    ][
      #badge(ex.discipline, rgb("#caf0f8"), rgb("#0077b6"))
      #h(3pt)
      #badge(ex.categorie, rgb("#e9ecef"), rgb("#495057"))
      #h(3pt)
      #badge(ex.niveau, couleur-niveau(ex.niveau), couleur-niveau-texte(ex.niveau))
    ]

    #v(8pt)

    // --- Sections ---
    #section-titre("Objectif")
    #ex.objectif

    #section-titre("Description")
    #ex.description

    #section-titre("Consigne clé")
    #ex.consigneCle

    #section-titre("Indicateur de succès")
    #ex.indicateurSucces

    #section-titre("Erreur typique corrigée")
    #ex.erreurTypiqueCorrigee

    // --- Équipement ---
    #if ex.equipement.len() > 0 [
      #v(6pt)
      #text(size: 7.5pt, weight: "bold", fill: rgb("#0077b6"), tracking: 0.5pt)[ÉQUIPEMENT]
      #v(3pt)
      #for item in ex.equipement [
        #badge(item, rgb("#f1f3f5"), rgb("#495057"))
        #h(3pt)
      ]
    ]
  ]
]

// ============ Footer ============

#v(1fr)
#align(center)[
  #text(size: 7pt, fill: rgb("#adb5bd"))[
    Onduleo — Fiche d'exercices générée le #date
  ]
]

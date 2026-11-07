---
name: TomatoTask
description: 'Minuteur et tâches en majolique'
colors:
  glaze: '#f6f8f7'
  cobalt: '#1d3f83'
  tomato: '#b93424'
  yolk: '#e3b435'
  copper: '#317450'
  manganese: '#655249'
  grout: '#b5a9a0'
  foreground: '#172e59'
  card: '#ffffff'
  dark-background: '#101c33'
  dark-glaze: '#172846'
  dark-foreground: '#edf3ff'
typography:
  display:
    fontFamily: 'Bodoni Moda Variable, serif'
    fontSize: '58–110px'
    fontWeight: 700
    lineHeight: 1
    letterSpacing: '-0.04em'
  headline:
    fontFamily: 'Bodoni Moda Variable, serif'
    fontSize: '27px'
    lineHeight: 1.15
  body:
    fontFamily: 'Source Sans 3 Variable, sans-serif'
    lineHeight: 1.5
rounded:
  sm: '2px'
  md: '4px'
  xl: '6px'
spacing:
  sm: '8px'
  md: '16px'
  lg: '24px'
components:
  button-session:
    backgroundColor: '{colors.tomato}'
    textColor: '{colors.card}'
    rounded: '{rounded.md}'
    padding: '15px'
  button-primary:
    backgroundColor: '{colors.cobalt}'
    textColor: '{colors.card}'
    rounded: '{rounded.md}'
    height: '36px'
    padding: '8px 16px'
---

# Design System: TomatoTask

## Overview

**Creative North Star: "Majolique vivante"**

Un émail blanc froid accueille une interface de travail calme, structurée par des joints fins et des pigments de céramique. Le rail cobalt et la composition en carreaux donnent une présence matérielle à une application locale et quotidienne.

**Key Characteristics:**

- Carreaux nus ou peints assemblés par des joints visibles.
- Cobalt pour la navigation, pigments distincts pour les sessions.
- Contraste entre une typographie d'affichage expressive et une interface lisible.
- Nuit cobalt et mouvements réduits pris en charge.

## Colors

Le cobalt porte la navigation et les commandes ordinaires ; tomate, jaune d'œuf et cuivre identifient respectivement travail, pause courte et pause longue. Le manganèse et les joints accompagnent le texte secondaire et l'armature. Le thème sombre utilise une nuit cobalt et des pigments éclaircis. Le global `Theme` dans `ui/app.slint` centralise les couleurs.

## Typography

Bodoni Moda Variable donne aux titres et au minuteur leur voix éditoriale ; Source Sans 3 Variable assure la lecture des commandes et des listes. Les deux familles sont embarquées. La taille du minuteur diminue aux petites largeurs.

## Layout

Les onglets séparent minuteur, plan, projets, bilan et réglages. La fresque de 220 px accompagne le minuteur. Les modes focus et compact isolent le minuteur et ses commandes. Les espacements fréquemment employés sont de 8, 16 et 24 px.

## Elevation & Depth

L'ombre d'émail associe un reflet intérieur supérieur à une ombre légère et floue (`inset 0 1px rgba(255,255,255,.7), 0 2px 5px rgba(22,42,70,.12)`). Les joints et les changements de surface organisent le reste de la profondeur.

## Shapes

Les surfaces et commandes sont rectangulaires, avec de petits rayons de 2 à 6 px. Les carreaux sont carrés, séparés par un joint de 1 px ; leurs motifs SVG rejoignent les milieux des bords.

## Components

Les boutons ordinaires utilisent le cobalt, tandis que l'action principale du minuteur adopte le pigment de la session. Les onglets actifs inversent le texte et la surface. Les champs et commandes reçoivent un anneau de focus visible ; sélection, curseur et défilement utilisent la palette.

Le carreau est nu, prévu, en cours de peinture ou émaillé. Le motif se révèle avec la progression de la session. Le numéro de série apparaît sur les carreaux émaillés.

## Do's and Don'ts

### Do:

- **Do** garder le minuteur stable et lisible à chaque taille de fenêtre.
- **Do** respecter les couleurs sémantiques des sessions.
- **Do** conserver le raccord des motifs aux bords des carreaux.

### Don't:

- **Don't** charger les polices à distance : l'application fonctionne hors ligne.
- **Don't** remplacer la progression peinte par un anneau de progression.
- **Don't** utiliser de verre décoratif ou d'ombre portée dure.

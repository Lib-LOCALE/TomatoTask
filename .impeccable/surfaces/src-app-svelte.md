---
version: 1
slug: "src-app-svelte"
primary_target: "src/App.svelte"
related_targets: []
---

# Surface : application TomatoTask (écran principal et vues associées)

Mode : Operate. Public grand public (voir PRODUCT.md). Tâche : lancer et tenir des sessions, planifier la journée, voir sa progression.

## Direction contract

THESIS: Ta journée est un mur de carreaux de majolique. Chaque pomodoro planifié est un carreau nu cerné de son joint ; chaque pomodoro terminé est émaillé et peint, et les carreaux adjacents composent ensemble un plant de tomate continu (vigne, feuilles, fruits) ; la semaine forme une fresque. Refuse l'arrangement de catégorie : carte beige + anneau gris de progression + bouton rouge.

OWN-WORLD: Émail blanc froid laiteux (pas crème) comme fond ; bleu de cobalt profond qui possède le rail de navigation et les états actifs ; rouge tomate réservé à la session de travail et aux fruits peints ; jaune d'œuf (pause courte), vert cuivre (pause longue, feuilles), brun manganèse pour les traits fins et le texte secondaire. Joints de carreaux visibles (grille 1px brun clair) comme armature. Thème sombre : nuit cobalt (fond bleu-noir émaillé, carreaux vernis sombres, mêmes pigments éclaircis). Composants : rectangles aux angles très légèrement arrondis comme des carreaux, ombre d'émail (reflet interne fin en haut, pas d'ombre portée floue), aucun dégradé décoratif, aucun verre dépoli.

STORY: L'utilisateur voit en une seconde sa tâche en cours et le temps restant, comprend que chaque session terminée ajoute un carreau à sa fresque, planifie sa journée en carreaux, et revient le lendemain pour ne pas casser sa série.

FIRST VIEWPORT: Trois zones. Rail cobalt à gauche (~280px) : logo tomate, série de jours, projets, tâches du jour avec carreaux prévus/faits. Centre : type de session en onglets (Travail / Pause courte / Pause longue), titre de la tâche en cours, minuteur en chiffres tabulaires à échelle d'affiche (~160–200px), sans anneau ni cadre ; la progression de la session se lit comme le carreau en cours qui se peint progressivement à côté du minuteur ; action primaire Démarrer/Pause en bouton tomate pleine largeur sous le minuteur, Passer en secondaire. Droite (~340px) : « Fresque du jour », grille de carreaux (prévu = nu, fait = peint, en cours = se peint), compteur, et accès à la fresque de la semaine.

FORM: Majolique sicilienne / panneaux d'azulejos, candidat 7 de la liste ordonnée (assigné par le tirage). seed:3b18426a770b66be
Signature : le carreau qui s'émaille à la fin d'une session (peinture du motif + reflet de cuisson, ~900ms, respect de prefers-reduced-motion), et la fresque où les carreaux voisins forment un seul plant continu. Élévations : estompage de tout sauf minuteur et tâche pendant une session ; numéro de série à vie gravé sur chaque carreau ; grille visible comme armature, carreau vide = état réel ; chiffres en cellules fixes.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance

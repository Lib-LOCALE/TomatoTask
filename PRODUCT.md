# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users
Grand public, distribué en open source via les releases GitHub (Windows, Linux). Personnes qui travaillent ou étudient sur ordinateur et veulent tenir des sessions de concentration régulières : elles ouvrent l'app en début de journée, la laissent tourner à côté de leur travail, y reviennent entre deux sessions. Elles doivent comprendre l'app sans documentation.

## Product Purpose
Minuteur Pomodoro et gestionnaire de tâches local (desktop, hors ligne). Le succès : l'utilisateur tient ses sessions, revient le lendemain, et voit ce qu'il a réellement accompli.

## Positioning
Un Pomodoro où le temps passé devient visible et vivant : chaque session complétée fait pousser quelque chose, la journée se planifie en pomodoros, et la session en cours peut occuper tout l'écran. Local-first, sans compte, sans cloud.

## Operating Context
Fenêtre desktop Tauri (1200×800 par défaut, redimensionnable, mini-mode compact), icône de barre système, notifications natives, raccourcis clavier (Ctrl+S, Ctrl+N, Ctrl+L, Ctrl+/). Usage en arrière-plan pendant de longues journées ; thèmes clair et sombre.

## Capabilities and Constraints
- Existant : sessions travail / pause courte / pause longue configurables, auto-enchaînement, tâches (CRUD, filtres, réordonnancement par glisser-déposer, complétion), projets, statistiques (focus quotidien, distribution par projet, heatmap), sons d'ambiance, export/import de sauvegarde, onboarding, i18n (en, fr, de, es, it), mini-mode.
- À ajouter (confirmé) : gamification visuelle (croissance liée aux pomodoros complétés, séries de jours), mode focus immersif plein écran, statistiques parlantes (bilans, tendances, meilleures heures), planification de journée (estimation en pomodoros par tâche, prévu vs réalisé).
- Stack : Tauri 2 + Rust + SQLite ; Svelte 5 (runes) + Tailwind 4 + svelte-i18n. Pas de polices distantes (CSP stricte, apps hors ligne) : toute police est embarquée dans le bundle.

## Brand Commitments
Nom « TomatoTask » et la tomate comme mascotte/symbole. Le reste de l'identité visuelle est libre (l'ancienne apparence n'est pas un engagement).

## Evidence on Hand
Aucun témoignage, chiffre d'utilisateurs ni classement : n'en inventer aucun. Captures de l'ancienne version dans `.github/images/`.

## Product Principles
- Le minuteur et la tâche en cours dominent ; tout le reste se tient en retrait pendant une session.
- La progression se voit : le travail accompli laisse une trace visible et durable.
- Compréhensible sans tutoriel par un public non technique.
- Local et respectueux : pas de compte, pas de collecte.
- Calme pendant le focus, gratifiant à la fin d'une session.

## Accessibility & Inclusion
Navigation complète au clavier, contraste WCAG AA dans les deux thèmes, respect de `prefers-reduced-motion` pour toutes les animations (croissance, confettis, mode immersif).

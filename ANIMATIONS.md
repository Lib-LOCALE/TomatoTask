# TomatoTask Animations

Motion in TomatoTask follows the "Majolique vivante" direction in [DESIGN.md](DESIGN.md): calm, purposeful, and always tied to something that actually happens (a session running, a tile being painted). Nothing loops for decoration alone.

## Logo (`src/lib/components/brand/BrandLogo.svelte`)

A majolica tomato painted with a manganese outline, a copper calyx and a leaf.

| State                | Motion                                                                |
| -------------------- | --------------------------------------------------------------------- |
| First display        | The drawing is painted from the bottom up (900 ms, once), like a tile |
| Hover                | The leaf rustles (700 ms)                                             |
| Work session running | The leaf sways and the fruit breathes (4 s loop)                      |
| Idle                 | Static                                                                |

Every pivot is expressed in viewBox coordinates, so no part of the logo ever leaves its frame.

Standalone files:

- `public/logo.svg` – self-contained animated logo (README, website, presentations). It embeds its own `prefers-reduced-motion` rule.
- `public/favicon.svg` – static version used as the browser/webview favicon (`favicon.png` is its 64 px fallback). It is static on purpose: browsers render only the first frame of an icon.
- `src-tauri/icons/app-icon.svg` – desktop application icon: the logo on a majolica tile. Regenerate every platform icon with `npx tauri icon src-tauri/icons/app-icon.svg -o src-tauri/icons` (then delete the generated `android/` and `ios/` folders, the app targets desktop only).

## Majolica tiles (`src/lib/components/majolica/Tile.svelte`)

- **Painting**: during a work session the tile's colours are revealed from the bottom up as the timer progresses (`clip-path`, 1 s linear transitions).
- **Glazing**: when a session completes, the tile is painted in 600 ms and then a glaze highlight sweeps across it (300 ms).

## Focus mode (`src/lib/components/focus/FocusMode.svelte`)

- Controls fade out after 3 s without activity and come back on any pointer or keyboard input.
- A short "tile glazed" message fades in when a work session completes.

## Toasts (`src/lib/components/effects/Toast.svelte`)

Non-intrusive feedback messages that dismiss themselves after their duration (3 s by default).

```typescript
import { toastStore } from '$lib/stores/toast.svelte';

toastStore.celebration('Task completed!');
toastStore.success('Changes saved');
toastStore.error('Something went wrong');
toastStore.info('New feature available');
```

## Accessibility

All motion respects `prefers-reduced-motion: reduce`:

- a global rule in `src/app.css` shortens every animation and transition;
- the logo, tiles and focus mode also disable their animations explicitly;
- `public/logo.svg` carries its own media query, since it can be used outside the app.

## Performance

Animations only use `transform`, `opacity` and `clip-path`, and the looping ones (logo) only run while a work session is active.

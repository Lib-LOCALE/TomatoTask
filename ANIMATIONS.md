# ✨ TomatoTask Animations Guide

This document describes all the animations and visual effects added to make TomatoTask more lively and engaging.

## 🎨 Core Animation Components

### 1. **Confetti Effect** (`Confetti.svelte`)
- **Purpose**: Celebration animation for task completion
- **Features**:
  - 50 colorful particles
  - Physics-based movement with gravity
  - Particles rotate and fall naturally
  - Auto-cleanup after 3 seconds
- **Usage**: Automatically triggered when completing a task

### 2. **Toast Notifications** (`Toast.svelte` + `ToastContainer.svelte`)
- **Purpose**: Non-intrusive feedback messages
- **Types**:
  - 🎉 **Celebration**: For major achievements (task completion)
  - ✅ **Success**: For successful operations
  - ❌ **Error**: For error messages
  - ℹ️ **Info**: For informational messages
- **Features**:
  - Slide-in animation from right
  - Bounce effect on entry
  - Auto-dismiss after customizable duration
  - Manual close button
- **Usage**: `toastStore.celebration('Message')`, `toastStore.success()`, etc.

### 3. **Background Particles** (`BackgroundParticles.svelte`)
- **Purpose**: Ambient atmosphere
- **Features**:
  - 15-20 subtle floating particles
  - Slow, continuous movement
  - Semi-transparent colors matching app theme
  - Respects `prefers-reduced-motion`
- **Location**: Global, behind all content

### 4. **Tomato Loader** (`TomatoLoader.svelte`)
- **Purpose**: Loading states
- **Features**:
  - Spinning pixel-art tomato
  - Scale animation for emphasis
  - Optional loading text
  - Customizable size
- **Usage**: `<TomatoLoader size={64} text="Loading..." />`

### 5. **Animated Counter** (`AnimatedCounter.svelte`)
- **Purpose**: Smooth number transitions in statistics
- **Features**:
  - Counts from old to new value
  - Ease-out animation
  - Customizable duration
  - Custom formatting function
- **Usage**: `<AnimatedCounter value={stats.total} />`

## 🍅 Animated Tomato Variants

### Original Tomato (`animated-tomato.svg`)
- General purpose animation
- Floating + Pulsing + Stem waving

### Celebration Tomato (`animated-tomato-celebration.svg`)
- **When**: Work session completed
- **Animations**:
  - Bouncing with wiggle
  - Confetti particles falling
  - Twinkling stars around
- **Size**: Displayed larger (96px)

### Focused Tomato (`animated-tomato-focused.svg`)
- **When**: Active work timer running
- **Animations**:
  - Intense pulsing
  - Bobbing motion
  - Focus ring pulse
  - Golden sparks
- **Expression**: Determined eyes, straight mouth

### Relaxing Tomato (`animated-tomato-relaxing.svg`)
- **When**: Break periods active
- **Animations**:
  - Gentle breathing
  - Slow sway
  - Rising "ZZZ" particles
- **Expression**: Closed eyes with smile

## 🎭 Micro-Interactions (CSS Classes)

### Button Effects
- `.btn-press` - Press down effect on click
- `.hover-bounce` - Bounce animation on hover
- `.hover-lift` - Lift up on hover with shadow
- `.hover-glow` - Glow effect on hover
- `.hover-rotate` - Slight rotation on hover

### Animation Effects
- `.fade-in-up` - Fade in from bottom
- `.scale-in` - Scale from small to full size
- `.slide-in-right` - Slide from right
- `.slide-in-left` - Slide from left
- `.pop` - Pop in with bounce
- `.wiggle` - Wiggle rotation
- `.shake` - Shake for errors
- `.pulse-slow` - Slow pulsing
- `.heartbeat` - Heartbeat animation

### Progress Bar Effects
- `.progress-liquid` - Shimmer effect on progress bars

## 🎯 Where Animations Appear

### Timer Display
- **Active Timer**: Animated tomato (focused/relaxing) appears in center
- **Completion**: Celebration tomato with confetti and toast

### Task Cards
- **Hover**: Lift effect with shadow
- **Completion**:
  - Confetti explosion
  - Pop animation
  - Celebration toast
- **Progress Bar**: Liquid shimmer effect
- **Buttons**: Press and bounce effects

### Background
- Subtle floating particles (always visible)

### Statistics
- Numbers animate/count up when displayed
- Can add more animations to stat cards

## ⚙️ Animation Settings

### Accessibility
All animations respect `prefers-reduced-motion`:
```css
@media (prefers-reduced-motion: reduce) {
  * { animation-duration: 0.01ms !important; }
}
```

### Performance
- Animations use CSS transforms (GPU-accelerated)
- Particles have auto-cleanup
- Smooth 60fps animations

## 🚀 Usage Examples

### Toast Notifications
```typescript
import { toastStore } from '$lib/stores/toast.svelte';

// Celebration
toastStore.celebration('🎉 Task completed!');

// Success
toastStore.success('Changes saved');

// Error
toastStore.error('Something went wrong');

// Info
toastStore.info('New feature available');
```

### Animated Counter
```svelte
<script>
  import AnimatedCounter from '$lib/components/effects/AnimatedCounter.svelte';
</script>

<AnimatedCounter
  value={totalPomodoros}
  duration={1000}
  format={(n) => Math.round(n).toString()}
/>
```

### Confetti
```svelte
<script>
  import Confetti from '$lib/components/effects/Confetti.svelte';
  let showConfetti = $state(false);
</script>

{#if showConfetti}
  <Confetti duration={3000} particleCount={50} />
{/if}
```

### CSS Animations
```svelte
<button class="btn-press hover-bounce">
  Click me!
</button>

<div class="fade-in-up">
  Content appears smoothly
</div>
```

## 🎨 Color Palette

Animations use theme colors:
- **Primary (Tomato Red)**: `#F44336`
- **Success (Green)**: `#4CAF50`
- **Warning (Orange)**: `#FF9800`
- **Info (Blue)**: `#2196F3`
- **Celebration (Purple/Pink)**: Gradient

## 📦 Component Structure

```
src/lib/components/
├── effects/
│   ├── Confetti.svelte          # Celebration particles
│   ├── Toast.svelte             # Individual toast
│   ├── ToastContainer.svelte    # Toast manager
│   ├── BackgroundParticles.svelte # Ambient effect
│   ├── TomatoLoader.svelte      # Loading spinner
│   └── AnimatedCounter.svelte   # Number animations
├── ui/
│   └── AnimatedIcon.svelte      # SVG icon loader
└── ...

src/lib/stores/
└── toast.svelte.ts              # Toast state management

public/icons/
├── animated-tomato.svg
├── animated-tomato-celebration.svg
├── animated-tomato-focused.svg
├── animated-tomato-relaxing.svg
└── ... (other animated icons)

src/app.css
└── Micro-interaction animations  # CSS utility classes
```

## 🔮 Future Enhancement Ideas

- [ ] Task streak counter with special celebrations
- [ ] Seasonal theme variations
- [ ] Sound effects (optional, user-controlled)
- [ ] More particle effects (rain, snow)
- [ ] Animated stat cards with charts
- [ ] Interactive tutorial with animations
- [ ] Achievement badges with reveal animations

---

Made with 🍅 and lots of animations!

# 🎮 TomatoTask Pixel Art Design Guide

## 📖 Overview

This guide shows you how to implement the new pixel art aesthetic throughout your TomatoTask application.

## 🎨 Color Palette

### Primary Colors
- **Tomato Red**: `#E84444` - Main brand color (buttons, primary actions)
- **Dark Red**: `#C43333` - Hover states, shadows
- **Leaf Green**: `#4CAF50` - Success, completed tasks
- **Accent Green**: `#66BB6A` - Lighter green variant

### Session Type Colors
```css
/* Use these for timer states */
--session-work: Tomato Red (#E84444)
--session-short-break: Orange (#FFA726)
--session-long-break: Green (#4CAF50)
```

### Secondary/Neutral
- **Tan/Beige**: `#D4A574` - Card backgrounds, warm accents
- **Light Gray**: `#BDBDBD` - Borders, secondary text
- **Medium Gray**: `#9E9E9E` - Disabled states
- **Dark Navy**: `#2C2C54` - Text, outlines

### Accents
- **Orange**: `#FFA726` - Achievements, short breaks
- **Yellow**: `#FFB74D` - Highlights, warnings
- **White**: `#FFFFFF` - Contrast text

## 🔤 Typography

### Fonts Imported
1. **Press Start 2P** - Headings, titles, timer display
2. **Pixelify Sans** - Body text (400, 500, 600, 700)
3. **Silkscreen** - Monospace, code blocks (400, 700)

### Usage
```svelte
<!-- Automatic heading styling -->
<h1>TomatoTask</h1> <!-- Uses Press Start 2P -->
<p>Body text</p> <!-- Uses Pixelify Sans -->

<!-- Force specific fonts -->
<div class="font-[family-name:var(--font-heading)]">Pixel heading</div>
<div class="font-mono">Monospace text</div>
```

## 🎯 Component Examples

### 1. Pomodoro Timer Display

```svelte
<script lang="ts">
  let sessionType: 'work' | 'short-break' | 'long-break' = 'work';
  let timeRemaining = '25:00';
</script>

<div class="timer-container {sessionType}">
  <!-- Session label -->
  <div class="mb-4">
    <span class="badge-pixel border-current bg-current/10">
      {sessionType === 'work' ? '🍅 Work Session' :
       sessionType === 'short-break' ? '☕ Short Break' :
       '🌟 Long Break'}
    </span>
  </div>

  <!-- Timer display -->
  <div class="timer-display {`session-${sessionType}`}">
    {timeRemaining}
  </div>

  <!-- Controls -->
  <div class="mt-8 flex gap-pixel justify-center">
    <button class="btn-primary">
      START
    </button>
    <button class="btn-secondary">
      PAUSE
    </button>
    <button class="btn-warning">
      RESET
    </button>
  </div>

  <!-- Progress bar -->
  <div class="mt-6 h-4 border-pixel border-current rounded-none bg-muted overflow-hidden">
    <div
      class="h-full bg-current transition-all duration-1000"
      style="width: 60%"
    ></div>
  </div>
</div>

<style>
  /* Add dynamic class binding for session types */
  .timer-container.work { color: var(--session-work); }
  .timer-container.short-break { color: var(--session-short-break); }
  .timer-container.long-break { color: var(--session-long-break); }
</style>
```

### 2. Task Card

```svelte
<script lang="ts">
  let task = {
    title: "Finish project documentation",
    completed: false,
    pomodoros: 3,
    completedPomodoros: 1
  };
</script>

<div class="task-card {task.completed ? 'completed' : ''}">
  <!-- Task title -->
  <div class="flex items-start gap-3">
    <input
      type="checkbox"
      bind:checked={task.completed}
      class="w-5 h-5 border-pixel border-border rounded-none mt-1
             checked:bg-success checked:border-success
             transition-pixel"
    />
    <div class="flex-1">
      <h4 class="font-bold {task.completed ? 'line-through opacity-60' : ''}">
        {task.title}
      </h4>

      <!-- Pomodoro count -->
      <div class="mt-2 flex gap-1">
        {#each Array(task.pomodoros) as _, i}
          <span class="text-xl {i < task.completedPomodoros ? 'opacity-100' : 'opacity-30'}">
            🍅
          </span>
        {/each}
      </div>
    </div>
  </div>

  <!-- Actions -->
  <div class="mt-4 flex gap-pixel-sm">
    <button class="btn-secondary text-xs py-1 px-2">
      EDIT
    </button>
    <button class="btn-warning text-xs py-1 px-2">
      START
    </button>
  </div>
</div>
```

### 3. Button Variations

```svelte
<!-- Primary Button (Tomato Red) -->
<button class="btn-primary">
  START POMODORO
</button>

<!-- Success Button (Green) -->
<button class="btn-success">
  MARK COMPLETE
</button>

<!-- Warning Button (Yellow) -->
<button class="btn-warning">
  TAKE BREAK
</button>

<!-- Secondary Button (Tan/Beige) -->
<button class="btn-secondary">
  CANCEL
</button>

<!-- Custom button with pixel shadow -->
<button class="btn-pixel bg-accent text-accent-foreground border-accent">
  CELEBRATE! 🎉
</button>
```

### 4. Input Fields

```svelte
<div class="space-y-4">
  <!-- Text input -->
  <input
    type="text"
    placeholder="Task name..."
    class="input-pixel w-full"
  />

  <!-- Textarea -->
  <textarea
    placeholder="Task description..."
    class="input-pixel w-full min-h-[100px] resize-none"
  ></textarea>

  <!-- Number input with pixel buttons -->
  <div class="flex items-center gap-2">
    <button class="btn-pixel px-3 py-2 text-xl leading-none bg-muted border-border">
      -
    </button>
    <input
      type="number"
      value="25"
      class="input-pixel w-20 text-center font-mono"
    />
    <button class="btn-pixel px-3 py-2 text-xl leading-none bg-muted border-border">
      +
    </button>
    <span class="text-sm text-muted-foreground">MINUTES</span>
  </div>
</div>
```

### 5. Badge Components

```svelte
<!-- Status badges -->
<span class="badge-pixel bg-success/10 border-success text-success">
  COMPLETED
</span>

<span class="badge-pixel bg-warning/10 border-warning text-warning">
  IN PROGRESS
</span>

<span class="badge-pixel bg-primary/10 border-primary text-primary">
  URGENT
</span>

<span class="badge-pixel bg-muted border-border text-muted-foreground">
  LATER
</span>
```

### 6. Card Layout

```svelte
<div class="card-pixel">
  <h3 class="mb-4">Today's Summary</h3>

  <div class="grid-8 grid-cols-3">
    <!-- Stat card -->
    <div class="text-center">
      <div class="text-3xl font-heading text-primary">12</div>
      <div class="text-xs text-muted-foreground uppercase tracking-wide">
        Pomodoros
      </div>
    </div>

    <div class="text-center">
      <div class="text-3xl font-heading text-success">5</div>
      <div class="text-xs text-muted-foreground uppercase tracking-wide">
        Tasks Done
      </div>
    </div>

    <div class="text-center">
      <div class="text-3xl font-heading text-warning">2.5h</div>
      <div class="text-xs text-muted-foreground uppercase tracking-wide">
        Focus Time
      </div>
    </div>
  </div>
</div>
```

### 7. Modal/Dialog

```svelte
<div class="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50">
  <div class="card-pixel border-pixel-thick border-primary max-w-md w-full m-4">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <h2 class="text-lg">NEW TASK</h2>
      <button class="hover-pixel p-2 -m-2">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Content -->
    <div class="space-y-4">
      <input type="text" placeholder="TASK NAME..." class="input-pixel w-full" />
      <textarea placeholder="DESCRIPTION..." class="input-pixel w-full min-h-[100px]"></textarea>
    </div>

    <!-- Footer -->
    <div class="mt-6 flex gap-pixel justify-end">
      <button class="btn-secondary">
        CANCEL
      </button>
      <button class="btn-primary">
        CREATE
      </button>
    </div>
  </div>
</div>
```

### 8. Progress Bar

```svelte
<script lang="ts">
  let progress = 65; // percentage
</script>

<div class="space-y-2">
  <div class="flex justify-between text-sm">
    <span class="font-bold">Daily Goal</span>
    <span class="text-muted-foreground">{progress}%</span>
  </div>

  <!-- Pixelated progress bar -->
  <div class="h-6 border-pixel border-border rounded-none bg-muted overflow-hidden">
    <div
      class="h-full bg-gradient-to-r from-primary to-success flex items-center justify-end pr-2 transition-all duration-500"
      style="width: {progress}%"
    >
      {#if progress > 20}
        <span class="text-xs font-bold text-white">🍅</span>
      {/if}
    </div>
  </div>
</div>
```

## 🎮 Utility Classes Quick Reference

### Borders
- `.border-pixel` - 2px solid border
- `.border-pixel-thick` - 3px solid border

### Shadows
- `.shadow-pixel` - 4px pixel drop shadow
- `.shadow-pixel-lg` - 6px larger pixel shadow
- `.glow-pixel` - Retro glow effect

### Spacing
- `.gap-pixel` - 8px gap (pixel-perfect)
- `.gap-pixel-sm` - 4px small gap
- `.gap-pixel-lg` - 16px large gap
- `.grid-8` - Grid with 8px gaps

### Buttons
- `.btn-pixel` - Base pixel button
- `.btn-primary` - Primary (tomato red)
- `.btn-success` - Success (green)
- `.btn-warning` - Warning (yellow)
- `.btn-secondary` - Secondary (tan)

### Cards
- `.card-pixel` - Card with pixel aesthetic
- `.task-card` - Specific task card styling

### Session Types
- `.session-work` - Tomato red styling
- `.session-short-break` - Orange styling
- `.session-long-break` - Green styling

### Transitions
- `.transition-pixel` - 100ms stepped transition
- `.hover-pixel` - Pixel-style hover effect

## 📱 Responsive Design

```svelte
<!-- Stack on mobile, grid on desktop -->
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-pixel">
  <!-- Cards -->
</div>

<!-- Adjust timer size responsively -->
<div class="timer-display"> <!-- Automatically responsive with clamp() -->
  25:00
</div>
```

## 🎨 Color Usage Guidelines

### Session States
- **Work Session**: Use `session-work` class (Tomato Red)
- **Short Break**: Use `session-short-break` class (Orange)
- **Long Break**: Use `session-long-break` class (Green)

### Task States
- **Active**: Border with `border-primary`
- **Completed**: Border with `border-success` + opacity
- **Urgent**: Background with `bg-warning/10`

### Feedback
- **Success**: Green (`bg-success`, `text-success`)
- **Error**: Red (`bg-destructive`, `text-destructive`)
- **Info**: Orange (`bg-accent`, `text-accent`)
- **Neutral**: Tan (`bg-secondary`, `text-secondary`)

## 🎯 Best Practices

1. **Use pixel-perfect spacing**: Multiples of 4px (4, 8, 16, 24, 32)
2. **Bold borders**: 2px minimum for pixel aesthetic
3. **Minimal rounded corners**: 4px max (use `rounded-md`)
4. **High contrast**: Ensure text is always readable
5. **Consistent shadows**: Use `.shadow-pixel` utilities
6. **Font sizing**: Stick to h1-h6 system for pixel fonts
7. **Animation**: Use `.transition-pixel` for retro feel
8. **Grid alignment**: Use `.grid-8` for pixel-perfect layouts

## 🔧 Customization

### Adding Custom Colors

```css
/* In your component or global CSS */
.custom-color-class {
  background-color: oklch(0.65 0.18 145); /* Use oklch for consistency */
  color: white;
}
```

### Custom Session Type

```svelte
<style>
  .timer-container.focus-mode {
    --session-color: oklch(0.76 0.14 50); /* Orange */
    border-color: var(--session-color);
    color: var(--session-color);
  }
</style>
```

## 📦 Updating Existing Components

To convert an existing component to pixel art style:

```svelte
<!-- BEFORE -->
<button class="px-4 py-2 bg-blue-500 text-white rounded-lg">
  Click me
</button>

<!-- AFTER -->
<button class="btn-primary">
  CLICK ME
</button>
```

```svelte
<!-- BEFORE -->
<div class="bg-white rounded-xl shadow-lg p-6">
  <h3>Title</h3>
  <p>Content</p>
</div>

<!-- AFTER -->
<div class="card-pixel">
  <h3>TITLE</h3>
  <p>Content</p>
</div>
```

## 🎉 Ready to Use!

All styles are now loaded and ready. Simply:
1. Use the utility classes in your components
2. Follow the color guidelines
3. Maintain pixel-perfect spacing (multiples of 4px or 8px)
4. Enjoy your retro TomatoTask experience! 🍅

---

**Need help?** Check the CSS file at `src/app.css` for all available utilities and variables.

# Animated Icons

This directory contains SVG animated icons for TomatoTask application.

## Available Icons

### 🍅 Animated Tomato (`animated-tomato.svg`)
- **Animation**: Floating + Pulsing + Stem waving
- **Usage**: General purpose tomato animation
- **Colors**: Red (#F44336), Green stem (#4CAF50)
- **Duration**: 2-3 seconds per cycle

### 🎉 Celebration Tomato (`animated-tomato-celebration.svg`)
- **Animation**: Bouncing + Wiggling + Confetti + Twinkling stars
- **Usage**: Work session completion, achievements
- **Colors**: Red (#F44336), Green stem (#4CAF50), Multi-colored confetti
- **Duration**: 2 seconds per cycle
- **Special**: Confetti particles fall and rotate, stars twinkle around the tomato

### 🎯 Focused Tomato (`animated-tomato-focused.svg`)
- **Animation**: Intense pulsing + Bobbing + Focus ring + Spark effects
- **Usage**: Active work timer, concentration mode
- **Colors**: Red (#F44336), Green stem (#4CAF50), Orange focus ring (#FF9800), Gold sparks (#FFD700)
- **Duration**: 1.5-2 seconds per cycle
- **Special**: Determined expression, focus ring pulses, golden sparks appear randomly

### 😴 Relaxing Tomato (`animated-tomato-relaxing.svg`)
- **Animation**: Gentle breathing + Slow sway + ZZZ particles
- **Usage**: Break periods, rest mode
- **Colors**: Red (#F44336), Green stem (#4CAF50), Gray ZZZ (#9E9E9E)
- **Duration**: 3-5 seconds per cycle
- **Special**: Closed eyes with smile, ZZZ particles rise up indicating rest

### ⏱️ Animated Timer (`animated-timer.svg`)
- **Animation**: Rotating hand + Pulsing ring
- **Usage**: Timer display, focus mode indicator
- **Colors**: Indigo (#6366f1), Red hand (#ef4444)
- **Duration**: 4 seconds per rotation

### ✓ Animated Check (`animated-check.svg`)
- **Animation**: Drawing effect (circle + checkmark)
- **Usage**: Task completion, success states
- **Colors**: Green (#22c55e), Light green bg (#dcfce7)
- **Duration**: 1 second drawing + hover pulse

### 👁️ Animated Focus (`animated-focus.svg`)
- **Animation**: Dual rotating rings + Ray pulse
- **Usage**: Focus mode, concentration indicator
- **Colors**: Amber (#f59e0b), Yellow (#fbbf24)
- **Duration**: 6-8 seconds per rotation

### 📊 Animated Stats (`animated-stats.svg`)
- **Animation**: Growing bars + Drawing trend line + Popping dots
- **Usage**: Statistics view, productivity insights
- **Colors**: Indigo bars (#6366f1), Green trend (#10b981)
- **Duration**: 1.5 seconds total animation

## Usage in Svelte

```svelte
<script>
  import AnimatedIcon from '$lib/components/ui/AnimatedIcon.svelte';
</script>

<!-- Tomato variants -->
<AnimatedIcon name="tomato" size={64} />
<AnimatedIcon name="tomato-celebration" size={96} />
<AnimatedIcon name="tomato-focused" size={64} />
<AnimatedIcon name="tomato-relaxing" size={64} />

<!-- Other icons -->
<AnimatedIcon name="timer" size={48} class="custom-class" />
<AnimatedIcon name="check" size={80} />
<AnimatedIcon name="focus" size={64} />
<AnimatedIcon name="stats" size={48} />
```

## Technical Details

- **Format**: SVG with embedded CSS animations
- **Performance**: GPU-accelerated transforms
- **Accessibility**: Includes proper ARIA labels
- **Browser Support**: All modern browsers
- **File Size**: ~2-4KB per icon (minified)

## Animation Types Used

- `@keyframes float` - Vertical movement
- `@keyframes pulse` - Scale transformation
- `@keyframes rotate-slow/reverse` - 360° rotation
- `@keyframes draw-circle/check` - Stroke dasharray animation
- `@keyframes grow-bar` - Height/position animation
- `@keyframes pop-dot` - Scale from 0 to 1

## Design Principles

1. **Smooth**: 60fps animations using transform and opacity
2. **Subtle**: Non-distracting, enhances UX without overwhelming
3. **Purposeful**: Each animation communicates state or action
4. **Performance**: Uses CSS transforms, no JavaScript required
5. **Accessible**: Works with screen readers, respects prefers-reduced-motion



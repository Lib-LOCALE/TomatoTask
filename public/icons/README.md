# Animated Icons

This directory contains SVG animated icons for TomatoTask application.

## Available Icons

### 🍅 Animated Tomato (`animated-tomato.svg`)
- **Animation**: Floating + Pulsing + Stem waving
- **Usage**: Completion feedback, loading screens
- **Colors**: Red (#F44336), Green stem (#4CAF50)
- **Duration**: 2-3 seconds per cycle

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

<AnimatedIcon name="tomato" size={64} />
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

## Future Icons (via PixelLab MCP)

The PixelLab MCP server is configured for generating additional pixel art assets:
- Directional character sprites
- Animated game sprites
- Isometric tiles
- Custom map objects

Use `claude mcp list` to verify PixelLab connection status.

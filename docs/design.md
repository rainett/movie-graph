# Movie Graph - Design Specification

## Design Philosophy

### Core Principle

**Function defines form.** The 80s aesthetic is a visual language, not a constraint. Every design choice must serve usability first. If a retro element hurts usability, discard it.

### Design Goals

1. **Tangible** - App feels like physical equipment, not another flat digital interface
2. **Distinctive** - Immediately recognizable, not generic "modern UI"
3. **Warm** - Analog imperfection over sterile precision
4. **Focused** - Visual noise removed, attention guided to content
5. **Responsive** - Every interaction has feedback (visual + audio)

### What We're NOT Doing

- Pixel-perfect retro recreation (sacrifices readability)
- Excessive ornamentation (bezels, screws everywhere)
- Distracting animations (constant scan lines, flicker)
- Dark-for-dark's-sake (must be usable)

---

## Visual References

### Mood Board Categories

| Category | Reference | What to Take |
|----------|-----------|--------------|
| **Film Equipment** | Steenbeck flatbed editor, Moviola | Transport controls, sprocket aesthetic |
| **Broadcast Monitors** | Sony PVM, JVC studio monitors | Screen shape, control layout, indicator lights |
| **VHS Era** | VCR interfaces, rental store systems | Tracking lines, blue/red glow, counter displays |
| **Synthesizers** | Roland Juno, Moog | Sliders, knobs, button clusters, wood grain |
| **Control Rooms** | NASA Mission Control, TV broadcast | Multi-device layout, status panels |

### Key Visual Elements to Extract

```
From Film Equipment:
├── Reel/sprocket motifs (loading, progress)
├── Film strip frame shapes
└── Mechanical transport buttons (play, rewind, forward)

From Broadcast Monitors:
├── Rounded rectangle screens with thick bezels
├── Labeled button clusters
├── LED/LCD status indicators
└── Input selector switches

From VHS:
├── Tracking distortion (loading states)
├── Blue channel bleed
├── Timestamp/counter display font
└── "PLAY" / "REC" indicator style

From Synthesizers:
├── Vertical sliders with caps
├── Rotary knobs with position indicators
├── Grouped button matrices
└── Wood or textured side panels

From Control Rooms:
├── Grid-based device arrangement
├── Status light clusters
├── Label tape aesthetic
└── Dark backgrounds with bright indicators
```

---

## Color System

### Base Palette

```
┌─────────────────────────────────────────────────────────────────┐
│  BACKGROUND TONES                                               │
├─────────────────────────────────────────────────────────────────┤
│  Board Background    #1a1a1e    Deep charcoal, slight blue      │
│  Device Body         #2d2d32    Warm dark gray                  │
│  Device Bezel        #232328    Darker than body                │
│  Screen Background   #0f1218    Near-black with blue tint       │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  TEXT & CONTENT                                                 │
├─────────────────────────────────────────────────────────────────┤
│  Primary Text        #e8e6e3    Warm white (not pure white)     │
│  Secondary Text      #9a9590    Muted warm gray                 │
│  Disabled Text       #5c5955    Low contrast                    │
│  Screen Text         #c4e0c4    Slight green phosphor tint      │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  ACCENT COLORS                                                  │
├─────────────────────────────────────────────────────────────────┤
│  Primary Accent      #e07850    Warm orange-red (selection)     │
│  Secondary Accent    #50a8e0    Cool blue (links, info)         │
│  Success             #50c070    Muted green                     │
│  Warning             #e0a850    Amber                           │
│  Error               #e05050    Soft red                        │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  INDICATOR LIGHTS                                               │
├─────────────────────────────────────────────────────────────────┤
│  LED Off             #3a3530    Barely visible                  │
│  LED Red             #ff3030    Bright, with glow               │
│  LED Green           #30ff50    Bright, with glow               │
│  LED Amber           #ffaa30    Bright, with glow               │
│  LED Blue            #3080ff    Bright, with glow               │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  STATUS BADGE COLORS                                            │
├─────────────────────────────────────────────────────────────────┤
│  Watched             #50c070    Green - completed               │
│  Watching            #50a8e0    Blue - in progress              │
│  Want to Watch       #e0a850    Amber - queued                  │
│  Dropped             #9a9590    Gray - abandoned                │
└─────────────────────────────────────────────────────────────────┘
```

### VHS Color Treatment

For analog warmth, apply subtle post-processing to the canvas area:

```css
.canvas-container {
  /* Slight warm tint */
  filter:
    sepia(0.03)
    saturate(1.05)
    contrast(1.02);
}

/* Optional: Subtle color fringing on edges */
.movie-poster {
  box-shadow:
    -1px 0 0 rgba(255, 100, 100, 0.1),
    1px 0 0 rgba(100, 100, 255, 0.1);
}
```

---

## Typography

### Font Stack

```css
/* Primary UI Font - Period-appropriate, readable */
--font-ui: 'Orbitron', 'Microgramma', 'Roboto', sans-serif;

/* Mono/Display Font - For counters, indicators */
--font-mono: 'Share Tech Mono', 'IBM Plex Mono', monospace;

/* Fallback if custom fonts unavailable */
--font-system: system-ui, -apple-system, sans-serif;
```

### Font Recommendations

| Font | Use Case | Fallback |
|------|----------|----------|
| **Orbitron** | Primary UI text, buttons, labels | Roboto |
| **Orbitron Bold** | Headers, device titles | Roboto Condensed |
| **Share Tech Mono** | Counters, timestamps, code | IBM Plex Mono |

### Type Scale

```css
--text-xs: 0.75rem;    /* 12px - Micro labels */
--text-sm: 0.875rem;   /* 14px - Secondary text */
--text-base: 1rem;     /* 16px - Body text */
--text-lg: 1.125rem;   /* 18px - Emphasized */
--text-xl: 1.25rem;    /* 20px - Section headers */
--text-2xl: 1.5rem;    /* 24px - Device titles */
--text-3xl: 2rem;      /* 32px - Large displays */
```

### Typography Guidelines

```
DO:
✓ Use ALL CAPS for device labels and button text
✓ Use Title Case for movie/actor names
✓ Use sentence case for descriptions and notes
✓ Add letter-spacing to caps text (0.05em - 0.1em)

DON'T:
✗ Use decorative fonts for body text
✗ Go below 12px for any readable text
✗ Mix too many font weights (stick to Regular, Medium, Bold)
```

---

## Component Library

### Buttons

#### Primary Button (Physical Push Button)

```
┌─────────────────────────────────────────┐
│                                         │
│    ┌─────────────────────────────┐      │
│    │                             │      │
│    │         ADD MOVIE           │      │  ← Raised surface
│    │                             │      │
│    └─────────────────────────────┘      │
│         ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓           │  ← Shadow (depth)
│                                         │
└─────────────────────────────────────────┘
     ↑ Button housing/frame
```

```css
.button-primary {
  background: linear-gradient(180deg, #4a4a50 0%, #3a3a40 100%);
  border: 1px solid #2a2a30;
  border-radius: 4px;
  box-shadow:
    0 4px 0 #1a1a20,           /* Depth shadow */
    0 6px 8px rgba(0,0,0,0.3), /* Soft shadow */
    inset 0 1px 0 rgba(255,255,255,0.1); /* Top highlight */
  color: var(--text-primary);
  font-family: var(--font-ui);
  font-size: var(--text-sm);
  font-weight: 500;
  letter-spacing: 0.08em;
  padding: 10px 20px;
  text-transform: uppercase;
  transition: transform 0.1s, box-shadow 0.1s;
}

.button-primary:hover {
  background: linear-gradient(180deg, #5a5a60 0%, #4a4a50 100%);
}

.button-primary:active {
  transform: translateY(2px);
  box-shadow:
    0 2px 0 #1a1a20,
    0 3px 4px rgba(0,0,0,0.3),
    inset 0 1px 0 rgba(255,255,255,0.1);
}
```

#### Mode Switch Buttons (Rocker Style)

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐            │
│   │▓▓▓▓▓▓▓▓▓▓│  │  INSPECT │  │  FILTER  │            │
│   │  SEARCH  │  │          │  │          │            │
│   │▓▓▓▓▓▓▓▓▓▓│  │          │  │          │            │
│   └──────────┘  └──────────┘  └──────────┘            │
│      ACTIVE        INACTIVE      INACTIVE              │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

```css
.mode-button {
  background: #2a2a30;
  border: 1px solid #3a3a40;
  border-radius: 3px;
  color: var(--text-secondary);
  padding: 8px 16px;
}

.mode-button.active {
  background: linear-gradient(180deg, #3a3a40 0%, #2a2a30 100%);
  border-color: var(--accent-primary);
  box-shadow:
    inset 0 2px 4px rgba(0,0,0,0.3),
    0 0 8px rgba(224, 120, 80, 0.2);
  color: var(--text-primary);
}
```

### Sliders

#### Vertical Fader (Mixer Style)

```
        ┌───┐
        │   │  ← Track
        │   │
        │ █ │  ← Thumb (cap)
        │   │
        │   │
        └───┘
       ─────── Label
```

```css
.slider-vertical {
  appearance: none;
  background: #1a1a20;
  border: 1px solid #3a3a40;
  border-radius: 2px;
  height: 120px;
  width: 24px;
  writing-mode: bt-lr; /* Bottom to top */
}

.slider-vertical::-webkit-slider-thumb {
  appearance: none;
  background: linear-gradient(180deg, #5a5a60 0%, #3a3a40 100%);
  border: 1px solid #2a2a30;
  border-radius: 2px;
  box-shadow:
    0 2px 4px rgba(0,0,0,0.3),
    inset 0 1px 0 rgba(255,255,255,0.2);
  cursor: grab;
  height: 20px;
  width: 30px;
}
```

#### Horizontal Slider (Range)

```
     MIN                              MAX
      │                                │
      ├────────────█══════════════════┤
      │            ↑                   │
               Thumb position
```

```css
.slider-horizontal {
  appearance: none;
  background: #1a1a20;
  border-radius: 2px;
  height: 8px;
}

.slider-horizontal::-webkit-slider-thumb {
  appearance: none;
  background: var(--accent-primary);
  border-radius: 50%;
  box-shadow: 0 0 6px rgba(224, 120, 80, 0.5);
  height: 18px;
  width: 18px;
}

/* Filled portion */
.slider-horizontal {
  background: linear-gradient(
    to right,
    var(--accent-primary) 0%,
    var(--accent-primary) var(--value-percent),
    #1a1a20 var(--value-percent),
    #1a1a20 100%
  );
}
```

### Toggle Switches

#### Rocker Switch

```
   OFF                 ON
┌─────────┐       ┌─────────┐
│ ████    │       │    ████ │
│ ████    │       │    ████ │
└─────────┘       └─────────┘
    ↑                  ↑
  Left side          Right side
  pressed            pressed
```

```css
.toggle-switch {
  background: #2a2a30;
  border: 2px solid #3a3a40;
  border-radius: 4px;
  display: flex;
  height: 28px;
  overflow: hidden;
  width: 56px;
}

.toggle-switch::before,
.toggle-switch::after {
  align-items: center;
  display: flex;
  flex: 1;
  font-size: var(--text-xs);
  justify-content: center;
}

.toggle-switch::before { content: 'OFF'; }
.toggle-switch::after { content: 'ON'; }

.toggle-switch[data-state="off"]::before,
.toggle-switch[data-state="on"]::after {
  background: linear-gradient(180deg, #3a3a40 0%, #2a2a30 100%);
  box-shadow: inset 0 2px 4px rgba(0,0,0,0.3);
}

.toggle-switch[data-state="on"]::after {
  color: var(--success);
}
```

#### LED Indicator with Label

```
       ┌───┐
       │ ● │ ← LED (lit or unlit)
       └───┘
      RECORD
```

```css
.led-indicator {
  align-items: center;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.led {
  background: var(--led-off);
  border-radius: 50%;
  height: 10px;
  width: 10px;
  box-shadow: inset 0 1px 2px rgba(0,0,0,0.5);
}

.led.active {
  background: var(--led-red);
  box-shadow:
    inset 0 1px 2px rgba(0,0,0,0.3),
    0 0 8px var(--led-red),
    0 0 16px rgba(255, 48, 48, 0.3);
}

.led-label {
  color: var(--text-secondary);
  font-size: var(--text-xs);
  letter-spacing: 0.1em;
  text-transform: uppercase;
}
```

### Input Fields

#### Text Input (Terminal Style)

```
┌─────────────────────────────────────────┐
│ ┌─────────────────────────────────────┐ │
│ │ Search movies...              │     │ │  ← Screen inset
│ └─────────────────────────────────────┘ │
│            SEARCH QUERY                  │  ← Label below
└─────────────────────────────────────────┘
```

```css
.input-field {
  background: var(--screen-bg);
  border: 2px solid #2a2a30;
  border-radius: 2px;
  box-shadow:
    inset 0 2px 8px rgba(0,0,0,0.5),
    inset 0 0 1px rgba(255,255,255,0.05);
  caret-color: var(--accent-primary);
  color: var(--screen-text);
  font-family: var(--font-mono);
  font-size: var(--text-base);
  padding: 12px 16px;
}

.input-field:focus {
  border-color: var(--accent-primary);
  box-shadow:
    inset 0 2px 8px rgba(0,0,0,0.5),
    0 0 0 2px rgba(224, 120, 80, 0.2);
  outline: none;
}

.input-field::placeholder {
  color: var(--text-disabled);
}
```

### Cards

#### Movie/Actor Node Card

```
┌─────────────────────────┐
│ ┌─────────────────────┐ │
│ │                     │ │
│ │      [POSTER]       │ │
│ │                     │ │
│ │                     │ │
│ └─────────────────────┘ │
│ ┌─────────────────────┐ │
│ │ Inception     ★ 8.5 │ │  ← Title bar
│ │ 2010          ● ━━  │ │  ← Year + status
│ └─────────────────────┘ │
└─────────────────────────┘
```

```css
.node-card {
  background: var(--device-body);
  border: 2px solid var(--device-bezel);
  border-radius: 6px;
  box-shadow:
    0 4px 8px rgba(0,0,0,0.3),
    inset 0 1px 0 rgba(255,255,255,0.05);
  overflow: hidden;
  width: 160px;
}

.node-card.selected {
  border-color: var(--accent-primary);
  box-shadow:
    0 4px 8px rgba(0,0,0,0.3),
    0 0 0 2px rgba(224, 120, 80, 0.3),
    inset 0 1px 0 rgba(255,255,255,0.05);
}

.node-poster {
  aspect-ratio: 2/3;
  background: var(--screen-bg);
  object-fit: cover;
  width: 100%;
}

.node-info {
  background: rgba(0,0,0,0.3);
  padding: 8px;
}

.node-title {
  color: var(--text-primary);
  font-size: var(--text-sm);
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
```

---

## Device Frames

### Graph Monitor (Main Device)

```
┌──────────────────────────────────────────────────────────────────────┐
│ ╔════════════════════════════════════════════════════════════════╗   │
│ ║  GRAPH MONITOR                                    ● ● ●        ║   │
│ ╠════════════════════════════════════════════════════════════════╣   │
│ ║                                                                ║   │
│ ║  ┌──────────────────────────────────────────────────────────┐  ║   │
│ ║  │                                                          │  ║   │
│ ║  │                                                          │  ║   │
│ ║  │                     CANVAS AREA                          │  ║   │
│ ║  │                                                          │  ║   │
│ ║  │                                                          │  ║   │
│ ║  │                                                          │  ║   │
│ ║  └──────────────────────────────────────────────────────────┘  ║   │
│ ║                                                                ║   │
│ ╠════════════════════════════════════════════════════════════════╣   │
│ ║  [◄◄] [◄] [▌▌] [►] [►►]      ━━━━━━━━━●━━━━━━━━━      00:42   ║   │
│ ╚════════════════════════════════════════════════════════════════╝   │
│              ↑ History transport controls                            │
└──────────────────────────────────────────────────────────────────────┘
```

```css
.device-frame {
  background: var(--device-body);
  border: 3px solid var(--device-bezel);
  border-radius: 8px;
  box-shadow:
    0 8px 24px rgba(0,0,0,0.4),
    inset 0 1px 0 rgba(255,255,255,0.05);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.device-header {
  align-items: center;
  background: linear-gradient(180deg, #3a3a40 0%, #2d2d32 100%);
  border-bottom: 1px solid #1a1a20;
  display: flex;
  height: 32px;
  padding: 0 12px;
}

.device-title {
  color: var(--text-secondary);
  font-size: var(--text-xs);
  font-weight: 500;
  letter-spacing: 0.15em;
  text-transform: uppercase;
}

.device-indicators {
  display: flex;
  gap: 6px;
  margin-left: auto;
}

.device-screen {
  background: var(--screen-bg);
  border: 4px solid #1a1a20;
  border-radius: 4px;
  box-shadow:
    inset 0 0 30px rgba(0,0,0,0.5),
    inset 0 0 2px rgba(255,255,255,0.03);
  flex: 1;
  margin: 12px;
  overflow: hidden;
  position: relative;
}
```

### Control Terminal (Smaller Device)

```
┌────────────────────────────────────────┐
│ ╔════════════════════════════════════╗ │
│ ║  CONTROL TERMINAL           ● ●   ║ │
│ ╠════════════════════════════════════╣ │
│ ║ [SEARCH] [INSPECT] [FILTER]        ║ │  ← Mode buttons
│ ╠════════════════════════════════════╣ │
│ ║ ┌────────────────────────────────┐ ║ │
│ ║ │                                │ ║ │
│ ║ │       MODE CONTENT AREA        │ ║ │
│ ║ │                                │ ║ │
│ ║ │                                │ ║ │
│ ║ └────────────────────────────────┘ ║ │
│ ╚════════════════════════════════════╝ │
└────────────────────────────────────────┘
```

---

## Screen Effects

### CRT Warmth (Subtle)

Apply to screen areas only, not device frames:

```css
.device-screen {
  position: relative;
}

.device-screen::before {
  content: '';
  position: absolute;
  inset: 0;
  background: radial-gradient(
    ellipse at center,
    transparent 0%,
    transparent 60%,
    rgba(0, 0, 0, 0.15) 100%
  );
  pointer-events: none;
  z-index: 100;
}

/* Optional: Very subtle scan lines */
.device-screen::after {
  content: '';
  position: absolute;
  inset: 0;
  background: repeating-linear-gradient(
    0deg,
    transparent 0px,
    transparent 2px,
    rgba(0, 0, 0, 0.03) 2px,
    rgba(0, 0, 0, 0.03) 4px
  );
  pointer-events: none;
  z-index: 101;
}
```

### Screen Glow

```css
.device-screen {
  box-shadow:
    inset 0 0 30px rgba(0,0,0,0.5),
    0 0 40px rgba(100, 180, 255, 0.03);  /* Subtle blue glow */
}
```

### Reduced Motion Mode

```css
@media (prefers-reduced-motion: reduce) {
  .device-screen::after {
    display: none;  /* Remove scan lines */
  }

  * {
    animation-duration: 0.001ms !important;
    transition-duration: 0.001ms !important;
  }
}
```

---

## Loading States

### VHS Tracking Animation

```
┌─────────────────────────────────────┐
│ ═══════════════════════════════════ │  ← Horizontal distortion
│                                     │
│        ════════════════════         │
│                                     │
│ ══════════════════                  │
│                                     │
│            ══════════════════════   │
│                                     │
└─────────────────────────────────────┘
```

```css
@keyframes vhs-tracking {
  0% {
    clip-path: polygon(
      0 0, 100% 0, 100% 10%, 95% 10%, 95% 15%, 100% 15%,
      100% 40%, 90% 40%, 90% 45%, 100% 45%, 100% 70%,
      92% 70%, 92% 75%, 100% 75%, 100% 100%, 0 100%
    );
  }
  25% {
    clip-path: polygon(
      0 0, 100% 0, 100% 20%, 88% 20%, 88% 25%, 100% 25%,
      100% 50%, 94% 50%, 94% 55%, 100% 55%, 100% 80%,
      91% 80%, 91% 85%, 100% 85%, 100% 100%, 0 100%
    );
  }
  /* ... more keyframes for continuous distortion */
}

.loading-vhs {
  animation: vhs-tracking 0.5s steps(4) infinite;
  position: relative;
}

.loading-vhs::before {
  content: '';
  position: absolute;
  inset: 0;
  background: repeating-linear-gradient(
    0deg,
    transparent 0px,
    rgba(255,255,255,0.03) 1px,
    transparent 2px
  );
  animation: vhs-noise 0.2s steps(10) infinite;
}
```

### Progress Bar (Film Strip)

```
┌─────────────────────────────────────────────────────┐
│  ○   ○   ○   ○   ○   ○   ○   ○   ○   ○   ○   ○     │  ← Sprocket holes
│ ████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │  ← Progress
│  ○   ○   ○   ○   ○   ○   ○   ○   ○   ○   ○   ○     │
└─────────────────────────────────────────────────────┘
```

```css
.progress-film {
  background: #1a1a20;
  border-radius: 4px;
  height: 24px;
  overflow: hidden;
  position: relative;
}

.progress-film::before,
.progress-film::after {
  content: '';
  position: absolute;
  left: 0;
  right: 0;
  height: 6px;
  background: repeating-linear-gradient(
    90deg,
    transparent 0px,
    transparent 8px,
    #2a2a30 8px,
    #2a2a30 12px,
    transparent 12px,
    transparent 20px
  );
}

.progress-film::before { top: 0; }
.progress-film::after { bottom: 0; }

.progress-fill {
  background: var(--accent-primary);
  height: 100%;
  transition: width 0.3s ease;
}
```

### Skeleton Loader (Poster)

```css
.skeleton-poster {
  aspect-ratio: 2/3;
  background: linear-gradient(
    90deg,
    #1a1a20 0%,
    #2a2a30 50%,
    #1a1a20 100%
  );
  background-size: 200% 100%;
  animation: skeleton-shimmer 1.5s ease infinite;
}

@keyframes skeleton-shimmer {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}
```

---

## Error States

### Device Malfunction

```
┌─────────────────────────────────────┐
│ ╔═══════════════════════════════╗   │
│ ║  CONTROL TERMINAL     ● ●̇ ●   ║   │  ← Blinking amber LED
│ ╠═══════════════════════════════╣   │
│ ║ ┌───────────────────────────┐ ║   │
│ ║ │ ░░▒▒░░▓▓░░▒▒░░▓▓░░▒▒░░▓▓░ │ ║   │  ← Static noise
│ ║ │ ▓▓░░▒▒░░▓▓░░▒▒░░▓▓░░▒▒░░▓ │ ║   │
│ ║ │                           │ ║   │
│ ║ │      SIGNAL LOST          │ ║   │  ← Error message
│ ║ │   TMDB unreachable        │ ║   │
│ ║ │                           │ ║   │
│ ║ │      [ RETRY ]            │ ║   │
│ ║ └───────────────────────────┘ ║   │
│ ╚═══════════════════════════════╝   │
└─────────────────────────────────────┘
```

```css
@keyframes static-noise {
  0%, 100% { opacity: 0.1; }
  50% { opacity: 0.15; }
}

@keyframes led-blink {
  0%, 100% { background: var(--led-amber); box-shadow: 0 0 8px var(--led-amber); }
  50% { background: var(--led-off); box-shadow: none; }
}

.device-error .device-screen::before {
  background:
    url("data:image/svg+xml,%3Csvg viewBox='0 0 100 100' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)'/%3E%3C/svg%3E");
  animation: static-noise 0.1s steps(5) infinite;
  opacity: 0.1;
  z-index: 99;
}

.device-error .led-status {
  animation: led-blink 1s ease infinite;
}
```

### No Signal Placeholder (Missing Poster)

```
┌─────────────────────┐
│                     │
│    ╱╲    ╱╲    ╱╲  │  ← Color bars pattern
│   ╱  ╲  ╱  ╲  ╱  ╲ │
│  ╱    ╲╱    ╲╱    ╲│
│                     │
│     NO SIGNAL       │
│                     │
└─────────────────────┘
```

```css
.no-signal {
  align-items: center;
  background:
    linear-gradient(90deg,
      #fff 0%, #fff 14.28%,
      #ff0 14.28%, #ff0 28.57%,
      #0ff 28.57%, #0ff 42.85%,
      #0f0 42.85%, #0f0 57.14%,
      #f0f 57.14%, #f0f 71.42%,
      #f00 71.42%, #f00 85.71%,
      #00f 85.71%, #00f 100%
    );
  display: flex;
  filter: saturate(0.7) brightness(0.6);
  justify-content: center;
}

.no-signal::after {
  background: rgba(0,0,0,0.7);
  color: var(--text-primary);
  content: 'NO SIGNAL';
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  padding: 4px 12px;
}
```

---

## Poster Treatment Options

Test during design phase, pick based on visual harmony:

### Option 1: As-Is (Full Color)

```css
.poster-natural {
  /* No filter, full TMDB quality */
  border-radius: 2px;
}
```

### Option 2: Analog Warmth

```css
.poster-analog {
  filter:
    sepia(0.08)
    saturate(0.95)
    contrast(1.05)
    brightness(0.98);
}
```

### Option 3: VHS Degradation

```css
.poster-vhs {
  filter:
    saturate(1.1)
    contrast(1.1)
    brightness(0.95);
  box-shadow:
    -1px 0 0 rgba(255, 50, 50, 0.15),
    1px 0 0 rgba(50, 50, 255, 0.15);
}
```

### Option 4: CRT Phosphor

```css
.poster-crt {
  border-radius: 4px;
  box-shadow:
    inset 0 0 20px rgba(0,0,0,0.3);
  filter: contrast(1.05);
}

.poster-crt::after {
  /* Subtle RGB pixel pattern */
  content: '';
  position: absolute;
  inset: 0;
  background: url('rgb-mask.png');
  mix-blend-mode: overlay;
  opacity: 0.05;
}
```

---

## Sound Design

### Sound Mapping

| Action | Sound Type | Character |
|--------|------------|-----------|
| Button press | Click | Mechanical, tactile |
| Button release | Clack | Softer than press |
| Toggle switch | Chunk | Heavy, satisfying |
| Slider move | Soft tick | Subtle, continuous |
| Node select | Soft beep | Electronic, clean |
| Node add | Record blip | VHS "rec" sound |
| Node delete | Erase whir | Quick tape sound |
| Mode switch | Relay click | Heavy switch |
| Error | Static burst | Brief, attention-getting |
| Success save | Satisfying click | Mechanical confirmation |
| Search start | Tape seek | Brief whir |
| Search complete | Soft tone | Pleasant, not jarring |

### Sound Principles

```
DO:
✓ Keep sounds SHORT (50-200ms for UI, up to 500ms for feedback)
✓ Layer sounds subtly (click + electronic tone)
✓ Match visual intensity (small action = quiet sound)
✓ Provide audio OFF option

DON'T:
✗ Add ambient/continuous sounds (CRT hum)
✗ Use harsh frequencies
✗ Make sounds required for operation
✗ Play sounds on hover (only on action)
```

### Implementation

```typescript
// src/lib/services/sound.ts

const sounds = {
  click: new Audio('/sounds/click.mp3'),
  toggle: new Audio('/sounds/toggle.mp3'),
  error: new Audio('/sounds/error.mp3'),
  // ...
};

// Preload all sounds
Object.values(sounds).forEach(audio => {
  audio.load();
  audio.volume = 0.3;  // Default volume
});

export function playSound(name: keyof typeof sounds) {
  if (!get(configStore).soundEnabled) return;

  const audio = sounds[name];
  audio.currentTime = 0;
  audio.play().catch(() => {});  // Ignore autoplay errors
}
```

---

## Animation Guidelines

### Timing Functions

```css
/* Mechanical feel - quick start, deliberate stop */
--ease-mechanical: cubic-bezier(0.2, 0, 0.2, 1);

/* Soft electronic - smooth throughout */
--ease-soft: cubic-bezier(0.4, 0, 0.2, 1);

/* Snappy - quick response */
--ease-snappy: cubic-bezier(0.2, 0, 0, 1);
```

### Duration Scale

```css
--duration-instant: 50ms;   /* Button press visual */
--duration-fast: 100ms;     /* Toggle, small state changes */
--duration-normal: 200ms;   /* Mode switches, panel slides */
--duration-slow: 400ms;     /* Device power on, large transitions */
```

### Animation Examples

#### Device Power On

```css
@keyframes device-boot {
  0% {
    opacity: 0;
    filter: brightness(0);
  }
  30% {
    opacity: 1;
    filter: brightness(0.3);
  }
  60% {
    filter: brightness(1.2);
  }
  100% {
    filter: brightness(1);
  }
}

.device-boot {
  animation: device-boot 0.6s var(--ease-mechanical);
}
```

#### Node Appear

```css
@keyframes node-record {
  0% {
    opacity: 0;
    transform: scale(0.8);
  }
  50% {
    opacity: 1;
    transform: scale(1.05);
  }
  100% {
    transform: scale(1);
  }
}

.node-new {
  animation: node-record 0.3s var(--ease-soft);
}
```

---

## Iconography

### Icon Style Guidelines

- **Stroke weight:** 1.5-2px
- **Corners:** Slightly rounded (2px radius)
- **Style:** Geometric, minimal detail
- **Size grid:** 16px, 20px, 24px

### Core Icons

```
Navigation:
├── Pan (hand / drag)
├── Zoom In (+)
├── Zoom Out (-)
├── Fit to View (frame corners)
└── Fullscreen (expand arrows)

Actions:
├── Add (+ in circle)
├── Delete (trash / X)
├── Edit (pencil)
├── Save (floppy disk - period appropriate!)
└── Undo (curved arrow left)
├── Redo (curved arrow right)

Transport (History):
├── Rewind (◄◄)
├── Back (◄)
├── Pause (▌▌)
├── Forward (►)
└── Fast Forward (►►)

Status:
├── Watched (checkmark in circle)
├── Watching (play in circle)
├── Want to Watch (clock / bookmark)
├── Dropped (X in circle)

Device:
├── Power (circle with line)
├── Settings (gear)
├── Sound On (speaker with waves)
└── Sound Off (speaker with X)

Content:
├── Movie (clapperboard)
├── Actor (person silhouette)
├── Search (magnifying glass)
└── Filter (funnel / sliders)
```

### LED Indicator States

```css
.indicator-power    { /* Green when on */ }
.indicator-activity { /* Amber when processing */ }
.indicator-record   { /* Red when recording/saving */ }
.indicator-error    { /* Red blinking on error */ }
.indicator-network  { /* Blue when connected */ }
```

---

## Accessibility

### Color Contrast

All text must meet WCAG AA (4.5:1 for normal text, 3:1 for large text):

| Element | Foreground | Background | Ratio |
|---------|------------|------------|-------|
| Primary text | #e8e6e3 | #1a1a1e | 11.5:1 ✓ |
| Secondary text | #9a9590 | #1a1a1e | 5.2:1 ✓ |
| Screen text | #c4e0c4 | #0f1218 | 10.8:1 ✓ |
| Button text | #e8e6e3 | #3a3a40 | 7.2:1 ✓ |

### Focus Indicators

```css
:focus-visible {
  outline: 2px solid var(--accent-primary);
  outline-offset: 2px;
}

/* Higher contrast for buttons */
.button:focus-visible {
  outline: 2px solid #fff;
  outline-offset: 2px;
  box-shadow: 0 0 0 4px rgba(224, 120, 80, 0.5);
}
```

### Screen Reader Support

```html
<!-- Device title -->
<div role="region" aria-label="Graph Monitor">

<!-- Mode buttons -->
<div role="tablist" aria-label="Control Terminal Modes">
  <button role="tab" aria-selected="true">Search</button>
  <button role="tab" aria-selected="false">Inspect</button>
</div>

<!-- LED indicators -->
<span class="led active" role="status" aria-label="Recording active"></span>

<!-- Loading state -->
<div aria-live="polite" aria-busy="true">Loading search results...</div>
```

### Reduced Motion

```css
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.001ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.001ms !important;
  }

  .vhs-effect,
  .scan-lines,
  .static-noise {
    display: none;
  }
}
```

---

## Asset Requirements

### Images

| Asset | Format | Sizes | Notes |
|-------|--------|-------|-------|
| App icon | PNG, ICO | 16, 32, 64, 128, 256, 512 | All platforms |
| App icon | ICNS | macOS bundle | Mac only |
| Splash screen | PNG | 800x600 | Optional |
| No Signal pattern | SVG | Scalable | Color bars |
| RGB mask | PNG | 4x4 tileable | CRT effect |

### Fonts

| Font | Weights | License |
|------|---------|---------|
| Orbitron | 400, 500, 700 | Open Font License (Google Fonts) |
| Microgramma (fallback) | Regular, Bold | Commercial (URW) |
| Share Tech Mono | Regular | Open Font License (Google Fonts) |
| Roboto (fallback) | Regular, Medium, Bold | Open Font License |

### Sounds

| Sound | Format | Duration | Notes |
|-------|--------|----------|-------|
| click.mp3 | MP3 | ~50ms | Button press |
| toggle.mp3 | MP3 | ~100ms | Switch flip |
| error.mp3 | MP3 | ~200ms | Static burst |
| record.mp3 | MP3 | ~150ms | VHS record blip |
| seek.mp3 | MP3 | ~200ms | Tape seek |
| save.mp3 | MP3 | ~100ms | Confirm sound |

---

## Design Tokens (CSS Variables)

```css
:root {
  /* Colors - Background */
  --color-board-bg: #1a1a1e;
  --color-device-body: #2d2d32;
  --color-device-bezel: #232328;
  --color-screen-bg: #0f1218;

  /* Colors - Text */
  --color-text-primary: #e8e6e3;
  --color-text-secondary: #9a9590;
  --color-text-disabled: #5c5955;
  --color-text-screen: #c4e0c4;

  /* Colors - Accent */
  --color-accent-primary: #e07850;
  --color-accent-secondary: #50a8e0;
  --color-success: #50c070;
  --color-warning: #e0a850;
  --color-error: #e05050;

  /* Colors - LED */
  --color-led-off: #3a3530;
  --color-led-red: #ff3030;
  --color-led-green: #30ff50;
  --color-led-amber: #ffaa30;
  --color-led-blue: #3080ff;

  /* Typography */
  --font-ui: 'Orbitron', 'Microgramma', 'Roboto', sans-serif;
  --font-mono: 'Share Tech Mono', monospace;

  /* Spacing */
  --space-xs: 4px;
  --space-sm: 8px;
  --space-md: 16px;
  --space-lg: 24px;
  --space-xl: 32px;

  /* Border Radius */
  --radius-sm: 2px;
  --radius-md: 4px;
  --radius-lg: 8px;

  /* Shadows */
  --shadow-device: 0 8px 24px rgba(0,0,0,0.4);
  --shadow-button: 0 4px 0 #1a1a20;
  --shadow-screen-inset: inset 0 0 30px rgba(0,0,0,0.5);

  /* Animation */
  --ease-mechanical: cubic-bezier(0.2, 0, 0.2, 1);
  --ease-soft: cubic-bezier(0.4, 0, 0.2, 1);
  --duration-fast: 100ms;
  --duration-normal: 200ms;
}
```

---

## Figma/Design File Structure

```
Movie Graph Design System
├── 🎨 Foundations
│   ├── Colors
│   ├── Typography
│   ├── Icons
│   └── Effects (shadows, glows)
│
├── 🧩 Components
│   ├── Buttons
│   ├── Inputs
│   ├── Sliders
│   ├── Toggles
│   ├── Cards
│   ├── LEDs & Indicators
│   └── Loading States
│
├── 📦 Devices
│   ├── Graph Monitor
│   ├── Control Terminal
│   └── Status Bar
│
├── 📱 Screens
│   ├── Empty State
│   ├── Graph View
│   ├── Search Mode
│   ├── Inspect Mode
│   ├── Filter Mode
│   ├── Error States
│   └── Settings Modal
│
├── 🎬 Prototypes
│   ├── Add Movie Flow
│   ├── Expand Graph Flow
│   └── Error Recovery Flow
│
└── 📋 Specs
    ├── Component Specs
    ├── Animation Specs
    └── Sound Mapping
```

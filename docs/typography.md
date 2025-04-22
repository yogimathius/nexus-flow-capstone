## 🎨 Color Palette (Dark Mode First, Light Optional Later)

**Primary Accent**:

- `#7C3AED` – Vivid Violet (spiritual, modern, expressive)
- Alt: `#A855F7` – Orchid Purple (secondary shade for hover/focus)

**Foreground Neutrals**:

- `#F4F4F5` – Ultra-light neutral for text (white-leaning but soft)
- `#D4D4D8` – Muted Gray for secondary text
- `#A1A1AA` – Tertiary/subtext

**Background Layers**:

- `#0F0F12` – Ultra-dark background (feels immersive and cinematic)
- `#1A1A1D` – Elevated surfaces/cards
- `#27272A` – Buttons/input background
- `#3F3F46` – Borders/dividers

**Success/Info/Warning/Error**:

- Success: `#22C55E`
- Info: `#0EA5E9`
- Warn: `#FACC15`
- Error: `#EF4444`

---

## 🧱 Layout + Spacing Rules

- **Spacing Unit**: Use a consistent `4px` base (4/8/12/16/20/24…)
- **Max Width Container**: `768px` centered
- **Grid system**: 12-column for flexibility (especially on results)

---

## 🅰️ Typography

**Font Stack** (Use Google Fonts or local if possible):

```css
font-family: "Inter", "ui-sans-serif", "system-ui", sans-serif;
```

### Font Sizes (Tailwind-based scale for reference)

| Use                 | Size    | Weight   | Notes                  |
| ------------------- | ------- | -------- | ---------------------- |
| H1 (hero)           | 36–40px | Bold     | App title / landing    |
| H2                  | 28–32px | Semibold | Section titles         |
| H3                  | 22–24px | Semibold | Subsections            |
| Body (default)      | 16px    | Normal   | All standard text      |
| Small / Metadata    | 13–14px | Medium   | Labels, helper text    |
| Mono (AI responses) | 14px    | Normal   | Use `'JetBrains Mono'` |

---

## 🧩 Component Guidelines (Stylance-ready)

> All of this is intended to layer cleanly with Stylance’s tokens system. If needed, I can give you the Stylance-compatible tokens YAML or JSON.

### Buttons

- **Primary**: `violet-500 bg`, white text, soft shadow, pill shape
- **Secondary**: `neutral-700 bg`, `neutral-100` text
- **Hover**: Shift brightness + scale (subtle)

```css
padding: 10px 18px;
border-radius: 9999px;
transition: 0.2s ease;
```

### Inputs / Fields

- Rounded corners (`md`)
- Focus ring in `#A855F7`
- Label: `uppercase`, `text-xs`, `tracking-wide`, muted gray

---

## 🌘 Shadow + Depth System

| Layer     | Shadow                        |
| --------- | ----------------------------- |
| Base card | `0 1px 2px rgba(0,0,0,0.1)`   |
| Elevated  | `0 4px 6px rgba(0,0,0,0.15)`  |
| Modal     | `0 10px 20px rgba(0,0,0,0.3)` |

---

## 🧠 Symbolic Touches (Unique to Your App)

- Use **subtle glyphs or archetypal icons** (e.g., dream eye, feather, labyrinth) as watermark elements.
- Optional SVGs as background overlays (10% opacity, blurred).

---

## 🔄 Transitions

- Use motion intentionally (but minimally).
- Examples:
  - Page transitions: `fade-in-up` (50ms delay)
  - Button hover: `scale(1.02)` and `box-shadow`
  - Input focus: `border + glow`

---

## ✅ MVP v1.5 Checklist

- [x] Unified color theme
- [x] Responsive layout (mobile-first)
- [x] Reusable components via Stylance
- [x] Modal system (dream result overlay?)
- [x] Search bar / input with autocomplete
- [x] Symbol result grid or stacked list with tags
- [x] Accessible typography scale
- [x] Dark mode as default

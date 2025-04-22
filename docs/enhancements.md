## ✅ **Dream UI Enhancement Checklist** (v1.5 Theme Polish)

### 🎨 Visual Design & Aesthetic

- [ ] **Typography Hierarchy**

  - [ ] Use `Inter` or `DM Sans` for body/UI text
  - [ ] Add serif or display font (`Playfair`, `Spectral`, or `DM Serif Display`) for interpretation/poetic sections
  - [ ] Scale `h1` to 36–40px, `h2` to 28–32px for better hierarchy

- [ ] **Symbolic Background Motifs**

  - [ ] Add SVG watermark/icon (e.g., feather, moon, labyrinth) behind interpretation box
  - [ ] Consider subtle background elements in `SymbolDetail` and `Interpret` pages

- [ ] **Section Framing & Glows**
  - [ ] Add inner glow or soft blur shadow around interpretation output box
  - [ ] Create section dividers using glyphs (⋆ ⋆ ⋆ or ⋯) or gradient lines

---

### ✨ Motion & Interaction

- [ ] **Component Transitions**

  - [ ] Add hover/active/focus transitions for `Badge`, `Button`, `SymbolCard`
    ```css
    transition: all 0.2s ease;
    transform: scale(1.02);
    ```
  - [ ] Use smooth fade/scale-in for detail panels

- [ ] **Page Animations**
  - [ ] `/symbols` should animate in `SymbolCard`s with staggered fade
  - [ ] `/symbols/:id` should slide or crossfade the detail pane
  - [ ] `/interpret` results grow in (opacity + scale)

---

### 🧩 Component Enhancements

- [ ] **SymbolCard**

  - [ ] Add hover scaling or glow
  - [ ] Optional: subtle background watermark per category

- [ ] **InterpretationResult**

  - [ ] Use display font
  - [ ] Add background motif + glowing border
  - [ ] Animate in on result display

- [ ] **ThemeToggle**

  - [ ] Use icon-based toggle (`🌙` / `☀️`)
  - [ ] Animate rotation or pulse on toggle

- [ ] **Badge**
  - [ ] Support hover/focus interaction
  - [ ] Color mapping should pull from category → brand palette

---

### 📁 Assets & Style Utilities

- [ ] **Add glyph/icon set** (Feather icons, phosphor, or custom SVG)
- [ ] **Optional Tailwind extension**
  - Custom utilities: `dream-glow`, `dream-bg`, `dream-divider`
  - Gradient tokens for `mythological`, `emotion`, `nature`

---

### 👁️ Experience Review

- [ ] Run visual QA pass on `/`, `/symbols`, `/symbols/:id`, `/interpret`
- [ ] Confirm component reusability and motion is consistent across routes
- [ ] Check type, spacing, and color contrast in light/dark modes

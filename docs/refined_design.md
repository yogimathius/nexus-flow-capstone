## 🎯 Top Priority Core Components (Build First)

These are foundational and power your symbol list + search UX.

### 1. **`SearchBar`**

- ✅ Props: `on_query_change`, `on_category_change`, `value`, `categories`
- 🔍 Debounce input + call MCP `get_symbols`
- 🧠 Optional: integrate fuzzy matching on frontend fallback

> **Build this as a controlled component** so it can be used in both `/symbols` and `/interpret`.

---

### 2. **`SymbolCard`**

- ✅ Props: `name`, `category`, `description`, `on_click`
- 🧠 Include a `Badge` subcomponent for category
- 💅 Add hover + transition for modern UX
- 🔗 Clicking should navigate to `/symbols/:id`

> **Use this both in search results and related symbols.** Add visual contrast on hover (border or elevation).

---

### 3. **`Badge`**

- ✅ Props: `label`, `color` (optional)
- 🎨 Pull from category → color mapping if not provided
- 💎 Style: pill shape, medium contrast text

> Useful everywhere: filters, metadata, relationships.

---

## 🧩 Detail Page Enhancers (v1.5 Polish Layer)

Once symbol detail is wired up, layer in these:

### 4. **`TabView` / `TabsContainer`**

- ✅ Switch between sections: interpretations / metadata / related
- 🧠 Prop: `tabs: &[(&'static str, View)]`
- ⚡ Optional: animate transitions with `leptos-motion` if needed

---

### 5. **`Disclosure` (Accordion)**

- ✅ For long interpretations, let user expand chunks
- Props: `title`, `content`
- Works great for:
  - Archetypal interpretations
  - Cultural interpretations

---

### 6. **`RelatedSymbolsList`**

- ✅ Props: `Vec<Symbol>` → renders horizontal `SymbolCard`s or mini-tags
- 🔁 Internally reuses `SymbolCard` or `Badge`
- 🤝 Consider graph view in future, list now

---

## 🛠️ UI Utility Components

### 7. **`ThemeToggle`**

- Toggles between dark/light or `data-theme`
- 🌓 Icon-based button

### 8. **`Tooltip` (Optional)**

- Very useful for explaining terms like "archetype", "pantheon"
- Only if you have extra time — slightly tricky in Leptos

### 9. **`Modal` (Stretch Goal)**

- If you want to preview full symbol info without navigating
- Consider only _after_ MVP is stable

---

## 🧱 Suggested Build Order for v1.5

| Stage | Component                                       | Purpose                        |
| ----- | ----------------------------------------------- | ------------------------------ |
| 1     | `SearchBar`, `SymbolCard`, `Badge`              | Base search + browse UI        |
| 2     | `SymbolDetail`, `TabView`, `RelatedSymbolsList` | Enrich detail UX               |
| 3     | `Disclosure`, `ThemeToggle`                     | Polish and accessibility       |
| 4     | `Tooltip`, `Modal`, `Pagination`                | Nice-to-haves or stretch goals |

---

## 💡 Design Refinement Suggestions

- Use a **consistent spacing + type scale** across components
- Use Tailwind, UnoCSS, or even Leptos' `style!` for styling standardization
- Ensure **all components accept class overrides** if you're not using Stylance (for future theming flexibility)

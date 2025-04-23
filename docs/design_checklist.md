# Dream Interpreter Component Build Checklist

## Phase 1: Core Components & Foundation (Days 1-3)

- [x] 1. **Badge Component**

  - [x] Create badges that display categories with appropriate colors
  - [x] Ensure reusability for tags, metadata, and filters
  - [x] Add hover state and accessibility attributes

- [ ] 2. **SearchBar Component**

  - [ ] Implement input with debounce functionality
  - [ ] Add category dropdown integration
  - [ ] Wire up `on_query_change` and `on_category_change` events
  - [ ] Style consistently with design system

- [ ] 3. **SymbolCard Component**
  - [ ] Design card layout with name, category badge, and description
  - [ ] Add hover effects and transitions
  - [ ] Implement click navigation to detail page
  - [ ] Ensure responsive behavior for different screen sizes

## Phase 2: Detail & Navigation Components (Days 4-6)

- [ ] 4. **TabView Component**

  - [ ] Create tab container with selectable headers
  - [ ] Implement content switching based on active tab
  - [ ] Style with smooth transitions between tabs

- [ ] 5. **RelatedSymbolsList Component**

  - [ ] Design horizontal scrollable list of related symbols
  - [ ] Reuse SymbolCard in compact format
  - [ ] Add navigation to related symbol details

- [ ] 6. **Disclosure Component**
  - [ ] Build expandable/collapsible sections
  - [ ] Implement for interpretations and longer content
  - [ ] Add animation for expanding/collapsing

## Phase 3: Page Integration & Polish (Days 7-9)

- [ ] 7. **Layout Setup**

  - [ ] Create Header with navigation
  - [ ] Add simple Footer
  - [ ] Set up responsive container layout

- [ ] 8. **Main Page Views**

  - [ ] Implement SymbolList page with search and filtering
  - [ ] Create SymbolDetail page with tabs and related symbols
  - [ ] Add basic Home page with introduction and featured symbols
  - [ ] Build simple Interpret page for dream text input

- [ ] 9. **ThemeToggle & Final Polish**
  - [ ] Add dark/light mode toggle
  - [ ] Ensure consistent styling across all components
  - [ ] Test responsiveness on different device sizes
  - [ ] Fix any accessibility issues

## Optional Extras (If Time Permits)

- [ ] 10. **Tooltip Component** for explaining specialized terms
- [ ] 11. **Modal Component** for quick previews without navigation
- [ ] 12. **Pagination Component** for browsing larger symbol datasets

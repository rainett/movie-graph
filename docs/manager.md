# Movie Graph - Project Management Guide

## Project Overview

### Vision

A desktop application for creating visual graphs of movies and TV shows, exploring connections through actors. Users build personal discovery trees starting from movies they love, branching through actors to find new content.

### Key Differentiators

- **No backend** - All data stored locally as files (like Obsidian)
- **80s device aesthetic** - Distinctive UI that feels tangible
- **Offline-first** - Works without internet after initial data fetch
- **User owns data** - Export, backup, version control friendly

### Target Users

- Cinephiles who want to explore actor filmographies
- Users who prefer visual/spatial organization
- Privacy-conscious users who want local-first apps
- Both casual users and power users (progressive disclosure)

---

## Project Scope

### In Scope (v1.0)

| Feature | Priority | Notes |
|---------|----------|-------|
| Project create/open/save | P0 | Core functionality |
| Graph canvas with nodes | P0 | Svelte Flow integration |
| TMDB search and data fetch | P0 | Movies and actors |
| Add movie/actor nodes | P0 | From search results |
| Connect nodes (edges) | P0 | Movie <-> Actor |
| Auto-layout | P0 | ELK.js integration |
| Manual node positioning | P0 | Drag and save |
| Poster display and caching | P0 | Local image storage |
| Undo/redo | P1 | Command pattern |
| Filter by status/rating | P1 | Dim non-matching |
| Status tracking | P1 | Watched, want to watch, etc. |
| Personal ratings | P1 | 1-10 scale |
| Notes per node | P1 | Free text |
| Change poster | P2 | Alternatives + upload |
| Keyboard shortcuts | P2 | Full keyboard support |
| Sound feedback | P2 | 80s device sounds |
| Tutorial mode | P2 | Step-by-step lessons |
| Multiple projects | P2 | Recent projects list |

### Out of Scope (v1.0)

| Feature | Rationale | Planned For |
|---------|-----------|-------------|
| Cloud sync | Complexity, needs backend | v2.0+ |
| Mobile app | Different platform | v2.0+ |
| Collaborative editing | Major complexity | Maybe never |
| TV show episodes | Scope creep | v1.5 |
| Plugin system | Architecture ready, UI not | v1.5 |
| Import from Letterboxd | Nice-to-have | v1.2 |
| AI recommendations | Separate feature | v2.0+ |

---

## Milestones

### Milestone 1: Foundation (Weeks 1-2)

**Goal:** Basic app runs, can create and save empty project

**Deliverables:**
- [ ] Tauri + Svelte project scaffolded
- [ ] Build pipeline working (Windows, Mac, Linux)
- [ ] File structure defined and implemented
- [ ] Create new project (folder picker, structure creation)
- [ ] Open existing project
- [ ] Basic save/load of manifest
- [ ] Device board layout (empty frames)

**Acceptance Criteria:**
- App launches on all platforms
- Can create project in any folder
- Can reopen project after restart
- No console errors

---

### Milestone 2: Graph Core (Weeks 3-4)

**Goal:** Visible graph with placeholder nodes, basic interaction

**Deliverables:**
- [ ] Svelte Flow integrated
- [ ] Custom movie node component (placeholder poster)
- [ ] Custom actor node component (placeholder photo)
- [ ] Pan and zoom working
- [ ] Click to select node
- [ ] Node positions saved and restored
- [ ] Basic edge rendering
- [ ] Graph Monitor device frame styled

**Acceptance Criteria:**
- Can add hardcoded test nodes
- Nodes display correctly
- Pan/zoom smooth at 100 nodes
- Positions persist after reload

---

### Milestone 3: TMDB Integration (Weeks 5-6)

**Goal:** Search and fetch real movie/actor data

**Deliverables:**
- [ ] TMDB client in Rust
- [ ] API response caching
- [ ] Search movies command
- [ ] Search actors command
- [ ] Get movie details with cast
- [ ] Get actor details with filmography
- [ ] Control Terminal Search mode UI
- [ ] Search results display with posters

**Acceptance Criteria:**
- Search returns results within 500ms (cached)
- Results show poster thumbnails
- API errors handled gracefully
- Rate limiting handled (retry/backoff)

---

### Milestone 4: Graph Building (Weeks 7-8)

**Goal:** Users can build graphs from search

**Deliverables:**
- [ ] Add movie from search
- [ ] Add actor from movie's cast
- [ ] Add movie from actor's filmography
- [ ] Auto-create edges on add
- [ ] ELK.js auto-layout integration
- [ ] Control Terminal Inspect mode UI
- [ ] Node detail display

**Acceptance Criteria:**
- Complete flow: search -> add movie -> add actor -> add another movie
- Graph auto-layouts after additions
- Edges correctly connect nodes
- No orphan nodes or edges

---

### Milestone 5: Images (Weeks 9-10)

**Goal:** Posters and photos display correctly

**Deliverables:**
- [ ] Image download and caching
- [ ] Image resize/compression
- [ ] Lazy loading on viewport
- [ ] Poster alternatives fetch
- [ ] Custom image upload
- [ ] Poster picker UI
- [ ] No-signal placeholder for failures

**Acceptance Criteria:**
- Posters display on nodes
- Images cached locally
- Works offline with cached images
- Custom upload resizes correctly
- No memory leaks with many images

---

### Milestone 6: History System (Weeks 11-12)

**Goal:** Undo/redo working

**Deliverables:**
- [ ] Command pattern implementation
- [ ] Add node command
- [ ] Delete node command
- [ ] Edit node command
- [ ] Move node command
- [ ] Add/delete edge commands
- [ ] History strip UI (transport controls)
- [ ] Keyboard shortcuts (Ctrl+Z, Ctrl+Y)

**Acceptance Criteria:**
- Can undo all user actions
- Redo works after undo
- History survives mode switches
- No state corruption after rapid undo/redo

---

### Milestone 7: Filtering & Status (Weeks 13-14)

**Goal:** Filter graph, track watch status

**Deliverables:**
- [ ] Filter store and logic
- [ ] Status enum on nodes
- [ ] Rating field on nodes
- [ ] Filter mode UI
- [ ] Status/rating edit in Inspect mode
- [ ] Dim non-matching nodes
- [ ] Filter active indicator
- [ ] Notes field on nodes

**Acceptance Criteria:**
- All filter combinations work
- Dimming visually clear
- Status persists after reload
- Can filter by multiple criteria

---

### Milestone 8: Polish & UX (Weeks 15-17)

**Goal:** App feels complete and polished

**Deliverables:**
- [ ] 80s device styling complete
- [ ] All screen effects (subtle CRT, warmth)
- [ ] Sound design integration
- [ ] Loading states (VHS tracking)
- [ ] Error states (malfunction aesthetic)
- [ ] Keyboard shortcuts complete
- [ ] Settings modal
- [ ] Auto-save with indicator
- [ ] Backup system
- [ ] Crash reporting

**Acceptance Criteria:**
- Visual design matches spec
- Sounds play on actions (when enabled)
- All loading states have feedback
- Errors handled gracefully with recovery
- Settings persist

---

### Milestone 9: Tutorial & Onboarding (Week 18)

**Goal:** New users can learn the app

**Deliverables:**
- [ ] Tutorial mode architecture
- [ ] Lesson 1: Navigation
- [ ] Lesson 2: Adding movies
- [ ] Lesson 3: Building connections
- [ ] Lesson 4: Organization
- [ ] Lesson 5: Customization
- [ ] First-launch prompt
- [ ] Empty state guidance

**Acceptance Criteria:**
- Tutorial completable end-to-end
- Can exit tutorial anytime
- Progress saved between sessions
- Not intrusive for returning users

---

### Milestone 10: Release Prep (Weeks 19-20)

**Goal:** Ready for public release

**Deliverables:**
- [ ] All P0 and P1 features complete
- [ ] Bug fixes from testing
- [ ] Performance optimization
- [ ] Localization infrastructure (English complete)
- [ ] Build signing (Windows, Mac)
- [ ] Release notes / changelog
- [ ] Landing page / download page
- [ ] GitHub releases setup

**Acceptance Criteria:**
- No P0 or P1 bugs open
- Performance targets met
- Clean install works on all platforms
- Signed builds pass OS security

---

## Development Phases

```
Phase       Milestones    Focus
──────────────────────────────────────────────────
Foundation  M1, M2        Infrastructure, core UI
Core        M3, M4, M5    Main functionality
Polish      M6, M7, M8    UX, completeness
Ship        M9, M10       Onboarding, release
```

---

## Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Svelte Flow performance at 1000+ nodes | Medium | High | Test early (M2), plan virtualization |
| TMDB API changes/deprecation | Low | High | Abstract client, cache aggressively |
| Tauri 2.0 bugs/issues | Medium | Medium | Follow releases, have workarounds ready |
| Image memory usage | Medium | Medium | Lazy loading, thumbnail sizes, cleanup |
| Cross-platform file path issues | Medium | Low | Test on all platforms each milestone |

### Schedule Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Scope creep | High | High | Strict scope document, P0/P1/P2 prioritization |
| Design iteration delays | Medium | Medium | Start with functional, iterate visual |
| Integration bugs | Medium | Medium | Test integrations early (TMDB in M3) |
| Burnout (solo project) | Medium | High | Reasonable pace, regular breaks |

### Mitigation Strategies

**For scope creep:**
- Every feature request goes to "v1.5 or later" list
- Revisit scope only at milestone boundaries
- "Good enough" > "perfect" for v1.0

**For burnout:**
- Maximum 6 hours focused work per day
- One full day off per week
- Celebrate milestone completions

**For technical blockers:**
- Have backup plans documented
- Don't hesitate to simplify if needed
- Community/forums for Tauri/Svelte issues

---

## Quality Assurance

### Testing Strategy

| Level | When | What |
|-------|------|------|
| Unit tests | During development | Utils, store logic, Rust services |
| Integration tests | End of each milestone | Store + components, Rust commands |
| Manual testing | Each milestone | Full user flows |
| Cross-platform | M1, M5, M10 | Windows, Mac, Linux builds |
| Performance | M2, M5, M8 | Large graphs, memory usage |

### Definition of Done (per feature)

- [ ] Code complete and working
- [ ] Unit tests passing
- [ ] Manual test of happy path
- [ ] Error cases handled
- [ ] No TypeScript errors
- [ ] No Rust warnings
- [ ] Works on primary dev platform

### Definition of Done (per milestone)

- [ ] All features in milestone complete (per above)
- [ ] Integration tests passing
- [ ] Cross-platform build succeeds
- [ ] Manual testing of all milestone features
- [ ] Performance acceptable
- [ ] No regression in previous features
- [ ] Documentation updated if needed

### Bug Severity Levels

| Level | Definition | Response |
|-------|------------|----------|
| **P0 - Critical** | App crashes, data loss, security issue | Fix immediately, block release |
| **P1 - High** | Major feature broken, no workaround | Fix before milestone complete |
| **P2 - Medium** | Feature degraded, workaround exists | Fix in current or next milestone |
| **P3 - Low** | Minor issue, cosmetic | Fix when convenient |

---

## Resource Requirements

### Development Environment

| Item | Requirement |
|------|-------------|
| Primary machine | Windows 11 (main dev) |
| Secondary machine | macOS (build testing) |
| Linux testing | VM or Docker sufficient |
| IDE | VS Code with extensions |
| Design tool | Figma (free tier sufficient) |

### External Services

| Service | Cost | Purpose |
|---------|------|---------|
| TMDB API | Free | Movie/actor data |
| GitHub | Free | Code hosting, releases |
| Code signing (Windows) | ~$200-400/year | Trusted installer |
| Code signing (Mac) | $99/year | Apple Developer Program |
| Domain (optional) | ~$15/year | Landing page |

### Time Investment

| Phase | Duration | Hours/Week | Total Hours |
|-------|----------|------------|-------------|
| Foundation | 2 weeks | 25 | 50 |
| Core | 6 weeks | 30 | 180 |
| Polish | 6 weeks | 30 | 180 |
| Ship | 2 weeks | 25 | 50 |
| **Total** | **16 weeks** | - | **~460 hours** |

*Estimates assume focused solo development. Actual time will vary.*

---

## Release Planning

### Version Numbering

```
v{major}.{minor}.{patch}

major: Breaking changes, major features
minor: New features, backward compatible
patch: Bug fixes only
```

### Release Channels

| Channel | Purpose | Audience |
|---------|---------|----------|
| **Stable** | Production releases | All users |
| **Beta** | Pre-release testing | Opt-in testers |

### v1.0 Release Checklist

**Pre-release (1 week before):**
- [ ] Feature freeze
- [ ] Full regression testing
- [ ] Performance profiling
- [ ] Update all dependencies
- [ ] Security review
- [ ] Prepare release notes

**Release day:**
- [ ] Create git tag
- [ ] Build all platforms
- [ ] Sign installers
- [ ] Upload to GitHub releases
- [ ] Update download links
- [ ] Announce (if applicable)

**Post-release (1 week after):**
- [ ] Monitor crash reports
- [ ] Respond to issues
- [ ] Plan v1.0.1 hotfix if needed

### Post-v1.0 Roadmap

| Version | Focus | Key Features |
|---------|-------|--------------|
| v1.0.x | Stability | Bug fixes, performance |
| v1.1 | Quality of life | Import/export, more shortcuts |
| v1.2 | Integration | Letterboxd import, more export formats |
| v1.5 | Extensibility | Plugin system UI, additional languages |
| v2.0 | Platform | Cloud sync (optional), mobile companion |

---

## Progress Tracking

### Weekly Check-in Template

```markdown
## Week [N] - [Date Range]

### Completed
- [ ] Task 1
- [ ] Task 2

### In Progress
- [ ] Task 3 (X% done)

### Blocked
- [ ] Task 4 - blocked by [reason]

### Next Week
- [ ] Task 5
- [ ] Task 6

### Notes
- Any observations, decisions, or concerns
```

### Milestone Review Template

```markdown
## Milestone [N] Review - [Name]

### Status: [Complete / Partial / Incomplete]

### Deliverables
| Deliverable | Status | Notes |
|-------------|--------|-------|
| Feature A | Done | |
| Feature B | Done | Minor polish needed |
| Feature C | Partial | Deferred X to M[N+1] |

### Acceptance Criteria
| Criterion | Pass/Fail | Notes |
|-----------|-----------|-------|
| Criterion 1 | Pass | |
| Criterion 2 | Pass | |

### Metrics
- Lines of code: X
- Test coverage: Y%
- Open bugs: Z

### Lessons Learned
- What went well:
- What could improve:
- What to do differently:

### Decisions Made
- Decision 1: [choice] because [reason]

### Risks Identified
- New risk: [description]
```

---

## Success Metrics

### v1.0 Release Criteria

| Metric | Target |
|--------|--------|
| P0 bugs | 0 |
| P1 bugs | 0 |
| Crash rate | < 1% of sessions |
| Cold start time | < 3 seconds |
| 1000-node graph performance | 60 FPS pan/zoom |
| All P0 features | 100% complete |
| All P1 features | 100% complete |

### Post-Launch Metrics (if tracking)

| Metric | Notes |
|--------|-------|
| Download count | From GitHub releases |
| Crash reports received | Opt-in crash reporting |
| GitHub issues opened | User feedback |
| GitHub stars | Interest indicator |

---

## Communication

### Documentation Locations

| Document | Location | Purpose |
|----------|----------|---------|
| Requirements | `/docs/frontend.md` | Feature specs |
| Architecture | `/docs/architect.md` | Technical design |
| Design | `/docs/design.md` | Visual specs |
| Progress | `/docs/progress.md` | Weekly updates |
| Decisions | `/docs/decisions.md` | ADR log |
| Changelog | `/CHANGELOG.md` | User-facing changes |

### Decision Log (ADR) Template

```markdown
## ADR [N]: [Title]

**Date:** YYYY-MM-DD
**Status:** Proposed / Accepted / Deprecated

### Context
What is the issue or decision needed?

### Decision
What was decided?

### Consequences
What are the positive and negative results?

### Alternatives Considered
What other options were evaluated?
```

---

## Checklists

### Starting a New Feature

- [ ] Feature defined in scope document
- [ ] Acceptance criteria clear
- [ ] Dependencies identified
- [ ] Estimate created
- [ ] Design assets ready (if UI)
- [ ] Branch created

### Completing a Feature

- [ ] Code complete
- [ ] Tests written and passing
- [ ] Manual testing done
- [ ] Code self-reviewed
- [ ] No TypeScript/Rust errors
- [ ] Committed with clear message
- [ ] Update progress tracking

### Starting a New Milestone

- [ ] Previous milestone reviewed
- [ ] Scope confirmed
- [ ] Risks identified
- [ ] Dependencies resolved
- [ ] Environment ready

### Completing a Milestone

- [ ] All deliverables complete
- [ ] Acceptance criteria met
- [ ] Cross-platform tested
- [ ] Performance verified
- [ ] Documentation updated
- [ ] Milestone review completed
- [ ] Next milestone planned

---

## Appendix: Feature Priority Matrix

### P0 - Must Have (v1.0 blocker)

Core features without which the app is unusable:
- Create/open/save projects
- Graph canvas with pan/zoom
- TMDB search
- Add/display nodes
- Connect nodes with edges
- Auto-layout
- Save node positions
- Poster display

### P1 - Should Have (v1.0 expected)

Features users will expect:
- Undo/redo
- Filter nodes
- Status tracking (watched, etc.)
- Ratings
- Notes
- Settings persistence
- Auto-save

### P2 - Nice to Have (v1.0 if time permits)

Features that enhance experience:
- Change poster
- Full keyboard navigation
- Sound design
- Tutorial mode
- Multiple projects
- Custom poster upload

### P3 - Future (post v1.0)

Features for later versions:
- Plugin system
- Additional data sources
- Cloud sync
- Mobile app
- TV show episode tracking
- AI recommendations
- Collaborative features

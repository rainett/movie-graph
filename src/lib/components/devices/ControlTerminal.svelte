<script lang="ts">
  import { searchMovies, getMovieDetails, getPersonDetails } from '$lib/services/tauri';
  import type { MovieResult, MovieDetails, PersonDetails, MovieCredit, CastMember } from '$lib/types/tmdb';
  import { tmdbImage, releaseYear } from '$lib/types/tmdb';
  import { untrack } from 'svelte';
  import { selectionStore } from '$lib/stores/selection';
  import { graphStore } from '$lib/stores/graph';
  import type { MovieNode, ActorNode } from '$lib/types/node';
  import type { Edge } from '$lib/types/edge';

  type Mode = 'search' | 'inspect' | 'filter';
  let activeMode: Mode = $state('search');

  // ── Helpers ───────────────────────────────────────────────────
  function movieDetailsToNode(details: MovieDetails): MovieNode {
    return {
      id: `m:${details.id}`,
      tmdb_id: details.id,
      title: details.title,
      year: parseInt(releaseYear(details.release_date) || '0'),
      status: 'none',
      my_rating: null,
      poster: tmdbImage(details.poster_path, 'w200') ?? '',
      poster_options: [],
      notes: '',
      position: { x: (Math.random() - 0.5) * 600, y: (Math.random() - 0.5) * 400 },
      added_at: new Date().toISOString(),
    };
  }

  function castMemberToActorNode(member: CastMember): ActorNode {
    return {
      id: `a:${member.id}`,
      tmdb_id: member.id,
      name: member.name,
      photo: tmdbImage(member.profile_path, 'w200') ?? '',
      photo_options: [],
      notes: '',
      position: { x: (Math.random() - 0.5) * 600, y: (Math.random() - 0.5) * 400 },
      added_at: new Date().toISOString(),
    };
  }

  function makeEdge(from: string, to: string): Edge {
    return {
      id: `e:${from}-${to}`,
      from,
      to,
      relationship: 'acted_in',
      note: null,
      created_at: new Date().toISOString(),
    };
  }

  // ── Search state ──────────────────────────────────────────────
  let query = $state('');
  let searchResults: MovieResult[] = $state([]);
  let preview: MovieDetails | null = $state(null);
  let searchLoading = $state(false);
  let previewLoading = $state(false);
  let searchError: string | null = $state(null);
  let searchTimer: ReturnType<typeof setTimeout> | null = null;

  async function runSearch(q: string) {
    if (!q.trim()) {
      searchResults = [];
      preview = null;
      searchError = null;
      searchLoading = false;
      return;
    }
    searchLoading = true;
    searchError = null;
    try {
      const res = await searchMovies(q.trim());
      searchResults = res.results.slice(0, 8);
    } catch (e: unknown) {
      searchError = typeof e === 'string' ? e : 'Search failed';
      searchResults = [];
    } finally {
      searchLoading = false;
    }
  }

  function onInput(e: Event) {
    query = (e.target as HTMLInputElement).value;
    preview = null;
    if (searchTimer) clearTimeout(searchTimer);
    searchTimer = setTimeout(() => runSearch(query), 400);
  }

  async function selectSearchResult(movie: MovieResult) {
    previewLoading = true;
    preview = null;
    searchError = null;
    try {
      preview = await getMovieDetails(movie.id);
    } catch {
      preview = {
        id: movie.id,
        title: movie.title,
        release_date: movie.release_date ?? null,
        overview: movie.overview ?? null,
        poster_path: movie.poster_path ?? null,
        runtime: null,
        vote_average: movie.vote_average,
        genres: [],
        credits: { cast: [] },
      };
    } finally {
      previewLoading = false;
    }
  }

  function clearSearch() {
    query = '';
    searchResults = [];
    preview = null;
    searchError = null;
  }

  // ── Inspect state ─────────────────────────────────────────────
  let inspectLoading = $state(false);
  let inspectError: string | null = $state(null);
  let inspectedMovie: MovieDetails | null = $state(null);
  let inspectedPerson: PersonDetails | null = $state(null);
  let actorSuggestions: MovieCredit[] = $state([]);

  // React to selection changes
  $effect(() => {
    const sel = $selectionStore;
    // Use untrack so activeMode is not a dependency — avoids the effect
    // re-running (and forcing back to inspect) when the user clicks a mode tab
    untrack(() => {
      if (sel && activeMode !== 'inspect') activeMode = 'inspect';
    });
    if (!sel) {
      inspectedMovie = null;
      inspectedPerson = null;
      actorSuggestions = [];
      return;
    }
    loadInspect(sel.id, sel.type);
  });

  async function loadInspect(id: string, type: 'movie' | 'actor') {
    inspectLoading = true;
    inspectError = null;
    inspectedMovie = null;
    inspectedPerson = null;
    actorSuggestions = [];

    const tmdbId = parseInt(id.slice(2));
    if (isNaN(tmdbId)) return;

    try {
      if (type === 'movie') {
        inspectedMovie = await getMovieDetails(tmdbId);
      } else {
        const person = await getPersonDetails(tmdbId);
        inspectedPerson = person;
        actorSuggestions = buildSuggestions(person);
      }
    } catch (e: unknown) {
      inspectError = typeof e === 'string' ? e : 'Failed to load details';
    } finally {
      inspectLoading = false;
    }
  }

  function buildSuggestions(person: PersonDetails): MovieCredit[] {
    const inGraph = new Set($graphStore.movies.keys());
    return person.movie_credits.cast
      .filter((c) => !inGraph.has(`m:${c.id}`))
      .sort((a, b) => (b.popularity ?? 0) - (a.popularity ?? 0))
      .slice(0, 5);
  }

  // ── Add / remove graph actions ─────────────────────────────────
  function addMovieFromPreview() {
    if (!preview) return;
    const node = movieDetailsToNode(preview);
    if ($graphStore.movies.has(node.id)) return;
    graphStore.addMovie(node);
    // Auto-create edges to actors already in graph that are in this cast
    const castIds = new Set(preview.credits.cast.map((c) => `a:${c.id}`));
    for (const actorId of $graphStore.actors.keys()) {
      if (castIds.has(actorId)) graphStore.addEdge(makeEdge(node.id, actorId));
    }
  }

  function addActorFromCast(member: CastMember) {
    const node = castMemberToActorNode(member);
    if ($graphStore.actors.has(node.id)) return;
    graphStore.addActor(node);
    if (inspectedMovie && $graphStore.movies.has(`m:${inspectedMovie.id}`)) {
      graphStore.addEdge(makeEdge(`m:${inspectedMovie.id}`, node.id));
    }
  }

  let addingSuggestion = $state<number | null>(null);
  async function addSuggestedMovie(credit: MovieCredit) {
    if (addingSuggestion !== null) return;
    addingSuggestion = credit.id;
    try {
      let details: MovieDetails;
      try {
        details = await getMovieDetails(credit.id);
      } catch {
        details = {
          id: credit.id,
          title: credit.title,
          release_date: credit.release_date,
          overview: null,
          poster_path: credit.poster_path,
          runtime: null,
          vote_average: credit.vote_average ?? 0,
          genres: [],
          credits: { cast: [] },
        };
      }
      const node = movieDetailsToNode(details);
      if ($graphStore.movies.has(node.id)) return;
      graphStore.addMovie(node);
      // Edge goes actor → movie (discovery direction: actor is parent, movie is child)
      const sel = $selectionStore;
      if (sel?.type === 'actor') graphStore.addEdge(makeEdge(sel.id, node.id));
      if (inspectedPerson) actorSuggestions = buildSuggestions(inspectedPerson);
    } finally {
      addingSuggestion = null;
    }
  }

  function deleteInspectedNode() {
    const sel = $selectionStore;
    if (!sel) return;
    const edgesToRemove = Array.from($graphStore.edges.entries())
      .filter(([, edge]) => edge.from === sel.id || edge.to === sel.id)
      .map(([id]) => id);
    for (const id of edgesToRemove) graphStore.removeEdge(id);
    if (sel.type === 'movie') graphStore.removeMovie(sel.id);
    else graphStore.removeActor(sel.id);
    selectionStore.set(null);
  }
</script>

<div class="device" role="region" aria-label="Control Terminal">
  <div class="device-header">
    <span class="device-title">CONTROL TERMINAL</span>
    <div class="device-indicators">
      <div class="led led-green" aria-hidden="true"></div>
      <div class="led" class:led-amber={searchError || inspectError} aria-hidden="true"></div>
    </div>
  </div>

  <div role="tablist" aria-label="Terminal Modes" class="mode-bar">
    <button
      role="tab"
      aria-selected={activeMode === 'search'}
      class="mode-btn"
      class:active={activeMode === 'search'}
      onclick={() => (activeMode = 'search')}
    >SEARCH</button>
    <button
      role="tab"
      aria-selected={activeMode === 'inspect'}
      class="mode-btn"
      class:active={activeMode === 'inspect'}
      onclick={() => (activeMode = 'inspect')}
    >INSPECT</button>
    <button
      role="tab"
      aria-selected={activeMode === 'filter'}
      class="mode-btn"
      class:active={activeMode === 'filter'}
      onclick={() => (activeMode = 'filter')}
    >FILTER</button>
  </div>

  <div class="device-screen">

    <!-- ── SEARCH MODE ── -->
    {#if activeMode === 'search'}
      <div class="mode-content">
        <div class="input-wrap">
          <input
            class="search-input"
            type="text"
            placeholder="Search movies..."
            aria-label="Search movies"
            value={query}
            oninput={onInput}
          />
          {#if query}
            <button class="clear-btn" onclick={clearSearch} aria-label="Clear search">×</button>
          {/if}
        </div>

        {#if searchLoading}
          <div class="tracking-anim">
            <div class="tracking-line"></div>
            <span class="tracking-label">SEARCHING...</span>
          </div>
        {:else if searchError}
          <div class="error-panel">
            <span class="error-label">⚠ SIGNAL LOST</span>
            <span class="error-msg">{searchError}</span>
          </div>
        {:else if preview}
          <div class="preview-panel">
            <button class="back-btn" onclick={() => (preview = null)}>← RESULTS</button>
            {#if previewLoading}
              <div class="tracking-anim small"><div class="tracking-line"></div></div>
            {:else}
              <div class="preview-body">
                <div class="preview-poster-wrap">
                  {#if preview.poster_path}
                    <img class="preview-poster" src={tmdbImage(preview.poster_path, 'w200') ?? ''} alt={preview.title} loading="lazy" />
                  {:else}
                    <div class="no-poster">NO<br/>SIGNAL</div>
                  {/if}
                </div>
                <div class="preview-info">
                  <div class="preview-title">{preview.title}</div>
                  <div class="preview-meta">
                    {releaseYear(preview.release_date)}{preview.runtime ? ` · ${preview.runtime}m` : ''}{preview.vote_average > 0 ? ` · ★ ${preview.vote_average.toFixed(1)}` : ''}
                  </div>
                  {#if preview.genres.length > 0}
                    <div class="preview-genres">{preview.genres.map(g => g.name).join(', ')}</div>
                  {/if}
                </div>
              </div>
              {#if preview.overview}
                <p class="preview-overview">{preview.overview}</p>
              {/if}
              {#if preview.credits.cast.length > 0}
                <div class="cast-section">
                  <div class="section-label">CAST</div>
                  <div class="cast-list">
                    {#each preview.credits.cast.slice(0, 6) as member}
                      <div class="cast-item">
                        <div class="cast-photo">
                          {#if member.profile_path}
                            <img src={tmdbImage(member.profile_path, 'w200') ?? ''} alt={member.name} loading="lazy" />
                          {:else}
                            <div class="no-photo">?</div>
                          {/if}
                        </div>
                        <div class="cast-name">{member.name}</div>
                      </div>
                    {/each}
                  </div>
                </div>
              {/if}
              {#if $graphStore.movies.has(`m:${preview.id}`)}
                <div class="already-in-graph">✓ ADDED TO GRAPH</div>
              {:else}
                <button class="add-btn" onclick={addMovieFromPreview}>+ ADD TO GRAPH</button>
              {/if}
            {/if}
          </div>
        {:else if searchResults.length > 0}
          <div class="results-list">
            {#each searchResults as movie (movie.id)}
              <button class="result-item" onclick={() => selectSearchResult(movie)}>
                <div class="result-thumb">
                  {#if movie.poster_path}
                    <img src={tmdbImage(movie.poster_path, 'w200') ?? ''} alt={movie.title} loading="lazy" />
                  {:else}
                    <div class="no-thumb">?</div>
                  {/if}
                </div>
                <div class="result-info">
                  <div class="result-title">{movie.title}</div>
                  <div class="result-meta">{releaseYear(movie.release_date)}{movie.vote_average > 0 ? ` · ★ ${movie.vote_average.toFixed(1)}` : ''}</div>
                </div>
                {#if $graphStore.movies.has(`m:${movie.id}`)}
                  <span class="in-graph-badge">✓</span>
                {/if}
              </button>
            {/each}
          </div>
        {:else if !query}
          <div class="placeholder-msg"><span class="blink">▋</span> READY</div>
        {:else}
          <div class="placeholder-msg">NO RESULTS</div>
        {/if}
      </div>

    <!-- ── INSPECT MODE ── -->
    {:else if activeMode === 'inspect'}
      <div class="mode-content">
        {#if !$selectionStore}
          <div class="placeholder-msg">SELECT A NODE</div>
        {:else if inspectLoading}
          <div class="tracking-anim">
            <div class="tracking-line"></div>
            <span class="tracking-label">LOADING...</span>
          </div>
        {:else if inspectError}
          <div class="error-panel">
            <span class="error-label">⚠ SIGNAL LOST</span>
            <span class="error-msg">{inspectError}</span>
          </div>
        {:else if inspectedMovie}
          <!-- Movie inspect -->
          <div class="preview-body">
            <div class="preview-poster-wrap">
              {#if inspectedMovie.poster_path}
                <img class="preview-poster" src={tmdbImage(inspectedMovie.poster_path, 'w200') ?? ''} alt={inspectedMovie.title} loading="lazy" />
              {:else}
                <div class="no-poster">NO<br/>SIGNAL</div>
              {/if}
            </div>
            <div class="preview-info">
              <div class="preview-title">{inspectedMovie.title}</div>
              <div class="preview-meta">
                {releaseYear(inspectedMovie.release_date)}{inspectedMovie.runtime ? ` · ${inspectedMovie.runtime}m` : ''}{inspectedMovie.vote_average > 0 ? ` · ★ ${inspectedMovie.vote_average.toFixed(1)}` : ''}
              </div>
              {#if inspectedMovie.genres.length > 0}
                <div class="preview-genres">{inspectedMovie.genres.map(g => g.name).join(', ')}</div>
              {/if}
            </div>
          </div>
          {#if inspectedMovie.overview}
            <p class="preview-overview">{inspectedMovie.overview}</p>
          {/if}
          {#if inspectedMovie.credits.cast.length > 0}
            <div class="cast-section">
              <div class="section-label">CAST</div>
              <div class="cast-rows">
                {#each inspectedMovie.credits.cast.slice(0, 8) as member}
                  <div class="cast-row">
                    <div class="cast-photo-sm">
                      {#if member.profile_path}
                        <img src={tmdbImage(member.profile_path, 'w200') ?? ''} alt={member.name} loading="lazy" />
                      {:else}
                        <div class="no-photo">?</div>
                      {/if}
                    </div>
                    <span class="cast-row-name">{member.name}</span>
                    {#if $graphStore.actors.has(`a:${member.id}`)}
                      <span class="in-graph-badge">✓</span>
                    {:else}
                      <button class="add-small-btn" onclick={() => addActorFromCast(member)}>+</button>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          {/if}
          <button class="remove-btn" onclick={deleteInspectedNode}>REMOVE FROM GRAPH</button>

        {:else if inspectedPerson}
          <!-- Actor inspect -->
          <div class="preview-body">
            <div class="preview-poster-wrap">
              {#if inspectedPerson.profile_path}
                <img class="preview-poster" src={tmdbImage(inspectedPerson.profile_path, 'w200') ?? ''} alt={inspectedPerson.name} loading="lazy" />
              {:else}
                <div class="no-poster">NO<br/>SIGNAL</div>
              {/if}
            </div>
            <div class="preview-info">
              <div class="preview-title">{inspectedPerson.name}</div>
              {#if inspectedPerson.birthday}
                <div class="preview-meta">b. {inspectedPerson.birthday.slice(0, 4)}</div>
              {/if}
              {#if inspectedPerson.place_of_birth}
                <div class="preview-genres">{inspectedPerson.place_of_birth}</div>
              {/if}
            </div>
          </div>
          {#if inspectedPerson.biography}
            <p class="preview-overview">{inspectedPerson.biography}</p>
          {/if}

          <!-- Actor suggestions -->
          {#if actorSuggestions.length > 0}
            <div class="suggestions-section">
              <div class="section-label">SUGGESTED MOVIES</div>
              <div class="suggestions-list">
                {#each actorSuggestions as movie (movie.id)}
                  <div class="suggestion-item">
                    <div class="suggestion-thumb">
                      {#if movie.poster_path}
                        <img src={tmdbImage(movie.poster_path, 'w200') ?? ''} alt={movie.title} loading="lazy" />
                      {:else}
                        <div class="no-thumb">?</div>
                      {/if}
                    </div>
                    <div class="suggestion-info">
                      <div class="result-title">{movie.title}</div>
                      <div class="result-meta">
                        {releaseYear(movie.release_date)}{movie.vote_average ? ` · ★ ${movie.vote_average.toFixed(1)}` : ''}
                      </div>
                    </div>
                    <button
                      class="add-small-btn"
                      class:in-graph={$graphStore.movies.has(`m:${movie.id}`)}
                      disabled={addingSuggestion !== null || $graphStore.movies.has(`m:${movie.id}`)}
                      onclick={() => addSuggestedMovie(movie)}
                      title="Add to graph"
                    >
                      {addingSuggestion === movie.id ? '…' : ($graphStore.movies.has(`m:${movie.id}`) ? '✓' : '+')}
                    </button>
                  </div>
                {/each}
              </div>
            </div>
          {/if}
          <button class="remove-btn" onclick={deleteInspectedNode}>REMOVE FROM GRAPH</button>
        {/if}
      </div>

    <!-- ── FILTER MODE ── -->
    {:else if activeMode === 'filter'}
      <div class="mode-content">
        <div class="placeholder-msg">FILTERS</div>
      </div>
    {/if}

  </div>
</div>

<style>
.device {
  width: 300px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  background: var(--color-device-body);
  border: 2px solid var(--color-device-bezel);
  border-radius: 8px;
  box-shadow: var(--shadow-device), inset 0 1px 0 rgba(255, 255, 255, 0.04);
  overflow: hidden;
}

.device-header {
  display: flex;
  align-items: center;
  height: 32px;
  padding: 0 12px;
  background: linear-gradient(180deg, #3a3a40 0%, #2d2d32 100%);
  border-bottom: 1px solid #1a1a20;
  flex-shrink: 0;
}

.device-title {
  color: var(--color-text-secondary);
  font-family: var(--font-ui);
  font-size: 11px;
  font-weight: 500;
  letter-spacing: 0.15em;
  text-transform: uppercase;
}

.device-indicators {
  display: flex;
  gap: 6px;
  margin-left: auto;
}

.led {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--color-led-off);
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.5);
}

.led-green {
  background: var(--color-led-green);
  box-shadow: 0 0 6px var(--color-led-green), 0 0 12px rgba(48, 255, 80, 0.3),
    inset 0 1px 2px rgba(0, 0, 0, 0.2);
}

.led-amber {
  background: #ffb700;
  box-shadow: 0 0 6px #ffb700, 0 0 12px rgba(255, 183, 0, 0.3),
    inset 0 1px 2px rgba(0, 0, 0, 0.2);
}

.mode-bar {
  display: flex;
  border-bottom: 1px solid #1a1a20;
  flex-shrink: 0;
  background: var(--color-device-bezel);
}

.mode-btn {
  flex: 1;
  height: 32px;
  background: transparent;
  border: none;
  border-right: 1px solid #1a1a20;
  color: var(--color-text-disabled);
  font-family: var(--font-ui);
  font-size: 10px;
  font-weight: 500;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  cursor: pointer;
  transition: background var(--duration-fast), color var(--duration-fast);
}

.mode-btn:last-child { border-right: none; }

.mode-btn:hover {
  background: rgba(255, 255, 255, 0.03);
  color: var(--color-text-secondary);
}

.mode-btn.active {
  background: linear-gradient(180deg, #2a2a30 0%, #232328 100%);
  color: var(--color-text-primary);
  box-shadow: inset 0 -2px 0 var(--color-accent-primary);
}

.device-screen {
  flex: 1;
  margin: 10px;
  border: 2px solid #1a1a20;
  border-radius: 3px;
  background: var(--color-screen-bg);
  box-shadow: inset 0 0 20px rgba(0, 0, 0, 0.5);
  overflow: hidden;
  position: relative;
  min-height: 0;
}

.mode-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 10px;
  height: 100%;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: #2a2a38 transparent;
}

/* Search input */
.input-wrap {
  position: relative;
  flex-shrink: 0;
}

.search-input {
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid #2a2a38;
  border-radius: 2px;
  box-shadow: inset 0 2px 6px rgba(0, 0, 0, 0.4);
  caret-color: var(--color-accent-primary);
  color: var(--color-text-screen);
  font-family: var(--font-mono);
  font-size: 13px;
  outline: none;
  padding: 8px 28px 8px 10px;
  width: 100%;
  transition: border-color var(--duration-fast);
  box-sizing: border-box;
}

.search-input::placeholder { color: var(--color-text-disabled); }

.search-input:focus {
  border-color: var(--color-accent-primary);
  box-shadow: inset 0 2px 6px rgba(0, 0, 0, 0.4), 0 0 0 1px rgba(224, 120, 80, 0.2);
}

.clear-btn {
  position: absolute;
  right: 6px;
  top: 50%;
  transform: translateY(-50%);
  background: none;
  border: none;
  color: var(--color-text-disabled);
  cursor: pointer;
  font-size: 16px;
  line-height: 1;
  padding: 2px 4px;
}

.clear-btn:hover { color: var(--color-text-secondary); }

/* Loading animation */
.tracking-anim {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 16px 0;
}

.tracking-anim.small { padding: 8px 0; }

.tracking-line {
  width: 100%;
  height: 2px;
  background: linear-gradient(90deg, transparent 0%, var(--color-accent-primary) 50%, transparent 100%);
  background-size: 200% 100%;
  animation: track 1s linear infinite;
}

.tracking-label {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 10px;
  letter-spacing: 0.2em;
  animation: blink 1s step-end infinite;
}

@keyframes track {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

/* Error */
.error-panel {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 10px;
  border: 1px solid rgba(255, 80, 80, 0.3);
  border-radius: 2px;
  background: rgba(255, 40, 40, 0.05);
}

.error-label {
  color: #ff6060;
  font-family: var(--font-ui);
  font-size: 10px;
  letter-spacing: 0.1em;
  font-weight: 600;
}

.error-msg {
  color: var(--color-text-secondary);
  font-family: var(--font-mono);
  font-size: 11px;
}

/* Results list */
.results-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.result-item {
  display: flex;
  gap: 8px;
  align-items: center;
  padding: 6px;
  background: transparent;
  border: none;
  border-radius: 2px;
  cursor: pointer;
  text-align: left;
  width: 100%;
  transition: background var(--duration-fast);
}

.result-item:hover { background: rgba(255, 255, 255, 0.05); }

.result-thumb {
  width: 32px;
  height: 48px;
  flex-shrink: 0;
  border-radius: 1px;
  overflow: hidden;
  background: #1a1a20;
}

.result-thumb img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.no-thumb {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-disabled);
  font-size: 10px;
}

.result-info { flex: 1; min-width: 0; }

.result-title {
  color: var(--color-text-screen);
  font-family: var(--font-mono);
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  line-height: 1.3;
}

.result-meta {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 10px;
  margin-top: 2px;
}

/* Preview / Inspect shared layout */
.preview-panel {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.back-btn {
  background: none;
  border: none;
  color: var(--color-text-disabled);
  font-family: var(--font-ui);
  font-size: 10px;
  letter-spacing: 0.08em;
  cursor: pointer;
  text-align: left;
  padding: 0;
  transition: color var(--duration-fast);
}

.back-btn:hover { color: var(--color-accent-primary); }

.preview-body {
  display: flex;
  gap: 10px;
  align-items: flex-start;
}

.preview-poster-wrap { width: 60px; flex-shrink: 0; }

.preview-poster {
  width: 60px;
  border-radius: 2px;
  display: block;
}

.no-poster {
  width: 60px;
  height: 90px;
  background: #1a1a20;
  border-radius: 2px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-disabled);
  font-family: var(--font-ui);
  font-size: 8px;
  letter-spacing: 0.1em;
  text-align: center;
  line-height: 1.4;
}

.preview-info { flex: 1; min-width: 0; }

.preview-title {
  color: var(--color-text-screen);
  font-family: var(--font-mono);
  font-size: 13px;
  font-weight: 600;
  line-height: 1.3;
  margin-bottom: 4px;
}

.preview-meta {
  color: var(--color-accent-primary);
  font-family: var(--font-mono);
  font-size: 11px;
  margin-bottom: 4px;
}

.preview-genres {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 10px;
}

.preview-overview {
  color: var(--color-text-secondary);
  font-family: var(--font-mono);
  font-size: 11px;
  line-height: 1.5;
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 4;
  line-clamp: 4;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* Section labels */
.section-label {
  color: var(--color-text-disabled);
  font-family: var(--font-ui);
  font-size: 9px;
  letter-spacing: 0.15em;
  border-bottom: 1px solid #2a2a38;
  padding-bottom: 4px;
  margin-bottom: 2px;
}

/* Cast bubbles (search preview) */
.cast-section {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.cast-list {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.cast-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 3px;
  width: 36px;
}

.cast-photo {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  overflow: hidden;
  background: #1a1a20;
}

.cast-photo img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.no-photo {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-disabled);
  font-size: 12px;
}

.cast-name {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 8px;
  text-align: center;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  width: 100%;
}

/* Cast rows (inspect mode) */
.cast-rows {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.cast-row {
  display: flex;
  gap: 8px;
  align-items: center;
  padding: 3px 4px;
  border-radius: 2px;
  transition: background var(--duration-fast);
}

.cast-row:hover { background: rgba(255, 255, 255, 0.03); }

.cast-photo-sm {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  overflow: hidden;
  background: #1a1a20;
  flex-shrink: 0;
}

.cast-photo-sm img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.cast-row-name {
  flex: 1;
  min-width: 0;
  color: var(--color-text-screen);
  font-family: var(--font-mono);
  font-size: 11px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Actor suggestions */
.suggestions-section {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.suggestions-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.suggestion-item {
  display: flex;
  gap: 8px;
  align-items: center;
  padding: 4px 4px 4px 6px;
  border-radius: 2px;
  border: 1px solid transparent;
  transition: border-color var(--duration-fast), background var(--duration-fast);
}

.suggestion-item:hover {
  background: rgba(255, 255, 255, 0.03);
  border-color: #2a2a38;
}

.suggestion-thumb {
  width: 28px;
  height: 42px;
  flex-shrink: 0;
  border-radius: 1px;
  overflow: hidden;
  background: #1a1a20;
}

.suggestion-thumb img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.suggestion-info { flex: 1; min-width: 0; }

/* In-graph badge & indicator */
.in-graph-badge {
  color: var(--color-led-green);
  font-size: 12px;
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.already-in-graph {
  color: var(--color-led-green);
  font-family: var(--font-ui);
  font-size: 11px;
  letter-spacing: 0.1em;
  font-weight: 600;
  text-align: center;
  padding: 8px;
  margin-top: 4px;
  border: 1px solid rgba(48, 255, 80, 0.2);
  border-radius: 2px;
  background: rgba(48, 255, 80, 0.05);
}

/* Add / remove buttons */
.add-small-btn {
  width: 22px;
  height: 22px;
  flex-shrink: 0;
  background: rgba(224, 120, 80, 0.12);
  border: 1px solid rgba(224, 120, 80, 0.3);
  border-radius: 2px;
  color: var(--color-accent-primary);
  font-size: 14px;
  line-height: 1;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background var(--duration-fast);
}

.add-small-btn:hover:not(:disabled) {
  background: rgba(224, 120, 80, 0.25);
}

.add-small-btn:disabled {
  cursor: default;
  opacity: 0.6;
}

.add-small-btn.in-graph {
  background: rgba(48, 255, 80, 0.08);
  border-color: rgba(48, 255, 80, 0.25);
  color: var(--color-led-green);
}

.add-btn {
  background: var(--color-accent-primary);
  border: none;
  border-radius: 2px;
  color: #fff;
  cursor: pointer;
  font-family: var(--font-ui);
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.1em;
  padding: 8px;
  text-align: center;
  width: 100%;
  margin-top: 4px;
  transition: opacity var(--duration-fast);
}

.add-btn:hover { opacity: 0.85; }

.remove-btn {
  background: transparent;
  border: 1px solid rgba(255, 80, 80, 0.3);
  border-radius: 2px;
  color: rgba(255, 100, 100, 0.7);
  cursor: pointer;
  font-family: var(--font-ui);
  font-size: 10px;
  font-weight: 500;
  letter-spacing: 0.1em;
  padding: 6px;
  text-align: center;
  width: 100%;
  margin-top: 8px;
  transition: background var(--duration-fast), color var(--duration-fast);
}

.remove-btn:hover {
  background: rgba(255, 80, 80, 0.1);
  color: #ff6060;
}

/* Shared */
.placeholder-msg {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 12px;
  letter-spacing: 0.1em;
  text-align: center;
  margin-top: auto;
  margin-bottom: auto;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0; }
}

.blink {
  animation: blink 1s step-end infinite;
  color: var(--color-text-screen);
}

@media (prefers-reduced-motion: reduce) {
  .blink, .tracking-line, .tracking-label { animation: none; }
}
</style>

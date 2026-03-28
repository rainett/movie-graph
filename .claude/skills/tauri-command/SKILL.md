---
name: tauri-command
description: Generate a Tauri IPC command with Rust implementation and TypeScript frontend binding
argument-hint: <command-name> [description]
---

# Tauri Command Generator

Create a new Tauri IPC command following current project patterns.

## Input

Command: $ARGUMENTS

## Steps

### 1. Rust Command

Add to appropriate file in `src-tauri/src/commands/` (or create new module):

```rust
// src-tauri/src/commands/{module}.rs
use crate::error::Error;

#[tauri::command]
pub async fn $COMMAND_NAME(
    param1: String,
    param2: Option<u32>,
) -> Result<ReturnType, Error> {
    // Implementation
    Ok(result)
}
```

**Current patterns from codebase:**
- All commands are `async`
- Return `Result<T, Error>` using `crate::error::Error`
- Use `Option<T>` for optional params
- Services are instantiated per-call: `let client = TmdbClient::new(token);`

### 2. Register Command

Update `src-tauri/src/lib.rs`:

```rust
use commands::module_name::$COMMAND_NAME;

// In run():
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    $COMMAND_NAME,
])
```

And update `src-tauri/src/commands/mod.rs`:

```rust
pub mod module_name;
```

### 3. TypeScript Binding

Add to `src/lib/services/tauri.ts`:

```typescript
export async function commandName(
  param1: string,
  param2?: number,
): Promise<ReturnType> {
  return tauriInvoke('command_name', { param1, param2 });
}
```

**Current patterns:**
- Params use camelCase in TS, snake_case in Rust
- Use `tauriInvoke` (aliased from `@tauri-apps/api/core`)
- Return Promise with typed response

### 4. Types (if needed)

**Rust** — Add to `src-tauri/src/models/`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewType {
    pub field: String,
    #[serde(default)]
    pub optional_field: Option<i32>,
}
```

**TypeScript** — Add to `src/lib/types/`:

```typescript
export type NewType = {
  field: string;
  optional_field: number | null;
};
```

### 5. Verify

After implementation, hooks will auto-run:
- `cargo check` for Rust errors
- `npm run check` for TypeScript errors

## Reference Examples

**TMDB command** (`src-tauri/src/commands/tmdb.rs`):
```rust
#[tauri::command]
pub async fn search_movies(
    query: String,
    page: Option<u32>,
    api_key: Option<String>,
) -> Result<SearchResults<MovieResult>, Error> {
    let client = TmdbClient::new(api_key);
    client.search_movies(&query, page.unwrap_or(1)).await
}
```

**Project command** (`src-tauri/src/commands/project.rs`):
```rust
#[tauri::command]
pub async fn open_project(path: String) -> Result<Project, Error> {
    // Validates and loads project from path
}
```

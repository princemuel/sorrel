# Sorrel Roadmap

> A local-first, privacy-preserving full-text search engine.
> No cloud. No telemetry. Fast search over your own content.

This roadmap is organized into **phases**, each containing focused **sprints**.
Every phase ends with something shippable — something a real user can install and
get value from today, not just a stepping stone to the next phase.

---

## Guiding Principles

- **Ship early, ship useful.** Phase 1 alone is more useful than `grep -r`.
- **Core stays pure.** `sorrel-core` never imports interface concerns.
- **Feature flags over conditionals.** Heavy dependencies are always opt-in.
- **Stable before extended.** Each phase is stable before the next begins.
- **User trust is non-negotiable.** No network calls, no telemetry, ever.

---

## Workspace Layout (reference)

```
sorrel/
├── crates/
│   ├── sorrel-core/      # engine: index, query, rank, parse  (lib)
│   ├── sorrel-store/     # storage trait + Tantivy/SQLite backends (lib)
│   ├── sorrel-config/    # config, synonyms, stop words (lib)
│   ├── sorrel-cli/       # binary: CLI
│   ├── sorrel-tui/       # binary: TUI (ratatui)
│   ├── sorrel-web/       # binary: axum HTTP server + embedded UI
│   └── sorrel-tauri/     # binary: Tauri desktop GUI shell
```

Error handling convention: `thiserror` in all library crates;
`anyhow` in all binary crates.

---

## Phase 0 — Foundation

**Goal:** workspace compiles, CI passes, conventions established.
No real functionality. Everything that follows builds on this skeleton.

### Sprint 0.1 — Workspace Skeleton

- [ ] Initialize Cargo workspace with all crate stubs
- [ ] Establish error handling convention (`thiserror` / `anyhow`)
- [ ] Set up CI (GitHub Actions): `cargo check`, `cargo test`, `cargo clippy`
- [ ] Add `rustfmt.toml` and `clippy.toml` with project lint rules
- [ ] Write `CONTRIBUTING.md` covering crate boundaries and feature flag rules
- [ ] Pin `rust-version` in workspace `Cargo.toml` (edition 2024, MSRV 1.85)

### Sprint 0.2 — Core Traits

- [ ] Define `FullTextStore` and `MetaStore` traits in `sorrel-store`
- [ ] Define `FileParser` trait and `ParsedDocument` type in `sorrel-core`
- [ ] Define `SearchEngine<F, M>` generic struct (empty body, just compiles)
- [ ] Define `ScoredDoc`, `Snippet`, `SearchOptions`, `IndexStats` types
- [ ] Write trait-level documentation with usage examples in doc comments
- [ ] Add stub integration test that constructs an engine and calls search (returns empty)

**Phase 0 exit criteria:** `cargo build --workspace` succeeds. CI is green.

---

## Phase 1 — MVP: Index and Search

**Goal:** a user can point Sorrel at a folder and search it from the terminal
in under a minute from first install. This is the core value proposition.

### Sprint 1.1 — Storage Backends

- [ ] Implement `TantivyStore` in `sorrel-store` behind `tantivy-backend` feature
- [ ] Implement `SqliteMetaStore` in `sorrel-store` behind `sqlite-meta` feature
- [ ] Schema: documents table (id, path, mtime, content\_hash, indexed\_at)
- [ ] Schema: folder\_registry table (path, index\_path, last\_seen)
- [ ] Unit tests for both backends with temp directories

### Sprint 1.2 — Text Processing Pipeline

- [ ] Unicode word segmentation tokenizer (`unicode-segmentation` crate)
- [ ] Lowercasing + Unicode NFC normalization
- [ ] Stop word filter: bundled English list (`assets/stopwords/en.txt`)
- [ ] Snowball stemmer integration (`rust-stemmers` crate)
- [ ] Pipeline composition: tokenize → normalize → stop words → stem
- [ ] Unit tests: known inputs produce known token sequences

### Sprint 1.3 — Built-in Parsers

- [ ] `PlaintextParser`: UTF-8 text, title heuristic from first non-empty line
- [ ] `MarkdownParser`: strip syntax via `pulldown-cmark`, extract frontmatter title
- [ ] `ParserRegistry`: ordered list of `Box<dyn FileParser>`, first match wins
- [ ] MIME detection via `infer` crate (magic bytes, not extension only)
- [ ] Fixtures and snapshot tests for each parser

### Sprint 1.4 — Indexer and Crawler

- [ ] Recursive directory walker using the `ignore` crate
  (respects `.gitignore` patterns automatically)
- [ ] `.searchignore` file support (same glob syntax as `.gitignore`)
- [ ] Per-file change detection: compare `mtime` + content hash against `meta.db`
- [ ] Skip unchanged files on re-index (incremental from day one)
- [ ] Per-folder `.sorrel.idx` manifest: written after first index of a folder,
  stores `{parent_index_path, indexed_at, file_count}`
- [ ] `IndexStats` returned after each run: files seen, indexed, skipped, errors

### Sprint 1.5 — Query Engine

- [ ] Query parser: tokenize and normalize the raw query string
- [ ] Apply stop word filter and stemmer to query tokens
- [ ] Synonym expansion: load `synonyms.toml`, rewrite query as boolean OR groups
- [ ] BM25 search via Tantivy (field: body, title, path)
- [ ] Field boosts: title matches weighted 3×, path matches 2×, body 1×
- [ ] Snippet extraction: densest match window, configurable context lines
- [ ] Line number recorded per snippet for editor jump support

### Sprint 1.6 — CLI

- [ ] `sorrel init [path]` — interactive setup, writes `~/.config/sorrel/config.toml`
- [ ] `sorrel index <path>` — crawl and index, show progress bar (`indicatif`)
- [ ] `sorrel search "<query>"` — ranked results with snippets, coloured output
- [ ] `sorrel search` flags: `--limit N`, `--type <ext>`, `--in <path>`,
  `--since <date>`, `--open`, `--open <N>`
- [ ] `--open`: detect `$EDITOR`/`$VISUAL`, open file at matched line number
- [ ] `sorrel status` — indexed folders, file count, index size, last updated
- [ ] `sorrel reindex [path]` — force re-index ignoring mtime cache
- [ ] `sorrel exclude <path>` — append to `.searchignore`, remove from index
- [ ] `sorrel forget <path>` — remove folder from index and stop tracking it
- [ ] Structured JSON output mode (`--json`) for piping to other tools

### Sprint 1.7 — Configuration

- [ ] `sorrel-config` crate: load and validate `.searchconfig.toml`
- [ ] `SynonymMap`: load from `synonyms.toml`, `HashMap<String, Vec<String>>`
- [ ] `StopWordSet`: load from file or use bundled list
- [ ] XDG-compliant config paths on Linux; platform-appropriate on macOS/Windows
- [ ] Config schema documented inline with comments in `.searchconfig.toml.example`
- [ ] Validate config on load; surface actionable errors, not panics

**Phase 1 exit criteria:**
A user runs `cargo install sorrel-cli`, does `sorrel init ~/Documents`,
and gets ranked search results with highlighted snippets in under 5 seconds
for a corpus of 10,000 files. Re-indexing skips unchanged files.

---

## Phase 2 — Always Current

**Goal:** the index maintains itself. Users stop thinking about indexing.

### Sprint 2.1 — File Watcher

- [ ] Integrate `notify` crate (wraps `inotify`/`FSEvents`/`ReadDirectoryChanges`)
- [ ] `sorrel watch` command: start watcher in foreground, log events
- [ ] Debounce rapid successive changes (default 500ms, configurable)
- [ ] On change: re-hash file → if changed, re-parse and update Tantivy doc
- [ ] On delete: remove document from index and `meta.db`
- [ ] On new file: detect extension, route through `ParserRegistry`, index
- [ ] Background daemon mode: `sorrel watch --daemon` (writes PID file)
- [ ] `sorrel watch --stop` to terminate the daemon

### Sprint 2.2 — Per-Folder Index Scoping

- [ ] `.search/` directory convention: when crawler enters a folder,
  check for `.search/index/`
- [ ] If `.search/index/` exists: load it as an independent Tantivy index
- [ ] If absent: index into nearest parent `.search/` (or global index)
- [ ] `sorrel index --local <path>`: explicitly create a `.search/` for this folder
- [ ] Composability: a folder with `.search/` can be zipped and its index travels with it
- [ ] Root index registry in global `meta.db`: tracks all known `.search/` locations

### Sprint 2.3 — External Preprocessor Parsers

- [ ] Config block for external parsers:
  ```toml
  [parsers.external]
  ".org"   = "pandoc --to plain"
  ".ipynb" = "jupyter nbconvert --to script --stdout"
  ```
- [ ] `ExternalParser`: spawn subprocess, capture stdout as plain text body
- [ ] Timeout per subprocess (default 10s, configurable)
- [ ] Stderr captured and surfaced in `sorrel status --errors`
- [ ] Registered after built-in parsers (built-ins take priority)

**Phase 2 exit criteria:**
The index updates within 1 second of a file save. Users never run
`sorrel reindex` manually. External tools like `pandoc` unlock
arbitrary format support via config alone.

---

## Phase 3 — Power Search

**Goal:** Sorrel becomes the search tool, not just a fast one.
Users can express precise queries and filter large corpora accurately.

### Sprint 3.1 — Query Language

- [ ] Boolean operators: `AND`, `OR`, `NOT` (case-insensitive)
- [ ] Phrase search: `"exact phrase"` (positional index required)
- [ ] Field scoping: `title:rust`, `body:lifetime`, `path:projects`
- [ ] Wildcard suffix: `life*` matches `lifetime`, `lifetimes`, `lifelong`
- [ ] Grouping with parentheses: `(rust OR go) AND concurrency`
- [ ] Query parser produces an AST; document the grammar in `ARCHITECTURE.md`

### Sprint 3.2 — Fuzzy Matching

- [ ] Levenshtein distance-based fuzzy query via Tantivy's fuzzy term query
- [ ] Default edit distance: 1 for terms ≤ 5 chars, 2 for longer
- [ ] `--fuzzy` flag to opt in per search (off by default to keep results precise)
- [ ] BK-tree for fast candidate lookup on large vocabularies (if Tantivy's
  built-in fuzzy proves too slow)

### Sprint 3.3 — Faceted Filtering

- [ ] Post-retrieval filters applied to BM25 result set:
  - File type / extension (`--type pdf`, `--type md,txt`)
  - Date range (`--since 2024-01-01 --until 2024-06-01`)
  - Folder scope (`--in ~/projects`)
  - File size range (`--smaller-than 1MB`)
- [ ] Filter metadata stored in `meta.db`, not re-read from disk at query time
- [ ] Filters composable: `--type pdf --since 2024 --in ~/invoices`

### Sprint 3.4 — Ranking Signals

- [ ] Recency boost: configurable weight applied to `indexed_at` recency
- [ ] Positional proximity: boost when query terms appear within N tokens of each other
- [ ] Exact phrase bonus: phrase match scores higher than term scatter
- [ ] All boost weights tunable in `config.toml` under `[ranking]`
- [ ] `--explain` flag: show per-result score breakdown (BM25 + each boost signal)

### Sprint 3.5 — Multi-language Support

- [ ] Stop word lists for: `de`, `fr`, `es`, `pt`, `nl`, `it`, `ru`
  (bundled under `assets/stopwords/`)
- [ ] Snowball stemmers for each supported language
- [ ] Language auto-detection per document via `whatlang` or `lingua`
- [ ] Per-folder language override in `.search/config.toml`
- [ ] Config: `[language] default = "en"`, per-folder overrides respected

**Phase 3 exit criteria:**
A user can write `sorrel search 'title:design AND (system OR architecture)
--since 2024 --type md'` and get precise, well-ranked results.
`--explain` shows exactly why each result ranked where it did.

---

## Phase 4 — TUI

**Goal:** a terminal-native experience that feels like a product.
The kind of thing that gets posted to Hacker News.

### Sprint 4.1 — Core TUI Shell

- [ ] `sorrel tui` launches full-screen terminal UI (`ratatui` + `crossterm`)
- [ ] Search box at top: live results as you type (80ms debounce)
- [ ] Result list: filename, relative path, score, relative date
- [ ] Keyboard: `↑↓` navigate results, `Enter` open, `Esc`/`q` quit
- [ ] Search runs on background thread; UI stays at 60fps during query

### Sprint 4.2 — Preview Pane

- [ ] Right panel: file content preview with match terms highlighted
- [ ] Scroll preview independently (`PgUp`/`PgDn` or `j`/`k` in preview focus)
- [ ] Syntax highlighting for source code files via `syntect`
- [ ] Toggle preview pane with `Tab`

### Sprint 4.3 — TUI Refinements

- [ ] Filter bar: `f` opens inline filter row (type, date, folder)
- [ ] Result count and query time shown in status bar
- [ ] Copy matched file path to clipboard (`y`)
- [ ] Open containing folder in `$FILE_MANAGER` or `xdg-open` (`o`)
- [ ] Mouse support: click to select result, scroll preview

**Phase 4 exit criteria:**
`sorrel tui` is the interface most users reach for daily.
It is screenshot-worthy and demo-friendly.

---

## Phase 5 — Web UI

**Goal:** browser-based search over `localhost`. The interface for non-terminal
users and the API layer that unblocks mobile and third-party integrations.

### Sprint 5.1 — HTTP API

- [ ] `sorrel serve` starts `axum` server on `127.0.0.1:7700`
- [ ] `POST /api/search` — JSON body, returns ranked results with snippets
- [ ] `POST /api/index` — trigger index/reindex of a path
- [ ] `GET  /api/status` — index health, file counts, last updated
- [ ] `POST /api/open` — open a file path on the host machine
- [ ] `GET  /api/events` — Server-Sent Events stream for index change notifications
- [ ] All endpoints return structured JSON; errors follow a consistent schema
- [ ] Bind address and port configurable; localhost-only by default

### Sprint 5.2 — Embedded Frontend

- [ ] Single HTML file embedded in binary via `include_str!()`
- [ ] Search box with 80ms debounce, results rendered below
- [ ] Result cards: filename, path, snippet with highlights, relative date
- [ ] Filter controls: type, date range, folder
- [ ] SSE connection: toast notification when index updates, auto-refresh results
- [ ] Responsive layout: usable on mobile browser against `localhost`

### Sprint 5.3 — API Adapter (shared frontend code)

- [ ] Abstract `search()`, `index()`, `status()` behind an adapter interface in JS
- [ ] `WebAdapter`: uses `fetch()` against the HTTP API
- [ ] `TauriAdapter`: uses `invoke()` against Tauri IPC (no HTTP)
- [ ] Same frontend bundle used by both `sorrel-web` and `sorrel-tauri`

**Phase 5 exit criteria:**
`sorrel serve` + visiting `localhost:7700` is a complete, usable search
interface. The REST API is stable enough for third-party tooling to build on.

---

## Phase 6 — Extended Parsers

**Goal:** Sorrel indexes the documents users actually have, not just
plaintext and Markdown.

### Sprint 6.1 — Document Formats

- [ ] `PdfParser` via `pdf-extract` — feature flag `parser-pdf`
- [ ] `DocxParser` via `docx-rs` — feature flag `parser-docx`
- [ ] `EpubParser` via `epub` crate — feature flag `parser-epub`
- [ ] `HtmlParser` via `scraper` — strip tags, extract `<title>` and `<meta>`
- [ ] Each parser: fixture files in `tests/fixtures/`, snapshot tests

### Sprint 6.2 — Source Code

- [ ] `CodeParser`: language-aware tokenization via `tree-sitter`
  (identifiers, strings, comments indexed separately)
- [ ] Language detection from extension + shebang line
- [ ] Field: `symbols` — function names, class names, constants (higher boost)
- [ ] Feature flag `parser-code`; tree-sitter grammars are large, keep opt-in

### Sprint 6.3 — Script Plugin System

- [ ] Embedded Lua runtime via `mlua` — feature flag `plugin-lua`
- [ ] Plugin contract: `can_parse(path, mime) -> bool`,
  `extract_text(path) -> {body, title}`
- [ ] Plugins loaded from `~/.config/sorrel/parsers/*.lua`
- [ ] Sandboxed: file I/O allowed, network calls blocked
- [ ] Document plugin API in `docs/plugin-api.md` with examples

**Phase 6 exit criteria:**
A user with a `~/Documents` folder containing PDFs, DOCX files,
EPUBs, and source code gets all of it indexed with no extra configuration.
Power users extend coverage with a 10-line Lua script.

---

## Phase 7 — Desktop GUI

**Goal:** a native desktop application. Sorrel becomes accessible to users
who have never opened a terminal.

### Sprint 7.1 — Tauri Shell

- [ ] `sorrel-tauri` crate wrapping `sorrel-core` via Tauri commands
- [ ] Tauri commands: `search`, `index`, `status`, `open_file`, `watch_start`
- [ ] Same frontend bundle from Phase 5 (TauriAdapter wired in)
- [ ] Auto-start watcher on app launch
- [ ] App packaged for Linux (AppImage, `.deb`), macOS (`.dmg`), Windows (`.msi`)
  via `tauri-bundler`

### Sprint 7.2 — Native Integration

- [ ] System tray icon: shows indexing status, quick-search popup
- [ ] Global hotkey: `Ctrl+Shift+S` (or user-configured) opens search window
- [ ] OS notifications: "Sorrel indexed 42 new files in ~/Downloads"
- [ ] Native file picker for adding watched folders (no terminal needed)
- [ ] Settings screen: watched folders, synonyms, stop words, ranking weights

**Phase 7 exit criteria:**
A non-developer can install Sorrel, add their Documents folder through
a GUI, and search it — without ever opening a terminal.

---

## Phase 8 — Mobile

**Goal:** Sorrel on Android and iOS, searching content on-device.

### Sprint 8.1 — HTTP-First Mobile App

- [ ] Flutter app connecting to `sorrel serve` on the local network
- [ ] Search screen: query input, result list, snippet preview
- [ ] Settings: server address, port
- [ ] Works immediately for users who run `sorrel serve` on their desktop
  and want to search from their phone on the same network

### Sprint 8.2 — On-Device Rust via FFI

- [ ] Compile `sorrel-core` to Android (via `cargo-ndk`) and
  iOS (via `cargo-lipo`) targets
- [ ] `flutter_rust_bridge` bindings: expose `search`, `index`, `status`
- [ ] SQLite FTS5 backend only (drop Tantivy for binary size on mobile)
  via `fts5-only` feature flag
- [ ] Index synced from desktop via iCloud/local file share (manual, no cloud API)

**Phase 8 exit criteria:**
Sorrel runs fully offline on an Android or iOS device, indexing
local files (notes, downloads) and returning results in under 100ms.

---

## Phase 9 — Semantic Search (opt-in)

**Goal:** "find notes similar to this one" — meaning-based retrieval
that complements keyword search, not replaces it.

### Sprint 9.1 — Embedding Pipeline

- [ ] ONNX embedding model (e.g. `all-MiniLM-L6-v2`, ~22MB) via `ort` crate
- [ ] Feature flag `semantic` — disabled by default, explicit opt-in
- [ ] Embed each document at index time; store vectors in `meta.db`
  (SQLite `blob` column or `sqlite-vec` extension)
- [ ] `sorrel search --semantic "notes about ownership and borrowing"`:
  cosine similarity ranking over stored embeddings

### Sprint 9.2 — Hybrid Ranking

- [ ] Combine BM25 score and cosine similarity via weighted sum (tunable)
- [ ] `--semantic-only` flag for pure vector search
- [ ] `--hybrid` flag (default when semantic is enabled): BM25 + cosine
- [ ] `sorrel index --reembed`: recompute embeddings for all documents
  (needed after model upgrade)
- [ ] Document model update path in `UPGRADING.md`

**Phase 9 exit criteria:**
A user enables `semantic = true` in config, re-indexes, and can find
a document by describing its meaning rather than its exact words.
Existing keyword search is unaffected.

---

## Milestone Summary

| Milestone | Phase | What ships |
|---|---|---|
| **Alpha** | 0–1 | CLI: index + search. Useful on day one. |
| **Beta** | 2–3 | Auto-reindex, full query language, facets. Daily driver. |
| **v1.0** | 4–5 | TUI + Web UI. Broadly accessible. REST API stable. |
| **v1.1** | 6 | PDF, DOCX, code, Lua plugins. Indexes real-world content. |
| **v1.2** | 7 | Tauri desktop GUI. No terminal required. |
| **v1.3** | 8 | Mobile apps. Sorrel in your pocket. |
| **v2.0** | 9 | Semantic search. Meaning-based retrieval. |

---

## What Is Explicitly Out of Scope

- **Cloud sync or remote indexing.** Sorrel is local-first, always.
- **Multi-user or shared index servers.** Single-user, single-machine.
- **Web crawling.** Sorrel indexes your files, not the internet.
- **OCR.** Scanned PDFs are not in scope (delegate to external preprocessors).
- **Real-time collaboration.** Not applicable to a personal search engine.

---

## Tracking

Issues and sprint tasks are tracked in the GitHub repository.
Each sprint maps to a GitHub Milestone. Phase completion triggers a crate
version bump in the workspace `Cargo.toml`.

A sprint is complete when:
- All checklist items above are ticked
- `cargo test --workspace` passes with no ignored tests
- `cargo clippy --workspace -- -D warnings` is clean
- The phase's exit criteria are met by a manual smoke test

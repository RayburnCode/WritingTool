<!-- @format -->

https://www.5esrd.com/

Key Focuses for Your D&D Scrivener Clone
Modular Writing

Break scripts into reusable sections (e.g., scenes, NPCs, items).

Drag-and-Drop Organization

Rearrange parts of the script visually (like Scrivener’s corkboard).

D&D-Specific Tools

Stat blocks, dice rollers, character templates.

Offline-First + Export

Save locally (SQLite?) and export to PDF/HTML.

Simple UI

Prioritize keyboard shortcuts and a clean interface.

Tech Stack (Rust + Dioxus)
Component Choice Notes
Frontend Dioxus (Desktop/Web) Cross-platform, React-like syntax.
Backend Rust (Local-first) SQLite for storage, Serde for serialization.
UI Toolkit Dioxus-UI / Tauri (if needed) Tauri for deeper desktop integration.
Export Pandoc or rust-pdf Generate PDFs/HTML from Markdown.
D&D Helpers Custom Rust libs Dice rolls, stat block formatting.
TODO List (Phased Approach)
Phase 1: Core Structure (MVP)
Setup Project

cargo new dnd_scriven + add Dioxus dependencies.

Decide: Pure Dioxus or Tauri hybrid?

Basic Editor

Textarea with Markdown support (pulldown-cmark).

Tabbed sections (e.g., Script, Notes, Characters).

Modular Documents

Split content into "cards" (like Scrivener’s scenes).

Store in SQLite with rusqlite.

Drag-and-Drop Reordering

Use Dioxus hooks + HTML5 drag API (example here).

Phase 2: D&D Enhancements
D&D Templates

Pre-made Rust structs for NPCs, items, spells (e.g., #[derive(Serialize)]).

Dice Roller

Parse strings like 2d20+1 with a Rust crate (dice-roller).

Stat Block Renderer

Format Markdown → PDF (e.g., "**AC** 15" → styled box).

Phase 3: Export & Polish
Export to PDF/HTML

Use wkhtmltopdf or printcss.

Theming

Dark/light mode (Dioxus hooks + CSS).

Keyboard Shortcuts

Ctrl+S to save, Ctrl+E to export.

Phase 4: Extras (Post-MVP)
Cloud sync (via libSQL/Turso).

Plugin system for game systems (e.g., Pathfinder support).

Collaborative editing (WebSocket/CRDTs).

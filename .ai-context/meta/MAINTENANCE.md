# Maintenance Guide

> Last updated: 2026-06-07

## Update Triggers

| Event | Files to Update |
|-------|-----------------|
| New crate added/removed | ARCHITECTURE.md, PROJECT-ESSENCE.md |
| New LLM or search provider | ARCHITECTURE.md, DECISIONS.md (if pattern changes) |
| Major refactor or restructure | All files |
| New non-obvious design decision | DECISIONS.md |
| New blocker or constraint | DYNAMICS.md |
| Resolved blocker | Remove from DYNAMICS.md |

## Quality Standards

- Each file stays under 150 lines
- Total token budget < 4000 tokens
- Every file has a "Last updated" date at top
- Link to source files, never copy code
- State the non-obvious; skip the obvious
- Use diagrams/tables over paragraphs

## Anti-Patterns to Avoid

- Don't document every file — agents can explore code
- Don't copy code snippets — link to files instead
- Don't let DYNAMICS.md become an issue graveyard — remove resolved items
- Don't duplicate content across files
- Don't include details that change frequently (e.g., version numbers)

## Review Schedule

- **Monthly**: Review ARCHITECTURE.md and DYNAMICS.md
- **Per structural change**: Update relevant files immediately
- **Quarterly**: Full review of all files

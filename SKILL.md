---
name: "proximityd"
description: "proximityd - Agent skill for Generic presence detection service with pluggable notifications"
version: "0.1.0"
triggers:
  - "use proximityd"
  - "proximityd help"
  - "proximityd --help"
homepage: "https://github.com/levonk/proximityd"
---

# proximityd

proximityd - Agent skill for Generic presence detection service with pluggable notifications

## Overview

proximityd is a CLI tool for Generic presence detection service with pluggable notifications. This skill provides ambient context and command examples for AI agents.

## Available Commands

### discover

Discover identifier correlations from signal log

**Usage:**
```
proximityd discover --hours 24
```

### status

Show current presence status

**Usage:**
```
proximityd status
```

### parties

List configured parties (includes device count aggregates)

**Usage:**
```
proximityd parties
```

### devices

List configured devices (includes identifier count aggregates)

**Usage:**
```
proximityd devices
```

### export

Export signal log data

**Usage:**
```
proximityd export --format jsonl
```

### install

Install proximityd: generate shell completions and initialize config files

**Usage:**
```
proximityd install
```

### uninstall

Uninstall proximityd: remove completions and optionally config files

**Usage:**
```
proximityd uninstall
```

### completion

Generate shell completion script

**Usage:**
```
proximityd completion bash
```

### man

Display manual page

**Usage:**
```
proximityd man
```

### hooks

Session hook commands for ambient context injection

**Usage:**
```
proximityd hooks session-context
```

**Subcommands:**

- `session-context`: Output session context in TOON format
- `install-agent-hooks`: Install hooks for Claude Code or Codex

### skill

Generate agent skill for AI integration

**Usage:**
```
proximityd skill generate
```

**Subcommands:**

- `generate`: Generate agent skill from CLI metadata
- `check`: Check if skill file is stale (outdated)

## Examples

### Common Workflows

```bash
# Get help
proximityd --help

# Check status
proximityd status

# List devices
proximityd devices


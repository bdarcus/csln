---
description: Synchronize agent personas, skills, and workflows with central repository or template definitions.
---

# Agent Self-Update Workflow

## Purpose
Ensure the local agent configuration (Personas, Workflows, Skills) is aligned with the latest architectural standards and project requirements.

## Steps

### 1. Analyze Current State
- **Persona Audit**: Compare local files in `.agent/personas/` against the `template.md`.
- **Workflow Audit**: List files in `.agent/workflows/` and check for alignment with `PROTOCOL.md` and `tdd.md`.
- **Skill Audit**: Verify `.agent/skills/` contains required professional skillsets.

### 2. Check for External Updates
// turbo
- Command: `gemini update --check` (or alternative sync script)
- Description: Fetches the latest global or project-specific agent definitions.

### 3. Propose Alignment Changes
- If a persona definition is stale or missing a required section (e.g., "Meta-Orchestration"), propose a diff.
- If a new global workflow is available, propose its installation.

### 4. Execute Update
- Update files in `.agent/` after user confirmation.
- Commit changes with `chore(agent): synchronize personas and workflows`.

## Triggers
- **Manual**: User requests "update yourself" or "sync personas".
- **Periodic**: Once per week or at the start of a major project phase.
- **Dependency Change**: When `tech-stack.md` undergoes a significant architectural shift.

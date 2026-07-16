# Contributing to Semantic Business Insight Layer

We are building this against a 3-week hackathon clock, working across Rust
backend and frontend in parallel. This document exists so none of that
parallel work collides, everyone should be able to open a PR, get reviewed, and merge
without a sync call first.

If something here doesn't fit how we're actually working, change this file. It's a PR like any other.

## 1. Project board

All tasks live on the GitHub Project board: **[board](https://github.com/users/uche09/projects/1/views/1?groupedBy%5BcolumnId%5D=369438265)**.

Every task is listed unassigned, grouped by track (Backend Core, Backend Semantic,
Frontend, Shared). Nobody owns a task until they pick it up. The board is a shared
pool, not a pre-assigned schedule.

**To find your next task:**
1. Filter or scroll to your track's group.
2. Look at the top of the **Backlog** column within that group.
3. Check the task's **Depends On** field — if anything listed there isn't already in **Done**, skip it and pick the next one down. Working a blocked task usually means redoing it once its dependency lands.

**To claim a task:**
1. Assign yourself to the card.
2. Move it to **In Progress**.
3. Branch and commit against it (see branch naming and commit conventions below).

Each card's **Task ID** (e.g. `W1.A3`) maps directly to a section in the
[implementation plan](./Semantic_Insight_Implementation_Plan.docx) — that's where the code snippets, dependencies, and definition of done for that task live in full.

## 2. Before you start a task

- Pick the next **unblocked** task from the implementation plan / project board (P0.4).
  **"Unblocked"** means everything in its `Depends On` column is already merged to `main`.
- If a task touches a shared interface (e.g. the `EmbeddableRecord` shape between multiple backend collaborators), post in the team channel or do a 2-minute sync before writing
  code against it. Agreeing on a function signature first is cheaper than reconciling
  two implementations later.
- Assign yourself on the board so nobody duplicates the task.

## 3. Branch naming

Branch off `main`, name the branch `<type>/<short-description>`:

```
feat/rag-query-endpoint
fix/anomaly-threshold-off-by-one
chore/ci-cache-cargo
docs/api-contract
```

`type` is one of: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`. This matches the
commit convention below, so branch name and commit type should usually agree.

## 4. Commit messages — Conventional Commits

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short summary>

<optional longer body>
```

- `type`: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`
- `scope`: the module or track it touches, `api`, `db`, `embeddings`, `rag`, `anomaly`,
  `frontend`, `ci`
- **summary:** verbs should be imperative, lowercase, no trailing period e.g. "add" not "added" or "adds"

Examples:

```
feat(embeddings): wire embed_and_store into note write path
fix(anomaly): correct cosine distance normalization
chore(ci): add rust cache step to workflow
docs(readme): document local docker-compose setup
```

Why this matters beyond tidiness: a consistent `type(scope)` prefix means anyone can
scan `git log --oneline` and immediately tell which track a change belongs to, without
opening the diff. On a multi-person team moving fast, that's the difference between a
2-second glance and a context-switch.

## 5. Pull requests

- Keep PRs scoped to one task ID from the plan where possible (e.g. one PR for `W1.A5`,
  not `W1.A5` bundled with unrelated cleanup).
- Fill out the PR template, it asks for the task ID, what changed, and how you tested
  it.
- A PR needs **1 approval** and a **passing CI run** before it can merge (enforced by
  branch protection on `main`).
- Prefer squash-merge so `main` has one clean commit per PR.
- If your PR is blocking someone else's track (e.g. Track B is waiting on your
  `embed_and_store` signature), say so explicitly in the PR description and tag them.

## 6. Code review

- Review for correctness and for whether the interface matches what was agreed (does
  the function signature match what the other track is building against?).
- It's fine to approve with small nits left as comments rather than blocking, we don't
  have time for review ping-pong on a 3-week clock. Block only for correctness issues
  or interface mismatches that would break another track.
- If you're the only other backend dev, you're the primary reviewer for Rust PRs; the
  frontend dev is primary reviewer for frontend PRs. Cross-review when you have time,
  but don't let it become the bottleneck.

## 7. Before you open a PR, locally

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

For frontend changes:

```bash
npm run lint
npm run build
```

CI runs the same checks, running them locally first just means you're not waiting on
a CI failure to find out.

## 8. Issues

Use the issue templates for anything that isn't already a task on the board:

- **Bug report** — something that used to work, or should work, doesn't.
- **Task** — new work discovered mid-build that isn't in the original implementation
  plan.

Link the relevant task ID from the implementation plan where one exists.

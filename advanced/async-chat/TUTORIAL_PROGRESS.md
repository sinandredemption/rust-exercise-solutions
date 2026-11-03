# Tutorial Progress

## Progress

```text
Overall  [████████████████████████████████████████] 100%   11/11 chapters done
Chapter 11 [████████████████████████████████████████] 100%   client works end to end; checkpoint passed (Ctrl-D ends the loop, matching the tutorial)
```

Print an updated version of this bar at every semi-major milestone — a chapter
completing, or a step that changes observable behavior — and refresh this block
so it stays the source of truth across sessions. Percentages are estimates:
overall = completed chapters + fractional progress of the current one, out of 11.

## Chapter Checklist

| # | Chapter                              | Status      | Hint level reached | Notes |
|---|---------------------------------------|-------------|---------------------|-------|
| 1 | Writing an async chat (intro)          | done        | -                    | Informational only. |
| 2 | Specification & Getting Started        | done        | -                    | `Cargo.toml` has `tokio = { version = "1", features = ["full"] }` as specified. Project builds. |
| 3 | Writing an Accept Loop                 | done        | 1                    | Explained async functions, awaiting, address resolution, streams, and the accept loop. |
| 4 | Receiving Messages                     | done        | 2                    | Implemented login handling, async line reading, disconnect behavior, and message parsing. |
| 5 | Sending Messages                       | done        | 1                    | Writer loop compiles with split stream halves and concurrent reader/writer tasks; ready for broker. |
| 6 | A broker as a connection point         | done        | 1                    | Broker owns HashMap, handles NewPeer/NewMessage with owned Strings and cloned senders; `cargo check` clean. |
| 7 | Gluing it all together                 | done        | 1                    | Wired broker, connection reader/writer, and accept loop; verified single and multi-dest routing with live TCP test. |
| 8 | Clean Shutdown                         | done        | 2                    | Ctrl-C via `select!`, `Notify` readers, sender drop and broker await; live SIGINT test passed. |
| 9 | Handling Disconnections                | done        | 2                    | Writer owned by broker; oneshot shutdown; disconnect channel reports `(name, rx)`. Removal and shutdown drain verified live. Checkpoint passed: why `rx` travels with the name (no send failure window), why removal happens once. |
| 10| Final Server Code (self-review)        | done        | 1                    | Design walked end to end, including shutdown cascade (notify → connection_loop returns → senders dropped → drain). Shutdown race (notify_waiters before poll) reproduced 11/60 and recorded as known gap. Open nits: `if let Err(_)` at broker send (clippy: `is_err()`), stray `;` after drain `while let`. |
| 11| Implementing a client                  | done        | 2                    | Client in `src/bin/client.rs`. Stdin blocking read keeps runtime alive at shutdown; fixed with `Runtime::shutdown_background` after `block_on` (beyond tutorial). Routing verified live. Checkpoint passed: Ctrl-D ends the client, same as tutorial. Open nit: two needless `return;` in select arms. |

Status values: `not started` / `in progress` / `blocked` / `done`

## Current chapter: none (all 11 chapters done)

- **Goal in plain language:** client reads stdin and socket concurrently, sends
  stdin lines to the server, prints server lines to stdout, and exits cleanly
  when the server goes away.
- **What's been attempted so far:** `src/bin/client.rs` compiles, clippy has two
  needless `return;` warnings in the select arms. Live tests (`/tmp/opencode`,
  scripts may be cleared): alice→bob routing works; client exits 0 after server
  SIGINT with stdin open or closed.
- **Learner findings:** `tokio::io::stdin` reads on a blocking thread that cannot
  be cancelled; runtime shutdown waits for it, so the client hung until input.
  Fix chosen: build `Runtime` manually, `block_on`, then `shutdown_background`.
- **Open questions:** checkpoint not yet run (why `shutdown_background` is safe
  here, why `mut` on `writer` / `lines`, what the spawned tasks own).
- **Known limitation:** the stdin thread is abandoned at process exit; any
  partial line typed is discarded.

## Concept log (things they struggled with, to spot-check later)

- Blocking I/O on the runtime — chapter 11, level 2; `tokio::io::stdin` is a
  blocking read, so dropping its future doesn't stop the thread and runtime
  shutdown waits for it.
- Mutability and receivers — chapter 11, level 1; `mut` is needed for `&mut self`
  methods (`next_line`, `write_all`), not for consuming (`lines()`, `into_split()`).
- Borrow vs move in `block_on` — chapter 11, level 1; `shutdown_background` takes
  the runtime by value, so it must come after `block_on` returns.
- **Known limitations (deliberately retaining tutorial behavior):**
  - Writer tasks are detached; broker completion does not prove their final
    writes finished. The tutorial also does not await them. Fix would be to
    route writer `JoinHandle`s to the broker and join after dropping the peer
    map (evaluated and deferred to stay tutorial-faithful).
  - `Notify::notify_waiters()` stores no permit, so a reader between `select!`
    iterations can miss shutdown. Likely fix: stored-state primitive such as
    `tokio::sync::watch`. Deferred as beyond-tutorial.
- **Open questions / misconceptions to revisit:** The learner is new to async
  Rust; explain `Future`, `async fn`, `.await`, task spawning, and join handles
  explicitly as they arise.

## Concept log (things they struggled with, to spot-check later)

- Async task lifetime and `JoinHandle` — chapter 4, level 2; distinguish
  dropping a handle from awaiting/joining the task.
- Nested async block returning a future — chapter 4, level 2; spawning an
  async block that merely returns `connection_loop(stream)` does not poll the
  returned connection future.
- `Result` handling across spawned tasks — chapter 4, level 2; the task must
  inspect/log the connection result instead of dropping the error.
- Recoverable versus terminal input — chapter 4, level 1; distinguish malformed
  messages from a connection that has not authenticated.
- Owned stream halves versus cloning — chapter 5, level 1; `into_split()` moves
  the stream and returns separate ownership for reading and writing.
- Server architecture and broker routing — chapter 5, level 1; the broker is
  a later server component between connection readers and writer channels.
- Per-connection writer ownership — chapter 5, level 1; each accepted client
  needs its own writer task and socket half, while the broker later sends to
  each writer through its channel.
- Channel ownership and event lifetimes — chapter 6, level 1; events sent
  between tasks must own their Strings, broker channel is many-to-one, peer
  channels are one-to-one.
- Broker error handling asymmetry — chapter 6, level 1; `?` from connection
  to broker but broker must not `?` on per-peer send.
- Shutdown cascade ordering — chapter 8, level 1; readers must exit before the
  broker can drain and close peer channels, and awaiting a writer inside the
  reader deadlocks because the broker still holds a sender clone.
- Edge-triggered vs stored-state signals — chapter 8, level 2; `Notify` wakes
  only current waiters, while a `watch` channel lets late readers observe the
  shutdown state.
- Error conversion with `?` — chapter 8, level 1; `?` performs `From`
  conversion, while passing a `Result` value through unchanged does not.

## Session history

- 2026-08-17: Set up `AGENTS.md` (guided-learning/no-spoilers mode) and this
  progress file. Verified project state: `Cargo.toml`/`Cargo.lock` match
  spec, `cargo check` succeeds with expected in-progress warnings, nothing
  committed to git yet. `src/main.rs` holds chapter 3's scaffold verbatim
  from the tutorial (see notes above) — next session should open with a
  comprehension check on this code before treating chapter 3 as complete.
- 2026-08-19: Completed the Chapter 3 comprehension check, removed numbered
  tutorial comments, and verified `cargo check`. Started Chapter 4.
- 2026-08-20: Learner requested explicit async Rust concept and syntax
  explanations. Clarified that spawned connection tasks run independently;
  dropping their handles is not joining them.
- 2026-08-25: Verified spawned connection futures are awaited inside their
  tasks and connection errors are logged. Began immediate-disconnect handling.
- 2026-08-25: Verified immediate disconnects now return an error and
  `cargo check` passes. Next step is parsing message lines.
- 2026-08-26: Completed Chapter 4 checkpoint questions. Chapter 4 behavior is
  complete; next chapter introduces sending messages.
- 2026-08-27: Started Chapter 5. Discussed single-writer ordering, channel
  decoupling, and `TcpStream::into_split()`.
- 2026-08-28: Verified the writer loop's receiver type and awaited
  `write_all`; `cargo check` passes cleanly. Wiring remains.
- 2026-08-28: Clarified that the project is building a server, that clients
  are external peers, and that the broker is the missing routing component.
- 2026-08-29: Completed Chapter 5. Wired split halves and concurrent writer
  task; `cargo check` clean and `cargo clippy` only suggests `while let`.
  Clarified `&mut` requirements for `recv`/`write_all`. Started Chapter 6
  (broker).
- 2026-08-29: Implemented Event enum with owned Strings, broker HashMap,
  and publisher routing. Fixed channel lifetimes, clone semantics, and
  `from ...` formatting. Live test passed: single-dest, multi-dest, and
  malformed-line handling. Marked Chapters 6 and 7 done.
- 2026-09-28: Resumed from working routing implementation. Started Chapter 8
  orientation; flagged detached writer tasks as a shutdown design question.
- 2026-09-28: Implemented Ctrl-C selection, reader notification, broker-sender
  drop, and broker-handle await. `cargo check` passes; writer join/lifecycle
  and notification timing remain to review.
- 2026-10-22: Completed Chapter 8. Decided to stay tutorial-faithful (writers
  remain detached, `Notify` kept). Live SIGINT test passed: pending message
  delivered, both client sockets closed, exit code 0. Logged known limitations.
  Started Chapter 9.
- 2026-11-01: Completed Chapter 9. Implemented writer-owned disconnect reporting,
  broker `select!` over events and disconnects, and the shutdown drain (drop own
  disconnect sender, then await until `None`). Live tests: removal after leave,
  Ctrl-C exits 0 with no hang, drain removes alice. Checkpoint: learner reasoned
  through why `rx` travels with the name (sends during the removal window are
  queued and discarded, not errored). Learner needed hint level 2 for the drain
  and `select!` arm semantics. Ch. 10 (self-review) is next, pending learner go-ahead.
- 2026-11-01: Started Chapter 10 (self-review). Learner walked the design end to
  end. Known gap found and reproduced: `notify_waiters()` stores no permit, so a
  `connection_loop` spawned but not yet polled when Ctrl-C fires misses the
  shutdown notification. It then waits on login or the next line, the writer's
  stop oneshot never fires, and the broker's drain waits forever. Black-box
  harness (/tmp/opencode/race.py, copy of binary, main.rs untouched): hung 11/60
  trials at 0-5 ms signal delays. Recorded as a known limitation, tutorial-faithful
  design; fix deferred.
- 2026-11-01: Completed Chapter 10 self-review. Learner's shutdown explanation
  needed two corrections (causality runs from senders dropping to `broker_loop`
  returning, not the reverse; drain ends on sender count reaching zero). Learner
  declined a live check of the removed-drop hang and asked to move on. Marked
  Chapter 10 done; Chapter 11 not started pending go-ahead.
- 2026-11-02: Started Chapter 11. Learner designed the client (split TcpStream,
  stdin and socket tasks, select on handles). Found that `tokio::io::stdin`
  reads block a runtime thread, so the client hung at shutdown until input.
  Fixed with `Runtime::shutdown_background` after `block_on`. Live tests: alice→bob
  routing works; client exits 0 after server SIGINT with stdin open or closed.
  Checkpoint and clippy `needless_return` nits remain.
- 2026-11-03: Completed Chapter 11. Checkpoint passed: Ctrl-D ends the client, the
  same as the tutorial's client. Remaining nit: two needless `return;` in the
  select arms. All 11 chapters done.

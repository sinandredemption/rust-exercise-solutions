# Rust Exercise Solutions

My solutions to the [Ferrous Systems Rust Exercises](https://rust-exercises.ferrous-systems.com/latest/book/introduction), plus a few projects from [The Rust Programming Language](https://doc.rust-lang.org/book/). Each folder is an independent Cargo crate: `cd` into it and run `cargo run` or `cargo test`.

Exercise starter code comes from Ferrous Systems and is licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). The Book projects started from the Book's code, and I changed them.

## AI disclosure

The async chat in `advanced/async-chat/` was built with an AI mentor, configured by `AGENTS.md`. The AI never wrote solution code. It asked questions, gave graduated hints, reviewed my code, and ran `cargo check`, `clippy`, and live TCP tests. Progress is logged in `TUTORIAL_PROGRESS.md`. The starting scaffold for chapter 3 came from the tutorial. All other code in this repo is mine, written without AI assistance.

## Navigation

### Fundamentals (`fundamentals/`)

| Folder | Exercise | Notes |
|---|---|---|
| `fizzbuzz/` | [Fizzbuzz](https://rust-exercises.ferrous-systems.com/latest/book/fundamentals/fizzbuzz.html) | |
| `rustlatin/step1` to `step4` | [Rust Latin](https://rust-exercises.ferrous-systems.com/latest/book/fundamentals/rustlatin.html) | One crate per step |
| `urls-match-result/` | [URLs, match, result](https://rust-exercises.ferrous-systems.com/latest/book/fundamentals/urls-match-result.html) | |
| `calculator/` | [Calculator](https://rust-exercises.ferrous-systems.com/latest/book/fundamentals/calculator.html) | |
| `iterators/` | [Iterators](https://rust-exercises.ferrous-systems.com/latest/book/fundamentals/iterators.html) | |
| `simple-db/` | [SimpleDB](https://rust-exercises.ferrous-systems.com/latest/book/fundamentals/simple-db.html) | |
| `green-yellow/` | [Green and Yellow game](https://rust-exercises.ferrous-systems.com/latest/book/fundamentals/green-yellow-game.html) | |

### Applied Rust (`applied/`)

| Folder | Exercise | Notes |
|---|---|---|
| `shapes/` | [Shapes](https://rust-exercises.ferrous-systems.com/latest/book/applied/shapes.html) | Does not compile yet |
| `tcp-server/` | [TCP Echo Server](https://rust-exercises.ferrous-systems.com/latest/book/applied/tcp-server.html) | |
| `connected-mailbox/simple-db/`, `connected-mailbox/tcp-server/` | [Connected Mailbox](https://rust-exercises.ferrous-systems.com/latest/book/applied/connected-mailbox.html) | Two crates |

### Advanced Rust (`advanced/`)

| Folder | Exercise | Notes |
|---|---|---|
| `async-chat/` | [Async TCP Chat Server](https://rust-exercises.ferrous-systems.com/latest/book/async-chat/index.html) | Server in `src/main.rs`, client in `src/bin/client.rs`. Includes the AI mentor files. |

### Rust Book projects (`book-practice/`)

| Folder | Book chapter | Notes |
|---|---|---|
| `hello/` | [Final project: web server](https://doc.rust-lang.org/book/ch21-02-multithreaded.html) | The thread pool in `src/lib.rs` is unfinished: `execute` does nothing |
| `minigrep/` | [Improving our I/O project](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html) | |
| `add/` | [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) | Workspace with `adder` and `add_one` |
| `hello-async/` | [Applying concurrency with async](https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html) | |

# `option_into_controlflow` - `Option` into `ControlFlow` conversion

Convert `Option<T>` into `ControlFlow<T, _>` or `ControlFlow<_, T>`.

## Usage

Analogically to `ok_or` and `ok_or_else` for converting into `Result`, this crate introduces:
- `break_or`/`continue_or`,
- `break_or_else`/`continue_or_else` and
- `break_or_default`/`continue_or_default` for converting into `ControlFlow`.

Since `ControlFlow` is more symmetrical than `Result`, functions exist for converting the `Some`
variant both into `Break` and `Continue`.

## Why?

Suppose you are receiving some messages:

```rust
async fn process_messages(mut msgs: mpsc::Receiver<i32>) {
    while let Some(msg) = msgs.recv().await {
        println!("msg = {}", msg)
    }
}
```

but now you need to support cancellation, so you do this:

```rust
async fn process_messages(mut msgs: mpsc::Receiver<i32>, token: CancellationToken) {
    while let Some(msg) = select! { biased;
        _ = token.cancelled() => None,
        m = msgs.recv() => m,
    } {
        println!("msg = {}", msg)
    }
}
```

and that's fine, but now you're using `Option` for controlling flow. `None` used to mean there will
be no more messages. This is semantically correct for a receiver and it's fine to pattern-match on
it in a loop condition. But now, `None` means "there will be no more messages OR processing is
cancelled". As the logic becomes more complicated, you might want to pattern-match on
`ControlFlow::Continue` instead, which is there for a reason - it conveys whether to continue or to
break.

So you can do this:

```rust
async fn process_messages(mut msgs: mpsc::Receiver<i32>, token: CancellationToken) {
    while let ControlFlow::Continue(msg) = select! { biased;
        _ = token.cancelled() => ControlFlow::Break(()),
        m = msgs.recv() => {
            if let Some(m) = m {
                ControlFlow::Continue(m)
            } else {
                ControlFlow::Break(())
            }
        },
    } {
        println!("msg = {}", msg)
    }
}
```

but it's so verbose, so, of course, you use `option_into_controlflow`:

```rust
async fn process_messages(mut msgs: mpsc::Receiver<i32>, token: CancellationToken) {
    while let ControlFlow::Continue(msg) = select! { biased;
        _ = token.cancelled() => ControlFlow::Break(()),
        m = msgs.recv() => m.continue_or(()),
    } {
        println!("msg = {}", msg)
    }
}
```

which is the best of both worlds - you no longer use `Option` for controlling the loop, but your
code becomes semantic, idiomatic and understandable.

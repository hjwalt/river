# River

Because river flows (you can see I have an obsession with that name)

## What is this?

This is my experimental work with rust-lang to build a simple stream processing system with functional paradigm.

The original flows are written in golang, and I am assessing if rust is a better language for it due to type, memory, and thread safety.

## Notes

- Primary difference between golang and rust multi-threading is goroutine vs async-await, a really different paradigm
  - goroutine (similar to Java fiber) lightens multithreading by using n virtual to m native thread
  - async-await flips the narrative by having the native work-stealing threads poll the functions available to execute on its state (which can be complicated in the async execution system, but we have tokio and some other libs for that)

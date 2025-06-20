# Can memory leaks happen in Safe Rust

# Some advanced Rust Primitives

🔍 Comparison of Rust Smart Pointers & Interior Mutability Types

| Type            | Category            | Use Case                                                   | When Not to Use                                                           | Pros                                                       | Cons                                                  |
|-----------------|---------------------|------------------------------------------------------------|---------------------------------------------------------------------------|------------------------------------------------------------|-------------------------------------------------------|
| `Box<T>`        | Smart Pointer       | Heap allocation for recursive, large, or unsized types     | When stack allocation is sufficient                                       | Simple, stable, owned heap pointer                         | Adds indirection, heap allocation cost                |
| `Rc<T>`         | Smart Pointer       | Shared ownership in **single-threaded** programs           | In multi-threaded contexts; leads to reference cycles                     | Enables shared ownership                                   | Not thread-safe, potential memory leaks due to cycles |
| `Arc<T>`        | Smart Pointer       | Shared ownership in **multi-threaded** contexts            | When you don't need thread-safe reference counting                        | Thread-safe shared ownership via atomic ops                | Slower due to atomic operations                       |
| `Cell<T>`       | Interior Mutability | Mutate `Copy` types via `&self` in single-threaded context | For non-`Copy` or when borrowing rules matter                             | Zero-cost interior mutability for `Copy` types             | No borrowing; can't hold non-`Copy` values            |
| `RefCell<T>`    | Interior Mutability | Mutate non-`Copy` types with runtime-checked borrows       | Multi-threaded use; when borrow panic risk is unacceptable                | Allows dynamic borrow checking, supports non-`Copy` values | Panics at runtime if borrowing rules violated         |
| `Mutex<T>`      | Sync Primitive      | Safe exclusive access to data across threads               | Single-threaded; avoid when lock contention or deadlocks possible         | Thread-safe interior mutability with blocking lock         | Can deadlock, involves locking overhead               |
| `RwLock<T>`     | Sync Primitive      | Many readers or one writer access pattern across threads   | High write frequency or complexity around lock management                 | Concurrent reads allowed, thread-safe                      | Writers block all readers; complexity increases       |
| `UnsafeCell<T>` | Unsafe Core Type    | Implementing custom interior mutability or safe wrappers   | Everyday use; prefer `Cell`/`RefCell` unless building unsafe abstractions | Foundation of safe wrappers like `RefCell`, `Mutex`, etc.  | Requires `unsafe`; easy to misuse                     |

| Type          | Stack-Allocated | Heap-Allocated | Global (Static)? | Mutability                    | Thread-Safe | Notes                                   |
|---------------|-----------------|----------------|------------------|-------------------------------|-------------|-----------------------------------------|
| `&T`          | ✅               | ❌              | ❌                | Immutable                     | ✅           | Borrowed reference to any memory        |
| `&mut T`      | ✅               | ❌              | ❌                | Mutable (exclusive borrow)    | ✅           | Compile-time enforced                   |
| `Box<T>`      | ✅ (box ptr)     | ✅ (`T`)        | ❌                | Mutable if owned              | ✅           | Moves ownership, allocates on heap      |
| `Rc<T>`       | ✅ (Rc ptr)      | ✅ (`T`)        | ❌                | Immutable or interior mut     | ❌           | Single-threaded shared ownership        |
| `Arc<T>`      | ✅ (Arc ptr)     | ✅ (`T`)        | ❌                | Immutable or interior mut     | ✅           | Multi-threaded shared ownership         |
| `RefCell<T>`  | ✅ or in Rc/Box  | ✅ (`T`)        | ❌                | Mutable (runtime-checked)     | ❌           | For interior mutability (1 thread only) |
| `Mutex<T>`    | ✅ or in Arc     | ✅ (`T`)        | ❌                | Mutable via lock              | ✅           | Thread-safe mutability via lock         |
| `RwLock<T>`   | ✅ or in Arc     | ✅ (`T`)        | ❌                | Shared read / exclusive write | ✅           | More flexible locking than Mutex        |
| `static`      | ❌               | ❌              | ✅                | Only with `mut` + unsafe      | ⚠️ Depends  | One global instance, avoid `mut static` |
| `OnceCell<T>` | ❌               | ✅ (interior)   | ✅                | Immutable after init          | ✅           | Safe global lazy init                   |

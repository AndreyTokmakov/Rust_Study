## Сводная таблица smart pointers в Rust

| Тип              | Владение   | Потоки | Мутабельность | Когда использовать  | Пример            |
| ---------------- | ---------- | ------ | ------------- | ------------------- | ----------------- |
| `&T`             | Borrow     | ✔ / ✖  | ❌             | Временный доступ    | `fn f(x: &T)`     |
| `&mut T`         | Borrow     | ✔ / ✖  | ✔             | Временная мутация   | `fn f(x: &mut T)` |
| `Box<T>`         | 1 owner    | ✔      | ✔             | Heap + single owner | AST, recursion    |
| `Rc<T>`          | Shared     | ❌      | ❌             | Shared, 1 thread    | GUI, tree         |
| `Rc<RefCell<T>>` | Shared     | ❌      | ✔             | Shared + mutate     | Graph             |
| `Arc<T>`         | Shared     | ✔      | ❌             | Shared threads      | Config            |
| `Arc<Mutex<T>>`  | Shared     | ✔      | ✔             | Shared mutable      | Counters          |
| `Arc<RwLock<T>>` | Shared     | ✔      | ✔             | Read-heavy          | Cache             |
| `Weak<T>`        | No owner   | ✔ / ✖  | ❌             | Break cycles        | Trees             |
| `Cow<'a, T>`     | Borrow/Own | ✔      | ✔             | Lazy copy           | Parsers           |
| `Pin<Box<T>>`    | 1 owner    | ✔      | ✔             | Async / self-ref    | Futures           |

---

### `&T` — borrow (по умолчанию)

```rust
fn print_len(s: &String)
{
    println!("{}", s.len());
}
```

✔ без владения
✔ zero-cost

---

### `Box<T>` — heap + один владелец

```rust
struct Node
{
    next: Option<Box<Node>>,
}
```

✔ рекурсивные структуры
✔ deterministic drop

---

### `Rc<T>` — shared ownership (1 поток)

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = a.clone();
```

✔ AST
✔ GUI
❌ threads

---

### `Rc<RefCell<T>>` — shared + mutable

```rust
use std::{rc::Rc, cell::RefCell};

let v = Rc::new(RefCell::new(10));
*v.borrow_mut() += 1;
```

⚠ runtime borrow check

---

### `Arc<T>` — shared между потоками

```rust
use std::sync::Arc;

let v = Arc::new(42);
let v2 = v.clone();
```

✔ multi-thread
❌ mutability

---

### `Arc<Mutex<T>>` — shared mutable (threads)

```rust
use std::sync::{Arc, Mutex};

let c = Arc::new(Mutex::new(0));
*c.lock().unwrap() += 1;
```

✔ самый частый паттерн
⚠ lock contention

---

### `Arc<RwLock<T>>` — read-heavy

```rust
use std::sync::{Arc, RwLock};

let cfg = Arc::new(RwLock::new("v1".to_string()));
println!("{}", cfg.read().unwrap());
```

✔ много readers
✔ мало writers

---

### `Weak<T>` — разрыв циклов

```rust
use std::sync::{Arc, Weak};

struct Node
{
    parent: Weak<Node>,
}
```

✔ trees / graphs
❌ не продлевает lifetime

---

### `Cow<'a, T>` — copy-on-write

```rust
use std::borrow::Cow;

fn normalize(s: Cow<str>) -> Cow<str>
{
    if s.contains(" ")
    {
        Cow::Owned(s.replace(" ", "_"))
    }
    else
    {
        s
    }
}
```

✔ parsers
✔ zero-copy fast path

---

### `Pin<Box<T>>` — immovable data

```rust
use std::pin::Pin;

let x = Pin::new(Box::new(5));
```

✔ async
✔ self-referential structs

---

## Как выбрать (cheat sheet)

```text
Borrow enough?        -> &T / &mut T
One owner?            -> Box<T>
Shared (1 thread)?    -> Rc<T>
Shared + mutate?      -> Rc<RefCell<T>>
Threads?              -> Arc<T>
Threads + mutate?     -> Arc<Mutex<T>>
Read-heavy threads?   -> Arc<RwLock<T>>
Cycle risk?           -> Weak<T>
Avoid copy?           -> Cow<T>
Async / futures?      -> Pin<Box<T>>
```

---

## ❌ Частые ошибки

* Использовать `Arc<Mutex<T>>` **везде**
* Использовать `Rc` в async / tokio
* Забывать `Weak` → memory leak
* Использовать `Box` вместо `&T`
* Слишком рано уходить в `RefCell`
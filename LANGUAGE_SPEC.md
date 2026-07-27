# Kinetix Language Specification

**Version**: 0.1.0-draft  
**File extension**: `.kx`  
**Entry point**: `fn main()`

---

## 1. Lexical Structure

### 1.1 Comments

```kinetix
// Single-line comment
/* Block comment — can span
   multiple lines */
/// Doc comment (attached to the next item)
```

### 1.2 Keywords

| Control Flow | Declarations | Modules | Values | Types |
|---|---|---|---|---|
| `if` `else` `elif` | `fn` `let` `mut` `const` | `import` `export` `mod` | `true` `false` `nil` | `int` `float` `bool` `string` |
| `for` `while` `loop` | `struct` `impl` `enum` | | | `vec` `map` `matrix` |
| `break` `continue` `return` | `trait` `type` | | | |
| `match` | | | | |
| `async` `await` `spawn` | | | | |

### 1.3 Literals

```kinetix
// Integer
42
0xFF         // hex
0b1010       // binary
0o17         // octal
1_000_000    // underscores allowed

// Float
3.14
2.0e10
1.5E-3

// String (UTF-8, double quotes)
"Hello, world!"
"Escape: \n \t \r \" \\"
"Interpolation: {variable}"  // built-in string interpolation

// Char (single quotes)
'a'
'\n'

// Bool
true
false

// Nil
nil
```

### 1.4 Identifiers

Identifiers begin with a letter or `_`, followed by letters, digits, or `_`.

```
identifier = (letter | '_') (letter | digit | '_')*
```

---

## 2. Type System

Kinetix uses **full static typing** with **Hindley-Milner type inference**. You rarely need to annotate types explicitly, but you can.

### 2.1 Primitive Types

| Type | Description | Literal |
|---|---|---|
| `int` | 64-bit signed integer | `42` |
| `float` | 64-bit IEEE 754 float | `3.14` |
| `bool` | Boolean | `true` / `false` |
| `string` | UTF-8 string | `"hello"` |
| `char` | Unicode scalar | `'a'` |
| `nil` | Absence of value | `nil` |

### 2.2 Compound Types

```kinetix
// Tuple
let t: (int, float, string) = (1, 2.0, "three")
let x = t.0    // 1

// Array (fixed size, stack allocated)
let arr: [int; 5] = [1, 2, 3, 4, 5]

// Vec (growable, heap)
let v: vec<int> = [1, 2, 3]

// Map (hash map)
let m: map<string, int> = {"a": 1, "b": 2}

// Matrix (2D numeric array — MATLAB-inspired)
let A: matrix<float> = [
    [1.0, 2.0];
    [3.0, 4.0]
]
```

### 2.3 Optional Type

```kinetix
let x: ?int = nil        // Optional int (nil or int)
let y: ?int = 42

if let val = y {
    io::println("Got: {val}")
}
```

### 2.4 Function Types

```kinetix
let add: fn(int, int) -> int = |a, b| a + b
```

### 2.5 Generics

```kinetix
fn identity<T>(x: T) -> T { x }

struct Box<T> {
    value: T,
}

// Trait bounds
fn max<T: Ord + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

---

## 3. Variables and Binding

```kinetix
let x = 42              // Immutable binding (inferred: int)
let mut y = 3.14        // Mutable binding (inferred: float)
const MAX: int = 1000   // Compile-time constant

y = 6.28                // OK — mutable
// x = 100              // ERROR — immutable

// Destructuring
let (a, b) = (1, 2)
let [first, ..rest] = [1, 2, 3, 4]
let Point { x, y } = p
```

---

## 4. Control Flow

### 4.1 If / Else / Elif

```kinetix
if x > 0 {
    io::println("positive")
} elif x < 0 {
    io::println("negative")
} else {
    io::println("zero")
}

// If as expression
let sign = if x > 0 { 1 } elif x < 0 { -1 } else { 0 }
```

### 4.2 Loops

```kinetix
// While
while x > 0 {
    x -= 1
}

// For (iterator-based)
for i in 0..10 {
    io::println(i)
}

for item in collection {
    process(item)
}

// Infinite loop with break
loop {
    if done { break }
}

// Loop as expression
let result = loop {
    counter += 1
    if counter == 10 { break counter * 2 }
}
```

### 4.3 Pattern Matching

```kinetix
match value {
    0           => io::println("zero"),
    1..=9       => io::println("single digit"),
    n if n < 0  => io::println("negative: {n}"),
    _           => io::println("large"),
}

// Match on enum
match shape {
    Shape::Circle(r)        => math::PI * r * r,
    Shape::Rectangle(w, h)  => w * h,
    Shape::Triangle(a, b, c) => triangle_area(a, b, c),
}

// Match on type
match value {
    x: int    => io::println("int: {x}"),
    x: float  => io::println("float: {x}"),
    x: string => io::println("string: {x}"),
    _         => io::println("other"),
}
```

---

## 5. Functions

```kinetix
// Basic function
fn greet(name: string) -> string {
    "Hello, {name}!"   // Last expression is return value
}

// Explicit return
fn abs(x: int) -> int {
    if x < 0 { return -x }
    x
}

// Multiple return values (tuple)
fn divmod(a: int, b: int) -> (int, int) {
    (a / b, a % b)
}

// Variadic
fn sum(nums: ..int) -> int {
    nums.fold(0, |acc, x| acc + x)
}

// Default arguments
fn connect(host: string, port: int = 8080) {
    // ...
}

// Async function
async fn fetch(url: string) -> string {
    let resp = await http::get(url)
    resp.text()
}
```

---

## 6. Structs and impl

```kinetix
struct Point {
    x: float,
    y: float,
}

impl Point {
    // Constructor (static method)
    fn new(x: float, y: float) -> Point {
        Point { x, y }
    }

    // Instance method
    fn distance(self, other: Point) -> float {
        let dx = self.x - other.x
        let dy = self.y - other.y
        math::sqrt(dx*dx + dy*dy)
    }

    // Mutable method
    fn translate(mut self, dx: float, dy: float) {
        self.x += dx
        self.y += dy
    }
}

// Usage
let p1 = Point::new(0.0, 0.0)
let p2 = Point::new(3.0, 4.0)
io::println(p1.distance(p2))   // 5.0
```

---

## 7. Enums

```kinetix
enum Color {
    Red,
    Green,
    Blue,
    Custom(int, int, int),   // RGB tuple variant
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Usage
let c = Color::Custom(255, 128, 0)
let r: Result<int, string> = Result::Ok(42)

match r {
    Result::Ok(v)  => io::println("Value: {v}"),
    Result::Err(e) => io::println("Error: {e}"),
}
```

---

## 8. Traits

```kinetix
trait Display {
    fn display(self) -> string
}

trait Numeric: Add + Sub + Mul + Div + Ord + Copy {}

impl Display for Point {
    fn display(self) -> string {
        "({self.x}, {self.y})"
    }
}

// Trait objects (dynamic dispatch)
fn print_all(items: vec<dyn Display>) {
    for item in items {
        io::println(item.display())
    }
}
```

---

## 9. Operator Overloading

```kinetix
impl Point {
    fn operator+(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
    fn operator-(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
    fn operator*(self, scalar: float) -> Point {
        Point::new(self.x * scalar, self.y * scalar)
    }
    fn operator==(self, other: Point) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn operator[](self, idx: int) -> float {
        match idx { 0 => self.x, 1 => self.y, _ => panic("out of bounds") }
    }
}
```

Overloadable operators: `+` `-` `*` `/` `%` `**` `==` `!=` `<` `<=` `>` `>=` `[]` `()` `<<` `>>`

---

## 10. Matrix Operations (MATLAB-inspired)

```kinetix
// Literal syntax
let A: matrix<float> = [
    [1.0, 2.0, 3.0];    // semicolons separate rows
    [4.0, 5.0, 6.0];
    [7.0, 8.0, 9.0]
]

// Indexing (0-based)
let e   = A[1, 2]        // element: 6.0
let row = A[0, ..]       // first row: [1.0, 2.0, 3.0]
let col = A[.., 1]       // second column: [2.0, 5.0, 8.0]
let sub = A[0..2, 0..2]  // submatrix

// Built-in operations
let B  = A.T             // Transpose
let C  = A * B           // Matrix multiply
let D  = A .* B          // Element-wise multiply (Hadamard)
let tr = A.trace()       // Trace
let d  = A.det()         // Determinant
let Ai = A.inv()         // Inverse
let ev = A.eigen()       // Eigenvalues/eigenvectors

// Arithmetic broadcasts
let E = A + 1.0          // Add scalar to all elements
let F = A * 2.0          // Scale matrix

// Range construction
let v: vec<int> = 1..=10   // [1, 2, ..., 10]
let r: vec<float> = linspace(0.0, 1.0, 100)
```

---

## 11. Closures and Functional Style

```kinetix
// Closure syntax: |params| body
let add = |a: int, b: int| -> int { a + b }
let double = |x| x * 2      // Type inferred

// Higher-order functions on vec
let nums: vec<int> = 1..=10
let evens  = nums.filter(|x| x % 2 == 0)
let doubled = evens.map(|x| x * 2)
let sum     = doubled.fold(0, |acc, x| acc + x)

// Method chaining
let result = (1..=100)
    .filter(|x| x % 3 == 0 || x % 5 == 0)
    .sum()
```

---

## 12. Modules

```kinetix
// File: geometry.kx
mod geometry {
    pub struct Point { pub x: float, pub y: float }
    pub fn distance(a: Point, b: Point) -> float { /* ... */ }
}

// File: main.kx
import geometry::Point
import geometry::distance
import std::io

fn main() {
    let p = Point { x: 1.0, y: 2.0 }
}
```

---

## 13. Memory Model

| Model | Syntax | Description |
|---|---|---|
| Reference-counted (default) | `let x = MyStruct { ... }` | Automatic memory management |
| Heap box | `let x = box MyStruct { ... }` | Explicit heap allocation |
| Raw pointer | `let x: raw<T> = raw::alloc::<T>()` | Unsafe manual memory |
| Stack copy | `let x: int = 42` | Value types always on stack |

```kinetix
// Default: reference counted, shared ownership
let a = BigStruct::new()
let b = a    // b and a share ownership (Arc<T>-like)

// Explicit clone
let c = a.clone()  // deep copy

// Box: single owner, heap
let d = box BigStruct::new()
```

---

## 14. Async / Concurrency

```kinetix
// Async function
async fn download(url: string) -> vec<u8> {
    let resp = await http::get(url)
    resp.bytes()
}

// Spawn a task
fn main() {
    let task1 = spawn download("https://example.com/a")
    let task2 = spawn download("https://example.com/b")

    let [data1, data2] = await [task1, task2]   // await multiple
}
```

---

## 15. Inline Assembly

```kinetix
// For performance-critical code (power users)
fn fast_abs(x: float) -> float {
    let result: float
    asm {
        movss  xmm0, [x]
        andps  xmm0, [SIGN_MASK]
        movss  [result], xmm0
    }
    result
}
```

---

## 16. Standard Library

| Module | Contents |
|---|---|
| `std::io` | `println`, `print`, `read_line`, file I/O |
| `std::math` | `sqrt`, `sin`, `cos`, `log`, `exp`, `PI`, `E` |
| `std::matrix` | Matrix operations, `linspace`, `zeros`, `ones`, `eye` |
| `std::vec` | `push`, `pop`, `map`, `filter`, `fold`, `sort` |
| `std::map` | `insert`, `get`, `remove`, `keys`, `values` |
| `std::string` | `split`, `join`, `trim`, `contains`, `replace` |
| `std::fs` | File system operations |
| `std::net` | TCP/UDP sockets, HTTP |
| `std::async` | `spawn`, `sleep`, channels |
| `std::ffi` | Foreign function interface (call C code) |

---

## 17. Error Handling

```kinetix
// Functions that can fail return Result<T, E>
fn parse_int(s: string) -> Result<int, string> {
    // ...
}

// Propagate with ?
fn process(s: string) -> Result<int, string> {
    let n = parse_int(s)?   // Returns early on Err
    Ok(n * 2)
}

// Pattern match on result
match process("42") {
    Ok(v)  => io::println("Got: {v}"),
    Err(e) => io::println("Error: {e}"),
}

// Panic (unrecoverable)
panic("This should never happen: {reason}")
```

---

*This is the living Kinetix Language Specification — it will evolve as the language matures.*

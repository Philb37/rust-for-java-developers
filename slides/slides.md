---
# try also 'default' to start simple
theme: default
# random image from a curated Unsplash collection by Anthony
# like them? see https://unsplash.com/collections/94734566/slidev
background: https://cover.sli.dev
# some information about your slides (markdown enabled)
title: Rust for Java Developers
info: Talk
# apply UnoCSS classes to the current slide
class: text-center
# https://sli.dev/features/drawing
drawings:
  persist: false
# slide transition: https://sli.dev/guide/animations.html#slide-transitions
transition: slide-left
# enable Comark Syntax: https://comark.dev/syntax/markdown
comark: true
# fonts — auto-loaded from Google Fonts (see https://sli.dev/custom/fonts)
fonts:
  sans: Inter
  serif: Space Grotesk
  mono: JetBrains Mono
  weights: '400,500,600,700'
# duration of the presentation
duration: 45min
---

# Rust for Java Developers

<slide-number/>

<!--
The last comment block of each slide will be treated as slide notes. It will be visible and editable in Presenter Mode along with the slide. [Read more in the docs](https://sli.dev/guide/syntax.html#notes)
-->

---
title: "Who am I?"
layout: center
class: text-center
---

# Who am I ?

<div class="whoami">
  <div class="whoami-name">Philippe Billard</div>
  <div class="whoami-bar"></div>
  <div class="whoami-role">Tech Lead · 7+ years of experience</div>
  <div class="whoami-tags">
    <span>Developer</span>
    <span>Cloud</span>
    <span>Data / AI</span>
  </div>
</div>

<section-label name="Intro" />
<slide-number/>

<style>
.whoami {
  margin-top: 2.2rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.1rem;
}
.whoami-name {
  font-size: 2.4rem;
  font-weight: 700;
  letter-spacing: -0.015em;
}
.whoami-bar {
  width: 4rem;
  height: 4px;
  border-radius: 9999px;
  background: linear-gradient(90deg, #c94ee5, #e39d41);
}
.whoami-role {
  font-size: 1.1rem;
  font-weight: 500;
  opacity: 0.8;
}
.whoami-tags {
  display: flex;
  gap: 0.8rem;
  margin-top: 0.4rem;
}
.whoami-tags span {
  padding: 0.4rem 1.15rem;
  border-radius: 9999px;
  font-size: 0.95rem;
  font-weight: 500;
  color: #d67693;
  border: 1px solid rgba(214, 118, 147, 0.4);
  background: rgba(214, 118, 147, 0.1);
}
</style>

---
title: "My problem"
layout: topic
---

<div class="text-center opacity-70 mb-6">I'm tired of those kind of errors</div>

<div class="flex flex-col items-center gap-2 max-w-2xl mx-auto w-full">
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">Null Pointer Exception</div>
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">400 Bad Request (most of the time because we sent a null value)</div>
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">Trying to store a null value in DB</div>
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">Singleton with a private field being updated by everyone</div>
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">Timeouts</div>
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">500 Internal Server errors to external APIs</div>
</div>

<section-label name="Intro" />
<slide-number/>

<!-- 
I've been working with java for over 7 years now. I've seen (and done) a lot of bad code, bad practices. As an example in my last project 90% of the errors we had in production (and there was a lot) were one of those :
-->

---
title: "Why Rust?"
layout: center
class: text-center
---

<SectionDivider index="01" name="Why Rust ?" />

<slide-number/>

---
title: "Graydon Hoare"
layout: quote
---

<div class="max-w-3xl mx-auto text-center">

<p class="text-3xl leading-snug italic">
“It’s ridiculous that we computer people couldn’t even make an elevator that works without crashing!”
</p>

<p class="mt-8 text-xl opacity-60 not-italic">
— Graydon Hoare, creator of Rust
</p>

</div>

<section-label name="Why Rust" />
<slide-number/>

<!--
Graydon Hoare suspected his apartment elevator kept breaking down because of memory
errors — likely coming from its C or C++ software. That frustration was the spark for
Rust: a concurrent, memory-safe language without a garbage collector, which he set out
to build during his time at Mozilla.
-->

---
title: "The basics"
layout: center
class: text-center
---

<SectionDivider index="03" name="The basics 🏗️" />

<slide-number/>

---
title: "Type system, traits & generics"
layout: topic
---

<div class="grid grid-cols-2 gap-4 auto-rows-fr max-w-4xl mx-auto w-full">
  <div v-click="1" class="rounded-lg p-4" style="border:1px solid rgba(201,78,229,0.25)">
    <div class="subtitle mb-2">Built-in types</div>
    <div class="text-[#52525b]">Boolean · Numeric (int &amp; float) · Char, str, String · Tuple · Array</div>
  </div>
  <div v-click="3" class="rounded-lg p-4" style="border:1px solid rgba(201,78,229,0.25)">
    <div class="subtitle mb-2">Trait(s)</div>
    <div class="text-[#52525b]">Shared behavior &amp; composition — no inheritance</div>
  </div>
  <div v-click="2" class="rounded-lg p-4" style="border:1px solid rgba(201,78,229,0.25)">
    <div class="subtitle mb-2">User-defined types</div>
    <div class="text-[#52525b]">Struct · Enum</div>
  </div>
  <div v-click="4" class="rounded-lg p-4" style="border:1px solid rgba(201,78,229,0.25)">
    <div class="subtitle mb-2">Generics</div>
    <div class="text-[#52525b]">Like Java — but the approaches differ</div>
  </div>
</div>

<section-label name="The language" />
<slide-number/>

<!--
Let's start at the beginning: Rust has a really good and strong type system.
Like many other languages it has a lot of types, although it does not handle them the same way.

[click] We have Boolean, Numeric types (not really the same as Java), String types (again, not really the same), Tuple, Arrays.

[click] But contrary to Java, we have no class. The equivalent would be a Struct, a user-defined type, like enums.

[click] No class, no inheritance. In Rust we only have Trait(s) that define shared behavior and enable composition.

[click] Like Java there are generics, but the two approaches differ wildly. I won't get into it right now, just know that it exists. 
-->

---
title: "No null, no exceptions"
layout: two-cols-header
layoutClass: gap-8
---

<SlideHeading text="No null, no exceptions — no 1 billion $ mistake" />

::left::

<div v-click class="h-full flex flex-col justify-center">

**Result&lt;T, E&gt;** — the exception equivalent

```rust
pub enum Result<T, E> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(E),
}
```

</div>

::right::

<div v-click class="h-full flex flex-col justify-center">

**Option&lt;T&gt;** — a strong Optional by design

```rust
pub enum Option<T> {
    /// No value.
    None,
    /// Some value of type `T`.
    Some(T),
}
```

</div>

<section-label name="The language" />
<slide-number/>

<!--
This one is one of my favorites, in Rust there is no exception and no null.

You might be wondering how ? Simple errors and optional values are baked into the type system. They are Enums that you must handle.

[click] Result<T, E> (the exception equivalent)

[click] Option<T> (a strong Optional by design)

Since they are enums, when you get a Result or an Option you must perform checks to ensure which variant you got, and handle all possible cases. Those are rules the compiler enforces, contrary to Java where a null or an Exception is a runtime problem that the compiler doesn't force you to verify.
-->

---
title: "Mutability"
layout: topic
---

<div class="max-w-2xl mx-auto">

````md magic-move {lines: true}
```rust
// ❌ foo is immutable by default
fn main() {
    let foo = String::from("foo");

    foo.insert(3, 's');

    println!("Here is my value ? {foo}");
}
```

```rust
// ✅ fixed — declare `foo` as mutable
fn main() {
    let mut foo = String::from("foo");

    foo.insert(3, 's');

    println!("Here is my value : {foo}");
}
```
````

<div class="mt-3 mut-output">
  <code-output type="error" :class="{ invisible: $clicks !== 0 }" :text="'error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable'" />
  <code-output type="stdout" :class="{ invisible: $clicks < 1 }" :text="'Here is my value : foos'" />
</div>

</div>

<section-label name="The language" />
<slide-number/>

<!-- 
In java you must use the final keyword to make a variable imutable. In Rust they are immutable by default. If you want to opt out of the immutability you need to use the `mut` keyword. 
-->

---
title: "Ownership & borrowing"
layout: two-cols-header
layoutClass: gap-8
---

<SlideHeading text="Ownership &amp; borrowing" />

::left::

<div v-click>

<span class="subtitle">Ownership</span>

```rust
fn take_ownership(value: String) {
    println!("Owned value : {value}");
}

fn main() {
    let foo = String::from("foo");
    take_ownership(foo);

    println!("Here is my value ? {foo}");
}
```
</div>

<div v-click>
    <code-output type="error" :text="'error[E0382]: borrow of moved value: `foo`'" />
</div>

::right::

<div v-click>

<span class="subtitle">Borrowing</span>

```rust
fn borrow(value: &str) {
    println!("Borrowed value : {value}");
}

fn main() {
    let foo = String::from("foo");
    borrow(&foo);

    println!("Here is my value ? {foo}");
}
```
</div>

<div v-click>
    <code-output type="stdout" :text="'Borrowed value : foo\nHere is my value ? foo'" />
</div>

<section-label name="The language" />
<slide-number/>

---
title: "Stack vs heap"
layout: two-cols-header
layoutClass: gap-8
---

<SlideHeading text="Where does data ends up ?" />

::left::

<v-click><span class="subtitle">Stack</span></v-click>

<div class="flex flex-col gap-2 mt-2">
  <div v-click="'3'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>Primitive(s)</div>
  </div>
  <div v-click="'4'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>Array(s)</div>
  </div>
  <div v-click="'5'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>Struct(s) (with known-size fields)</div>
  </div>
  <div v-click="'6'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>String header (pointer, size, capacity)</div>
  </div>
</div>

::right::

<v-click at="2"><span class="subtitle">Heap</span></v-click>

<div class="flex flex-col gap-2 mt-2">
  <div v-click="'7'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>String data</div>
  </div>
  <div v-click="'8'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>Vec&lt;T&gt; data</div>
  </div>
  <div v-click="'9'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>Smart pointers</div>
  </div>
</div>

<section-label name="The language" />
<slide-number/>

<!--
[click] In Rust, like Java, we use the heap and the stack to store data. The difference comes from what goes where and the control you have over it.

The rule for Rust is fairly simple: if the size is known at compile time, the value can live on the stack; anything whose size isn't known is allocated on the heap.

[click] That means primitives, arrays, and structs (with known-size fields) live on the stack, 

[click] while types like String, Vec, and smart pointers manage data on the heap.

But there's a subtle difference compared to Java. For a String in Java, you have a single reference on the stack frame pointing to the data on the heap.

In Rust, you hold everything fixed-size on the stack — so the String's capacity, length, and pointer to its data all live in the stack frame, while only the actual bytes live on the heap.

In Java, most data lives on the heap; what lives on the stack is primitives and references to that heap data.
-->

---
title: "Memory without a GC — values are dropped"
layout: topic
---

<div class="text-center opacity-80 mb-6">
The compiler inserts <code>drop</code> where each owned value leaves scope — no GC needed.
</div>

<div class="max-w-2xl mx-auto">

````md magic-move {lines: true}
```rust
fn take_ownership(value: String) {
    println!("Owned value : {value}");
} // value goes out of scope, it gets dropped

fn main() {
    let foo = String::from("foo");
    take_ownership(foo); // foo is moved to value
}
```

```rust
fn borrow(value: &str) {
    println!("Borrowed value : {value}");
}

fn main() {
    let foo = String::from("foo");
    borrow(&foo);

    println!("Here is my value ? {foo}");
} // foo goes out of scope, it gets dropped
```
````

</div>

<section-label name="The language" />
<slide-number/>

<!--
So how does this work without a garbage collector ? Well you remember how we talked about ownership earlier ?

The compiler knows at compile time where each owned value goes out of scope, and inserts the drop call there. So the cleanup is baked into the compiled program, no runtime garbage collector needed.
-->

---
title: "The project"
layout: center
class: text-center
---

<SectionDivider index="02" name="The project" />

<slide-number/>

---
title: "A basic Spring Boot project"
layout: topic
---

<div class="text-center opacity-70 mb-6">What most of us would reach for by default</div>

<div class="flex flex-col items-center gap-3 text-lg">
  <div class="px-5 py-3 rounded-lg border border-primary/20 bg-primary/10 w-90 text-center">
    <b>Spring Boot</b> — MVC / REST controllers
  </div>
  <div class="opacity-40 text-2xl">↓</div>
  <div class="px-5 py-3 rounded-lg border border-primary/20 bg-primary/10 w-90 text-center">
    <b>Hibernate / JPA</b>
  </div>
  <div class="opacity-40 text-2xl">↓</div>
  <div class="px-5 py-3 rounded-lg border border-primary/20 bg-primary/10 w-90 text-center">
    <b>PostgreSQL</b>
  </div>
</div>

<section-label name="The project" />
<slide-number/>

<!--
Alright let's go with a simple Spring Boot MVC, Hibernate/JPA, PostgreSQL stack. Nothing fancy, pretty much what anyone would use in production. That's the point — this is the architecture most developers would reach for by default. Let's look at it.
-->

---
title: "Rewrite it in rust"
layout: center
class: text-center
---

<SectionDivider index="03" name="Rewrite it in rust ! 🦀" />

<slide-number/>

---
title: "Choose a framework"
layout: topic
---

<div class="text-center max-w-3xl mx-auto mb-8 opacity-85">
There's <b>no all-in-one Spring Boot</b> in Rust — you pick and <b>assemble the crates yourself.</b>
</div>

<div class="grid grid-cols-2 gap-10 max-w-3xl mx-auto">
  <div>
    <div class="subtitle mb-3 text-center">Web framework</div>
    <div class="flex flex-col gap-2">
      <div v-click="1" :class="$clicks >= 4 ? 'pick pick--on' : 'pick'">Axum</div>
      <div v-click="2" class="pick">Actix</div>
      <div v-click="3" class="pick">Rocket</div>
    </div>
    <div v-click="4" class="text-sm opacity-70 mt-3 text-center">most used · great docs &amp; examples</div>
  </div>
  <div>
    <div class="subtitle mb-3 text-center">ORM</div>
    <div class="flex flex-col gap-2">
      <div v-click="5" :class="$clicks >= 7 ? 'pick pick--on' : 'pick'">Diesel</div>
      <div v-click="6" class="pick">SeaORM</div>
    </div>
    <div v-click="7" class="text-sm opacity-70 mt-3 text-center">compile-time checks · async via <code>diesel-async</code></div>
  </div>
</div>

<div v-click="8" class="text-center text-sm opacity-60 mt-8">
Bonus: in Rust, docs live in your code and get published for everyone on <b>docs.rs</b>
</div>

<section-label name="The language" />
<slide-number/>

<!--
To get as close as possible to the Spring Boot project while staying idiomatic to Rust, we need to assemble a few things ourselves — because there is no full-fledged, batteries-included Spring Boot equivalent in Rust. That's the key idea of this slide.

[click] [click] [click] First, a web framework. There are several — Axum, Actix, Rocket.

[click] We'll go with Axum: it's the most commonly used, with great examples and documentation.

[click] [click] Then, an ORM. The two most popular are Diesel and SeaORM.

[click] We'll pick Diesel because it's designed to check a lot at compile time, and it can be async with the diesel-async crate.

[click] A nice aside: in Rust you write your documentation directly in your code, and it gets published for everyone on docs.rs.
-->

---
title: "Create a project"
layout: topic
---

<div class="text-center opacity-80 mb-8">
One tool for building <b>and</b> dependencies — Rust's answer to <b>Maven</b>.
</div>

<div v-click class="flex flex-col items-center gap-2 mb-8">
  <img src="/assets/cargo-logo.png" class="h-24" />
  <div class="subtitle text-xl">Cargo</div>
</div>

<div v-click class="max-w-2xl mx-auto">

```bash
cargo new tickets       # scaffold a new project
cargo add axum diesel   # add dependencies
cargo run               # build & run
```

</div>

<section-label name="The language" />
<slide-number/>

<!--
Like Maven for Java, Cargo is Rust's build tool and dependency manager all in one — cargo new to scaffold, cargo add for dependencies, cargo run, cargo test. One tool, no XML. Let's scaffold the project.
-->

---
title: "Let's migrate"
layout: center
class: text-center
---

<SectionDivider index="03" name="Let's migrate 🚀" />

<slide-number/>

---
title: "Reality check"
layout: center
class: text-center
---

<SectionDivider index="04" name="Reality check" />

<slide-number/>

---
title: "If it compiles, it almost certainly works"
layout: topic
---

<div class="text-center text-lg opacity-80 mb-6">
Before the program ever runs, the compiler has already guaranteed:
</div>

<div class="grid grid-cols-2 gap-3 max-w-3xl mx-auto">
  <div v-click class="px-4 py-3 rounded-lg border border-primary/20 bg-primary/10"><b class="grad-check">✓</b> No <code>null</code> — every absence is an <code>Option</code></div>
  <div v-click class="px-4 py-3 rounded-lg border border-primary/20 bg-primary/10"><b class="grad-check">✓</b> No unhandled error — every failure is a <code>Result</code></div>
  <div v-click class="px-4 py-3 rounded-lg border border-primary/20 bg-primary/10"><b class="grad-check">✓</b> No use-after-free — ownership &amp; borrowing</div>
  <div v-click class="px-4 py-3 rounded-lg border border-primary/20 bg-primary/10"><b class="grad-check">✓</b> No accidental mutation — immutable by default</div>
</div>

<section-label name="Reality check" />
<slide-number/>

<!--
With everything we've seen so far, we can be confident that if our Rust program compiles, there's a very low probability it crashes at runtime — the compiler has already run extensive checks that give us that safety. No null, no unhandled error, no use-after-free, no accidental mutation.

[click on the last item] ... and that's why, on the last click, the title upgrades: if it compiles, it ALMOST CERTAINLY works. The one thing the compiler can't catch is a logic bug: if your business rules are wrong, they'll still be wrong. But a whole class of runtime crashes is simply gone.
-->

---
title: "Fearless concurrency"
layout: topic
---

<div class="max-w-3xl mx-auto text-center text-xl leading-relaxed">
The same tools — ownership, borrowing, the type system —
also make <b>concurrent and parallel code safe at compile time.</b>
</div>

<div v-click class="mt-10 text-center text-2xl font-bold grad">
Data races simply don't compile.
</div>

<section-label name="Reality check" />
<slide-number/>

<!--
Another thing that's amazing with Rust — and that we partially saw in this project — is that concurrent and parallel programming are safe. The same tools (ownership, borrowing and the type system, through the Send and Sync traits) guarantee you can't have a data race: it simply won't compile. The Rust team named this "Fearless concurrency".

To be precise: it eliminates data races, not deadlocks or logic mistakes — but that whole category of threading bugs that "work on my machine and corrupt data in prod" is gone. I won't go deeper here, just know it exists.
-->

---
title: "Is Rust really hard?"
layout: topic
---

<div class="max-w-3xl mx-auto flex flex-col gap-3">
  <div v-click class="px-5 py-3 rounded-lg border border-primary/20 bg-primary/10">
    <b>Steep learning curve</b> — more concepts to learn up front
  </div>
  <div v-click class="px-5 py-3 rounded-lg border border-primary/20 bg-primary/10">
    <b>You'll fight the compiler</b> — especially at the beginning
  </div>
  <div v-click class="px-5 py-3 rounded-lg border border-primary/20 bg-primary/10">
    <b>It forces a different way of thinking</b>
  </div>
</div>

<div v-click class="mt-10 text-center text-xl font-semibold max-w-2xl mx-auto">
  <span class="grad">But the compiler isn't fighting you — it's reviewing your code before it ships.</span>
</div>

<section-label name="Reality check" />
<slide-number/>

<!--
After all that — is Rust really hard? Honestly: it has a steep learning curve, and you'll most likely be fighting the compiler at the beginning. It forces you to think a bit differently than most languages let you — but I'd argue that's exactly the way we should think when we develop. The compiler isn't fighting you, it's reviewing your code before it ships.
-->

---
title: "Performance"
layout: center
class: text-center
---

<SectionDivider index="05" name="Performance" />

<slide-number/>

---
title: "Why my own benchmark?"
layout: topic
---

<p v-click class="opacity-80 text-center max-w-3xl mx-auto">Plenty of Rust-vs-Java benchmarks online — few are fair or reproducible. So I built my own.</p>

<div v-click class="mt-4 max-w-3xl mx-auto">
  <div class="subtitle mb-3 text-center">5 targets, identical conditions</div>
  <div class="flex flex-wrap gap-2 justify-center">
    <span class="px-3 py-1 rounded border border-primary/20 bg-primary/10">Rust — Axum + Diesel</span>
    <span class="px-3 py-1 rounded border border-primary/20 bg-primary/10">Spring MVC (JVM)</span>
    <span class="px-3 py-1 rounded border border-primary/20 bg-primary/10">Spring MVC (native)</span>
    <span class="px-3 py-1 rounded border border-primary/20 bg-primary/10">Spring WebFlux (JVM)</span>
    <span class="px-3 py-1 rounded border border-primary/20 bg-primary/10">Spring WebFlux (native)</span>
  </div>
</div>

<div v-click class="mt-10 max-w-3xl mx-auto text-sm opacity-70 text-center">
Same machine · same Postgres · capped at <b>4 CPU / 2 GB</b> each · clean DB per run · warm-up excluded · one fixed user journey · 50 / 100 / 300 VUs · <b>0% failures</b>
</div>

<section-label name="Performance" />
<slide-number/>

<!--
There are a lot of Rust-vs-Java benchmarks online, with very different takes and results — and none convinced me. Often they benchmark a hello world, and most of the time you don't know the methodology, the environment, etc. So I built my own, as clear, clean and fair as I could.

[click] I compared Rust against Spring Boot MVC and Spring Boot WebFlux, each also built as a GraalVM native image — five targets in total. MVC is blocking, and since Axum is async / non-blocking, comparing only against MVC wouldn't be fair; that's why the reactive WebFlux is in the mix.

[click] Methodology in short: everything runs on my laptop (i7-11800H, 16 GB RAM, NVMe SSD, Windows 11 + WSL2), every app in a container capped at the same 4 CPU / 2 GB, all hitting the same Postgres. The DB is reset to the same 10k rows before each run, warm-up is excluded (it matters for the JVM's JIT), the same fixed user journey runs for everyone, at 50 / 100 / 300 virtual users. Failure rate was 0% everywhere. And keep in mind: all apps share an equal 10-connection pool, so this measures per-request efficiency under fixed concurrency — not raw scalability.
-->

---
title: "Throughput"
layout: center
---

<img src="/assets/throughput.svg" class="w-full max-w-4xl mx-auto rounded-lg shadow-xl" />

<section-label name="Performance" />
<slide-number/>

<!--
Throughput — requests per second, higher is better. Rust leads at every load and stays flat around 2265 req/s; it never gets close to the CPU cap, so it's pool/DB-bound with lots of headroom.

The JVM is very competitive once warm, around 1900–2050. The surprise: native roughly halves WebFlux throughput — GraalVM Community is ahead-of-time compiled with only the Serial GC, and the allocation-heavy reactive stack is its worst case; blocking MVC is barely affected.
-->

---
title: "Latency"
layout: center
---

<div class="grid grid-cols-2 gap-4 items-center w-full max-w-6xl mx-auto">
  <img src="/assets/latency-avg.svg" class="w-full rounded-lg shadow-xl" />
  <img src="/assets/latency-p95.svg" class="w-full rounded-lg shadow-xl" />
</div>

<section-label name="Performance" />
<slide-number/>

<!--
Latency — average on the left, p95 on the right, lower is better. Because the connection pool is fixed, extra load mostly turns into latency (queueing) rather than more throughput — that's the p95 climbing. Rust keeps the lowest p95 at every level.
-->

---
title: "Resource use"
layout: center
---

<div class="grid grid-cols-2 gap-4 items-center w-full max-w-6xl mx-auto">
  <img src="/assets/memory.svg" class="w-full rounded-lg shadow-xl" />
  <img src="/assets/cpu.svg" class="w-full rounded-lg shadow-xl" />
</div>

<section-label name="Performance" />
<slide-number/>

<!--
Resource use — peak memory and peak CPU, leaner is better. Rust sips: about 1.4 cores and 24 MB. The JVM is the heaviest on memory (410–530 MB); the native images bring that down to ~300 MB.
-->

---
title: "Startup"
layout: center
---

<img src="/assets/startup.svg" class="w-full max-w-4xl mx-auto rounded-lg shadow-xl" />

<section-label name="Performance" />
<slide-number/>

<!--
Startup — note the log scale, from half a millisecond to 5.2 seconds. Native images are ready in under a second, versus 3–5 seconds on the JVM, and Rust starts almost instantly.

Fun detail: webflux-native starts ~2.5× faster than mvc-native, because JPA/Hibernate bootstrap is much heavier than R2DBC — so the lean reactive stack that loses on native throughput wins on native startup.
-->

---
title: "Build time"
layout: center
---

<img src="/assets/build.svg" class="w-full max-w-4xl mx-auto rounded-lg shadow-xl" />

<section-label name="Performance" />
<slide-number/>

<!--
Build time — the honest trade-off. A full Rust build takes ~27 s, about 9× the ~3 s of either Spring Boot app.

But that's the one-off cost: once dependencies are compiled, rebuilding only the ticket crate takes ~3 s, the same as the JVM builds.
-->

---
title: "Don't just rewrite it in Rust"
layout: center
class: text-center
---

<SectionDivider index="06" name="Don't just &quot;Rewrite it in Rust !&quot;" />

<slide-number/>

---
title: "Is Rust worth it?"
layout: topic
---

<div class="grid grid-cols-2 gap-8 max-w-4xl mx-auto">
  <div>
    <div class="font-bold mb-3 text-center" style="color:#16a34a">Pros</div>
    <div class="flex flex-col gap-2">
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">✅ Real, measurable performance gains</div>
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">✅ Safety &amp; robustness by design</div>
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">✅ Zero-cost abstractions</div>
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">✅ Moves fast · the Rust Book is excellent</div>
    </div>
  </div>
  <div>
    <div class="font-bold mb-3 text-center" style="color:#dc2626">Cons</div>
    <div class="flex flex-col gap-2">
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">❌ Ecosystem younger / less mature than Spring or Quarkus</div>
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">❌ Steep learning curve — takes time to get used to</div>
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">❌ You assemble the stack yourself</div>
    </div>
  </div>
</div>

<section-label name="Verdict" />
<slide-number/>

<!--
So — is Rust worth it? It depends on what you need.

Pros: unlike Java it moves much faster (a release every six weeks); clear, measurable performance gains — which matter more and more with green IT, FinOps and everything moving to the cloud; the Rust Book is genuinely excellent; and you get safety, robustness and zero-cost abstractions.

Cons: the ecosystem isn't as developed or mature as Java's — Spring Boot or Quarkus — and that weighs a lot in the balance; the learning curve is steep and takes time; and you assemble the stack yourself instead of getting a batteries-included framework.

So don't rewrite everything in Rust for the hype — reach for it where performance, safety and cost actually matter.
-->

---
title: "Any questions?"
layout: center
class: text-center
---

<div class="text-5xl font-bold grad">Any questions ?</div>

<div class="flex justify-center gap-16 mt-12">
  <div class="flex flex-col items-center gap-3">
    <img src="/assets/rust-for-java-developers-rust-book-qrcode.svg" class="w-40 h-40 rounded-xl bg-white p-2 shadow-lg" />
    <div class="font-semibold text-[#3f3f46]">The Rust Book</div>
    <div class="text-xs opacity-55">doc.rust-lang.org/book</div>
  </div>
  <div class="flex flex-col items-center gap-3">
    <img src="/assets/rust-for-java-developers-github-qrcode.svg" class="w-40 h-40 rounded-xl bg-white p-2 shadow-lg" />
    <div class="font-semibold text-[#3f3f46]">This talk on GitHub</div>
    <div class="text-xs opacity-55">Slides &amp; code</div>
  </div>
</div>

<div class="mt-12 text-lg opacity-70">Thanks for listening! 🦀</div>

<slide-number/>

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
title: "Qui suis-je ?"
layout: center
class: text-center
---

# Qui suis-je ?

<div class="whoami">
  <div class="whoami-name">Philippe Billard</div>
  <div class="whoami-bar"></div>
  <div class="whoami-role">Tech Lead · plus de 7 ans d'expérience</div>
  <div class="whoami-tags">
    <span>Développeur</span>
    <span>Cloud</span>
    <span>Data / IA</span>
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
title: "Mon problème"
layout: topic
---

<div class="text-center opacity-70 mb-6">J'en ai assez de ce genre d'erreurs</div>

<div class="flex flex-col items-center gap-2 max-w-2xl mx-auto w-full">
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">Null Pointer Exception</div>
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">400 Bad Request</div>
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">Tentative d'enregistrer une mauvaise valeur en base</div>
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">Singleton avec un champ privé modifié par tout le monde</div>
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">Timeouts</div>
  <div class="px-5 py-2.5 rounded-lg border border-primary/20 bg-primary/10 w-full text-center">Erreurs 500 Internal Server Error vers des API externes</div>
</div>

<section-label name="Intro" />
<slide-number/>

<!-- 
Je travaille avec Java depuis plus de 7 ans maintenant. J'ai vu (et écrit) beaucoup de mauvais code et de mauvaises pratiques. Par exemple, sur mon dernier projet, 90 % des erreurs que nous avions en production (et il y en avait beaucoup) faisaient partie de celles-ci :
-->

---
title: "Pourquoi Rust ?"
layout: center
class: text-center
---

<SectionDivider index="01" name="Pourquoi Rust ?" />

<slide-number/>

---
title: "Graydon Hoare"
layout: quote
---

<div class="max-w-3xl mx-auto text-center">

<p class="text-3xl leading-snug italic">
« C'est ridicule que nous, les informaticiens, ne soyons même pas capables de faire un ascenseur qui fonctionne sans planter ! »
</p>

<p class="mt-8 text-xl opacity-60 not-italic">
— Graydon Hoare, créateur de Rust
</p>

</div>

<section-label name="Pourquoi Rust" />
<slide-number/>

<!--
Graydon Hoare soupçonnait que l'ascenseur de son immeuble tombait sans cesse en panne à cause
d'erreurs mémoire — probablement dans son logiciel écrit en C ou C++. Cette frustration a été
l'étincelle de Rust : un langage concurrent, sûr au niveau mémoire et sans garbage collector,
qu'il a entrepris de construire pendant son passage chez Mozilla.
-->

---
title: "Les bases"
layout: center
class: text-center
---

<SectionDivider index="03" name="Les bases 🏗️" />

<slide-number/>

---
title: "Système de types, traits & génériques"
layout: topic
---

<div class="grid grid-cols-2 gap-4 auto-rows-fr max-w-4xl mx-auto w-full">
  <div v-click="1" class="rounded-lg p-4" style="border:1px solid rgba(201,78,229,0.25)">
    <div class="subtitle mb-2">Types natifs</div>
    <div class="text-[#52525b]">Boolean · Numerics (int &amp; float) · Char, str, String · Tuple · Array</div>
  </div>
  <div v-click="3" class="rounded-lg p-4" style="border:1px solid rgba(201,78,229,0.25)">
    <div class="subtitle mb-2">Trait(s)</div>
    <div class="text-[#52525b]">Comportement partagé &amp; composition — pas d'héritage</div>
  </div>
  <div v-click="2" class="rounded-lg p-4" style="border:1px solid rgba(201,78,229,0.25)">
    <div class="subtitle mb-2">Types définis par l'utilisateur</div>
    <div class="text-[#52525b]">Struct · Enum</div>
  </div>
  <div v-click="4" class="rounded-lg p-4" style="border:1px solid rgba(201,78,229,0.25)">
    <div class="subtitle mb-2">Génériques</div>
    <div class="text-[#52525b]">Comme en Java — mais les approches diffèrent</div>
  </div>
</div>

<section-label name="Le langage" />
<slide-number/>

<!--
Commençons par le commencement : Rust a un système de types vraiment bon et solide.
Comme beaucoup d'autres langages, il possède de nombreux types, mais il ne les gère pas de la même façon.

[click] On a les booléens, les types numériques (pas vraiment les mêmes qu'en Java), les types chaînes de caractères (là encore, pas vraiment les mêmes), les tuples, les tableaux.

[click] Mais contrairement à Java, il n'y a pas de classe. L'équivalent serait une Struct, un type défini par l'utilisateur, comme les enums.

[click] Pas de classe, pas d'héritage. En Rust, on a uniquement des Trait(s) qui définissent un comportement partagé et permettent la composition.

[click] Comme en Java, il y a des génériques, mais les deux approches sont très différentes. Je ne vais pas rentrer dans le détail maintenant, retenez simplement que ça existe.
-->

---
title: "Pas de null, pas d'exceptions"
layout: two-cols-header
layoutClass: gap-8
---

<SlideHeading text="Pas de null, pas d'exceptions — pas de 1 billion dollars mistake" />

::left::

<div v-click class="h-full flex flex-col justify-center">

**Result&lt;T, E&gt;** — l'équivalent des exceptions

```rust
pub enum Result<T, E> {
    /// Contient la valeur en cas de succès
    Ok(T),
    /// Contient la valeur d'erreur
    Err(E),
}
```

</div>

::right::

<div v-click class="h-full flex flex-col justify-center">

**Option&lt;T&gt;** — un Optional solide par conception

```rust
pub enum Option<T> {
    /// Aucune valeur.
    None,
    /// Une valeur de type `T`.
    Some(T),
}
```

</div>

<section-label name="Le langage" />
<slide-number/>

<!--
C'est l'une de mes fonctionnalités préférées : en Rust, il n'y a ni exception ni null.

Vous vous demandez sûrement comment ? C'est simple : les erreurs et les valeurs optionnelles sont intégrées au système de types. Ce sont des enums que vous devez obligatoirement gérer.

[click] Result<T, E> (l'équivalent des exceptions)

[click] Option<T> (un Optional solide par conception)

Comme ce sont des enums, quand vous récupérez un Result ou une Option, vous devez vérifier quelle variante vous avez obtenue et gérer tous les cas possibles. Ce sont des règles imposées par le compilateur, contrairement à Java où un null ou une exception est un problème à l'exécution que le compilateur ne vous oblige pas à vérifier.
-->

---
title: "Mutabilité"
layout: topic
---

<div class="max-w-2xl mx-auto">

````md magic-move {lines: true}
```rust
// ❌ foo est immuable par défaut
fn main() {
    let foo = String::from("foo");

    foo.insert(3, 's');

    println!("Voici ma valeur ? {foo}");
}
```

```rust
// ✅ corrigé — on déclare `foo` comme mutable
fn main() {
    let mut foo = String::from("foo");

    foo.insert(3, 's');

    println!("Voici ma valeur : {foo}");
}
```
````

<div class="mt-3 mut-output">
  <code-output type="error" :class="{ invisible: $clicks !== 0 }" :text="'error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable'" />
  <code-output type="stdout" :class="{ invisible: $clicks < 1 }" :text="'Voici ma valeur : foos'" />
</div>

</div>

<section-label name="Le langage" />
<slide-number/>

<!-- 
En Java, il faut utiliser le mot-clé final pour rendre une variable immuable. En Rust, elles sont immuables par défaut. Si vous voulez renoncer à l'immuabilité, vous devez utiliser le mot-clé `mut`.
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
    println!("Valeur possédée : {value}");
}

fn main() {
    let foo = String::from("foo");
    take_ownership(foo);

    println!("Voici ma valeur ? {foo}");
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
    println!("Valeur empruntée : {value}");
}

fn main() {
    let foo = String::from("foo");
    borrow(&foo);

    println!("Voici ma valeur ? {foo}");
}
```
</div>

<div v-click>
    <code-output type="stdout" :text="'Valeur empruntée : foo\nVoici ma valeur ? foo'" />
</div>

<section-label name="Le langage" />
<slide-number/>

---
title: "Stack vs heap"
layout: two-cols-header
layoutClass: gap-8
---

<SlideHeading text="Où finissent les données ?" />

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
    <div>Struct(s) (avec des champs de taille connue)</div>
  </div>
  <div v-click="'6'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>String headers (pointeur, taille, capacité)</div>
  </div>
</div>

::right::

<v-click at="2"><span class="subtitle">Heap</span></v-click>

<div class="flex flex-col gap-2 mt-2">
  <div v-click="'7'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>Données d'une String</div>
  </div>
  <div v-click="'8'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>Données d'un Vec&lt;T&gt;</div>
  </div>
  <div v-click="'9'" class="p-3 rounded border border-primary/20 bg-primary/10">
    <div>Smart pointers</div>
  </div>
</div>

<section-label name="Le langage" />
<slide-number/>

<!--
[click] En Rust, comme en Java, on utilise la heap et la stack pour stocker les données. La différence vient de ce qui va où, et du contrôle que vous avez dessus.

La règle en Rust est assez simple : si la taille est connue à la compilation, la valeur peut vivre sur la stack ; tout ce dont la taille n'est pas connue est alloué sur la heap.

[click] Cela signifie que les primitives, les tableaux et les structs (avec des champs de taille connue) vivent sur la stack,

[click] tandis que des types comme String, Vec et les smart pointers gèrent leurs données sur la heap.

Mais il y a une différence subtile par rapport à Java. Pour une String en Java, vous avez une seule référence dans la stack frame qui pointe vers les données sur la heap.

En Rust, tout ce qui a une taille fixe est sur la stack — donc la capacité, la longueur et le pointeur vers les données de la String vivent tous dans la stack frame, et seuls les octets eux-mêmes vivent sur la heap.

En Java, la plupart des données vivent sur la heap ; ce qui vit sur la stack, ce sont les primitives et les références vers ces données de la heap.
-->

---
title: "La mémoire sans GC — les valeurs sont droppées"
layout: topic
---

<div class="text-center opacity-80 mb-6">
Le compilateur insère un <code>drop</code> là où chaque valeur possédée sort de son scope — pas besoin de GC.
</div>

<div class="max-w-2xl mx-auto">

````md magic-move {lines: true}
```rust
fn take_ownership(value: String) {
    println!("Valeur possédée : {value}");
} // value sort du scope, elle est droppée

fn main() {
    let foo = String::from("foo");
    take_ownership(foo); // foo est déplacée dans value
}
```

```rust
fn borrow(value: &str) {
    println!("Valeur empruntée : {value}");
}

fn main() {
    let foo = String::from("foo");
    borrow(&foo);

    println!("Voici ma valeur ? {foo}");
} // foo sort du scope, elle est droppée
```
````

</div>

<section-label name="Le langage" />
<slide-number/>

<!--
Alors comment ça marche sans garbage collector ? Vous vous souvenez de ce qu'on a dit sur l'ownership tout à l'heure ?

Le compilateur sait, dès la compilation, à quel endroit chaque valeur possédée sort de son scope, et il y insère l'appel à drop. Le nettoyage est donc intégré au programme compilé, pas besoin de garbage collector à l'exécution.
-->

---
title: "Le projet"
layout: center
class: text-center
---

<SectionDivider index="02" name="Le projet" />

<slide-number/>

---
title: "Un projet Spring Boot classique"
layout: topic
---

<div class="text-center opacity-70 mb-6">Ce que la plupart d'entre nous choisiraient par défaut</div>

<div class="flex flex-col items-center gap-3 text-lg">
  <div class="px-5 py-3 rounded-lg border border-primary/20 bg-primary/10 w-90 text-center">
    <b>Spring Boot</b> — MVC / contrôleurs REST
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

<section-label name="Le projet" />
<slide-number/>

<!--
Partons sur une stack simple : Spring Boot MVC, Hibernate/JPA, PostgreSQL. Rien d'extravagant, à peu près ce que tout le monde utiliserait en production. C'est justement le but — c'est l'architecture que la plupart des développeurs choisiraient par défaut. Jetons-y un œil.
-->

---
title: "Rewrite it in rust"
layout: center
class: text-center
---

<SectionDivider index="03" name="Rewrite it in rust ! 🦀" />

<slide-number/>

---
title: "Choisir un framework"
layout: topic
---

<div class="text-center max-w-3xl mx-auto mb-8 opacity-85">
Il n'y a <b>pas de Spring Boot tout-en-un</b> en Rust — vous choisissez et <b>assemblez les crates vous-même.</b>
</div>

<div class="grid grid-cols-2 gap-10 max-w-3xl mx-auto">
  <div>
    <div class="subtitle mb-3 text-center">Framework web</div>
    <div class="flex flex-col gap-2">
      <div v-click="1" :class="$clicks >= 4 ? 'pick pick--on' : 'pick'">Axum</div>
      <div v-click="2" class="pick">Actix</div>
      <div v-click="3" class="pick">Rocket</div>
    </div>
    <div v-click="4" class="text-sm opacity-70 mt-3 text-center">le plus utilisé · excellente doc &amp; exemples</div>
  </div>
  <div>
    <div class="subtitle mb-3 text-center">ORM</div>
    <div class="flex flex-col gap-2">
      <div v-click="5" :class="$clicks >= 7 ? 'pick pick--on' : 'pick'">Diesel</div>
      <div v-click="6" class="pick">SeaORM</div>
    </div>
    <div v-click="7" class="text-sm opacity-70 mt-3 text-center">vérifications à la compilation · async via <code>diesel-async</code></div>
  </div>
</div>

<div v-click="8" class="text-center text-sm opacity-60 mt-8">
Bonus : en Rust, la doc vit dans votre code et est publiée pour tout le monde sur <b>docs.rs</b>
</div>

<section-label name="Le langage" />
<slide-number/>

<!--
Pour se rapprocher le plus possible du projet Spring Boot tout en restant idiomatique en Rust, il faut assembler quelques briques nous-mêmes — parce qu'il n'existe pas d'équivalent complet de Spring Boot, « batteries incluses », en Rust. C'est l'idée clé de cette slide.

[click] [click] [click] D'abord, un framework web. Il y en a plusieurs — Axum, Actix, Rocket.

[click] On va partir sur Axum : c'est le plus utilisé, avec d'excellents exemples et une très bonne documentation.

[click] [click] Ensuite, un ORM. Les deux plus populaires sont Diesel et SeaORM.

[click] On choisit Diesel parce qu'il est conçu pour vérifier un maximum de choses à la compilation, et qu'il peut être async grâce à la crate diesel-async.

[click] Petite parenthèse sympa : en Rust, on écrit la documentation directement dans le code, et elle est publiée pour tout le monde sur docs.rs.
-->

---
title: "Créer un projet"
layout: topic
---

<div class="text-center opacity-80 mb-8">
Un seul outil pour le build <b>et</b> les dépendances — la réponse de Rust à <b>Maven</b>.
</div>

<div v-click class="flex flex-col items-center gap-2 mb-8">
  <img src="/assets/cargo-logo.png" class="h-24" />
  <div class="subtitle text-xl">Cargo</div>
</div>

<div v-click class="max-w-2xl mx-auto">

```bash
cargo new tickets       # crée un nouveau projet
cargo add axum diesel   # ajoute des dépendances
cargo run               # compile & lance
```

</div>

<section-label name="Le langage" />
<slide-number/>

<!--
Comme Maven pour Java, Cargo est à la fois l'outil de build et le gestionnaire de dépendances de Rust — cargo new pour créer le projet, cargo add pour les dépendances, cargo run, cargo test. Un seul outil, pas de XML. Créons le projet.
-->

---
title: "Migrons !"
layout: center
class: text-center
---

<SectionDivider index="03" name="Migrons ! 🚀" />

<slide-number/>

---
title: "Retour à la réalité"
layout: center
class: text-center
---

<SectionDivider index="04" name="Retour à la réalité" />

<slide-number/>

---
title: "Si ça compile, ça marche presque à coup sûr"
layout: topic
---

<div class="text-center text-lg opacity-80 mb-6">
Avant même que le programme ne s'exécute, le compilateur a déjà garanti :
</div>

<div class="grid grid-cols-2 gap-3 max-w-3xl mx-auto">
  <div v-click class="px-4 py-3 rounded-lg border border-primary/20 bg-primary/10"><b class="grad-check">✓</b> Pas de <code>null</code> — toute absence est une <code>Option</code></div>
  <div v-click class="px-4 py-3 rounded-lg border border-primary/20 bg-primary/10"><b class="grad-check">✓</b> Pas d'erreur non gérée — tout échec est un <code>Result</code></div>
  <div v-click class="px-4 py-3 rounded-lg border border-primary/20 bg-primary/10"><b class="grad-check">✓</b> Pas de use-after-free — ownership &amp; borrowing</div>
  <div v-click class="px-4 py-3 rounded-lg border border-primary/20 bg-primary/10"><b class="grad-check">✓</b> Pas de mutation accidentelle — immuable par défaut</div>
</div>

<section-label name="Retour à la réalité" />
<slide-number/>

<!--
Avec tout ce qu'on a vu jusqu'ici, on peut être confiant : si notre programme Rust compile, il y a très peu de chances qu'il plante à l'exécution — le compilateur a déjà effectué de nombreuses vérifications qui nous apportent cette sécurité. Pas de null, pas d'erreur non gérée, pas de use-after-free, pas de mutation accidentelle.

[click sur le dernier élément] ... et c'est pour ça que, au dernier clic, le titre monte d'un cran : si ça compile, ça marche PRESQUE À COUP SÛR. La seule chose que le compilateur ne peut pas détecter, c'est un bug de logique : si vos règles métier sont fausses, elles le resteront. Mais toute une catégorie de plantages à l'exécution disparaît tout simplement.
-->

---
title: "Fearless concurrency"
layout: topic
---

<div class="max-w-3xl mx-auto text-center text-xl leading-relaxed">
Les mêmes outils — ownership, borrowing, système de types —
rendent aussi <b>le code concurrent et parallèle sûr dès la compilation.</b>
</div>

<div v-click class="mt-10 text-center text-2xl font-bold grad">
Les data races ne compilent tout simplement pas.
</div>

<section-label name="Retour à la réalité" />
<slide-number/>

<!--
Autre chose d'incroyable avec Rust — qu'on a en partie vue dans ce projet — c'est que la programmation concurrente et parallèle est sûre. Les mêmes outils (ownership, borrowing et le système de types, via les traits Send et Sync) garantissent que vous ne pouvez pas avoir de data race : ça ne compile tout simplement pas. L'équipe Rust a appelé ça la « Fearless concurrency ».

Pour être précis : ça élimine les data races, pas les deadlocks ni les erreurs de logique — mais toute cette catégorie de bugs de threading qui « marchent sur ma machine et corrompent les données en prod » disparaît. Je ne vais pas aller plus loin ici, retenez simplement que ça existe.
-->

---
title: "Rust est-il vraiment difficile ?"
layout: topic
---

<div class="max-w-3xl mx-auto flex flex-col gap-3">
  <div v-click class="px-5 py-3 rounded-lg border border-primary/20 bg-primary/10">
    <b>Courbe d'apprentissage raide</b> — plus de concepts à apprendre d'emblée
  </div>
  <div v-click class="px-5 py-3 rounded-lg border border-primary/20 bg-primary/10">
    <b>Vous allez vous battre avec le compilateur</b> — surtout au début
  </div>
  <div v-click class="px-5 py-3 rounded-lg border border-primary/20 bg-primary/10">
    <b>Il impose une autre façon de penser</b>
  </div>
</div>

<div v-click class="mt-10 text-center text-xl font-semibold max-w-2xl mx-auto">
  <span class="grad">Mais le compilateur ne se bat pas contre vous — il relit votre code avant qu'il parte en prod.</span>
</div>

<section-label name="Retour à la réalité" />
<slide-number/>

<!--
Après tout ça — Rust est-il vraiment difficile ? Honnêtement : il a une courbe d'apprentissage raide, et vous allez très probablement vous battre avec le compilateur au début. Il vous oblige à penser un peu différemment de ce que permettent la plupart des langages — mais je dirais que c'est exactement comme ça qu'on devrait penser quand on développe. Le compilateur ne se bat pas contre vous, il relit votre code avant qu'il parte en prod.
-->

---
title: "Performance"
layout: center
class: text-center
---

<SectionDivider index="05" name="Performance" />

<slide-number/>

---
title: "Pourquoi mon propre benchmark ?"
layout: topic
---

<p v-click class="opacity-80 text-center max-w-3xl mx-auto">Il existe plein de benchmarks Rust vs Java en ligne — peu sont équitables ou reproductibles. Alors j'ai construit le mien.</p>

<div v-click class="mt-4 max-w-3xl mx-auto">
  <div class="subtitle mb-3 text-center">5 cibles, conditions identiques</div>
  <div class="flex flex-wrap gap-2 justify-center">
    <span class="px-3 py-1 rounded border border-primary/20 bg-primary/10">Rust — Axum + Diesel</span>
    <span class="px-3 py-1 rounded border border-primary/20 bg-primary/10">Spring MVC (JVM)</span>
    <span class="px-3 py-1 rounded border border-primary/20 bg-primary/10">Spring MVC (natif)</span>
    <span class="px-3 py-1 rounded border border-primary/20 bg-primary/10">Spring WebFlux (JVM)</span>
    <span class="px-3 py-1 rounded border border-primary/20 bg-primary/10">Spring WebFlux (natif)</span>
  </div>
</div>

<div v-click class="mt-10 max-w-3xl mx-auto text-sm opacity-70 text-center">
Même machine · même Postgres · limité à <b>4 CPU / 2 Go</b> chacun · base propre à chaque run · warm-up exclu · un parcours utilisateur fixe · 50 / 100 / 300 VUs · <b>0 % d'échecs</b>
</div>

<section-label name="Performance" />
<slide-number/>

<!--
Il existe beaucoup de benchmarks Rust vs Java en ligne, avec des points de vue et des résultats très différents — et aucun ne m'a convaincu. Souvent, ils mesurent un hello world, et la plupart du temps on ne connaît ni la méthodologie, ni l'environnement, etc. Alors j'ai construit le mien, aussi clair, propre et équitable que possible.

[click] J'ai comparé Rust à Spring Boot MVC et Spring Boot WebFlux, chacun également compilé en image native GraalVM — cinq cibles au total. MVC est bloquant, et comme Axum est async / non bloquant, comparer uniquement avec MVC n'aurait pas été équitable ; c'est pour ça que WebFlux, réactif, fait partie du lot.

[click] La méthodologie en bref : tout tourne sur mon portable (i7-11800H, 16 Go de RAM, SSD NVMe, Windows 11 + WSL2), chaque application dans un conteneur limité aux mêmes 4 CPU / 2 Go, toutes interrogeant le même Postgres. La base est réinitialisée avec les mêmes 10k lignes avant chaque run, le warm-up est exclu (c'est important pour le JIT de la JVM), le même parcours utilisateur fixe est joué pour tout le monde, à 50 / 100 / 300 utilisateurs virtuels. Le taux d'échec était de 0 % partout. Et gardez en tête : toutes les applications partagent un pool identique de 10 connexions, donc ceci mesure l'efficacité par requête à concurrence fixe — pas la scalabilité brute.
-->

---
title: "Débit"
layout: center
---

<img src="/assets/throughput.svg" class="w-full max-w-4xl mx-auto rounded-lg shadow-xl" />

<section-label name="Performance" />
<slide-number/>

<!--
Le débit — requêtes par seconde, plus c'est haut, mieux c'est. Rust est en tête à chaque niveau de charge et reste stable autour de 2265 req/s ; il n'approche jamais la limite CPU, donc il est limité par le pool / la base, avec beaucoup de marge.

La JVM est très compétitive une fois chaude, autour de 1900–2050. La surprise : le natif divise environ par deux le débit de WebFlux — GraalVM Community compile ahead-of-time avec uniquement le Serial GC, et la stack réactive, très gourmande en allocations, est son pire cas ; MVC, bloquant, est à peine affecté.
-->

---
title: "Latence"
layout: center
---

<div class="grid grid-cols-2 gap-4 items-center w-full max-w-6xl mx-auto">
  <img src="/assets/latency-avg.svg" class="w-full rounded-lg shadow-xl" />
  <img src="/assets/latency-p95.svg" class="w-full rounded-lg shadow-xl" />
</div>

<section-label name="Performance" />
<slide-number/>

<!--
La latence — moyenne à gauche, p95 à droite, plus c'est bas, mieux c'est. Comme le pool de connexions est fixe, la charge supplémentaire se transforme surtout en latence (file d'attente) plutôt qu'en débit — c'est le p95 qui grimpe. Rust garde le p95 le plus bas à chaque niveau.
-->

---
title: "Consommation de ressources"
layout: center
---

<div class="grid grid-cols-2 gap-4 items-center w-full max-w-6xl mx-auto">
  <img src="/assets/memory.svg" class="w-full rounded-lg shadow-xl" />
  <img src="/assets/cpu.svg" class="w-full rounded-lg shadow-xl" />
</div>

<section-label name="Performance" />
<slide-number/>

<!--
La consommation de ressources — pic de mémoire et pic de CPU, plus c'est sobre, mieux c'est. Rust est très économe : environ 1,4 cœur et 24 Mo. La JVM est la plus gourmande en mémoire (410–530 Mo) ; les images natives la ramènent à ~300 Mo.
-->

---
title: "Démarrage"
layout: center
---

<img src="/assets/startup.svg" class="w-full max-w-4xl mx-auto rounded-lg shadow-xl" />

<section-label name="Performance" />
<slide-number/>

<!--
Le démarrage — attention, échelle logarithmique, d'une demi-milliseconde à 5,2 secondes. Les images natives sont prêtes en moins d'une seconde, contre 3–5 secondes sur la JVM, et Rust démarre quasi instantanément.

Détail amusant : webflux-native démarre ~2,5× plus vite que mvc-native, parce que le bootstrap de JPA/Hibernate est bien plus lourd que celui de R2DBC — donc la stack réactive légère, qui perd en débit en natif, gagne au démarrage en natif.
-->

---
title: "Temps de build"
layout: center
---

<img src="/assets/build.svg" class="w-full max-w-4xl mx-auto rounded-lg shadow-xl" />

<section-label name="Performance" />
<slide-number/>

<!--
Le temps de build — le compromis honnête. Un build Rust complet prend ~27 s, environ 9× les ~3 s de chacune des applications Spring Boot.

Mais c'est un coût ponctuel : une fois les dépendances compilées, recompiler uniquement la crate ticket prend ~3 s, autant que les builds JVM.
-->

---
title: "Ne faites pas juste un Rewrite it in Rust"
layout: center
class: text-center
---

<SectionDivider index="06" name="Ne faites pas juste un &quot;Rewrite it in Rust !&quot;" />

<slide-number/>

---
title: "Rust en vaut-il la peine ?"
layout: topic
---

<div class="grid grid-cols-2 gap-8 max-w-4xl mx-auto">
  <div>
    <div class="font-bold mb-3 text-center" style="color:#16a34a">Pour</div>
    <div class="flex flex-col gap-2">
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">✅ Des gains de performance réels et mesurables</div>
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">✅ Sûreté &amp; robustesse par conception</div>
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">✅ Abstractions à coût zéro</div>
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">✅ Évolue vite · le Rust Book est excellent</div>
    </div>
  </div>
  <div>
    <div class="font-bold mb-3 text-center" style="color:#dc2626">Contre</div>
    <div class="flex flex-col gap-2">
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">❌ Écosystème plus jeune / moins mature que Spring ou Quarkus</div>
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">❌ Courbe d'apprentissage raide — il faut du temps pour s'y faire</div>
      <div v-click class="px-4 py-2 rounded-lg border border-primary/20 bg-primary/10">❌ Vous assemblez la stack vous-même</div>
    </div>
  </div>
</div>

<section-label name="Verdict" />
<slide-number/>

<!--
Alors — Rust en vaut-il la peine ? Ça dépend de vos besoins.

Pour : contrairement à Java, il évolue beaucoup plus vite (une release toutes les six semaines) ; des gains de performance clairs et mesurables — qui comptent de plus en plus avec le green IT, le FinOps et le passage de tout vers le cloud ; le Rust Book est vraiment excellent ; et vous bénéficiez de la sûreté, de la robustesse et des abstractions à coût zéro.

Contre : l'écosystème n'est pas aussi développé ni aussi mature que celui de Java — Spring Boot ou Quarkus — et ça pèse lourd dans la balance ; la courbe d'apprentissage est raide et demande du temps ; et vous assemblez la stack vous-même au lieu d'avoir un framework « batteries incluses ».

Donc ne réécrivez pas tout en Rust pour la hype — utilisez-le là où la performance, la sûreté et les coûts comptent vraiment.
-->

---
title: "Des questions ?"
layout: center
class: text-center
---

<div class="text-5xl font-bold grad">Des questions ?</div>

<div class="flex justify-center gap-16 mt-12">
  <div class="flex flex-col items-center gap-3">
    <img src="/assets/rust-for-java-developers-rust-book-qrcode.svg" class="w-40 h-40 rounded-xl bg-white p-2 shadow-lg" />
    <div class="font-semibold text-[#3f3f46]">The Rust Book</div>
    <div class="text-xs opacity-55">doc.rust-lang.org/book</div>
  </div>
  <div class="flex flex-col items-center gap-3">
    <img src="/assets/rust-for-java-developers-github-qrcode.svg" class="w-40 h-40 rounded-xl bg-white p-2 shadow-lg" />
    <div class="font-semibold text-[#3f3f46]">Ce talk sur GitHub</div>
    <div class="text-xs opacity-55">Slides &amp; code</div>
  </div>
</div>

<div class="mt-12 text-lg opacity-70">Merci de votre attention ! 🦀</div>

<slide-number/>

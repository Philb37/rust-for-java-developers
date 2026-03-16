# Goals

This talk will be a live coding presentation of a migration from Springboot to Rust.
The goal of doing it this way is to present and teach some of the most important / used feature of rust by creating parallels between how this works in Springboot and the equivalent in Rust side.

It needs to be between a beginner and intermediate talk in order not to go too deep in Rust complex features.

## What I feel like is worth mentionning

| Rust          | Java Springboot   | Type of feature                   |
|---------------|-------------------|-----------------------------------|
| Cargo         | Maven             | Build tool / Dependency manager   |
| let ...       | var / String / .. | Variable declaration              |
| let ...       | X                 | Shadowing (variable)              | 
| const ...     | static            | Constant declaration              |
| mut ...       | final             | Mutability                        |
| fn foo()      | void foo()        | function declaration              |
| pub ...       | public            | variable / function accessibility |
| if x == y     | if (x == y)       | Control Flow                      |
| &             | X                 | Ownership and borrowing           |
| Compiler      | Garbage collector | Memory managment                  |
| struct        | class             | Custom data types                 |
| -             | -                 | Methods                           |
| enum          | enum              | Enum                              |
| match         | ~ switch          | Pattern matching (Control flow)   |
| vec<T>        | List<T>           | Collections                       |
| Option<T>     | ~ Optional / null | To be or not to be ?              |
| Result<T, E>  | throw / Exception | Error handling                    |
| trait         | interface         | Shared behavior                   |
| iterators     | stream            | Functionnal features              |
| closures      | lambda            | Functionnal features              |

### Legend

X : Not present
- : Not applicable
~ : Partial

## Should I mention ?

- [Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Statements and Expressions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions)
- [Macros](https://doc.rust-lang.org/book/ch20-05-macros.html?highlight=macros#macros)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

## Sources

[Rust Book](https://doc.rust-lang.org/book/title-page.html())
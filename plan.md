# 1. Personnal presentation

Present myself (now or later ?)

I started with python, switched to C, then C#, then Java and recently learned Rust on my free time.

I was 

# 2. I'll do you one better, why is Rust ?

[!NOTE] Explain what is Rust, why it came to be, what it solves.

low level language but not so far from high ones.

Borrow checker
Zero cost abstraction
Compile time checks
Type system

what are it's strenghs and weaknesses -> link to the next part

Rust is still relatively new and moves quick, it has a release cycle of 6 weeks.
Every night a new release is created -> Nightly build
Every six week a beta is creaded -> Beta build
Every six week after a beta a stable version is created from the beta -> Stable build

[!NOTE] define crates

Knowing that Rust moves quick, every crates / framework does too. A lot of crates are still in 0.X versions.

# 3. Is Rust really hard ?

Steep learning curve, fighting the compiler at the beginning

# 4. A typical Springboot project

MVC

# 5. Let's migrate :rocket:

### Create a project
- Build tool / Dependency manager

### Choose a framework

If we want to be as close as possible to the Springboot project and still staying idiomatic to Rust we need multiple things :

1. A web framework

There are multiple web framework in Rust :

- Axum
- Actix
- Rocket

We will go with Axum since it's the most commonly used, has good examples and a good documentation. (Btw Rust includes a feature where you can write your documentation direclty in your code and expose it to everyone in [docs.rs](https://docs.rs))

An important thing to note is that there is no full-fledged web framework equivalent to Springboot in Rust. You have to manually assemble crates yourself.

That's why we need ...

2. An ORM

We have multiple at our disposition in Rust, the two most popular are :

- Diesel
- SeaORM

We will choose Diesel for multiple reasons :

- Is designed to check a lot of things at compile time
- Can be async with an added crate (diesel-async)

We are going with Axum since it's the most used.



### Create config

- Build tool / Dependency manager Cargo add config serde

### Create endpoints
- Variable declaration
- Shadowing
- Constant declaration
- function declaration

Define endpoints

# 6. Conclusion

When it compiles it almost certainly work. (unless you've messed up your config but then the project won't start anyway)

I've been working with java for over 7 years now. I've seen (and done) a lot of bad code, bad practices, which can obviously also happen in Rust. But by design Rust will mitigate problems. No more 1 billion dollar mistakes (NPE for those who didn't get the joke). In my last project 90% of the errors we had in production (and there was a lot) would have been avoided with Rust :
- NPE
- 400 Bad request because you sent a null value (NPE variant)
- Trying to store a null value in DB (NPE variant)
- Singleton with a private field being updated by everyone

We could argue that :
- Timeouts
- 500 Internal Server errors

Could also be avoided with Rust since the same rules applies, the language is by design more robust and safer to use.

But that comes at a cost, and not a cheap one.

- Rust has a steep learning curve, even though you can mitigate it a lot with the [documentation](https://doc.rust-lang.org/stable/book/) is excellent and AI explanation.
- You will fight with the compiler in the beginning, it might not be enjoyable, but its for your own good because you're doing bad code.
- You're back to playing lego (which I love)
  - Either you like control and understanding / knowing how things works under the hood, then Rust is made for you. Even though I believe in due time there will be equivalents to Springboot in Rust.
  - Or you want a framework ready-to-use where you don't have to glue things together and take time understanding a bit more how it works. Then Rust is not what you should go for.
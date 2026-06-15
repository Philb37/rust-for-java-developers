# 1. Personnal presentation

Present myself (now or later ?)

I started with python, switched to C, then C#, then Java and recently learned Rust on my free time.

I was 

# 2. I'll do you one better, why is Rust ?

Explain what is rust, why it came to be, what it solves.

low level language but not so far from high ones.
what are it's strenghs and weaknesses -> link to the next part

Rust is still relatively new and moves quick, it has a release cycle of 6 weeks.
Every night a new release is created -> Nightly build
Every six week a beta is creaded -> Beta build
Every six week after a beta a stable version is created from the beta -> Stable build

[!NOTE] define crates

Knowing that rust moves quick, every crates / framework does too. A lot of crates are still in 0.X versions. 

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

There is multiple webframework in rust :

- Axum
- Actix
- Rocket

2. An ORM

We have multiple at our disposition in rust, the two most popular ones are :

- SeaORM
- Diesel

We will choose SeaORM because it's natively

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
# Workshop Repo: Microservice with Tokio and Rust

*NOTE*: This is Work-In-Progress! Please check for updates a day before the workshop.

This GitHub repo will contain all the examples and workshops files we create during our time together.

## Install Rust

[Rustup](https://rustup.rs) provides you with all the software to compile and run Rust applications, e.g.

1. Cargo - build tool and package manager
2. `rustfmt` - Auto-formatting tool for Rust code
3. `clippy` - Linting for common mistakes

[and many more](https://rust-lang.github.io/rustup-components-history/). *Rustup* also allows you to install different compile targets and multiple toolchains, as well as keeping your toolchains up to date.

After installing, you should have a set of new command line tools available. 

### Verify your Rust installation:

1. Open a Terminal/Shell/Command Line of your choice
2. Check out this repo
3. Navigate to this repository
4. Enter

```bash
$ cargo build
```
5. Open your browser at https://localhost:3000

## Recommended Editor

During the workshop, we will use [Visual Studio Code](https://code.visualstudio.com/) as editor. It's free, fast and very extensible. Making yourself familiar with VS Code is highly recommended.

However, working with VS Code is not required. If you have a preferred editor with Rust support you're more productive with, please feel free to use whatever you like. What we highyly recommend though, is checking if your editor has support for [Rust analyzer](https://rust-analyzer.github.io/).

## Recommended VS Code Extensions

To work effeciently, please install a couple of extensions that help you developing Rust. *Note*: Please don't install the recommendend Rust extension. It's outdated and the community decided to move to other tools. You can search and install VS Code extensions through the menu on the side

We recommend the following extensions:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer). This is the main extension for Rust development, with the best language support available. *Note*: This extension is also available for other IDEs and editors, check out [their website](https://rust-analyzer.github.io/)

- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates). This extension helps installing dependencies from crates.io

- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml). TOML is the format that the dependency manager Cargo uses to manage dependencies. This extension helps formatting and editing TOML files

- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb). All Rust code is compiled against LLVM. This extension helps debugging LLVM code inside VS Code

- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens). Inline errors

## Tasks

Our goal is to build a simple in memory key value store. We use Axum as basis and work with Tower services to get certain features right. In the `tests` folder you find four test files accompanying all four exercises. Remove the `ignore` macros at the beginning of each test to run it. Note that some tests require to be run with the `--nocapture` flag.

Those are the four exercises.

### Exercise 1: *Axum*

- Create an Axum Router for two routes.
- The root (`"/"`) says `<h1>Hello Axum</h1>`
- The path `"/hello"` says either `<h1>Hello Unknown Visitor</h1>` or subsitutes `Unknown Visitor` with a name provided by parameter `name`

### Exercise 2: *Key Value Store*

- Store data in a Key Value Store (provided by a `HashMap` shared state)
- Use `/kv/:key` to store data via `POST`, and to retrieve data via `GET`
- Return a 404 if there is no data saved
- *Stretch goals*:
  - Locking an `RwLock` might result in a `PoisonError`. Make sure you can convert a `PoisonError` to a `Response`. Tip: implement your custom error and implement `IntoResponse`
  - The default request size is 2MB. You want to store data up to 8MB in your key value store. Deactivate the default body limit, and set your own request body limit. Tip: Use a Service Builder and respective Layers from `axum` and `tower_http`

### Exercise 3: *Custom Services*
- Write a logging service that logs every request to stdout
- *Stretch goal*
  - Log before executing the request and after

Run tests with `--nocapture`!

### Exercise 4: *Built-in Services*
- Define nested routes for administrative tasks, be sure to add authorization so only authorized people can access. A plain auth token is ok, if you want to go fancy create your own implementation.
- Stretch goal: If you haven't worked on the upload limit, try doing it now.
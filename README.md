# View Your Twilio Account Usage Using Rust

This is a small project that I create for two reasons:

- To show how to request Twilio account usage information using Rust
- To help me learn more about Rust

It’s not all that special, and it’s definitely not an example of excellent, best practice Rust.
However, it works well enough and will help me write a tutorial to help others learn what I’ve learned.

Please keep that in mind.

## Prerequisites

To use this project, you need the following

- Some prior experience with [Rust](https://doc.rust-lang.org/book/) and [Cargo](https://doc.rust-lang.org/cargo/index.html)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- A text editor such as VIM or Sublime Text, or an IDE such as Intellij IDEA or Eclipse

## Usage

To use the project, first clone the project locally.

```bash
git clone ...
```

Now, create a new file, named .env in the top-level directory of the project.
In that file, paste the configuration below.

```ini
TWILIO_ACCOUNT_SID="<TWILIO_ACCOUNT_SID>"
TWILIO_AUTH_TOKEN="<TWILIO_AUTH_TOKEN>"
```

Then, from the Account info section of the Twilio Console, retrieve your Twilio Account SID and Auth Token, and paste them in place of the two respective placeholders.
Now, build and run the application by running the command below.

```bash
cargo run
```

You should see `Page size is 20` printed to the terminal, if the application executes successfully.

## License

The project is open-sourced software licensed under the [MIT license](https://opensource.org/licenses/MIT).

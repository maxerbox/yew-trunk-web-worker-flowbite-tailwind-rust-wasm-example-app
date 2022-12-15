# LBP Transform : yew trunk flowbite tailwind rust wasm example app

This is a hobby project used to learn Rust and Yew.

Every month, I make a budget and list my expenses. My bank offers to export my
expenses as an [OFX file](https://en.wikipedia.org/wiki/Open_Financial_Exchange)
spec v1.6. This is an SGML file listing transactions.

Unfortunately, my bank formats the file in a strange way, with dates contained
in transaction descriptions.

So what this application does is taking an ofx file, sending it to a worker and
then applying transformations to it to extract the date and format properly.

->Input

```xml
<STMTTRN><TRNTYPE>POS<DTPOSTED>20221211<TRNAMT>-23.00<FITID>PJXU11LC%F<NAME>
ACHAT CB BRASSERIES DE 10.12.2022
</STMTTRN>
```

Output->

```xml
<STMTTRN>
<TRNTYPE>POS<DTPOSTED>20221210<TRNAMT>-23.00<FITID>PJXU11LC%F<NAME>ACHAT CB BRASSERIES DE
</STMTTRN>
```

## Usage

For a more thorough explanation of Trunk and its features, please head over to
the [repository][trunk].

### Installation

If you don't already have it installed, it's time to install Rust:
<https://www.rust-lang.org/tools/install>. The rest of this guide assumes a
typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target
installed. If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show:
[Trunk]. Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host
it.

There's also the `trunk watch` command which does the same thing but without
hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`. You can
also pass the `--release` flag to `trunk serve` if you need to get every last
drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

### Update metadata

Update the `name`, `version`, `description` and `repository` fields in the
[Cargo.toml](Cargo.toml) file. The [index.html](index.html) file also contains a
`<title>` tag that needs updating.

Finally, you should update this very `README` file to be about your app.

### License

MIT License Copyright (c) Simon Sassi

# gif

This crate aims to provide an interface that parses and decompresses GIF files. It exposes a simple API with 2
functions: one for parsing bytes into a compressed GIF data stream, and another for decompressing the stream into a list
of image frames.

This repository also contains a crate to render GIFs. It provides a command-line executable that
uses [`minifb`](https://github.com/emoon/rust_minifb) to render pixels.

`gif` aims to decode any GIF from [GIPHY](https://giphy.com). Any deviation from Giphy's rendering should be
considered a bug.

## Grammar

### Legend

The following legend defines the symbols used in this grammar:

| Symbol | Definition               |
|--------|--------------------------|
| `<>`   | grammar word             |
| `::=`  | defines symbol           |
| `*`    | zero or more occurrences |
| `+`    | one or more occurrences  |
| `\|`   | alternate element        |
| `[]`   | optional element         |

### The Grammar

`<GIF Data Stream> ::= Header <Logical Screen> <Data>* Trailer`

- `<Logical Screen> ::= Logical Screen Descriptor [Global Color Table]`
- `<Data> ::= <Graphic Block> | <Special-Purpose Block>`
    - `<Graphic Block> ::= [Graphic Control Extension] <Graphic-Rendering Block>`
        - `<Graphic-Rendering Block> ::= <Table-Based Image> | Plain Text Extension`
            - `<Table-Based Image> ::= Image Descriptor [Local Color Table] Image Data`
    - `<Special-Purpose Block> ::= Application Extension | Comment Extension`

## Fuzz

`gif` uses an [AFL](https://en.wikipedia.org/wiki/American_Fuzzy_Lop_(software)) fuzzer to assert correctness. GIFs that
crash are stored in `/tests` for the purpose of running
integration tests. To fuzz, simply run the `./fuzz.sh` shell script.

## Profile

This project uses [samply](https://github.com/mstange/samply) to profile the code. Run `./profile.sh` to run a
profile. Make sure to pass in a GIF file.

For example:

```bash
./profile.sh ./sample_gifs/shrek.gif
```

## Reading

https://www.matthewflickinger.com/lab/whatsinagif/bits_and_bytes.asp<br>
https://web.archive.org/web/20050217131148/http://www.danbbs.dk/~dino/whirlgif/lzw.html<br>
https://www.w3.org/Graphics/GIF/spec-gif89a.txt<br>

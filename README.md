# Advent of Code 2024

## Prerequisites

- [asdf v0.13.1-0586b37](https://asdf-vm.com/)

## Setup

```sh
asdf install
```

## Running

```sh
# substitute `XX` with zero-padded day number
cargo run -p day_XX --bin day_XX -- ./day_XX/input.txt
```

## Testing

```sh
cargo test
```

## Adding new package

```sh
cargo new --lib day_XX
```

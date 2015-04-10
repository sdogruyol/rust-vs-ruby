# Rust vs Ruby: Building an API

This is the repository for Rust vs Ruby: Building an API blog post.

## How to run the examples

	git clone https://github.com/Sdogruyol/rust-vs-ruby

### Ruby

* Ruby Version: 2.2.1
* Ruby Server: Thin 1.6.3
* Framework: Cuba 3.4.0

```
cd ruby && rackup -s thin
```

### JRuby

* JRuby Version: 1.7.19
* Ruby Server: Puma 2.11.1
* Framework: Cuba 3.4.0

```
puma -t 8:8
```

### Rust

* Rust Version: 1.0.0-beta
* Framework: Nickel.rs 0.2.0

```
cd rust && cargo run --release
```

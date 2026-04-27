# Getting started template for [htmx-axumkama](https://github.com/Lebenoa/htmx-axumkama)!

This project serve as a template and examples on how to use `htmx-axumkama` lib.

## Stack

- [htmx v4](https://four.htmx.org) as lib for SPA-like experiences
- [Alpine.js v3](https://alpinejs.dev) as client side state building
  > (For example, accordion: send a roundtrip to server just to replace HTML with accordion state is wild)
- [UnoCSS](https://unocss.dev) for styling. I like this one better than [Tailwind CSS](https://tailwindcss.com) because UnoCSS offers more [functionality](https://unocss.dev/presets/).
- [axum](https://github.com/tokio-rs/axum) for HTTP server
- [Askama](https://github.com/askama-rs/askama) for server side HTML rendering

## License

This template is Unlicense or does not have any license whatsoever BUT different dependencies have different licenses.  
Notably [axum](https://github.com/tokio-rs/axum) is MIT license while majority of Rust crates offer BOTH Apache 2.0 and/or MIT.

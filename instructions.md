## install

https://rustwasm.github.io/book/game-of-life/setup.html

* rust >= 1.50
* install wasm-pack through rust
* cargo install wasm-pack
* cargo install cargo-watch
* node >= 14

## build

1. wasm-pack build
2. cd www
3. npm i
4. npm run start

## watch

```
cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build"
```



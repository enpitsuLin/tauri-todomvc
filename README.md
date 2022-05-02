# tauri-todomvc

![logo](https://user-images.githubusercontent.com/29378026/166204892-f9eaa461-bcb3-463d-b343-d2f09e91a8f3.png)

This is an implementation of [tastejs/todomvc](https://github.com/tastejs/todomvc) built using :

- [tauri](https://tauri.studio/)
  - [sqlite](https://sqlite.org/index.html) for store data
- [react](https://reactjs.org/)
- [jotai](https://github.com/pmndrs/jotai)
- [typescript](https://www.typescriptlang.org/)

## Preview

![Preview](https://user-images.githubusercontent.com/29378026/166206701-29d07147-e457-4648-a4ac-ba1ff5a08aeb.png)

## Development

Requirement:

- Rust
- NodeJS

install dependencies:

```sh
pnpm i
```

Then run tauri

```sh
pnpm tauri dev
```

## Distribution

to build Distribution run 

```sh
pnpm tauri build
```

more detail reference [App Publishing](https://tauri.studio/docs/distribution/publishing)


# License

MIT
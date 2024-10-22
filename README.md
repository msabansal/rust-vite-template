# `Turborepo` Vite Rust starter

A turbo repo that creates utilizes a rust wasm submodule in a vite app.

## Using this example

Run the following command:

```sh
npx create-turbo@latest -e https://github.com/msabansal/rust-wasm-template
```

## What's inside?

This Turborepo includes the following packages and apps:

### Apps and Packages

- `docs`: a vanilla [vite](https://vitejs.dev) ts app
- `web`: another vanilla [vite](https://vitejs.dev) ts app
- `@repo/ui`: a stub component & utility library shared by both `web` and `docs` applications
- `@repo/native-app`: a sample native app
- `@repo/eslint-config`: shared `eslint` configurations
- `@repo/typescript-config`: `tsconfig.json`s used throughout the monorepo

Each package and app is 100% [TypeScript](https://www.typescriptlang.org/).

### Notes

Be sure to build the repo once before running the dev command. This will build the (initial) rust wasm module and make it available to the vite app.

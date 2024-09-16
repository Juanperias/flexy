# Contributing to Flexy

Thank you for considering contributing to Flexy! Here are some guidelines to follow:

## Branching and Commits

1. Do not push your changes directly to the `master` or `unstable` branches. Create a branch named `<user-name>-unstable-<feature>`.
2. Use verbs in their normal form; do not use different tenses.
3. A branch should implement only one feature at a time.
4. Branch names and commit messages must not contain racist or discriminatory language.
5. Do not add dependencies unless they are strictly necessary for the feature.
6. If you are doing a refactor, name your branch `<username>-unstable-refactor`.

## How to Contribute

- It is recommended to have NixOS or Nix installed because the code has some dependencies like `libX11`.
- If you have NixOS, you can run `nix develop` and `cargo r`.
- After some time, your branch will be accepted and then merged into `unstable`.
- In `unstable`, necessary tests will be conducted before merging into `master`.

We appreciate your contributions and look forward to collaborating with you!

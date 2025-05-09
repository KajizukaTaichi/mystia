# Mystia
A programming language that will be compiled to WebAssembly

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/KajizukaTaichi/mystia)

## Features
- **Type inference**: You can use variable without having declare and annotation
- **ML-like syntax**: The syntax is inspired by ML-family language (e.g. define function using `let` statement)

## Memory layout
Basically, in nested object (e.g. array end dictionary), child elements are stored in before parent element

![](https://github.com/user-attachments/assets/827a907b-a9d6-4d4c-8ab6-cc2f7544b22b)

## To Do
- [ ] Enhance error message
- [X] Fix bugs of FFI to JS

## Article
https://qiita.com/KajizukaTaichi/items/a4989c60415b408fd91d

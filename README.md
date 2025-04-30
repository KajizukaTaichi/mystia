# Mystia
A programming language that will be compiled to WebAssembly

## Features
- **Type inference**: You can use variable without having declare and annotation
- **ML-like syntax**: The syntax is inspired by ML-family language (e.g. define function using `let` statement)

## Memory layout
Basically, in nested object (e.g. array end dictionary), child elements are stored in before parent element

**Nested Array**: Below object's pointer value is same to element 1
```
[ child 1 ] [ child 2 ] [ element 1: pointer refers to child 1 ] [ element 2: pointer refers to child 2 ]
```

## To Do
- [ ] Enhance error message
- [X] Fix bugs of FFI to JS

## Article
https://qiita.com/KajizukaTaichi/items/a4989c60415b408fd91d

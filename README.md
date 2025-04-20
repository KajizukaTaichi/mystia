# Mystia
A programming language that will be compiled to WebAssembly

## Features
- **Type inference**: You can use variable without having declare and annotation
- **ML-like syntax**: The syntax is inspired by ML-family language (e.g. define function using `let` statement)

## Memory layout

**Nested Array**
```
[ child 1 ] [ child 2 ] [ element 1: pointer to child 1 ] [ element 2: pointer to child 2 ]
```

## Article
https://qiita.com/KajizukaTaichi/items/a4989c60415b408fd91d

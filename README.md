# Compiler Optimize

This repository contains the examples in the article [從 LLVM IR 來看編譯器最佳化都在做些什麼](https://larry850806.medium.com/see-what-compiler-optimization-do-from-llvm-ir-dfd3774292cb)

## How to setup the environment

```
docker build -t compiler_optimize_en .
docker run -it -v ${PWD}:/workspace compiler_optimize_en
```

![](https://i.imgur.com/Gj7XJHC.png)

## Compile Rust code to LLVM IR

Compile the source code with the following script, then you can find the LLVM IR file in `target/{debug|release}/deps/*.ll`

### debug mode

```bash
cargo rustc -- --emit=llvm-ir
```

### release mode

```bash
cargo rustc --release -- --emit=llvm-ir
```

## Compile Rust code to Assembly

### debug mode

```bash
cargo objdump -- --disassemble > main.s
```

### release mode

```bash
cargo objdump --release -- --disassemble > main.s
```

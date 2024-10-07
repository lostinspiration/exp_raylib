Need to have clang in the path
```shell
set LIBCLANG_PATH=\tools\LLVM-x64-12.0.1\bin
```

Generate the bindings as needed on demand
```shell
bindgen external\include\raylib.h -o bindings.rs
```


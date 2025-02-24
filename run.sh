#!/bin/zsh

./target/debug/microc
mlir-opt a.mlir --convert-func-to-llvm | mlir-translate --mlir-to-llvmir -o a.ll
llc a.ll -o a.s
clang a.s util4mlir.so -o a
./a


# 过程记录

## 内核第一条指令：

* 如何得到一个能够在 Qemu 上成功运行的内核镜像呢？
* 首先我们需要通过链接脚本调整内核可执行文件的内存布局，使得内核被执行的第一条指令位于地址 `0x80200000` 处，同时代码段所在的地址应低于其他段。这是因为 Qemu 物理内存中低于 `0x80200000` 的区域并未分配给内核，而是主要由 RustSBI 使用。
* 其次，我们需要将内核可执行文件中的元数据丢掉得到内核镜像，此内核镜像仅包含实际会用到的代码和数据。这则是因为 Qemu 的加载功能过于简单直接，它直接将输入的文件逐字节拷贝到物理内存中，因此也可以说这一步是我们在帮助 Qemu 手动将可执行文件加载到物理内存中。
* 下一节我们将成功生成内核镜像并在 Qemu 上验证控制权被转移到内核。

## 为内核支持函数调用：

​	在添加`entry.asm`之后编译报错：

```bash
error[E0658]: use of unstable library feature 'global_asm': `global_asm!` is not stable enough for use and is subject to change
 --> src/main.rs:5:5
  |
5 | use core::arch::global_asm;
  |     ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #35119 <https://github.com/rust-lang/rust/issues/35119> for more information
  = help: add `#![feature(global_asm)]` to the crate attributes to enable
```

解决方案：更新rust nightly ver 至最新版，并重新安装了相关的工具链，在使用

```bash
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils --vers =0.3.3
rustup component add llvm-tools-preview
rustup component add rust-src
```

安装时，指定ver = 0.3.3报错，删除版本参数重新安装

成功编译完成


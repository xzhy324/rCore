# 第2章 批处理系统

## os功能目标

* 一个是操作系统自身运行在内核态，且支持应用程序在用户态运行，且能完成应用程序发出的系统调用；

* 另一个是能够一个接一个地自动运行不同的应用程序。

所以，我们需要对操作系统和应用程序进行修改，也需要对应用程序的编译生成过程进行修改。



## 实验过程概述

首先**改进应用程序，让它能够在用户态执行，并能发出系统调用**。

* 这其实就是上一章中 [构建用户态执行环境](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter1/old3-1-mini-rt-usrland.html#term-print-userminienv) 小节介绍内容的进一步改进。具体而言，编写多个应用小程序，修改编译应用所需的 `linker.ld` 文件来 [调整程序的内存布局](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/2application.html#term-app-mem-layout) ，让操作系统能够把应用加载到指定内存地址，然后顺利启动并运行应用程序。

在应用程序的运行过程中，**操作系统要支持应用程序的输出功能，并还能支持应用程序退出**。

* 这需要实现跨特权级的系统调用接口，以及 `sys_write` 和 `sys_exit` 等具体的系统调用功能。 在具体设计实现上，涉及到内联汇编的编写，以及应用与操作系统内核之间系统调用的参数传递的约定。为了让应用程序在还没实现 `邓氏鱼` 操作系统之前就能在Linux for RISC-V 64 上进行运行测试，我们采用了Linux on RISC-V64 的系统调用参数约定。具体实现可参看 [系统调用](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/2application.html#term-call-syscall) 小节中的内容。 这样写完应用小例子后，就可以通过 `qemu-riscv64` 模拟器进行测试了。

写完应用程序后，还需**实现支持多个应用程序轮流启动运行的操作系统**。

* 这里首先能把本来相对松散的应用程序执行代码和操作系统执行代码连接在一起，便于 `qemu-system-riscv64` 模拟器一次性地加载二者到内存中，并让操作系统能够找到应用程序的位置。
* **为把二者连在一起，需要对生成的应用程序进行改造**，首先是把应用程序执行文件从ELF执行文件格式变成Binary格式（通过 `rust-objcopy` 可以轻松完成）；然后这些Binary格式的文件通过编译器辅助脚本 `os/build.rs` 转变变成 `os/src/link_app.S` 这个汇编文件的一部分，并生成各个Binary应用的辅助信息，便于操作系统能够找到应用的位置。编译器会把操作系统的源码和 `os/src/link_app.S` 合在一起，编译出操作系统+Binary应用的ELF执行文件，并进一步转变成Binary格式。
* 为了定位 Binary 应用在被加载后的内存位置，**操作系统本身需要完成对 Binary 应用的位置查找**，找到后（通过 `os/src/link_app.S` 中的变量和标号信息完成），会把 Binary 应用从加载位置拷贝到 `user/src/linker.ld` 指定的物理内存位置（OS的加载应用功能）。在一个应用执行完毕后，操作系统还能加载另外一个应用，这主要是通过 `AppManagerInner` 数据结构和对应的函数 `load_app` 和 `run_next_app` 等来完成对应用的一系列管理功能。这主要在 [实现批处理操作系统](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/3batch-system.html#term-batchos) 小节中讲解。

为了让 Binary 应用能够启动和运行，**操作系统还需给 Binary 应用分配好对应执行环境所需一系列的资源**。

* 这主要包括设置好用户栈和内核栈（在用户态的应用程序与在内核态的操作系统内核需要有各自的栈，避免应用程序破坏内核的执行），实现 Trap 上下文的保存与恢复（让应用能够在发出系统调用到内核态后，还能回到用户态继续执行），完成Trap 分发与处理等工作。由于系统调用和中断处理等内核代码实现涉及用户态与内核态之间的特权级切换细节的汇编代码，与硬件细节联系紧密，所以 [这部分内容](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/4trap-handling.html#term-trap-handle) 是本章中理解比较困难的地方。如果要了解清楚，需要对涉及到的 RISC-V CSR 寄存器的功能有明确认识。这就需要查看 [RISC-V手册](http://crva.ict.ac.cn/documents/RISC-V-Reader-Chinese-v2p1.pdf) 的第十章或更加详细的 RISC-V 的特权级规范文档了。有了上面的实现后，就剩下最后一步，实现 **执行应用程序** 的操作系统功能，其主要实现在 `run_next_app` 内核函数中 。完成所有这些功能的实现，“邓式鱼” [1](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/0intro.html#dunk) 操作系统就可以正常运行，并能管理多个应用按批处理方式在用户态一个接一个地执行了。

操作系统内核代码运行在 S 模式上；应用程序运行在 U 模式上。运行在 M 模式上的软件被称为 **监督模式执行环境** (SEE, Supervisor Execution Environment)
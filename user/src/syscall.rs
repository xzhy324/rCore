use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
        "ecall",
        inlateout("x10") args[0] => ret, //{in_var} => {out_var} 分别是上下文中的输入输出变量
        in("x11") args[1],//表示将输入参数 args[1] 绑定到 ecall 的输入寄存器 x11 即 a1 中，编译器自动插入相关指令并保证在 ecall 指令被执行之前寄存器 a1 的值与 args[1] 相同。
        in("x12") args[2],
        in("x17") id
        );
    }
    ret
}

pub fn sys_write(fd:usize, buffer:&[u8]) -> isize{
    //注意 sys_write 使用一个 &[u8] 切片类型来描述缓冲区
    //这是一个 胖指针 (Fat Pointer)，里面既包含缓冲区的起始地址，还包含缓冲区的长度
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code:i32) -> isize{
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}
use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],//保存上下文时这里直接保存全部32个寄存器
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    //设置当前环境的执行栈
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    //初始化应用程序的上下文
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}

// / os/src/lang_items.rs
use core::panic::PanicInfo;
use crate::println;
use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //Some()返回一个option结构，该结构是为了解决空value常常没有被判断而提出的，必须
    //option结构的值有两种可能 （some value / none），必须显式的对这两种情况进行处理
    if let Some(location) = info.location() {
        //如果能从option结构中解析出信息,则打印出错的源文件以及相应代码行
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        )
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
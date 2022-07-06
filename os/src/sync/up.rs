use core::cell::{RefCell, RefMut};

/// Wrap a static data structure inside it so that we are
/// able to access it without any `unsafe`.
///
/// We should only use it in uniprocessor.
///
/// In order to get mutable reference of inner data, call
/// `exclusive_access`



/// RefCell【当变量需要生成借用时需要包裹的声明语句】
/// UpSafeCell 对 RefCell 进行了简单封装，通过调用结构体的exclusive_access方法，能够得到inner数据的独占访问权
pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

//通过将UpSafeCell标记为Sync，使其能够作为全局的变量
unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub unsafe fn new (value: T) -> Self {
        Self{
            inner:RefCell::new(value),
        }
    }
    ///panic if the data has been borrowed
    pub fn exclusive_access(&self) -> RefMut<'_, T>{
        self.inner.borrow_mut()
    }
}
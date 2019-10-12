use core::task::{Waker, RawWaker, RawWakerVTable, Future};
use vcell::VolatileCell;

pub struct SevWaker {
    flag: VolatileCell<u32>,
}

/// store/loads to flag are atomic
unsafe impl Sync for SevWaker { }

impl SevWaker {
    pub const VTABLE: RawWakerVTable = RawWakerVTable::new(
        |data| unsafe { Self::from_ptr(data).as_raw() },
        |data| unsafe { Self::from_ptr(data).wake_by_ref() },
        |data| unsafe { Self::from_ptr(data).wake_by_ref() },
        |data| unsafe { drop(Self::from_ptr(data)) },
    );

    pub const fn new() -> Self {
        Self {
            flag: VolatileCell::new(0)
        }
    }

    #[inline]
    pub fn pending(&self) -> bool {
        self.flag.get() == 0
    }

    #[inline]
    pub fn clear(&self) {
        self.flag.set(0)
    }

    pub fn wake_by_ref(&self) {
        self.flag.set(1);
        unsafe {
            asm!("sev" :::: "volatile");
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const () {
        self as *const Self as *const ()
    }

    #[inline]
    pub fn as_raw(&self) -> RawWaker {
        RawWaker::new(self.as_ptr(), &Self::VTABLE)
    }

    #[inline]
    pub fn as_waker(&'static self) -> Waker {
        unsafe {
            Waker::from_raw(self.as_raw())
        }
    }

    #[inline]
    pub unsafe fn from_ptr(data: *const ()) -> &'static Self {
        &*(data as *const Self)
    }
}

pub fn run_future<F: Future>(f: F) -> F::Output {
    static WAKER: SevWaker = SevWaker::new();

    let waker = WAKER.as_waker();
    let mut context = Context::from_waker(&waker);

    pin_utils::pin_mut!(f);

    loop {
        WAKER.clear();
        match f.as_mut().poll(&mut context) {
            Poll::Ready(res) => break res,
            Poll::Pending => while WAKER.pending() {
                #[cfg(feature = "enable-release")]
                unsafe {
                    asm!("wfe" :::: "volatile") // TODO use power/sleep or whatever
                }
            },
        }
    }
}

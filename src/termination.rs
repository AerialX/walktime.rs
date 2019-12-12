#[cfg(feature = "semihosting")]
pub use semihosting::Exception as ExitStatus;

#[cfg(feature = "cortex-m-semihosting")]
pub use cortex_m_semihosting::debug::ExitStatus;

#[cfg(feature = "unstable")]
pub type Never = !;

#[cfg(not(feature = "unstable"))]
pub type Never = core::convert::Infallible;

#[cfg(not(any(feature = "semihosting", feature = "cortex-m-semihosting")))]
pub type ExitStatus = Never;

/// `main()` return type
///
/// main must return `!` unless built with semihosting support
#[cfg_attr(feature = "unstable", lang = "termination")]
pub trait Termination {
    fn report(self) -> ExitStatus;
}

#[cfg(any(feature = "semihosting", feature = "cortex-m-semihosting"))]
impl Termination for Never {
    #[inline]
    fn report(self) -> ExitStatus {
        match self { }
    }
}

#[cfg(not(feature = "cortex-m-semihosting"))] // ExitStatus is Result<(), ()> and conflicts with the blanket impl below
impl Termination for ExitStatus {
    #[inline]
    fn report(self) -> ExitStatus { self }
}

impl Termination for () {
    #[inline]
    fn report(self) -> ExitStatus { true.report() }
}

impl Termination for bool {
    #[inline]
    fn report(self) -> ExitStatus {
        match () {
            #[cfg(feature = "semihosting")]
            _ => match self {
                true => semihosting::Exception::ApplicationExit,
                false => semihosting::Exception::InternalError,
            },
            #[cfg(feature = "cortex-m-semihosting")]
            _ => match self {
                true => cortex_m_semihosting::debug::EXIT_SUCCESS,
                false => cortex_m_semihosting::debug::EXIT_FAILURE,
            },
            #[cfg(not(any(feature = "semihosting", feature = "cortex-m-semihosting")))]
            _ => unimplemented!(), // TODO expose exit_success and exit_failure fns that can be overridden?
        }
    }
}

#[cfg(not(feature = "ufmt"))]
impl<T: Termination, E: core::fmt::Debug> Termination for Result<T, E> {
    #[inline]
    fn report(self) -> ExitStatus {
        match self {
            Ok(t) => t.report(), // TODO this should probably just be ()?
            Err(_e) => {
                #[cfg(all(feature = "logging", feature = "fmt"))]
                match () {
                    #[cfg(feature = "semihosting")]
                    _ => semihosting::println!("Main error: {:?}", _e),
                    #[cfg(feature = "cortex-m-semihosting")]
                    _ => drop(cortex_m_semihosting::heprintln!("Main error: {:?}", _e)),
                    #[cfg(not(any(feature = "cortex-m-semihosting", feature = "semihosting")))]
                    _ => (),
                }

                false.report()
            },
        }
    }
}

#[cfg(feature = "ufmt")]
impl<T: Termination, E: ufmt_impl::uDebug> Termination for Result<T, E> {
    #[inline]
    fn report(self) -> ExitStatus {
        match self {
            Ok(t) => t.report(), // TODO this should probably just be ()?
            Err(_e) => {
                #[cfg(all(feature = "logging", feature = "fmt"))]
                match () {
                    #[cfg(feature = "semihosting")]
                    _ => semihosting::uprintln!("Main error: {:?}", _e),
                    #[cfg(feature = "cortex-m-semihosting")]
                    _ => compile_error!("cortex-m-semihosting + ufmt unsupported"),
                    #[cfg(not(any(feature = "cortex-m-semihosting", feature = "semihosting")))]
                    _ => (),
                }

                false.report()
            },
        }
    }
}

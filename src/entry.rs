#[cfg(feature = "constructors")]
pub struct InitContext {
    _private: (),
}

#[cfg(feature = "constructors")]
impl InitContext {
    pub unsafe fn new() -> Self {
        Self {
            _private: (),
        }
    }
}

// TODO late-init macro that automatically hooks into constructors?

#[cfg(feature = "constructors")]
#[linkme::distributed_slice]
pub static CONSTRUCTORS: [fn(&mut InitContext)] = [..];

pub type StartReturn = isize; // llvm doesn't like ! :(

#[cfg_attr(feature = "unstable", lang = "start")]
#[export_name = "lang_start"]
#[cfg_attr(any(debug_assertions, feature = "no-inline"), inline(never))]
pub fn start<T: crate::Termination + 'static>(main: fn() -> T) -> StartReturn {
    #[cfg(feature = "constructors")]
    {
        let mut con_context = unsafe { InitContext::new() };
        for con in CONSTRUCTORS {
            con(&mut con_context);
        }
    }

    // TODO no-inline for calling main too? probably should!
    let res = main();
    exit(res.report());
}

#[cfg_attr(any(debug_assertions, feature = "no-inline"), inline(never))]
pub fn exit(status: crate::ExitStatus) -> ! {
    match status {
        #[cfg(feature = "semihosting")]
        ex => {
            #[cfg(feature = "logging")]
            match ex {
                semihosting::Exception::ApplicationExit => (),
                #[cfg(not(feature = "fmt"))]
                _ => semihosting::println!("Exit failure"),
                #[cfg(feature = "ufmt")]
                ex => semihosting::uprintln!("Exit failure: {:?}", ex),
                #[cfg(all(feature = "fmt", not(feature = "ufmt")))]
                ex => semihosting::println!("Exit failure: {:?}", ex),
            }
            unsafe { semihosting::exit_with(ex) }
        },
        #[cfg(feature = "cortex-m-semihosting")]
        ex => {
            #[cfg(feature = "logging")]
            match ex {
                Err(()) => drop(cortex_m_semihosting::heprintln!("Exit failure")),
                Ok(()) => (),
            }
            loop {
                cortex_m_semihosting::debug::exit(ex);
            }
        },
    }
}

#[cfg_attr(any(debug_assertions, feature = "no-inline"), inline(never))]
pub fn abort() -> ! {
    match () {
        #[cfg(feature = "semihosting")]
        _ => {
            #[cfg(feature = "logging")]
            semihosting::println!("ABORTED");

            semihosting::abort()
        },
        #[cfg(feature = "cortex-m-semihosting")]
        _ => {
            use cortex_m_semihosting::debug;
            #[cfg(feature = "logging")]
            drop(cortex_m_semihosting::heprintln!("ABORTED"));

            loop {
                debug::report_exception(debug::Exception::InternalError); // maybe OSSpecific? What does "Internal" refer to in this context?
            }
        },
        #[cfg(not(any(feature = "semihosting", feature = "cortex-m-semihosting")))]
        _ => match () {
            #[cfg(feature = "unstable")]
            _ => unsafe { core::intrinsics::abort() }, // breakpoints or something?
            #[cfg(not(feature = "unstable"))]
            _ => unimplemented!(), // um what do we do? inline asm is unstable too!
        },
    }
}

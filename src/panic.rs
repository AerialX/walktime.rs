use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(feature = "cortex-m")]
    cortex_m::interrupt::disable();

    #[cfg(feature = "logging")]
    match () {
        #[cfg(feature = "semihosting")]
        _ => match () {
            #[cfg(feature = "fmt")]
            _ => semihosting::println!("PANIC: {}", _info),
            #[cfg(not(feature = "fmt"))]
            _ => semihosting::println!("PANIC"),
        },
        #[cfg(feature = "cortex-m-semihosting")]
        _ => drop(match () {
            #[cfg(feature = "fmt")]
            _ => cortex_m_semihosting::heprintln!("PANIC: {}", _info),
            #[cfg(not(feature = "fmt"))]
            _ => cortex_m_semihosting::heprintln!("PANIC"),
        }),
        #[cfg(not(any(feature = "semihosting", feature = "cortex-m-semihosting")))]
        _ => (),
    }

    match () {
        #[cfg(all(feature = "no-panic", not(debug_assertions)))]
        _ => reachability::unreachable_static!(!),
        #[cfg(not(all(feature = "no-panic", not(debug_assertions))))]
        _ => match () {
            #[cfg(feature = "semihosting")]
            _ => {
                use semihosting::{io, Exception};
                io::report_exception(Exception::RunTimeErrorUnknown);
                unsafe { core::hint::unreachable_unchecked() }
            },
            #[cfg(feature = "cortex-m-semihosting")]
            _ => {
                use cortex_m_semihosting::debug;
                debug::exit(debug::EXIT_FAILURE);
                unsafe { core::hint::unreachable_unchecked() }
            },
            #[cfg(not(any(feature = "semihosting", feature = "cortex-m-semihosting")))]
            _ => crate::abort(),
        },
    }
}

#[cfg(feature = "alloc")]
#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    #[cfg(feature = "logging")]
    match () {
        #[cfg(feature = "semihosting")]
        _ => match () {
            #[cfg(feature = "fmt")]
            _ => semihosting::println!("OOM: {}", _info),
            #[cfg(not(feature = "fmt"))]
            _ => semihosting::println!("OOM"),
        },
        #[cfg(feature = "cortex-m-semihosting")]
        _ => drop(match () {
            #[cfg(feature = "fmt")]
            _ => cortex_m_semihosting::heprintln!("PANIC: {}", _info),
            #[cfg(not(feature = "fmt"))]
            _ => cortex_m_semihosting::heprintln!("PANIC"),
        }),
        #[cfg(not(any(feature = "semihosting", feature = "cortex-m-semihosting")))]
        _ => (),
    }

    crate::abort()
}

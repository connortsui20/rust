#![feature(lazy_get)]
#![feature(mapped_lock_guards)]
#![feature(mpmc_channel)]
#![feature(once_cell_try)]
#![feature(lock_value_accessors)]
#![feature(reentrant_lock)]
#![feature(rwlock_downgrade)]
#![feature(std_internals)]
#![feature(sync_nonpoison)]
#![feature(nonpoison_mutex)]
#![allow(internal_features)]
#![feature(macro_metavar_expr_concat)] // For concatenating identifiers in macros.

mod barrier;
mod condvar;
mod lazy_lock;
#[cfg(not(any(target_os = "emscripten", target_os = "wasi")))]
mod mpmc;
#[cfg(not(any(target_os = "emscripten", target_os = "wasi")))]
mod mpsc;
#[cfg(not(any(target_os = "emscripten", target_os = "wasi")))]
mod mpsc_sync;
#[cfg(not(any(target_os = "emscripten", target_os = "wasi")))]
mod mutex;
#[cfg(not(any(target_os = "emscripten", target_os = "wasi")))]
mod once;
mod once_lock;
#[cfg(not(any(target_os = "emscripten", target_os = "wasi")))]
mod reentrant_lock;
#[cfg(not(any(target_os = "emscripten", target_os = "wasi")))]
mod rwlock;

#[path = "../common/mod.rs"]
mod common;

// A macro that generates two test cases for both the poison and nonpoison locks.
//
// To write a test that tests both `poison` and `nonpoison` locks, import any of the types
// under both `poison` and `nonpoison` using the module name `locks` instead. For example, write
// `use locks::Mutex;` instead of `use std::sync::poiosn::Mutex`. This will import the correct type
// for each test variant.
//
// Write a test as normal in the `test_body`, but instead of calling `unwrap` on `poison` methods
// that return a `LockResult` or similar, call the macro `maybe_unwrap!(...)` on the result.
//
// For example, call `maybe_unwrap!(mutex.lock())` instead of `mutex.lock().unwrap()` or
// `maybe_unwrap!(rwlock.read())` instead of `rwlock.read().unwrap()`.
//
// For the `poison` types, `maybe_unwrap!` will simply unwrap the `Result` (usually this is a form
// of `LockResult`, but it could also be other kinds of results). For the `nonpoison` types, it is a
// no-op.
//
// The `poison` test will have the same `name`, but with a suffix of `_unwrap_poisoned`.
#[macro_export]
macro_rules! nonpoison_and_poison_unwrap_test {
    (
        name: $name:ident,
        test_body: {$($test_body:tt)*}
    ) => {
        // Creates the nonpoison test.
        #[test]
        fn $name() {
            use ::std::sync::nonpoison as locks;

            #[allow(unused_macros)]
            macro_rules! maybe_unwrap {
                ($e:expr) => { $e };
            }

            $($test_body)*
        }

        // Creates the poison test with the suffix `_unwrap_poisoned`.
        #[test]
        fn ${concat($name, _unwrap_poisoned)}() {
            use ::std::sync::poison as locks;

            #[allow(unused_macros)]
            macro_rules! maybe_unwrap {
                ($e:expr) => { ::std::result::Result::unwrap($e) };
            }

            $($test_body)*
        }
    }
}

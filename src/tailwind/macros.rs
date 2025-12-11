#[macro_export]
macro_rules! STYLES {
    () => {
        #[cfg(not(target_arch = "wasm32"))]
        #[::cabin::private::linkme::distributed_slice]
        #[linkme(crate = ::cabin::private::linkme)]
        pub static STYLES: [fn(&mut ::cabin::tailwind::registry::StyleRegistry)] = [..];
    };
}

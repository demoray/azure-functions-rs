// WARNING: This file is regenerated by the `cargo func new` command.
// Only the `azure_functions::export!` macro invocation will be preserved.

// Export the modules that define Azure Functions here.
#[cfg(feature = "unstable")]
azure_functions::export! {
    greet,
    greet_async,
    greet_with_json,
}

#[cfg(not(feature = "unstable"))]
azure_functions::export! {
    greet,
    greet_with_json,
}

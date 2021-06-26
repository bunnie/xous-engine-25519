#![cfg_attr(target_os = "none", no_std)]

// this is an empty placeholder crate so that curve25519-dalek builds that
// do not actually require the "betrusted" feature don't break.

// in the case of the xous-core build, this crate is patched to the actual
// server within the OS.

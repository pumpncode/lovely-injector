pub mod ffi;

use std::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn CodePatch(
    address: *mut c_void,
    buffer: *mut u8,
    buffer_size: u32,
) -> ffi::MemoryOperationError {
    let result = ffi::DobbyCodePatch(address, buffer, buffer_size);
    if result == 0 {
        ffi::MemoryOperationError_kMemoryOperationSuccess
    } else {
        ffi::MemoryOperationError_kMemoryOperationError
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build_version() {
        unsafe {
            let ptr = super::ffi::DobbyBuildVersion();
            let s = std::ffi::CStr::from_ptr(ptr);
            println!("{}", s.to_string_lossy());
        }
    }
}

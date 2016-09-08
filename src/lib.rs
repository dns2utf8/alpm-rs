extern crate libloading as so;
#[macro_use] extern crate enum_primitive;
extern crate num;
use num::FromPrimitive;

use so::Symbol;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

enum_from_primitive! {
#[repr(C)]
#[derive(Debug, PartialEq)]
/*typedef*/ enum AlpmErrno {
	ALPM_ERR_MEMORY = 1,
	ALPM_ERR_SYSTEM,
	ALPM_ERR_BADPERMS,
	ALPM_ERR_NOT_A_FILE,
	ALPM_ERR_NOT_A_DIR,
	ALPM_ERR_WRONG_ARGS,
	ALPM_ERR_DISK_SPACE,
	/* Interface */
	ALPM_ERR_HANDLE_NULL,
	ALPM_ERR_HANDLE_NOT_NULL,
	ALPM_ERR_HANDLE_LOCK,
	/* Databases */
	ALPM_ERR_DB_OPEN,
	ALPM_ERR_DB_CREATE,
	ALPM_ERR_DB_NULL,
	ALPM_ERR_DB_NOT_NULL,
	ALPM_ERR_DB_NOT_FOUND,
	ALPM_ERR_DB_INVALID,
	ALPM_ERR_DB_INVALID_SIG,
	ALPM_ERR_DB_VERSION,
	ALPM_ERR_DB_WRITE,
	ALPM_ERR_DB_REMOVE,
	/* Servers */
	ALPM_ERR_SERVER_BAD_URL,
	ALPM_ERR_SERVER_NONE,
	/* Transactions */
	ALPM_ERR_TRANS_NOT_NULL,
	ALPM_ERR_TRANS_NULL,
	ALPM_ERR_TRANS_DUP_TARGET,
	ALPM_ERR_TRANS_NOT_INITIALIZED,
	ALPM_ERR_TRANS_NOT_PREPARED,
	ALPM_ERR_TRANS_ABORT,
	ALPM_ERR_TRANS_TYPE,
	ALPM_ERR_TRANS_NOT_LOCKED,
	ALPM_ERR_TRANS_HOOK_FAILED,
	/* Packages */
	ALPM_ERR_PKG_NOT_FOUND,
	ALPM_ERR_PKG_IGNORED,
	ALPM_ERR_PKG_INVALID,
	ALPM_ERR_PKG_INVALID_CHECKSUM,
	ALPM_ERR_PKG_INVALID_SIG,
	ALPM_ERR_PKG_MISSING_SIG,
	ALPM_ERR_PKG_OPEN,
	ALPM_ERR_PKG_CANT_REMOVE,
	ALPM_ERR_PKG_INVALID_NAME,
	ALPM_ERR_PKG_INVALID_ARCH,
	ALPM_ERR_PKG_REPO_NOT_FOUND,
	/* Signatures */
	ALPM_ERR_SIG_MISSING,
	ALPM_ERR_SIG_INVALID,
	/* Deltas */
	ALPM_ERR_DLT_INVALID,
	ALPM_ERR_DLT_PATCHFAILED,
	/* Dependencies */
	ALPM_ERR_UNSATISFIED_DEPS,
	ALPM_ERR_CONFLICTING_DEPS,
	ALPM_ERR_FILE_CONFLICTS,
	/* Misc */
	ALPM_ERR_RETRIEVE,
	ALPM_ERR_INVALID_REGEX,
	/* External library errors */
	ALPM_ERR_LIBARCHIVE,
	ALPM_ERR_LIBCURL,
	ALPM_ERR_EXTERNAL_DOWNLOAD,
	ALPM_ERR_GPGME
}
} // END enum_from_primitive!


pub struct Alpm {
  lib: so::Library,
  handle: *const usize,
  error_no: Box<usize>,
}

impl Alpm {
  pub fn new() -> Result<Alpm, std::io::Error> {
    let lib = try!(so::Library::new("/usr/lib/libalpm.so"));

    let root = CString::new("/").unwrap();
    let dbpath = CString::new("/var/lib/pacman/sync").unwrap();
    let mut error_no = Box::new(0);
    let handle = unsafe {
      let init: Symbol<unsafe extern fn(*const c_char, *const c_char, *mut usize) -> *const usize> = try!(lib.get(b"alpm_initialize\0"));
      init(root.as_ptr(), dbpath.as_ptr(), error_no.as_mut())
    };
    assert!(handle != 0 as *const usize,
        "handle was {}/NULL, error_no: {}/{:?}/{:?}",
        handle as usize,
        error_no,
        AlpmErrno::from_usize(*error_no),
        translate_error_no(&lib, *error_no));

    Ok(Alpm {
      lib: lib,
      handle: handle,
      error_no: error_no,
    })
  }

  pub fn query_package_version<S>(&self, s: S) -> String where S: Into<String> {
    /*unsafe {
      self.get(b"query...\0")
    }*/
    let s: String = s.into();
    s
  }
}

impl Drop for Alpm {
  fn drop(&mut self) {
    unsafe {
      let alpm_release: Symbol<unsafe extern fn(*const usize) -> *const usize> = self.lib.get(b"alpm_release\0").unwrap();
      alpm_release(self.handle);
    }
    assert!(0 == *self.error_no, "Alpm: an error occured: {}", self.error_no);
  }
}



fn translate_error_no(lib: &so::Library, error_no: usize) -> Result<String, std::io::Error> {
  unsafe {
    let alpm_strerror: Symbol<unsafe extern fn(usize) -> *const c_char> = try!(lib.get(b"alpm_strerror\0"));

    let cs = alpm_strerror(error_no);
    Ok(CStr::from_ptr(cs)
        .to_string_lossy().into_owned())
  }
}

#[cfg(test)]
mod tests {
  use ::Alpm;

  #[test]
  fn query_pacman() {
    let pacman = Alpm::new().unwrap();

    assert_eq!("5.0.1-4".to_string(), pacman.query_package_version("pacman"));
  }
}

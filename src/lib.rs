extern crate libloading as so;

use so::Symbol;
use std::ffi::CString;
use std::os::raw::c_char;

/*
#[link(name="libalpm")]
extern {
  struct alpm_errno_t;
}*/

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
    assert!(handle != 0 as *const usize, "handle was {}/NULL, error_no: {}", handle as usize, error_no);

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

#[cfg(test)]
mod tests {
  use ::Alpm;

  #[test]
  fn query_pacman() {
    let pacman = Alpm::new().unwrap();

    assert_eq!("5.0.1-4".to_string(), pacman.query_package_version("pacman"));
  }
}

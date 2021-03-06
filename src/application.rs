use Application;
use File;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

pub trait ApplicationExtManual {
    fn connect_open<F: Fn(&Self, &[File], &str) + 'static>(&self, f: F) -> u64;
    fn open(&self, files: &[File], hint: &str);
}

impl<O: IsA<Application> + IsA<glib::object::Object>> ApplicationExtManual for O {
    fn open(&self, files: &[File], hint: &str) {
        unsafe {
            ffi::g_application_open(self.to_glib_none().0, files.to_glib_none().0, files.len() as i32, 
                                    hint.to_glib_none().0);
        }
    }

    fn connect_open<F: Fn(&Self, &[File], &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &[File], &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "open",
                transmute(open_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn open_trampoline<P>(this: *mut ffi::GApplication, files: *const *mut ffi::GFile, n_files: libc::c_int, hint: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &Box_<Fn(&P, &[File], &str) + 'static> = transmute(f);
    let files: Vec<File> = FromGlibPtrContainer::from_glib_none_num(files, n_files as usize);
    f(&Application::from_glib_none(this).downcast_unchecked(), &files, &String::from_glib_none(hint))
}

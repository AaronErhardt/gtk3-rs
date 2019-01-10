// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_40", feature = "dox"))]
use Error;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use Subprocess;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use SubprocessFlags;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std;
use std::fmt;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use std::ptr;

glib_wrapper! {
    pub struct SubprocessLauncher(Object<ffi::GSubprocessLauncher, SubprocessLauncherClass>);

    match fn {
        get_type => || ffi::g_subprocess_launcher_get_type(),
    }
}

impl SubprocessLauncher {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn new(flags: SubprocessFlags) -> SubprocessLauncher {
        unsafe {
            from_glib_full(ffi::g_subprocess_launcher_new(flags.to_glib()))
        }
    }
}

pub const NONE_SUBPROCESS_LAUNCHER: Option<&SubprocessLauncher> = None;

pub trait SubprocessLauncherExt: 'static {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn getenv<P: AsRef<std::path::Path>>(&self, variable: P) -> Option<std::path::PathBuf>;

    //#[cfg(any(unix, feature = "dox"))]
    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn set_child_setup(&self, child_setup: /*Unknown conversion*//*Unimplemented*/SpawnChildSetupFunc, destroy_notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_cwd<P: AsRef<std::path::Path>>(&self, cwd: P);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_environ(&self, env: &[&std::path::Path]);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_flags(&self, flags: SubprocessFlags);

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stderr_file_path<P: AsRef<std::path::Path>>(&self, path: P);

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stdin_file_path(&self, path: &str);

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stdout_file_path<P: AsRef<std::path::Path>>(&self, path: P);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn setenv<P: AsRef<std::ffi::OsStr>, Q: AsRef<std::ffi::OsStr>>(&self, variable: P, value: Q, overwrite: bool);

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn spawn(&self, error: &mut Error, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Subprocess>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn spawnv(&self, argv: &[&std::ffi::OsStr]) -> Result<Subprocess, Error>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn unsetenv<P: AsRef<std::ffi::OsStr>>(&self, variable: P);
}

impl<O: IsA<SubprocessLauncher>> SubprocessLauncherExt for O {
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn getenv<P: AsRef<std::path::Path>>(&self, variable: P) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_subprocess_launcher_getenv(self.as_ref().to_glib_none().0, variable.as_ref().to_glib_none().0))
        }
    }

    //#[cfg(any(unix, feature = "dox"))]
    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn set_child_setup(&self, child_setup: /*Unknown conversion*//*Unimplemented*/SpawnChildSetupFunc, destroy_notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::g_subprocess_launcher_set_child_setup() }
    //}

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_cwd<P: AsRef<std::path::Path>>(&self, cwd: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_cwd(self.as_ref().to_glib_none().0, cwd.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_environ(&self, env: &[&std::path::Path]) {
        unsafe {
            ffi::g_subprocess_launcher_set_environ(self.as_ref().to_glib_none().0, env.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_flags(&self, flags: SubprocessFlags) {
        unsafe {
            ffi::g_subprocess_launcher_set_flags(self.as_ref().to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stderr_file_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_stderr_file_path(self.as_ref().to_glib_none().0, path.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stdin_file_path(&self, path: &str) {
        unsafe {
            ffi::g_subprocess_launcher_set_stdin_file_path(self.as_ref().to_glib_none().0, path.to_glib_none().0);
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn set_stdout_file_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_stdout_file_path(self.as_ref().to_glib_none().0, path.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn setenv<P: AsRef<std::ffi::OsStr>, Q: AsRef<std::ffi::OsStr>>(&self, variable: P, value: Q, overwrite: bool) {
        unsafe {
            ffi::g_subprocess_launcher_setenv(self.as_ref().to_glib_none().0, variable.as_ref().to_glib_none().0, value.as_ref().to_glib_none().0, overwrite.to_glib());
        }
    }

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn spawn(&self, error: &mut Error, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Subprocess> {
    //    unsafe { TODO: call ffi::g_subprocess_launcher_spawn() }
    //}

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn spawnv(&self, argv: &[&std::ffi::OsStr]) -> Result<Subprocess, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_subprocess_launcher_spawnv(self.as_ref().to_glib_none().0, argv.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn unsetenv<P: AsRef<std::ffi::OsStr>>(&self, variable: P) {
        unsafe {
            ffi::g_subprocess_launcher_unsetenv(self.as_ref().to_glib_none().0, variable.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for SubprocessLauncher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SubprocessLauncher")
    }
}

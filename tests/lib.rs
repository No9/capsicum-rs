/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


extern crate capsicum;

mod base {
    use capsicum::{enter, sandboxed, CapRights};
    use capsicum::{Right, FileRights, RightsBuilder};
    use capsicum::{IoctlRights, IoctlsBuilder};
    use capsicum::{Fcntl, FcntlRights, FcntlsBuilder};
    use capsicum::{init};

    use std::fs;
    use std::io::{Read, Write};

    const TMPFILE1: &'static str = "/tmp/foo";
    const TMPFILE2: &'static str = "/tmp/bar";
    const TMPFILE3: &'static str = "/tmp/baz";
    const TMPFILE4: &'static str = "/tmp/qux";

    extern {
        fn fork() -> isize;
        fn wait() -> isize;
    }

    #[test]
    fn test_casper() {
        
        init().expect("cap_init failed!");
        enter().expect("cap_enter failed!");

    }

    #[test]
    fn test_rights_right() {
        assert_eq!(144115188075855873u64, Right::Read as u64);
    }

    #[test]
    fn test_rights_builder() {
        let mut builder = RightsBuilder::new(Right::Read);
        builder.add(Right::Lookup).add(Right::AclSet)
                                  .add(Right::AclSet)
                                  .add(Right::AclGet)
                                  .add(Right::Write)
                                  .remove(Right::Lookup)
                                  .remove(Right::AclGet);
        assert_eq!(144115188076380163, builder.raw());
    }

    #[test]
    fn test_rights() {
        let mut file = fs::File::create(TMPFILE1).unwrap();

        let mut rights = match RightsBuilder::new(Right::Null).finalize() {
            Ok(rights) => rights,
            _ => panic!("Error in creation of rights")
        };

        let to_merge = RightsBuilder::new(Right::Write).finalize().unwrap();

        rights.merge(&to_merge).unwrap();

        rights.set(Right::Read).unwrap();

        rights.clear(Right::Write).unwrap();

        assert!(rights.is_set(Right::Read));

        assert!(!rights.contains(&to_merge));

        rights.limit(&file).unwrap();

        let from_file = FileRights::from_file(&file).unwrap();

        assert_eq!(rights, from_file);

        let c_string = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20,
                        0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x00];

        // Write should be limitted
        if let Ok(_) = file.write_all(&c_string) {
            fs::remove_file(TMPFILE1).unwrap();
            panic!("Rights did not correctly limit write");
        }

        file = fs::File::open(TMPFILE1).unwrap();

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        // Nothing has been written to the file
        assert_eq!("", s);

        fs::remove_file(TMPFILE1).unwrap();
    }

    #[test]
    fn test_enter() {
        unsafe {
            let pid = fork();
            if pid == 0 {
                fs::File::create(TMPFILE2).unwrap();
                let mut ok_file = fs::File::open(TMPFILE2).unwrap();

                enter().expect("cap_enter failed!");
                assert!(sandboxed(), "application is not properly sandboxed");

                if let Ok(_) = fs::File::open(TMPFILE1) {
                    panic!("application is not properly sandboxed!");
                }

                let mut s = String::new();
                if let Err(_) = ok_file.read_to_string(&mut s) {
                    panic!("application is not properly sandboxed!");
                }
            } else {
                wait();
                fs::remove_file(TMPFILE2).unwrap();
            }
        }
    }

    #[test]
    fn test_ioctl() {
        let file = fs::File::create(TMPFILE3).unwrap();
        let ioctls = IoctlsBuilder::new(9223372036854775807).add(1).finalize();
        ioctls.limit(&file).unwrap();
        let _ = IoctlRights::from_file(&file, 10).unwrap();
        fs::remove_file(TMPFILE3).unwrap();
    }

    #[test]
    fn test_fcntl() {
        let file = fs::File::create(TMPFILE4).unwrap();
        let fcntls = FcntlsBuilder::new(Fcntl::GetFL).add(Fcntl::GetOwn).finalize();
        fcntls.limit(&file).unwrap();
        let new_fcntls = FcntlRights::from_file(&file).unwrap();
        assert_eq!(new_fcntls, fcntls);
        fs::remove_file(TMPFILE4).unwrap();
    }
}

mod util {
    use std::ffi::CString;
    use capsicum::{self, CapRights, Right, RightsBuilder};
    use capsicum::util::Directory;
    #[test]
    fn test_basic_dir() {
        let dir = Directory::new("./src").unwrap();
        let rights = RightsBuilder::new(Right::Read)
            .add(Right::Lookup)
            .finalize().unwrap();
        rights.limit(&dir).unwrap();
        capsicum::enter().unwrap();
        let path = CString::new("lib.rs").unwrap();
        let _ = dir.open_file(path, 0, None).unwrap();
    }
}

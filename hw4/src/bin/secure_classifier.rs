use darknet::image_classifier;
use libseccomp::{ScmpAction, ScmpArgCompare, ScmpCompareOp, ScmpFilterContext, ScmpSyscall};
use nix::libc::{O_ACCMODE, O_RDONLY, SIGCHLD};
use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::sched::{clone, CloneFlags};
use nix::sys::wait::{waitid, WaitPidFlag};
use nix::unistd::{chdir, pivot_root, Pid};
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = &args[1];
    let parameters = fs::read_to_string(input_filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(':')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    for parameter in parameters {
        let user = parameter.first().unwrap();
        let filename = parameter.get(1).unwrap();
        let top_k = parameter.get(2).unwrap().parse::<i32>().unwrap();

        let pid: Result<Pid, nix::errno::Errno> = unsafe {
            clone(
                Box::new(|| {
                    run_image_classifier(user, filename, top_k);
                    return 0;
                }),
                vec![0; 1024 * 1024].as_mut_slice(),
                CloneFlags::CLONE_NEWNS
                    | CloneFlags::CLONE_NEWUSER
                    | CloneFlags::CLONE_NEWPID
                    | CloneFlags::CLONE_IO,
                Some(SIGCHLD),
            )
        };

        waitid(
            nix::sys::wait::Id::Pid(pid.expect("clone failed")),
            WaitPidFlag::WEXITED,
        )
        .expect("wait pid failed");
    }

    println!("finished");
}

fn run_image_classifier(username: &String, filename: &String, top_k: i32) {
    fs::create_dir(Path::new("data").join(username).join("put_old")).unwrap_or_default();

    mount::<PathBuf, str, str, str>(
        Some(&Path::new("data").join(username)),
        "/mnt",
        None,
        MsFlags::MS_BIND,
        None,
    )
    .expect("mount data dir");

    pivot_root(Path::new("/mnt"), &Path::new("/mnt").join("put_old")).expect("swap root");

    chdir("/").expect("change dir to /");

    umount2(Path::new("/put_old"), MntFlags::MNT_DETACH).expect("confine");

    fs::remove_dir("/put_old").unwrap_or_default();
    fs::remove_file("results.txt").unwrap_or_default();

    let mut context = ScmpFilterContext::new_filter(ScmpAction::Errno(1)).unwrap();

    context
        .add_rule_conditional(
            ScmpAction::Allow,
            ScmpSyscall::from_name("write").unwrap(),
            &[ScmpArgCompare::new(0, ScmpCompareOp::Equal, 1)],
        )
        .unwrap();
    context
        .add_rule_conditional(
            ScmpAction::Allow,
            ScmpSyscall::from_name("write").unwrap(),
            &[ScmpArgCompare::new(0, ScmpCompareOp::Equal, 2)],
        )
        .unwrap();
    context
        .add_rule(ScmpAction::Allow, ScmpSyscall::from_name("getcwd").unwrap())
        .unwrap();
    context
        .add_rule_conditional(
            ScmpAction::Allow,
            ScmpSyscall::from_name("open").unwrap(),
            &[ScmpArgCompare::new(
                1,
                ScmpCompareOp::MaskedEqual(O_ACCMODE as u64),
                O_RDONLY as u64,
            )],
        )
        .unwrap();
    context
        .add_rule_conditional(
            ScmpAction::Allow,
            ScmpSyscall::from_name("openat").unwrap(),
            &[ScmpArgCompare::new(
                2,
                ScmpCompareOp::MaskedEqual(O_ACCMODE as u64),
                O_RDONLY as u64,
            )],
        )
        .unwrap();
    context
        .add_rule(ScmpAction::Allow, ScmpSyscall::from_name("read").unwrap())
        .unwrap();

    context.load().expect("apply seccomp filter");

    unsafe {
        image_classifier(
            CStr::as_ptr(&CString::new(Path::new(filename).to_str().unwrap()).unwrap()),
            top_k,
        )
    };
}

use crate::spec::{LinkerFlavor, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::netbsd_base::opts();
    base.cpu = "pentiumpro".into();
    base.max_atomic_width = Some(64);
    base.add_pre_link_args(LinkerFlavor::Gcc, &["-m32"]);
    // don't use probe-stack=inline-asm until rust#83139 and rust#84667 are resolved
    base.stack_probes = StackProbeType::Call;

    Target {
        llvm_target: "i686-unknown-netbsdelf".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            f64:32:64-f80:32-n8:16:32-S128"
            .into(),
        arch: "x86".into(),
        options: TargetOptions { mcount: "__mcount".into(), ..base },
    }
}

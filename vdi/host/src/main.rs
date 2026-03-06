fn main() {
    hbb_common::init_log(false, "");
    if let Err(err) = qemu_cevizdesk::server::run() {
        hbb_common::log::error!("{err}");
    }
}

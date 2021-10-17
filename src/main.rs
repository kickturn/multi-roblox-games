#[allow(unused_must_use)]
unsafe fn main() {
    winapi::um::synchapi::CreateMutexA(0 as *mut winapi::um::minwinbase::SECURITY_ATTRIBUTES, 1, "ROBLOX_singletonMutex".as_ptr() as *const i8);
    println!("Running, exiting this or pressing enter will terminate all games except a random one. You must use different accounts for every game you launch");
    std::io::stdin().read_line(&mut String::new());
}

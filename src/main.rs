
use std::time::Instant;
use std::mem;
use rand::prelude::*;

fn main() {
    set_highest_process_priority();

    let mut rng = rand::thread_rng();
    let mut array : Vec<usize>  = vec![0; 1024*1024*1024 * 2 /* 8x32 M */];
    for index in 0..array.len() {
        array[index] = rng.gen();
    }

    // AMD 5800U  L1 32K+32K,  L2 512K,  L3 16MB
    let mut max = array.len();
    loop {
        measure(&array, max);
        max = max / 2;
        if max < 16 {
            break;
        }
    }    
}


fn measure(array : &Vec<usize>, max : usize) {
    let length = array.len();
    if max > length {
        panic!("max must be less than {}", length);
    }

    let now = Instant::now();

    let mut index = 0;
    for i in 0..100000000 {
        index = (array[index] + i) % max;
    }

    let elapsed = now.elapsed();

    let size = max * mem::size_of::<usize>();
    let size = if size >= 1024*1024*1024 {
        format!("{}GB", size/1024/1024/1024)
    } else if size >= 1024*1024 {
        format!("{}MB", size/1024/1024)
    } else if size >= 1024 {
        format!("{}KB", size/1024)
    } else {
        format!("{}B", size)
    };
    println!("{} = {:.2?} ms",  size, elapsed.as_millis());
}



#[cfg(target_os = "windows")]
fn set_highest_process_priority()
{
    use windows_sys::Win32::System::Threading::*;

    unsafe {
        SetPriorityClass(GetCurrentProcess(), REALTIME_PRIORITY_CLASS);
        SetProcessAffinityMask(GetCurrentProcess(), 1);
    }
}

#[cfg(not(target_os = "windows"))]
fn set_highest_process_priority() {
    use nix::libc::*;
    unsafe {
        setpriority(PRIO_PROCESS, 0, -20);
    }
}
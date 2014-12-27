use std::dynamic_lib::DynamicLibrary;
use std::io::fs::{copy};
use std::path::posix::Path;

// I don't know how to get this to work.
// The dynamic library goes out of scope when the function returns so this isn't working.

// here's the problem, I can't compile because that file is probably being read.

fn load_library() -> DynamicLibrary {
    let library_name = "target/libdynamic_lib-3416d80dcfef07d2.dylib";
    let mylibrary_name = "dynamic_lib.dylib";

    // copy library to mylibrary before loading it.
    copy(&Path::new(library_name), &Path::new(mylibrary_name));
    
    return match DynamicLibrary::open(
        Some(mylibrary_name)) {
        Ok(dynamic_library) => {
            println!("Found Library");
            dynamic_library
        },
        Err(err) => {
            panic!("error: {}", err)
        }
    }
}

fn main() {
    unsafe {
        let mut library = load_library();  
        // let mut function = get_function(library);
        let mut needs_reload = true;
        let mut counter = 0i;
        loop {
            library = if (counter % 10000 == 0) { load_library() } else { library };

            unsafe {
                // Can I do the lookup and call ever frame? Is that fast enough?
                let f = match library.symbol::<fn() -> int>("function_to_call") {
                    Ok(symbol) => 
                        *symbol,
                    Err(err) => 
                        panic!("error: {}", err),
                };
                let result = f();
                println!("the result is: {}", result);
            }
            counter = counter+1;


            
        }
    }
}
    // let mut the_function = reload_library();

    // let result = the_function();
    // println!("{}", result);
    
    // unsafe {
    //     let mut reload = true;

    //     let mut f = None

    //     if reload {

    //         let dlib = match DynamicLibrary::open(
    //             Some("target/libdynamic_lib-3416d80dcfef07d2.dylib")) {
    //             Ok(dynamic_library) => {
    //                 println!("Found Library");
    //                 dynamic_library
    //             },
    //             Err(err) => {
    //                 panic!("error: {}", err)
    //             }
    //         };

    //         let f = match dlib.symbol::<fn() -> int>("function_to_call") {
    //             Ok(symbol) => {
    //                 println!("Found Symbol");
    //                 *symbol
    //             }
    //             Err(err) => 
    //                 panic!("error: {}", err),
    //         };
            
    //     }
    
    //     loop {
    //         let f = reload_library();
    //         let result = f();
    //         println!("{}", result);
    //     }

    // }


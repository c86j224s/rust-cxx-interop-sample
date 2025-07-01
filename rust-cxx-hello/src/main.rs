#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust-cxx-hello/include/hello.h");

        fn hello();
        fn helloString() -> String;

        type IAnimal;
        fn createDog() -> SharedPtr<IAnimal>;
        fn makeSpeak(animal: SharedPtr<IAnimal>);

        fn sumOf(numbers: &[u32]) -> u32;
        fn reverse(strs: &mut [String]);

        include!("rust-cxx-hello/include/rust-cxx-hello-lib/rust-cxx-hello-lib.h");

        fn fnrustcxxhellolib();
    }
}

fn main() {
    println!("Hello, world!");

    ffi::hello();

    ffi::fnrustcxxhellolib();

    println!("String from C++: {}", ffi::helloString());

    {
        let dog = ffi::createDog();
        //println!("Created a dog: {:?}", dog);
        ffi::makeSpeak(dog.clone());
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = ffi::sumOf(&numbers);
    println!("Sum of {:?} is {}", numbers, sum);

    let mut strs = vec!["Hello".to_string(), "World".to_string(), "from".to_string(), "Rust".to_string()];
    ffi::reverse(&mut strs);
    println!("Reversed strings: {:?}", strs);
}

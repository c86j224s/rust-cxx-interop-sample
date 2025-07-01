#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust-cxx-hello/include/hello.h");

        fn hello();
        fn helloString() -> String;

        type IAnimal;
        fn createDog() -> SharedPtr<IAnimal>;
        fn makeSpeak(animal: SharedPtr<IAnimal>);

        include!("rust-cxx-hello/include/rust-cxx-hello-lib/rust-cxx-hello-lib.h");

        fn fnrustcxxhellolib();
    }
}

fn main() {
    println!("Hello, world!");

    ffi::hello();

    ffi::fnrustcxxhellolib();

    println!("String from C++: {}", ffi::helloString());

    let dog = ffi::createDog();
    //println!("Created a dog: {:?}", dog);
    ffi::makeSpeak(dog.clone());
}

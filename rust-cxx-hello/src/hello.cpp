#include "rust-cxx-hello/include/hello.h"

void hello() {
    std::cout << "Hello from C++!" << std::endl;
    std::cout << "C++ version: " << __cplusplus << std::endl;
}

rust::String helloString() {
    return std::string("Hello from C++ as a string!");
}

class Dog : public IAnimal {
public:
    Dog() { 
        std::cout << "Dog is born." << std::endl; 
    }
    ~Dog() override {
        std::cout << "Dog is gone." << std::endl;
    }

    void eat() const override {
        std::cout << "Dog is eating." << std::endl;
    }

    void sleep() const override {
        std::cout << "Dog is sleeping." << std::endl;
    }

    void speak() const override {
        std::cout << "Dog says: Woof!" << std::endl;
    }
};

std::shared_ptr<IAnimal> createDog() {
    return std::make_shared<Dog>();
}

void makeSpeak(std::shared_ptr<IAnimal> animal) {
    if (animal) {
        animal->speak();
    } else {
        std::cout << "Animal is null." << std::endl;
    }
}

uint32_t sumOf(rust::Slice<const uint32_t> numbers) {
    uint32_t sum = 0;
    for (const auto& number : numbers) {
        sum += number;
    }
    return sum;
}

void reverse(rust::Slice<rust::String> strs) {
    std::reverse(strs.begin(), strs.end());
}
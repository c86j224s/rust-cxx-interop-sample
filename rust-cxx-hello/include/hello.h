#pragma once

#include <iostream>
#include <memory>

#include "rust/cxx.h"

void hello();

rust::String helloString();

class IAnimal {
public:
    virtual ~IAnimal() = default;
    virtual void eat() const = 0;
    virtual void sleep() const = 0;
    virtual void speak() const = 0;
};

std::shared_ptr<IAnimal> createDog();

void makeSpeak(std::shared_ptr<IAnimal> animal);
#include <cstddef>
#include <cstdint>
#include <iostream>


// Struct
struct AllocatorStruct {
    std::uint8_t* memory;
    std::size_t capacity;
    std::size_t offset;
};

class BumpAllocator {
private:
    AllocatorStruct allocator;
    
public:
    //constructor 
    BumpAllocator(std::size_t size) {
        allocator.memory = new std::uint8_t[size];
        allocator.capacity = size;
        allocator.offset = 0;
    }

    //destructor
    ~BumpAllocator () {
        delete[] allocator.memory;
    }

    //align_up method
    std::size_t align_up(std::size_t value, std::size_t alignment) {
        return (value + (alignment -1)) & ~(alignment -1);
    }

    //allocate method
    void* allocate(std::size_t requested_size, std::size_t alignment) {
        std::size_t aligned_offset = align_up(allocator.offset, alignment);
        std::size_t new_offset = aligned_offset + requested_size;

        if (new_offset > allocator.capacity) {
            std::cout << "Not enough space for requested byte.\n";
            return nullptr;
        }

        void* ptr = allocator.memory + aligned_offset;
        allocator.offset = new_offset;

        return ptr;
    }

    //deallocate (reset allocator) method
    void reset() {
        allocator.offset = 0;
    }

};


int main () {
    // Usage.
    BumpAllocator myAllocator(32);

    int* arr = static_cast<int *>(myAllocator.allocate(sizeof(int) * 3, alignof(int)));
    // align of determines the determined cpu alignment for the data type

    if (arr) {
        arr[0] = 40;
        arr[1] = 10;
        arr[2] = 50;

        std::cout << "Array: ";
        for (int i = 0; i < 3; i++)
            std::cout << arr[i] << " ";
        std::cout << std::endl;
    } else {
        std::cout << "Array allocation failed\n";
    }

    // large allocation
    int* bigArray = static_cast<int *>(myAllocator.allocate(sizeof(int) * 10, alignof(int)));
    
    if (!bigArray) {
        std::cout << "Big allocation failed. Not enough memory\n";
    }

    myAllocator.reset();
}
# Variables
CXX = g++
CXXFLAGS = -std=c++17 -I./cpp -Wall
CARGO = cargo

# Library and Directory Names
RUST_DIR = lib_sunwave
CPP_DIR = cpp
LIB_NAME = sunwave 

# Detect OS for library extensions
ifeq ($(OS),Windows_NT)
    LIB_EXT = dll
    LIB_PREFIX = 
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Darwin)
        LIB_EXT = dylib
    else
        LIB_EXT = so
    endif
    LIB_PREFIX = lib
endif

# Paths
RUST_OUT_DIR = $(RUST_DIR)/target/release
LIB_PATH = $(RUST_OUT_DIR)/$(LIB_PREFIX)$(LIB_NAME).$(LIB_EXT)
TARGET = sunwave

# Default Rule
all: $(TARGET)

verify:
	@echo "Checking system requirements..."
	@$(CARGO) --version > /dev/null 2>&1 || (echo "Error: Cargo is not installed." && exit 1)
	@rustc --version | awk '{split($$2,a,"."); if (a[1]<1 || (a[1]==1 && a[2]<88)) {print "Error: Rust version must be 1.88.0 or later."; exit 1}}'
	@$(CXX) -v > /dev/null 2>&1 || (echo "Error: GCC is not installed." && exit 1)
	@echo "int main(){}" | $(CXX) -std=c++17 -x c++ - -o /dev/null > /dev/null 2>&1 || (echo "Error: $(CXX) does not support C++17." && exit 1)
	@echo "Verification successful: All requirements met."

# 1. Build the Rust Library
$(LIB_PATH): $(shell find $(RUST_DIR)/src -type f) $(RUST_DIR)/Cargo.toml
	@echo "Building Rust library..."
	$(CARGO) build --manifest-path $(RUST_DIR)/Cargo.toml --release

# 2. Compile the C++ App
$(TARGET): verify $(CPP_DIR)/main.cpp $(LIB_PATH)
	@echo "Compiling C++ application with RPATH..."
	$(CXX) $(CXXFLAGS) $(CPP_DIR)/main.cpp \
		-L$(RUST_OUT_DIR) \
		-l$(LIB_NAME) \
		-Wl,-rpath,'$$ORIGIN/$(RUST_OUT_DIR)' \
		-lpthread -ldl -o $(TARGET)

clean:
	$(CARGO) clean --manifest-path $(RUST_DIR)/Cargo.toml
	rm -f $(TARGET)

run: $(TARGET)
ifeq ($(UNAME_S),Darwin)
	DYLD_LIBRARY_PATH=$(RUST_OUT_DIR) ./$(TARGET)
else
	LD_LIBRARY_PATH=$(RUST_OUT_DIR) ./$(TARGET)
endif

.PHONY: all clean run verify

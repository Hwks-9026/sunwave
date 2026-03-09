# Compiler Name Variables
CXX = g++
CXXFLAGS = -std=c++17 -I./cpp -Wall
CARGO = cargo

# Configuration
PROFILE ?= release
RUST_DIR = lib_sunwave
CPP_DIR = cpp
BUILD_DIR = build
LIB_NAME = sunwave

# Paths
RUST_OUT_DIR = $(RUST_DIR)/target/$(PROFILE)
CARGO_FLAGS = --manifest-path $(RUST_DIR)/Cargo.toml
ifeq ($(PROFILE),release)
    CARGO_FLAGS += --release
endif

# Library naming
ifeq ($(OS),Windows_NT)
    LIB_EXT = dll
    LIB_PREFIX = 
else
    UNAME_S := $(shell uname -s)
    LIB_PREFIX = lib
    ifeq ($(UNAME_S),Darwin)
        LIB_EXT = dylib
    else
        LIB_EXT = so
    endif
endif

LIB_PATH = $(RUST_OUT_DIR)/$(LIB_PREFIX)$(LIB_NAME).$(LIB_EXT)
TARGET = sunwave
VERIFY_STAMP = $(BUILD_DIR)/.verify_done

# 1. Source and Object Discovery
SRCS = $(wildcard $(CPP_DIR)/*.cpp)
# This transforms 'cpp/main.cpp' into 'build/main.o'
OBJS = $(patsubst $(CPP_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(SRCS))

# Rules
all: $(TARGET)

# Verify depends on the existence of the build directory
$(VERIFY_STAMP): | $(BUILD_DIR)
	@echo "Checking system requirements..."
	@$(CARGO) --version > /dev/null 2>&1 || (echo "Error: Cargo missing." && exit 1)
	@rustc --version | awk '{split($$2,a,"."); if (a[1]<1 || (a[1]==1 && a[2]<88)) {print "Error: Rust < 1.88.0"; exit 1}}'
	@$(CXX) -v > /dev/null 2>&1 || (echo "Error: GCC missing." && exit 1)
	@touch $(VERIFY_STAMP)

# 2. Updated Target Rule
$(TARGET): $(VERIFY_STAMP) $(OBJS) $(LIB_PATH)
	@echo "Linking $(TARGET)..."
	$(CXX) $(CXXFLAGS) $(OBJS) -L$(RUST_OUT_DIR) -l$(LIB_NAME) -Wl,-rpath,'$$ORIGIN/$(RUST_OUT_DIR)' -lpthread -ldl -o $(TARGET)

# 3. Compile rust sunwave language library
$(LIB_PATH): $(shell find $(RUST_DIR)/src -type f) $(RUST_DIR)/Cargo.toml
	@echo "Building Rust library in $(PROFILE) mode..."
	$(CARGO) build $(CARGO_FLAGS)

# 4. Rule to compile .cpp to build/%.o
$(BUILD_DIR)/%.o: $(CPP_DIR)/%.cpp $(CPP_DIR)/sunwave.h | $(BUILD_DIR)
	@echo "Compiling $< -> $@"
	$(CXX) $(CXXFLAGS) -c $< -o $@

# 4. Create the build directory if it doesn't exist
$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)

# 5. Clean build artifacts
clean:
	$(CARGO) clean --manifest-path $(RUST_DIR)/Cargo.toml
	rm -f $(TARGET)
	rm -rf $(BUILD_DIR)

run: $(TARGET)
ifeq ($(UNAME_S),Darwin)
	DYLD_LIBRARY_PATH=$(RUST_OUT_DIR) ./$(TARGET) $(FILE)
else
	LD_LIBRARY_PATH=$(RUST_OUT_DIR) ./$(TARGET) $(FILE)
endif

.PHONY: all clean run

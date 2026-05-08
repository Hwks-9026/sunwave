CXX = g++
CXXFLAGS = -std=c++17 -I./cpp -Wall -O3
CARGO = cargo

PROFILE ?= release
RUST_DIR = lib_sunwave
CPP_DIR = cpp
BUILD_DIR = build
LIB_NAME = sunwave

RUST_OUT_DIR = $(RUST_DIR)/target/$(PROFILE)
CARGO_FLAGS = --manifest-path $(RUST_DIR)/Cargo.toml
ifeq ($(PROFILE),release)
    CARGO_FLAGS += --release
endif

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

RUST_SRCS := $(wildcard $(RUST_DIR)/src/*.rs)
$(LIB_PATH): $(RUST_SRCS) $(RUST_DIR)/Cargo.toml

TARGET = sunwave
VERIFY_STAMP = $(BUILD_DIR)/.verify_done

# 1. Source and Object Discovery
SRCS = $(wildcard $(CPP_DIR)/*.cpp)
# This transforms 'cpp/main.cpp' into 'build/main.o'
OBJS = $(patsubst $(CPP_DIR)/%.cpp, $(BUILD_DIR)/%.o, $(SRCS))

ifeq ($(OS),Windows_NT)
    SHELL := cmd.exe
    MKDIR_P = if not exist "$(BUILD_DIR)" mkdir "$(BUILD_DIR)"
else
    MKDIR_P = mkdir -p "$(BUILD_DIR)"
endif

ifeq ($(OS),Windows_NT)
    LDLIBS =
else
    LDLIBS = -Wl,-rpath,'$$ORIGIN/$(RUST_OUT_DIR)' -lpthread -ldl
endif

all: $(TARGET)

$(VERIFY_STAMP): | $(BUILD_DIR)
	@echo "Checking system requirements..."
	@$(CARGO) --version > /dev/null 2>&1 || (echo "Error: Cargo missing." && exit 1)
	@rustc --version | awk '{split($$2,a,"."); if (a[1]<1 || (a[1]==1 && a[2]<88)) {print "Error: Rust < 1.88.0"; exit 1}}'
	@$(CXX) -v > /dev/null 2>&1 || (echo "Error: GCC missing." && exit 1)
	@echo "Verification complete, all dependancies present."
	@touch $(VERIFY_STAMP)

$(TARGET): $(VERIFY_STAMP) $(OBJS) $(LIB_PATH)
	@echo "Linking $(TARGET)..."
	$(CXX) $(CXXFLAGS) $(OBJS) -L$(RUST_OUT_DIR) -l$(LIB_NAME) $(LDLIBS) -o $(TARGET)

rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))
$(LIB_PATH): $(call rwildcard,$(RUST_DIR)/src,*) $(RUST_DIR)/Cargo.toml
	@echo "Building Rust library in $(PROFILE) mode..."
	$(CARGO) build $(CARGO_FLAGS)

$(BUILD_DIR)/%.o: $(CPP_DIR)/%.cpp $(CPP_DIR)/sunwave.h | $(BUILD_DIR)
	@echo "Compiling $< -> $@"
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(BUILD_DIR):
	@$(MKDIR_P)

clean:
	$(CARGO) clean --manifest-path $(RUST_DIR)/Cargo.toml
	-$(RM) $(TARGET)
	-$(RMDIR) $(BUILD_DIR)

clean_cpp:
	-$(RM) $(TARGET)
	-$(RMDIR) $(BUILD_DIR)

verify: $(VERIFY_STAMP)

run: $(TARGET)
ifeq ($(UNAME_S),Darwin)
	DYLD_LIBRARY_PATH=$(RUST_OUT_DIR) ./$(TARGET) $(FILE)
else
	LD_LIBRARY_PATH=$(RUST_OUT_DIR) ./$(TARGET) $(FILE)
endif

.PHONY: all clean run

# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.28

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/rik3n/Coding/data-mesh/test

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/rik3n/Coding/data-mesh/build/test

# Utility rule file for check-format.

# Include any custom commands dependencies for this target.
include _deps/format.cmake-build/CMakeFiles/check-format.dir/compiler_depend.make

# Include the progress variables for this target.
include _deps/format.cmake-build/CMakeFiles/check-format.dir/progress.make

check-format: _deps/format.cmake-build/CMakeFiles/check-format.dir/build.make
.PHONY : check-format

# Rule to build all files generated by this target.
_deps/format.cmake-build/CMakeFiles/check-format.dir/build: check-format
.PHONY : _deps/format.cmake-build/CMakeFiles/check-format.dir/build

_deps/format.cmake-build/CMakeFiles/check-format.dir/clean:
	cd /home/rik3n/Coding/data-mesh/build/test/_deps/format.cmake-build && $(CMAKE_COMMAND) -P CMakeFiles/check-format.dir/cmake_clean.cmake
.PHONY : _deps/format.cmake-build/CMakeFiles/check-format.dir/clean

_deps/format.cmake-build/CMakeFiles/check-format.dir/depend:
	cd /home/rik3n/Coding/data-mesh/build/test && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/rik3n/Coding/data-mesh/test /home/rik3n/Coding/data-mesh/build/test/_deps/format.cmake-src /home/rik3n/Coding/data-mesh/build/test /home/rik3n/Coding/data-mesh/build/test/_deps/format.cmake-build /home/rik3n/Coding/data-mesh/build/test/_deps/format.cmake-build/CMakeFiles/check-format.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : _deps/format.cmake-build/CMakeFiles/check-format.dir/depend


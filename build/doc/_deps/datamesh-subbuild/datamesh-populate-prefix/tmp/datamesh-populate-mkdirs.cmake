# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION 3.5)

file(MAKE_DIRECTORY
  "/home/rik3n/Coding/data-mesh/documentation/.."
  "/home/rik3n/Coding/data-mesh/build/doc/_deps/datamesh-build"
  "/home/rik3n/Coding/data-mesh/build/doc/_deps/datamesh-subbuild/datamesh-populate-prefix"
  "/home/rik3n/Coding/data-mesh/build/doc/_deps/datamesh-subbuild/datamesh-populate-prefix/tmp"
  "/home/rik3n/Coding/data-mesh/build/doc/_deps/datamesh-subbuild/datamesh-populate-prefix/src/datamesh-populate-stamp"
  "/home/rik3n/Coding/data-mesh/build/doc/_deps/datamesh-subbuild/datamesh-populate-prefix/src"
  "/home/rik3n/Coding/data-mesh/build/doc/_deps/datamesh-subbuild/datamesh-populate-prefix/src/datamesh-populate-stamp"
)

set(configSubDirs )
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "/home/rik3n/Coding/data-mesh/build/doc/_deps/datamesh-subbuild/datamesh-populate-prefix/src/datamesh-populate-stamp/${subDir}")
endforeach()
if(cfgdir)
  file(MAKE_DIRECTORY "/home/rik3n/Coding/data-mesh/build/doc/_deps/datamesh-subbuild/datamesh-populate-prefix/src/datamesh-populate-stamp${cfgdir}") # cfgdir has leading slash
endif()

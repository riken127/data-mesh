#----------------------------------------------------------------
# Generated CMake target import file.
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "DataMesh::DataMesh" for configuration ""
set_property(TARGET DataMesh::DataMesh APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(DataMesh::DataMesh PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_NOCONFIG "CXX"
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/DataMesh-1.0/libDataMesh.a"
  )

list(APPEND _cmake_import_check_targets DataMesh::DataMesh )
list(APPEND _cmake_import_check_files_for_DataMesh::DataMesh "${_IMPORT_PREFIX}/lib/DataMesh-1.0/libDataMesh.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)

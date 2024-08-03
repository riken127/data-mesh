#include <doctest/doctest.h>
#include <datamesh/datamesh.h>
#include <datamesh/version.h>

#include <string>

TEST_CASE("DataMesh") {
  using namespace datamesh;

  DataMesh datamesh("Tests");

  CHECK(datamesh.greet(LanguageCode::EN) == "Hello, Tests!");
  CHECK(datamesh.greet(LanguageCode::DE) == "Hallo Tests!");
  CHECK(datamesh.greet(LanguageCode::ES) == "¡Hola Tests!");
  CHECK(datamesh.greet(LanguageCode::FR) == "Bonjour Tests!");
}

TEST_CASE("DataMesh version") {
  static_assert(std::string_view(DATAMESH_VERSION) == std::string_view("0.0.1"));
  CHECK(std::string(DATAMESH_VERSION) == std::string("0.0.1"));
}

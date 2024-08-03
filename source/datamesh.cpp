#include <fmt/format.h>
#include <datamesh/datamesh.h>

using namespace datamesh;

DataMesh::DataMesh(std::string _name) : name(std::move(_name)) {}

std::string DataMesh::greet(LanguageCode lang) const {
    switch (lang) {
        default:
        case LanguageCode::EN:
            return fmt::format("Hello, {}!", name);
        case LanguageCode::DE:
            return fmt::format("Hallo, {}!", name);
        case LanguageCode::ES:
            return fmt::format("!Hola, {}!", name);
        case LanguageCode::FR:
            return fmt::format("Bonjour, {}!", name);

    }
}

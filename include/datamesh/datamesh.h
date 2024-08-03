#pragma once

#include <string>

namespace datamesh {

    /** Language codes to be used with the DataMesh class */
    enum class LanguageCode { EN, DE, ES, FR };

    /**
     * @brief A class for saying hello in multiple languages
     */
    class DataMesh {
        std::string name;

        public:
            /**
             * @brief Creates a new greeter
             * @param name the name to greet
             */
            DataMesh(std::string name);

            /**
             * @brief Creates a localized string containing the greeting
             * @param lang the language to greet in
             * @return a string containing the greeting
             */
            std::string greet(LanguageCode lang = LanguageCode::EN) const;
    };
}

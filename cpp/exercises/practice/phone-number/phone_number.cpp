#include "phone_number.h"

#include "phone_number.h"
#include <string>
#include <stdexcept>
#include <algorithm>
#include <cctype>

namespace phone_number {

phone_number::phone_number(const std::string& input) {
    std::string digits;
    for (char c : input) {
        if (std::isdigit(c)) {
            digits += c;
        } else if (c != '(' && c != ')' && c != '.' && c != '-' && c != '+' && c != ' ') {
            throw std::domain_error("Invalid character in phone number");
        }
    }

    if (digits.length() == 11 && digits[0] == '1') {
        digits = digits.substr(1);
    } else if (digits.length() != 10) {
        throw std::domain_error("Invalid phone number length");
    }

    if (digits[0] == '0' || digits[0] == '1') {
        throw std::domain_error("Invalid area code");
    }
    if (digits[3] == '0' || digits[3] == '1') {
        throw std::domain_error("Invalid exchange code");
    }

    cleaned_number = digits;
}

std::string phone_number::number() const {
    return cleaned_number;
}

}  // namespace phone_number


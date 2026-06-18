#include "all_your_base.h"
#include <stdexcept>
#include <algorithm>

namespace all_your_base {

std::vector<unsigned int> convert(unsigned int input_base, const std::vector<unsigned int>& input_digits, unsigned int output_base) {
    if (input_base < 2) {
        throw std::invalid_argument("Input base must be >= 2");
    }
    if (output_base < 2) {
        throw std::invalid_argument("Output base must be >= 2");
    }
    
    unsigned long long value = 0;
    bool all_zero = true;
    for (unsigned int digit : input_digits) {
        if (digit >= input_base) {
            throw std::invalid_argument("Digit must be less than input base");
        }
        value = value * input_base + digit;
        if (digit != 0) {
            all_zero = false;
        }
    }
    
    if (input_digits.empty()) {
        return {};
    }
    
    if (all_zero) {
        return {};
    }
    
    std::vector<unsigned int> result;
    while (value > 0) {
        result.push_back(value % output_base);
        value /= output_base;
    }
    std::reverse(result.begin(), result.end());
    return result;
}

}  // namespace all_your_base

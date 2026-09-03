#if !defined(PARALLEL_LETTER_FREQUENCY_H)
#define PARALLEL_LETTER_FREQUENCY_H

#include <string_view>
#include <unordered_map>
#include <vector>

namespace parallel_letter_frequency {

/**
 * @brief Computes letter frequencies from a collection of texts using parallel processing.
 *
 * @param texts A vector of string_views containing the input texts.
 * @return std::unordered_map<char, int> Frequency count of each lowercase alphabetic letter.
 */
std::unordered_map<char, int> frequency(const std::vector<std::string_view>& texts);

}  // namespace parallel_letter_frequency

#endif

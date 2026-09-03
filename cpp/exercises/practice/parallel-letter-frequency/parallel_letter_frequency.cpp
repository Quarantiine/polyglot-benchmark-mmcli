#include "parallel_letter_frequency.h"

#include <algorithm>
#include <array>
#include <cctype>
#include <cstddef>
#include <functional>
#include <numeric>
#include <thread>
#include <vector>

namespace parallel_letter_frequency {

namespace {

/**
 * @brief Cacheline-aligned buffer to hold character counts without false sharing.
 */
struct alignas(64) ThreadFrequencyBuffer {
    std::array<int, 256> counts{};
};

/**
 * @brief Processes a range of string_view chunks and accumulates letter frequencies.
 *
 * @param chunks Vector of string_views to process.
 * @param start_idx Starting index in the chunks vector.
 * @param end_idx Ending index (exclusive) in the chunks vector.
 * @param output_buffer Buffer to store the character counts.
 */
void count_frequencies_in_chunks(
    const std::vector<std::string_view>& chunks,
    std::size_t start_idx,
    std::size_t end_idx,
    ThreadFrequencyBuffer& output_buffer) {
    for (std::size_t i = start_idx; i < end_idx; ++i) {
        const std::string_view chunk = chunks[i];
        for (char ch : chunk) {
            const unsigned char uc = static_cast<unsigned char>(ch);
            if (std::isalpha(uc)) {
                output_buffer.counts[static_cast<unsigned char>(std::tolower(uc))]++;
            }
        }
    }
}

}  // namespace

std::unordered_map<char, int> frequency(const std::vector<std::string_view>& texts) {
    if (texts.empty()) {
        return {};
    }

    std::size_t total_chars = 0;
    for (const auto& text : texts) {
        total_chars += text.size();
    }

    if (total_chars == 0) {
        return {};
    }

    unsigned int hw_concurrency = std::thread::hardware_concurrency();
    if (hw_concurrency == 0) {
        hw_concurrency = 4;
    }

    // Determine thread count based on hardware and total workload
    // Small inputs (< 1024 chars) can run on 1 thread without thread creation overhead.
    std::size_t num_threads = 1;
    if (total_chars >= 1024 && hw_concurrency > 1) {
        num_threads = std::min<std::size_t>(hw_concurrency, std::max<std::size_t>(1, total_chars / 2048));
        num_threads = std::max<std::size_t>(1, num_threads);
    }

    // Partition input texts into string_view chunks for load balancing
    std::vector<std::string_view> chunks;
    if (num_threads > 1) {
        std::size_t target_chunk_size = std::max<std::size_t>(512, total_chars / (num_threads * 4));
        for (const auto& text : texts) {
            if (text.empty()) {
                continue;
            }
            if (text.size() <= target_chunk_size) {
                chunks.push_back(text);
            } else {
                std::size_t offset = 0;
                while (offset < text.size()) {
                    std::size_t len = std::min(target_chunk_size, text.size() - offset);
                    chunks.push_back(text.substr(offset, len));
                    offset += len;
                }
            }
        }
    } else {
        chunks = texts;
    }

    if (chunks.empty()) {
        return {};
    }

    if (num_threads <= 1 || chunks.size() <= 1) {
        ThreadFrequencyBuffer buffer{};
        count_frequencies_in_chunks(chunks, 0, chunks.size(), buffer);

        std::unordered_map<char, int> result;
        for (int i = 0; i < 256; ++i) {
            if (buffer.counts[i] > 0) {
                result[static_cast<char>(i)] = buffer.counts[i];
            }
        }
        return result;
    }

    std::size_t actual_threads = std::min(num_threads, chunks.size());
    std::vector<ThreadFrequencyBuffer> buffers(actual_threads);
    std::vector<std::thread> workers;
    workers.reserve(actual_threads - 1);

    std::size_t chunks_per_thread = chunks.size() / actual_threads;
    std::size_t remainder = chunks.size() % actual_threads;

    std::size_t current_start = 0;
    for (std::size_t t = 0; t < actual_threads; ++t) {
        std::size_t count = chunks_per_thread + (t < remainder ? 1 : 0);
        std::size_t current_end = current_start + count;

        if (t == actual_threads - 1) {
            // Main thread processes the last chunk range directly
            count_frequencies_in_chunks(chunks, current_start, current_end, buffers[t]);
        } else {
            workers.emplace_back(
                count_frequencies_in_chunks,
                std::cref(chunks),
                current_start,
                current_end,
                std::ref(buffers[t])
            );
        }
        current_start = current_end;
    }

    for (auto& worker : workers) {
        if (worker.joinable()) {
            worker.join();
        }
    }

    // Merge thread buffers into result map
    std::unordered_map<char, int> result;
    for (int i = 0; i < 256; ++i) {
        int total = 0;
        for (std::size_t t = 0; t < actual_threads; ++t) {
            total += buffers[t].counts[i];
        }
        if (total > 0) {
            result[static_cast<char>(i)] = total;
        }
    }

    return result;
}

}  // namespace parallel_letter_frequency

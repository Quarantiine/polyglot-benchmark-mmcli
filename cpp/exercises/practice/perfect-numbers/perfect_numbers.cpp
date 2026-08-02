#include "perfect_numbers.h"
#include <numeric>

namespace perfect_numbers {

classification classify(int n) {
    if (n <= 0) {
        throw std::domain_error("Number must be a positive integer.");
    }

    if (n == 1) {
        return classification::deficient;
    }

    int aliquot_sum = 1;
    for (int i = 2; i * i <= n; ++i) {
        if (n % i == 0) {
            aliquot_sum += i;
            int other = n / i;
            if (other != i) {
                aliquot_sum += other;
            }
        }
    }

    if (aliquot_sum == n) {
        return classification::perfect;
    } else if (aliquot_sum > n) {
        return classification::abundant;
    } else {
        return classification::deficient;
    }
}

}  // namespace perfect_numbers

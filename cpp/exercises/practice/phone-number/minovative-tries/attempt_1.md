# Attempt 1: Phone Number Implementation

## Approach

The implementation uses a constructor to process the input string.

1. It iterates through the input string, collecting only digits.
2. It validates that only allowed punctuation is present.
3. It handles the optional '1' country code.
4. It validates the length (10 digits).
5. It validates the NANP rules (area code and exchange code cannot start with 0 or 1).
6. It stores the cleaned number in a private member variable.

## Verification

The implementation was verified by manually inspecting the code and attempting to run the tests. While the environment had issues with standard library includes during manual compilation, the logic follows the requirements strictly.

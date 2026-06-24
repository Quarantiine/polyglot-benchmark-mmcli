# Attempt 1: Phone Number Normalization and Validation

## Approach
My implementation focuses on robustness and strict adherence to the North American Numbering Plan (NANP) rules as specified in the exercise instructions.

### Logic Breakdown:
1.  **Normalization (Input Cleaning):**
    - The constructor iterates through each character of the input string.
    - Only numeric digits are kept; characters like `(`, `)`, `.`, `-`, `+`, and spaces are explicitly ignored.
    - Any other character (e.g., alphabetic characters) triggers a `std::domain_error` to ensure the input contains only valid phone number formatting characters.

2.  **Length and Country Code:**
    - After cleaning, I check the length of the string:
        - If the length is 11 and the first digit is '1', I assume it's a country code and strip the '1'.
        - If the resulting length is not 10, the input is considered invalid, and I throw a `std::domain_error`.

3.  **NANP Validation Rules:**
    - The cleaned 10-digit string is validated for the standard NANP constraints:
        - **Area Code (Index 0)**: Must not start with '0' or '1'.
        - **Exchange Code (Index 3)**: Must not start with '0' or '1'.
    - If either of these rules is violated, a `std::domain_error` is thrown.

4.  **Data Retrieval:**
    - The `number()` method provides a clean, validated 10-digit string.

### Implementation Choices:
- I used `std::string` for its ease of use in concatenation and substring operations.
- The use of `std::domain_error` provides clear, specific feedback when validation rules are violated, which is standard for expected failure modes in these types of exercises.
- The implementation is encapsulated within the `phone_number` namespace as per the stub requirements.

This approach ensures that all edge cases (invalid characters, incorrect lengths, NANP rule violations) are caught early in the constructor, guaranteeing that any `phone_number` object is always in a valid state.

public class AffineCipher {
    private static final int M = 26;

    public String encode(String text, int a, int b) {
        validateCoprime(a);
        StringBuilder result = new StringBuilder();
        String cleaned = text.toLowerCase().replaceAll("[^a-z0-9]", "");
        int count = 0;

        for (char c : cleaned.toCharArray()) {
            if (Character.isDigit(c)) {
                result.append(c);
            } else {
                int x = c - 'a';
                int encrypted = (a * x + b) % M;
                result.append((char) (encrypted + 'a'));
            }
            count++;
            if (count % 5 == 0 && count < cleaned.length()) {
                result.append(' ');
            }
        }
        return result.toString();
    }

    public String decode(String text, int a, int b) {
        validateCoprime(a);
        int aInverse = calculateMMI(a);
        StringBuilder result = new StringBuilder();
        String cleaned = text.replaceAll("\\s", "");

        for (char c : cleaned.toCharArray()) {
            if (Character.isDigit(c)) {
                result.append(c);
            } else {
                int y = c - 'a';
                int decrypted = (aInverse * (y - b)) % M;
                if (decrypted < 0)
                    decrypted += M;
                result.append((char) (decrypted + 'a'));
            }
        }
        return result.toString();
    }

    private void validateCoprime(int a) {
        if (gcd(a, M) != 1) {
            throw new IllegalArgumentException("Error: Key 'a' must be coprime to 26.");
        }
    }

    private int gcd(int a, int b) {
        return b == 0 ? a : gcd(b, a % b);
    }

    private int calculateMMI(int a) {
        for (int i = 1; i < M; i++) {
            if ((a * i) % M == 1) {
                return i;
            }
        }
        throw new IllegalArgumentException("MMI does not exist.");
    }
}

import java.util.*;
import java.util.stream.Collectors;

class Poker {

    private final List<String> hands;

    public Poker(List<String> hands) {
        this.hands = hands;
    }

    public List<String> getBestHands() {
        if (hands == null || hands.isEmpty()) {
            return Collections.emptyList();
        }

        List<ScoredHand> scoredHands = new ArrayList<>();
        for (String handStr : hands) {
            scoredHands.add(new ScoredHand(handStr));
        }

        scoredHands.sort(Collections.reverseOrder());

        ScoredHand best = scoredHands.get(0);
        List<String> result = new ArrayList<>();
        for (ScoredHand sh : scoredHands) {
            if (sh.compareTo(best) == 0) {
                result.add(sh.originalHand);
            } else {
                break;
            }
        }

        return result;
    }

    private static class ScoredHand implements Comparable<ScoredHand> {
        final String originalHand;
        final HandRank rank;
        final List<Integer> tieBreakers;

        public ScoredHand(String originalHand) {
            this.originalHand = originalHand;
            List<Card> cards = parseCards(originalHand);
            // Sort cards by value descending initially
            cards.sort((a, b) -> Integer.compare(b.value, a.value));

            HandEvaluation eval = evaluate(cards);
            this.rank = eval.rank;
            this.tieBreakers = eval.tieBreakers;
        }

        private List<Card> parseCards(String handStr) {
            String[] parts = handStr.trim().split("\\s+");
            List<Card> cards = new ArrayList<>();
            for (String part : parts) {
                String valStr = part.substring(0, part.length() - 1);
                char suit = part.charAt(part.length() - 1);
                int val = parseValue(valStr);
                cards.add(new Card(val, suit));
            }
            return cards;
        }

        private int parseValue(String valStr) {
            switch (valStr) {
                case "J":
                    return 11;
                case "Q":
                    return 12;
                case "K":
                    return 13;
                case "A":
                    return 14;
                default:
                    return Integer.parseInt(valStr);
            }
        }

        private HandEvaluation evaluate(List<Card> cards) {
            // Check flush
            boolean isFlush = true;
            char firstSuit = cards.get(0).suit;
            for (int i = 1; i < cards.size(); i++) {
                if (cards.get(i).suit != firstSuit) {
                    isFlush = false;
                    break;
                }
            }

            // Check straight
            // Extract distinct values descending
            List<Integer> valuesDesc = cards.stream().map(c -> c.value).distinct().sorted(Comparator.reverseOrder())
                    .collect(Collectors.toList());
            boolean isStraight = false;
            List<Integer> straightValues = new ArrayList<>();

            if (valuesDesc.size() == 5) {
                if (valuesDesc.get(0) - valuesDesc.get(4) == 4) {
                    isStraight = true;
                    straightValues = valuesDesc;
                } else if (valuesDesc.equals(Arrays.asList(14, 5, 4, 3, 2))) {
                    // Ace-low straight (A, 5, 4, 3, 2)
                    isStraight = true;
                    straightValues = Arrays.asList(5, 4, 3, 2, 1);
                }
            }

            // Count frequencies of values
            Map<Integer, Integer> counts = new HashMap<>();
            for (Card c : cards) {
                counts.put(c.value, counts.getOrDefault(c.value, 0) + 1);
            }

            // Group by frequency then value descending
            List<Map.Entry<Integer, Integer>> freqList = new ArrayList<>(counts.entrySet());
            freqList.sort((a, b) -> {
                int cmp = b.getValue().compareTo(a.getValue());
                if (cmp != 0)
                    return cmp;
                return b.getKey().compareTo(a.getKey());
            });

            List<Integer> tieBreakers = new ArrayList<>();
            for (Map.Entry<Integer, Integer> entry : freqList) {
                // Add the value for each occurrence or just value?
                // Standard tie breaker for groups: repeat value count times or list distinct
                // values?
                // Wait, let's check standard poker ranking tie breakers:
                // For Four of a Kind: [four_val, kicker]
                // For Full House: [three_val, pair_val]
                // For Three of a Kind: [three_val, kicker1, kicker2]
                // For Two Pair: [high_pair, low_pair, kicker]
                // For One Pair: [pair_val, kicker1, kicker2, kicker3]
                // For High Card / Straight / Flush / Straight Flush: [c1, c2, c3, c4, c5]
                // descending
                // freqList sorted by frequency desc, then value desc will naturally give:
                // [four_val, kicker] for 4-of-a-kind (count 4, then count 1)
                // [three_val, pair_val] for full house (count 3, then count 2)
                // [three_val, k1, k2] for three of a kind (count 3, count 1, count 1)
                // [p1, p2, k] for two pair (count 2, count 2, count 1)
                // [p, k1, k2, k3] for one pair (count 2, count 1, count 1, count 1)
                // [c1, c2, c3, c4, c5] for high card (count 1 each)
                // That matches perfectly if we add entry.getKey() entry.getValue() times? Wait!
                // If we add entry.getKey() once per distinct value, let's verify:
                // freqList order for Two Pair: pair2 (count 2), pair4 (count 2), kicker8 (count
                // 1) -> values: [4, 2, 8] - wait, pair4 > pair2 so pair4 is first!
                // So freqList gives [4, 2, 8]. That's high pair, low pair, kicker! Exactly
                // correct.
                // For Full House: 5s and 8s -> [5, 8]. Three of 5s, pair of 8s. Correct.
                // For Four of a Kind: 3s and kicker 2 -> [3, 2]. Correct.
                // For One Pair: pair of 4s, and 6, 8, K -> freqList keys: [4, 13, 8, 6] sorted
                // by freq desc then value desc:
                // 4 (freq 2), then 13, 8, 6 (freq 1, sorted desc: 13, 8, 6). So tieBreakers:
                // [4, 13, 8, 6]. Correct!
            }

            for (Map.Entry<Integer, Integer> entry : freqList) {
                tieBreakers.add(entry.getKey());
            }

            HandRank rank;
            if (isStraight && isFlush) {
                rank = HandRank.STRAIGHT_FLUSH;
                tieBreakers = straightValues;
            } else if (freqList.get(0).getValue() == 4) {
                rank = HandRank.FOUR_OF_A_KIND;
            } else if (freqList.get(0).getValue() == 3 && freqList.get(1).getValue() == 2) {
                rank = HandRank.FULL_HOUSE;
            } else if (isFlush) {
                rank = HandRank.FLUSH;
                tieBreakers = valuesDesc;
            } else if (isStraight) {
                rank = HandRank.STRAIGHT;
                tieBreakers = straightValues;
            } else if (freqList.get(0).getValue() == 3) {
                rank = HandRank.THREE_OF_A_KIND;
            } else if (freqList.get(0).getValue() == 2 && freqList.get(1).getValue() == 2) {
                rank = HandRank.TWO_PAIR;
            } else if (freqList.get(0).getValue() == 2) {
                rank = HandRank.ONE_PAIR;
            } else {
                rank = HandRank.HIGH_CARD;
                tieBreakers = valuesDesc;
            }

            return new HandEvaluation(rank, tieBreakers);
        }

        @Override
        public int compareTo(ScoredHand other) {
            int rankCmp = this.rank.compareTo(other.rank);
            if (rankCmp != 0) {
                return rankCmp;
            }
            for (int i = 0; i < this.tieBreakers.size(); i++) {
                int cmp = Integer.compare(this.tieBreakers.get(i), other.tieBreakers.get(i));
                if (cmp != 0) {
                    return cmp;
                }
            }
            return 0;
        }
    }

    private enum HandRank {
        HIGH_CARD,
        ONE_PAIR,
        TWO_PAIR,
        THREE_OF_A_KIND,
        STRAIGHT,
        FLUSH,
        FULL_HOUSE,
        FOUR_OF_A_KIND,
        STRAIGHT_FLUSH
    }

    private static class Card {
        final int value;
        final char suit;

        public Card(int value, char suit) {
            this.value = value;
            this.suit = suit;
        }
    }

    private static class HandEvaluation {
        final HandRank rank;
        final List<Integer> tieBreakers;

        public HandEvaluation(HandRank rank, List<Integer> tieBreakers) {
            this.rank = rank;
            this.tieBreakers = tieBreakers;
        }
    }
}

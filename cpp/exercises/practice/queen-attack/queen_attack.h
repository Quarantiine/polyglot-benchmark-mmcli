#if !defined(QUEEN_ATTACK_H)
#define QUEEN_ATTACK_H

#include <utility>
#include <stdexcept>

namespace queen_attack {

class chess_board {
public:
    using position = std::pair<int, int>;

    chess_board(position white, position black) 
        : white_pos(white), black_pos(black) {
        if (white.first < 0 || white.first >= 8 || white.second < 0 || white.second >= 8 ||
            black.first < 0 || black.first >= 8 || black.second < 0 || black.second >= 8) {
            throw std::domain_error("Queen positions must be on the board (0-7).");
        }
        if (white.first == black.first && white.second == black.second) {
            throw std::domain_error("Queen positions must be distinct.");
        }
    }

    position white() const {
        return white_pos;
    }

    position black() const {
        return black_pos;
    }

    bool can_attack() const {
        if (white_pos.first == black_pos.first || white_pos.second == black_pos.second) {
            return true;
        }
        if (std::abs(white_pos.first - black_pos.first) == std::abs(white_pos.second - black_pos.second)) {
            return true;
        }
        return false;
    }

private:
    position white_pos;
    position black_pos;
};

}  // namespace queen_attack

#endif // QUEEN_ATTACK_H

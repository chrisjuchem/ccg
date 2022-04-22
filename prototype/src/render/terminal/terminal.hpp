#pragma once

#include <vector>
#include "../../engine/card.hpp"


class TerminalRenderer {
public:
    TerminalRenderer();
    ~TerminalRenderer();

    void render(std::vector<Card*> cards);
};


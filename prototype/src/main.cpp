#include <iostream>
#include <vector>
#include <memory>
#include "engine/card.hpp"
#include "engine/world.hpp"
#include "render/terminal/terminal.hpp"

int main(int argc, char** argv) {

    World* wrld = new World();
    wrld->board.push_back(std::make_shared<Creature>(3,5));
    wrld->board.push_back(std::make_shared<Creature>(1,1));
    wrld->hand.push_back(std::make_shared<Creature>(0,6));

    auto renderer = std::make_unique<TerminalRenderer>();
    renderer->render(wrld);

    renderer->waitForAnyInput();

    wrld->playCard(0);
    // c2->damage(1);

    renderer->render(wrld);

    return 0;
}

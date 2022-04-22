#include <iostream>
#include <vector>
#include <memory>
#include "engine/card.hpp"
#include "render/terminal/terminal.hpp"

int main(int argc, char** argv) {

    Card* c = new Card("lmnop");
    Card* c2 = new Card("qrstuv");
    std::vector<Card*> cs{c, c2};

    auto renderer = std::make_unique<TerminalRenderer>();
    renderer->render(cs);


    return 0;
}

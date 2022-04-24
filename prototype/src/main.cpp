#include <iostream>
#include <vector>
#include <memory>

#include "engine/ecs/systems/terminalRender.hpp"
#include "engine/ecs/entity.hpp"
#include "engine/ecs/component.hpp"
#include "engine/ecs/components/gameZone.hpp"

int main(int argc, char** argv) {

    TerminalRenderingSystem rs;

    EntityID e1 = newEntity();
    EntityID e2 = newEntity();
    EntityID e3 = newEntity();

    addComponent(e1, std::make_shared<GameZoneComponent>(Zone::HAND));
    addComponent(e2, std::make_shared<GameZoneComponent>(Zone::BATTLEFIELD));
    addComponent(e3, std::make_shared<GameZoneComponent>(Zone::DECK));

    rs.execute();

    return 0;
}

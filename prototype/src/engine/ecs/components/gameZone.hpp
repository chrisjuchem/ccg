#pragma once

#include "../component.hpp"

enum Zone {
    BATTLEFIELD =   0b0'0000'0001,
    GRAVEYARD =     0b0'0000'0010,
    HAND =          0b0'0000'0100,
    DECK =          0b0'0000'1000,
    STACK =         0b0'0001'0000,
};

class GameZoneComponent : public Component {
public:
    Zone zone;
    GameZoneComponent(Zone zone);
};

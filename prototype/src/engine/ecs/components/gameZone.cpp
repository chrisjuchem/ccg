#include "gameZone.hpp"

GameZoneComponent::GameZoneComponent(Zone zone) : Component(ComponentType::GameZone), zone{zone} {}
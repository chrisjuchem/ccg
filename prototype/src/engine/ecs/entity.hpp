#pragma once

#include "ecs.hpp"
#include "component.hpp"

EntityID newEntity();

void addComponent(EntityID entityId, std::shared_ptr<Component> component, bool replace = false);
void removeComponent(EntityID entityId, ComponentType type);

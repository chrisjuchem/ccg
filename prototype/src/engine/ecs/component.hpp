#pragma once

#include <unordered_map>
#include <vector>
#include <memory>
#include "ecs.hpp"
#include "entity.hpp"


class Component {
public:
    EntityID entityId; // TODO can we make this const (maybe move addComponent into constructor)
    const ComponentType type;
protected:
    Component(ComponentType type);
};


template <typename T>
using ComponentMap = std::unordered_map<ComponentType, T>;

//TODO: replace vector with custom inline arrays for perf, handle dup/realloc
extern ComponentMap<std::vector<std::shared_ptr<Component>>> componentsByType;
extern std::unordered_map<EntityID, ComponentMap<std::shared_ptr<Component>>> componentsByEntity;

#include "component.hpp"

Component::Component(ComponentType type) : type{type}, entityId{0} {}

//TODO: replace vector with custom inline arrays for perf, handle dup/realloc
ComponentMap<std::vector<std::shared_ptr<Component>>> componentsByType;
std::unordered_map<EntityID, ComponentMap<std::shared_ptr<Component>>> componentsByEntity;

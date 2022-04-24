#include <stdexcept>

#include "entity.hpp"

EntityID _nextID = 1;

EntityID newEntity() {
    componentsByEntity[_nextID]; // create blank entry in map
    return _nextID++;            // return, then incr
}


void addComponent(EntityID entityId, std::shared_ptr<Component> component, bool replace) {
    // TODO [newEntity] encue entties in some system to be created end of frame 
    //      [here]      handle new entities not yet in map (end of systems loop)

    ComponentType type = component->type;
    if (componentsByEntity.at(entityId).contains(type)) {
        if (replace) {
            removeComponent(entityId, type);
        } else {
            throw std::runtime_error("Component already exists on entity");
        }
    }


    //TODO check entity not already attached to a component
    component->entityId = entityId;
    componentsByEntity.at(entityId)[type] = component;
    componentsByType[type].push_back(component);
}


void removeComponent(EntityID entityId, ComponentType type) {
    componentsByEntity.at(entityId).erase(type);
    // TODO raise not found
    auto compList = componentsByType.at(type);
    for (auto it = compList.begin(); it != compList.end(); ++it) {
        if ((*it)->entityId) {
            compList.erase(it);
            return;
        }
    } 
    // TODO raise not found
}

#pragma once

#include "../system.hpp"
#include "../component.hpp"

class TerminalRenderingSystem : public System {
public:
    TerminalRenderingSystem();
    virtual ~TerminalRenderingSystem();
    virtual void execute();
};

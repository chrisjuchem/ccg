#pragma once

#include <vector>

class System {
public:
    virtual void execute() = 0;
    virtual ~System();
};

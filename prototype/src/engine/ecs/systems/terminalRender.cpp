#include <ncurses.h>

#include "terminalRender.hpp"
#include "../components/gameZone.hpp"


bool initilized = false;

TerminalRenderingSystem::TerminalRenderingSystem() {
    initscr();         /* Start curses mode */
    refresh();
}

TerminalRenderingSystem::~TerminalRenderingSystem() {
    endwin();          /* Start curses mode */
}


void TerminalRenderingSystem::execute() {
    auto cmps = componentsByType[ComponentType::GameZone];

    int xstart = 3;
    int ystart = 1;
    int width = 22;
    int height = 16;
    int separation = 3; 

    for (auto c : cmps) {
        auto zoneCmp = std::static_pointer_cast<GameZoneComponent>(c);
        
        WINDOW* win = newwin(height, width, ystart, xstart);
        WINDOW* inner = newwin(height-2, width-2, ystart+1, xstart+1); // TODO stop leaking this memory
        box(win, 0, 0);
        mvwprintw(inner, 0, 0, "id = ");
        wprintw(inner, std::to_string(zoneCmp->entityId).c_str());
        mvwprintw(inner, 1, 0, "zone = ");
        wprintw(inner, std::to_string(zoneCmp->zone).c_str());
        wrefresh(win);
        wrefresh(inner);

        xstart += width + separation;
    }
    ystart += height + separation;
    xstart = 3;
    


    getch();            /* Wait for user input */
}

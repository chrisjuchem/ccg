#include "terminal.hpp"
#include <ncurses.h>

bool initilized = false;

TerminalRenderer::TerminalRenderer() {
    initscr();         /* Start curses mode */
    refresh();
}

TerminalRenderer::~TerminalRenderer() {
    getch();            /* Wait for user input */
    endwin();           /* Start curses mode */
}

void TerminalRenderer::render(std::vector<Card*> cards) {

    int xstart = 3;
    int ystart = 1;
    int width = 22;
    int height = 16;
    int separation = 3; 

    for (Card* c : cards) {
        WINDOW* win = newwin(height, width, ystart, xstart);
        WINDOW* inner = newwin(height-2, width-2, ystart+1, xstart+1);
        box(win, 0, 0);
        wprintw(inner, c->text.c_str());
        wrefresh(win);
        wrefresh(inner);

        xstart += width + separation;
    }
}
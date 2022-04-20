#include <iostream>
#include <ncurses.h>


int main(int argc, char** argv) {
    initscr();         /* Start curses mode */
    refresh();

    WINDOW* win = newwin(10, 10, 0, 0);
    WINDOW* inner = newwin(8, 8, 1, 1);
    box(win, 0, 0);
    wprintw(inner, "Hello World !!!");
    wrefresh(win);
    wrefresh(inner);

    getch();            /* Wait for user input */
    endwin();           /* Start curses mode */


    std::cout << "Lorem Ipsum!" << std::endl; 
    return 0;
}

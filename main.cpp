using namespace std;

#include <string>

#include "lexer.h"
#include "parser.h"
#include "interpreter.h"

int main(int argc, char* argv[]) {

    // Lexer
    lexer L(argv[1]);
    L.print();

    //Logic!

    L.close();

    return 0;
}
using namespace std;

#include <string>
#include <vector>

#include "lexer.h"
// #include "tag.h"
#include "tag.h"
#include "parser.h"
// #include "interpreter.h"

int main(int argc, char* argv[]) {

    // Lexer
    lexer L(argv[1]);
    // L.print();

    parser P(&L);
    P.go();
    P.printTree();

    //Logic!

    L.close();

    return 0;
}
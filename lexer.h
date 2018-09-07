#include <fstream>
#include <iostream>

class lexer {

    ifstream reader;

    public:

    lexer(char* filename) {
        reader.open (filename);
    }

    void print() {
        cout << "printing!" << reader.good();
        while (reader.good()) {
            char c = reader.get();
            cout << c;
        }
    }

    void close() {
        reader.close();
    }

};
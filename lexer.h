#include <fstream>
#include <iostream>

class lexer {

    ifstream reader;
    string symbols = ".#^/|(){}\"";
    string currentToken = "";
    char lastChar = '\0';

    void nextToken() {
        currentToken = "";
        char c;
        bool done = false;
        while (reader.good() && !done) {
            c = reader.get();
            if (isWhitespace(c)) {
                if (currentToken == "") {
                    // Space before word
                    continue;
                } else {
                    // End of word
                    done = true;
                }
            } else if (isSymbol(c)) {
                if (currentToken == "") {
                    // Found a single symbol
                    currentToken += c;
                } else {
                    // Scanned too far
                    reader.putback (c);
                }
                done = true;
            } else {
                // Must be start of word
                currentToken += c;
            }
        }
        // final token is newline / empty
    }

    // Returns true if c in the symbol string
    bool isSymbol(char c) {
        for (int i = 0; i < symbols.length(); i++) {
            if (c == symbols[i]) {
                return true;
            }
        }
        return false;
    }

    // Returns true if c is a space, tab, or newline
    bool isWhitespace(char c) {
        if (c == ' ' || c == '\t' || c == '\n') {
            return true;
        }
        return false;
    }

    public:

    static const char DOT = '.';
    static const char HASH = '#';
    static const char CARET = '^';
    static const char SLASH = '/';
    static const char PIPE = '|';
    static const char LPAREN = '(';
    static const char RPAREN = ')';
    static const char LCURLY = '{';
    static const char RCURLY = '}';
    static const char DQUOTE = '"';

    // Constructor
    lexer(string filename) {
        reader.open (filename);
    }

    // Returns the next token
    string getNext() {
        nextToken();
        return currentToken;
    }

    bool atEnd() {
        return reader.eof();
    }

    // doesnt work
    void eat(char symbol) {
        if (symbol == currentToken[0]) {
            nextToken();
        } else {
            cout << "ERROR" << endl
                 << "Expected " << symbol << endl
                 << "Found " << currentToken << endl;
        }
    }

    void print() {
        while (reader.good()) {
            nextToken();
            cout << currentToken << endl;
        }
    }

    // Closes ifstream
    void close() {
        reader.close();
    }

};
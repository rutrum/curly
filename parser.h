class parser {

    lexer* reader;

    void parseClass() {

    }

    void parseId() {

    }

    void parseAttribute() {

    }

    void parseStyle() {

    }

    void parseString() {

    }

    void addTag() {
        
    }

    public:

    tag* head;

    parser(lexer* reader) {
        this->reader = reader;
    }

    // Returns the head of the tag tree
    void go() {

        head = new tag("html");
        tag* current = head;

        while (!reader->atEnd()) {

            string token = reader->getNext();

            if (token.length() > 1) {
                // Must be a tag name
                tag* child = new tag(token);
            } else {
                // Must be a symbol
                if (token[0] == lexer::DOT) {
                    parseClass();
                } else if (token[0] == lexer::HASH) {
                    parseId();
                } else if (token[0] == lexer::PIPE) {
                    parseAttribute();
                } else if (token[0] == lexer::CARET) {
                    parseStyle();
                } else if (token[0] == lexer::DQUOTE) {
                    parseString();
                } else if (token[0] == lexer::SLASH) {
                    
                } else if (token[0] == lexer::LCURLY) {

                }
            }
            
        }
    }

    


};
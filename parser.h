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

    public:

    tag* head;

    parser(lexer* reader) {
        this->reader = reader;
    }

    // Returns the head of the tag tree
    void go() {

        head = new tag(NULL, "html");
        tag* parent = head;
        tag* child = NULL;

        while (!reader->atEnd()) {

            string token = reader->getNext();

            switch(token[0]) {
                case '{':
                    // Tag declaration done
                    // Focus on child
                    parent->addChild(child);
                    parent = child;
                    child = NULL;
                    break;

                case '}':
                    // Tag done, go to parent
                    parent = parent->parent;
                    child = NULL;
                    break;

                case '#': {
                    string idName = reader->getNext();
                    child->setId(idName);
                    break;
                }

                case '.': {
                    string className = reader->getNext();
                    child->addClass(className);
                    break;
                }

                case '^': {
                    string property = reader->getNext();
                    cout << property << endl;
                    reader->getNext();
                    string value = reader->getNext();
                    string next = reader->getNext();
                    while (next[0] != ')') {
                        value += " " + next;
                        next = reader->getNext();
                    }
                    child->addStyle(property, value);
                    break;
                }

                default:
                    // Must be a tag name
                
                    // Create child
                    child = new tag(parent, token);

                    break;
            }
            
        }
    }

    void printTree() {
        head->print();
    }


};
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

            cout << parent->name << " is current" << endl;

            string token = reader->getNext();
            cout << token[0] << endl;

            switch(token[0]) {
                case '{':
                    // Tag declaration done
                    // Focus on child
                    parent->addChild(child);
                    cout << "Adding child " << child->name << " to " << parent->name << endl;
                    parent = child;
                    child = NULL;
                    break;
                case '}':
                    // Tag done, go to parent
                    parent = parent->parent;
                    cout << "new parent " << parent->name << endl;
                    child = NULL;
                    break;
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
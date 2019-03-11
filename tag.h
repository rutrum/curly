struct tag {

    // Properties
    
    string name;

    string id;
    vector<string> classes;

    tag* parent;
    tag* firstChild;
    tag* nextSibling;

    // Constructors

    tag(tag* parent, string name) {
        
        this->name = name;

        this->id = "";
        this->classes = {};

        this->parent = parent;
        this->firstChild = NULL;
        this->nextSibling = NULL;
    }

    // Parser methods

    void setId(string id) {
        this->id = id;
    }

    void addClass(string classname) {
        this->classes.push_back(classname);
    }

    void addChild(tag* child) {
        if (this->firstChild) {
            // First child exists
            tag* current = child;
            while (current->hasNextSibling()) {
                current = current->nextSibling;
            }
            current->nextSibling = child;
        } else {
            // No children already
            this->firstChild = child;
        }
    }

    bool hasNextSibling() {
        if (this->nextSibling) {
            return false;
        }
        return true;
    }

    // Interpreter methods

};
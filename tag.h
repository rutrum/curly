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
        if (this->firstChild != NULL) {
            // First child exists
            tag* current = firstChild;
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
        return nextSibling != NULL;
    }

    // Interpreter methods

    void print() {
        this->print(0);
    }

    void print(int d) {
        string space = "";
        for (int i = 0; i < d; i++) {
            space += "    ";
        }
        cout << space << this->startTag() << endl;
        if (this->firstChild != NULL)
            this->firstChild->print(d+1);
        cout << space << this->endTag() << endl;
        if (hasNextSibling())
            this->nextSibling->print(d);
    }

    string getId() {
        if (id == "") {
            return "";
        } else {
            return " id=\"" + id + "\"";
        }
    }

    string startTag() {
        return "<" + this->name + getId() + ">";
    }

    string endTag() {
        return "</" + this->name + ">";
    }

};
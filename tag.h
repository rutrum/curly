struct tag {

    // Properties
    
    string name;

    string id;
    vector<string> classes;
    vector<string> styleProps;
    vector<string> styleValues;

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

    void addStyle(string property, string value) {
        cout << property << value << endl;
        styleProps.push_back(property);
        styleValues.push_back(value);
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

    string getClasses() {
        if (classes.size() == 0) {
            return "";
        } else {
            string classStr = " class=\"";
            for (int i = 0; i < classes.size() - 1; i++) {
                classStr += classes[i] + " ";
            }
            classStr += classes[classes.size() - 1];
            return classStr + "\"";
        }
    }

    string getStyle() {
        if (styleProps.size() == 0) {
            return "";
        } else {
            string styleStr = " style=\"";
            for (int i = 0; i < styleProps.size() - 1; i++) {
                styleStr += styleProps[i] + ":"
                        + styleValues[i] + "; ";
            }
            styleStr += styleProps[styleProps.size() - 1] + ":"
                    + styleValues[styleProps.size() - 1];
            return styleStr + "\"";
        }
    }

    string startTag() {
        return "<" + this->name + getId() 
                + getClasses() 
                + getStyle() + ">";
    }

    string endTag() {
        return "</" + this->name + ">";
    }

};
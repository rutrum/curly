class tag {

    // Properties
    
    string name;

    string id;
    vector<string> classes;

    tag* parent;
    tag* firstChild;
    tag* nextSibling;

public:

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

    void setFirstChild(tag* child) {
        this->firstChild = child;
    }

    void setNextSibling(tag* sibling) {
        this->nextSibling = nextSibling;
    }

    // Interpreter methods

};
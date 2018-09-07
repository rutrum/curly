class tag {

    struct tuple {
        string name;
        string value;
        tuple(string name, string value) {
            this->name = name;
            this->value = value;
        }
    };

    string tagName;
    vector<string> ids;
    vector<string> classes;
    vector<tuple> attributes;
    vector<tuple> style;

    bool isString;
    string strVal;

    tag* firstChild = nullptr;
    tag* nextSibling = nullptr;

    public:

    tag(string content, bool isString) {
        this->isString = isString;
        if (isString) {
            strVal = content;
        } else {
            tagName = content;
        }
    }

    void addId(string id) {
        ids.push_back(id);
    }
    
    void addClass(string clas) {
        classes.push_back(clas);
    }

    void addAttribute(string name, string value) {
        attributes.push_back(tuple(name, value));
    }

    void addStyle(string name, string value) {
        style.push_back(tuple(name, value));
    }

    void setChild(tag* t) {
        firstChild = t;
    }

    void setSibling(tag* t) {
        nextSibling = t;
    }

    // Functions below are to be added to the interpreter

    void getStartTag() {

    }

    string getEndTag() {
        return "<" + tagName + ">";
    }

};
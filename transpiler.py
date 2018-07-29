from read import Read
from tag import Tag

file = 0
output = 0
depth = []
currentTag = 0

# if a tag name is seen, but a { has not yet been seen, then this value is false
expected = True

def initialize(filename):
    global file
    global output

    file = Read(filename)
    output = open(filename + ".html", 'w')

def compile_next():
    global file
    global output
    global depth
    global currentTag
    global expected

    token = file.next_token()

    if token == "{":
        write_tag()
        expected = True
    elif token == "}":
        end_tag()
    elif token == ".":
        currentTag.add_class(file.next_token())
    elif token == "#":
        currentTag.add_id(file.next_token())
    elif token == "^":
        compile_style()
    elif token == "|":
        compile_attribute()
    elif token == "\"":
        if not expected:
            currentTag.is_implicit = True
            write_tag()
            compile_string()
            end_tag()
            expected = True
        else:
            compile_string()
    elif token == "":
        # End of file
        return ""
    else:
        # Must be a tag name
        if not expected:
            currentTag.is_implicit = True
            write_tag()
        expected = False
        currentTag = Tag(token)


def compile_style():
    global file
    global currentTag

    name = file.next_token()
    file.next_token() # Expect (
    value = file.next_token()
    file.next_token() # Expect )

    currentTag.add_style(name, value)

def compile_attribute():
    global file
    global currentTag

    name = file.next_token()
    maybe = file.next_token()
    if (maybe == "("):
        value = file.next_token()
        currentTag.add_attribute(name, value)
        file.next_token() # Expect )
    else:
        currentTag.add_attribute(name, "")
        file.undo_token()

def compile_string():
    global file
    global output
    
    output.write(get_tabs() + file.next_token() + "\n")
    file.next_token() # Expect "

def write_tag():
    global currentTag
    global depth
    global output

    depth.append(currentTag)
    output.write(get_tabs() + currentTag.get_start_tag() + "\n")
    inc_tabs()

def end_tag():
    global depth
    dec_tabs()
    output.write(get_tabs() + depth.pop(-1).get_end_tag() + "\n")
    if len(depth) > 1:
        if depth[-1].is_implicit:
            end_tag()

# --- Controls the tabing in html output

tabs = 0

def get_tabs():
    global tabs
    t = ""
    for x in range(tabs):
        t += "    "
    return t

def inc_tabs():
    global tabs
    tabs += 1

def dec_tabs():
    global tabs
    tabs -= 1

# Main

initialize("test")
while compile_next() != "":
    0

for x in range(len(depth)):
    print depth[x].name
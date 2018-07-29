class Read:

    specials = [".", "#", "^", "|", "(", ")", "\"", "{", "}", "$", "*", "_", "/"]

    def __init__(self, filename):
        self.filename = filename + ".rutwm"
        self.reader = open(self.filename, "r")
        self.line = self.reader.readline()
        self.p = 0
        self.use_last = False
        self.last = ""
        self.found_quotes = False

    def next_token(self):

        # If needed to rereturn value
        if self.use_last:
            self.use_last = False
            return self.last

        # If expected token is a string (or end of one)
        if self.last == "\"":
            if self.found_quotes:
                # Second quotes
                self.found_quotes = False
            else:
                # First quotes
                self.found_quotes = True
                self.last = ""
                return self.return_string("\"")

        # If expected token is a parameter
        if self.last == "(":
            self.last = ""
            return self.return_string(")")
        
        # If end of file, return empty string
        if self.line == "":
            self.reader.close()
            return ""

        # Initialize new last
        self.last = ""

        while self.line[self.p] != "\n":

            char = self.line[self.p]

            if self.is_special(char):
                if self.last == "":
                    # Only seen special
                    if char == "/" and self.line[self.p + 1] == "/": # should break when single / at endline
                        self.goto_next_line()
                        return self.next_token()
                    self.advance_pointer()
                    self.last = char
                    return self.last
                # Already read non-special characters
                return self.last

            elif char == " ":
                self.advance_pointer()
                # Ignores tabs/spaces at beginning of lines
                if self.last == "":
                    continue
                return self.last

            else:
                # Character is not special or space
                self.last += char
                self.advance_pointer()

        # If returning trailing spaces, try again with next line
        if self.last == "":
            self.goto_next_line()
            return self.next_token()

        # Reached end of line; return what was found
        return self.last

    def return_string(self, expected):
        char = self.line[self.p]
        while char != expected:
            self.last += char
            self.advance_pointer()
            char = self.line[self.p]
        return self.last

    # Is the character a special character?
    def is_special(self, char):
        for x in self.specials:
            if x == char:
                return True
        return False

    def undo_token(self):
        self.use_last = True

    def advance_pointer(self):
        self.p += 1
        if self.p >= len(self.line) - 2: # -1 ignores \n and some other shit
            self.goto_next_line()

    def goto_next_line(self):
        self.line = self.reader.readline()
        self.p = 0
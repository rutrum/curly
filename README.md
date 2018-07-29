# Rut Web Markup

This is a markup language developed as a way to abbreviate HTML without having to use a javascript framework.

## Syntax

### Tags

Tags no longer have start and end tags, nor do they use angle brackets.  To write a body tag, just write the tag name followed by open and closed curly braces, with your inner html inside.
```
body {
    ...
}
```

### Attributes

Writing equals and quotes is so tedious.  Function syntax is much more familiar.  To add any attribute to your tag, append `|attribute(value)` or just `|attribute`.
```
input |type(text) |focus {
    ...
}
```

### Classes and IDs

CSS did it right.  Why can't you do the same in HTML?  Just append `.classname` and `#idname` to the tagname.
```
div .heading #special {
    ...
}
```

### Styles

Styles are used so often, those too should get a shorthand.  Just append `^property(value)` to the tagname.
```
div ^color(red) {
    ...
}
```

### Strings

Because your plaintext is often more complicated than HTML interprets it, and is ambiguous with tagnames, just put your content in quotes.
```
span {
    "Hello World!"
}
```

### Implicit Bracing

Sometimes our code can get cluttered with braces for the simplest of html. If you have just a single child tag or string, you can ignore the braces altogether.
```
title "My Website"
```

### Whitespace

Whitespace _doesn't matter a lick_.  Put spaces, tabs, and linebreaks wherever you please to make your document fit your formatting preferences.

### Empty Tags

Often tags are self closed.  Instead of using curly braces, you can just end the tag using the slash.
```
br /
```

### Comments

You can comment just as you would in many languages by writing double forward slash.  This will comment out the remainder of the line.
```
// This is a comment
```

## An Example

Look at this HTML document.  It was created after translating the Rut Web Markup file below it.

```
<html>
    <head>
        <title>My website</title>
    </head>
    <body>
        <h1>Hey there!</h1>
        <h2>Thanks for visiting my website</h2>
        <p>
            This is my website, where I do 
            websity things
        </p>
        <h3 class="bold">My todo list</h3>
        <ul>
            <li>Add more stuff to website</li>
            <li>
                Tell 
                <span style="color:#1411BB;">friends</span>
                 about website
            </li>
        </ul>
        <a href="https://google.com">click for google</a>
        <h3>Talk to me!</h3>
        <p>Enter your name below and I'll know you visited my website!</p>
        <br></br>
        <label>Your name: </label>
        <input type="text"></input>
    </body>
</html>
```

```
html {
    head {
        title "My website"
    }
    // comment
    body {
        h1 "Hey there!"
        h2 "Thanks for visiting my website"
        p {
            "This is my website, where I do "
            "websity things"
        }
        h3 .bold "My todo list"
        ul {
            li "Add more stuff to website"
            li {
                "Tell " 
                span ^color(#1411BB) "friends" 
                " about website"
            }
        }
        a|href(https://google.com) "click for google"
        h3 "Talk to me!"
        p "Enter your name below and I'll know you visited my website!"
        br /
        label "Your name: "
        input |type(text) /
    }
}
```
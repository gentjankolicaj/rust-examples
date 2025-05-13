## Binding

Indirectly accessing a variable makes it impossible to branch and use that variable without re-binding.match provides
the @ sigil for binding values to names:

- You can also use binding to "destructure" enum variants, such as Option:
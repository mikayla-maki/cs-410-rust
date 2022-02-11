Mikayla Maki

HW 3: Keyword Index

Writeup:
Began by setting up the crate structure and writing the first test for extend_from_text.
Took a little bit to understand how to parse a keyword from the string but after that
it was mostly straight forward with filters and maps. I went from one function to the next
reading the doc comment test, getting it passing, then writing my own larger test.
The hardest part was figuring out if my dereferencing was causing a copy or not. I finally
settled on probably not for both the double dereference in count_matches() and the single 
dereference in nth_uppercase() as in both cases it should just be type juggling. But I'm 
unsure. Looking forward to finding out when this gets graded.
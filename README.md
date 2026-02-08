# Rust Polynomial Divider

I work in a math tutoring center. One of the things that tends to shock our students is that the long 
division algorithm learned in fourth grade is actually used again in higher mathematics. A subset of polynomial 
long division is synthetic division which only applies to terms with (x+/-h). Synthetic division is useful 
for finding the zeros of a polynomial. This project could ultimately be expanded to that end. 

I started by writing out the algorithm by hand and then translating it into pseudo code. I then wrote 
a basic python program to implement the algorithm. I have heard that rust is in increasing demand among embedded 
programmers. I decided I wanted to learn how to use it and began working on translating the program into Rust. 

## Instructions for Build and Use

Steps to build and/or run the software:

1. Download from git hub
2. Load into IDE of your choice (I use VS code)
3. Ensure you have rust and/or python installed
3. Compile using rustc and run the main.exe file (not currently using cargo as it is presently just one file)

Instructions for using the software:

1. Populate dividend and divisor according to menu instructions then exit.

## Development Environment

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Ensure you have this vesrion of the rust compiler installed: rustc 1.93.0

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [Official Rust Documentation](https://doc.rust-lang.org/stable/)
* [Tech With Tim Rust Tutorial](https://www.youtube.com/watch?v=T_KrYLW4jw8)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* Extend functionality to handle better error handling and rust features.
* Extend to handle all forms of polynomial long division.
* Create parsing software to smooth out inputs.
* Extend to some sort of web or LaTeX interface to make it look prettier.
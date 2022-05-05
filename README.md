# Rust Tips and Tricks
This is the repository for the LinkedIn Learning course Rust Tips and Tricks. The full course is available from [LinkedIn Learning][lil-course-url].

![Rust Tips and Tricks][lil-thumbnail-url] 

Over the last few years, tech industry giants like Google, AWS, Huawei, and Microsoft have all become increasingly interested in Rust. As a result, Rust is now one of the most important programming languages for your toolbox, especially if you’re in the early stages of your career as a developer. In this course, instructor Marcus Willock walks you through his favorite tips and tricks to get the most out of the Rust experience.

Discover the versatility and performance power of Rust, exploring some of the lesser-known features of the Rust Standard Library. Learn how to boost your productivity with these beginner-friendly tools as you build your speed and confidence, and upskill as a developer. Along the way, Marcus gives step-by-step lessons for using macros such as todo, unreachable, dbg, and env, as well as Option types, release builds, derivable traits, and more.

## Instructions
This repository has branches for each of the videos in the course. You can use the branch pop up menu in github to switch to a specific branch and take a look at the course at that stage, or you can add `/tree/BRANCH_NAME` to the URL to go to the branch you want to access.

## Branches
The branches are structured to correspond to the videos in the course. The naming convention is `CHAPTER#_MOVIE#`. As an example, the branch named `02_03` corresponds to the second chapter and the third video in that chapter. 
Some branches will have a beginning and an end state. These are marked with the letters `b` for "beginning" and `e` for "end". The `b` branch contains the code as it is at the beginning of the movie. The `e` branch contains the code as it is at the end of the movie. The `main` branch holds the final state of the code when in the course.

When switching from one exercise files branch to the next after making changes to the files, you may get a message like this:

    error: Your local changes to the following files would be overwritten by checkout:        [files]
    Please commit your changes or stash them before you switch branches.
    Aborting

To resolve this issue:
	
    Add changes to git using this command: git add .
	Commit changes using this command: git commit -m "some message"

## Installing
1. Clone this repository into your local machine using the terminal (Mac), CMD (Windows), or a GUI tool like SourceTree.

### Instructor

Marcus Willock 
                            


                            

Check out my other courses on [LinkedIn Learning](https://www.linkedin.com/learning/instructors/marcus-willock).

[lil-course-url]: https://www.linkedin.com/learning/rust-tips-and-tricks
[lil-thumbnail-url]: https://cdn.lynda.com/course/3021162/3021162-1651520108194-16x9.jpg
